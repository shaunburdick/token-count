//! HTTP client for Anthropic API token counting
//!
//! This module provides an async HTTP client for calling Anthropic's
//! count_tokens API with retry logic and proper error handling.

use crate::error::TokenError;
use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HTTP client for Anthropic API token counting
pub struct ClaudeApiClient {
    client: reqwest::Client,
    api_key: String,
}

impl ClaudeApiClient {
    /// API endpoint for token counting
    const API_ENDPOINT: &'static str = "https://api.anthropic.com/v1/messages/count_tokens";

    /// API version header value
    const API_VERSION: &'static str = "2023-06-01";

    /// Request timeout in seconds
    const TIMEOUT_SECS: u64 = 30;

    /// Maximum retry attempts for transient failures
    const MAX_RETRIES: u32 = 3;

    /// Create a new API client with the given API key
    ///
    /// # Arguments
    /// * `api_key` - Anthropic API key (format: sk-ant-api03-...)
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully created client
    /// * `Err(TokenError)` - Failed to build HTTP client
    pub fn new(api_key: String) -> Result<Self, TokenError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(Self::TIMEOUT_SECS))
            .user_agent(format!("token-count/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| TokenError::ApiError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, api_key })
    }

    /// Count tokens via API with automatic retry logic
    ///
    /// Implements exponential backoff (2s, 4s, 8s) for transient failures.
    /// Retries on: 429 (rate limit), 500-599 (server errors), network errors.
    /// Does not retry on: 400, 401, 403 (client errors).
    ///
    /// # Arguments
    /// * `model` - Claude model ID (e.g., "claude-sonnet-4-6")
    /// * `text` - Text to count tokens for
    ///
    /// # Returns
    /// * `Ok(usize)` - Token count from API
    /// * `Err(TokenError)` - API error or network failure
    pub async fn count_tokens(&self, model: &str, text: &str) -> Result<usize, TokenError> {
        let mut attempts = 0;

        loop {
            match self.try_count_tokens(model, text).await {
                Ok(count) => return Ok(count),
                Err(e) if Self::is_retryable(&e) && attempts < Self::MAX_RETRIES - 1 => {
                    let backoff_ms = 2u64.pow(attempts) * 1000; // 2s, 4s, 8s
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    attempts += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Check if an error is retryable
    ///
    /// Retryable errors: RateLimited, ApiServerError
    /// Non-retryable: InvalidApiKey, MissingApiKey, ApiError (client errors)
    fn is_retryable(error: &TokenError) -> bool {
        matches!(error, TokenError::RateLimited | TokenError::ApiServerError(_))
    }

    /// Single API request attempt (no retry logic)
    async fn try_count_tokens(&self, model: &str, text: &str) -> Result<usize, TokenError> {
        let request = CountTokensRequest {
            model: model.to_string(),
            messages: vec![Message { role: "user".to_string(), content: text.to_string() }],
        };

        let response = self
            .client
            .post(Self::API_ENDPOINT)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", Self::API_VERSION)
            .json(&request)
            .send()
            .await
            .map_err(|e| TokenError::ApiError(format!("Network error: {}", e)))?;

        let status = response.status();

        if !status.is_success() {
            return Err(Self::parse_api_error(status.as_u16(), response).await);
        }

        let body: CountTokensResponse = response
            .json()
            .await
            .map_err(|e| TokenError::ApiError(format!("Failed to parse response: {}", e)))?;

        Ok(body.input_tokens)
    }

    /// Parse API error response into appropriate TokenError
    async fn parse_api_error(status_code: u16, response: reqwest::Response) -> TokenError {
        // Try to parse error body for detailed message
        let error_msg = match response.json::<ApiErrorResponse>().await {
            Ok(err_resp) => format!("{}: {}", err_resp.error.error_type, err_resp.error.message),
            Err(_) => format!("HTTP {}", status_code),
        };

        match status_code {
            401 => TokenError::InvalidApiKey,
            429 => TokenError::RateLimited,
            500..=599 => TokenError::ApiServerError(status_code),
            _ => TokenError::ApiError(error_msg),
        }
    }
}

/// Request to Anthropic count_tokens API
#[derive(Debug, Serialize)]
struct CountTokensRequest {
    model: String,
    messages: Vec<Message>,
}

/// Message in API request
#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

/// Response from Anthropic count_tokens API (success)
#[derive(Debug, Deserialize)]
struct CountTokensResponse {
    input_tokens: usize,
}

/// API error response
#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    error: ApiError,
}

/// Error details from API
#[derive(Debug, Deserialize)]
struct ApiError {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success_response() {
        let json = r#"{"input_tokens": 42}"#;
        let response: CountTokensResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.input_tokens, 42);
    }

    #[test]
    fn test_parse_error_response() {
        let json = r#"{"error": {"type": "authentication_error", "message": "invalid key"}}"#;
        let response: ApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.error.error_type, "authentication_error");
        assert_eq!(response.error.message, "invalid key");
    }

    #[test]
    fn test_serialize_request() {
        let request = CountTokensRequest {
            model: "claude-sonnet-4-6".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello, Claude!".to_string(),
            }],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("claude-sonnet-4-6"));
        assert!(json.contains("Hello, Claude!"));
        assert!(json.contains("user"));
    }

    #[test]
    fn test_is_retryable() {
        assert!(ClaudeApiClient::is_retryable(&TokenError::RateLimited));
        assert!(ClaudeApiClient::is_retryable(&TokenError::ApiServerError(500)));
        assert!(!ClaudeApiClient::is_retryable(&TokenError::InvalidApiKey));
        assert!(!ClaudeApiClient::is_retryable(&TokenError::ApiError("test".to_string())));
    }

    #[test]
    fn test_client_creation() {
        let client = ClaudeApiClient::new("test-key".to_string());
        assert!(client.is_ok());
    }
}
