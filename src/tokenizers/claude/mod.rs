//! Tokenizer implementation for Anthropic Claude models
//!
//! This module provides tokenization for Claude models using a hybrid approach:
//! - **Default**: Adaptive estimation based on content type (code vs. prose)
//! - **Optional**: Exact counting via Anthropic API (requires API key and --accurate flag)

mod api_client;
mod estimation;
mod models;

pub use api_client::ClaudeApiClient;
pub use estimation::{detect_content_type, estimate_tokens, ContentType};
pub use models::claude_models;

use crate::error::TokenError;
use crate::tokenizers::registry::ModelConfig;
use crate::tokenizers::{ModelInfo, Tokenizer};
use std::env;

/// Tokenizer for Anthropic Claude models
pub struct ClaudeTokenizer {
    /// Model configuration (name, context window, etc.)
    config: ModelConfig,

    /// Optional API client (only if --accurate flag set and API key available)
    api_client: Option<ClaudeApiClient>,
}

impl ClaudeTokenizer {
    /// Create a new Claude tokenizer
    ///
    /// # Arguments
    /// * `config` - Model configuration (name, context window, etc.)
    /// * `use_accurate` - Whether to use API for exact counts (--accurate flag)
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully created tokenizer
    /// * `Err(TokenError::MissingApiKey)` - API key required but not found
    pub fn new(config: ModelConfig, use_accurate: bool) -> Result<Self, TokenError> {
        let api_client = if use_accurate {
            // Check for API key
            match env::var("ANTHROPIC_API_KEY") {
                Ok(key) if !key.is_empty() => Some(ClaudeApiClient::new(key)?),
                _ => return Err(TokenError::MissingApiKey { model: config.name.clone() }),
            }
        } else {
            None
        };

        Ok(Self { config, api_client })
    }

    /// Count tokens using estimation or API (async helper)
    ///
    /// This method handles the hybrid approach:
    /// - If API client available, try API call with graceful fallback to estimation
    /// - Otherwise, use estimation
    async fn count_tokens_async(&self, text: &str) -> anyhow::Result<usize> {
        if let Some(client) = &self.api_client {
            // Try API, fall back to estimation on error
            match client.count_tokens(&self.config.name, text).await {
                Ok(count) => Ok(count),
                Err(e) => {
                    eprintln!("Warning: API call failed ({}), falling back to estimation", e);
                    Ok(estimate_tokens(text))
                }
            }
        } else {
            // Estimation mode
            Ok(estimate_tokens(text))
        }
    }
}

impl Tokenizer for ClaudeTokenizer {
    fn count_tokens(&self, text: &str) -> anyhow::Result<usize> {
        // Create a simple tokio runtime for the async API call
        // This is necessary because the Tokenizer trait is sync
        if self.api_client.is_some() {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
            runtime.block_on(self.count_tokens_async(text))
        } else {
            // Pure estimation (no async needed)
            Ok(estimate_tokens(text))
        }
    }

    fn get_model_info(&self) -> ModelInfo {
        ModelInfo {
            name: self.config.name.clone(),
            encoding: "anthropic-claude".to_string(),
            context_window: self.config.context_window,
            description: self.config.description.clone(),
        }
    }
}
