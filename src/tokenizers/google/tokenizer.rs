//! Wrapper around gemini-tokenizer crate

use crate::error::TokenError;
use anyhow::Result;
use gemini_tokenizer::LocalTokenizer;

/// Wrapper around gemini-tokenizer's LocalTokenizer
///
/// Provides a simplified interface for token counting.
pub struct GeminiTokenizer {
    tokenizer: LocalTokenizer,
}

impl GeminiTokenizer {
    /// Create a new Gemini tokenizer
    ///
    /// # Arguments
    /// * `model_name` - Any Gemini model name (all use same tokenizer)
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully initialized tokenizer
    /// * `Err(TokenError::Tokenization)` - Failed to initialize
    pub fn new(model_name: &str) -> Result<Self, TokenError> {
        let tokenizer = LocalTokenizer::new(model_name).map_err(|e| {
            TokenError::Tokenization(format!("Failed to initialize Gemini tokenizer: {}", e))
        })?;

        Ok(Self { tokenizer })
    }

    /// Count tokens in the given text
    ///
    /// # Arguments
    /// * `text` - Input text to tokenize
    ///
    /// # Returns
    /// * `Ok(usize)` - Total token count
    /// * `Err(anyhow::Error)` - Tokenization failed
    pub fn count_tokens(&self, text: &str) -> Result<usize> {
        let result = self.tokenizer.count_tokens(text, None);
        Ok(result.total_tokens)
    }

    /// Get detailed token information (IDs and decoded text)
    ///
    /// # Arguments
    /// * `text` - Input text to tokenize
    ///
    /// # Returns
    /// * `Ok(Vec<(u32, String)>)` - Vec of (token_id, decoded_text) pairs
    /// * `Err(anyhow::Error)` - Tokenization failed
    pub fn compute_token_details(&self, text: &str) -> Result<Vec<(u32, String)>> {
        let result = self.tokenizer.compute_tokens(text);

        let mut details = Vec::new();
        let mut total_tokens = 0;

        // Extract token IDs and decoded text from tokens_info
        for info in &result.tokens_info {
            // Limit to first 10 tokens total
            for (token_id, token_bytes) in info.token_ids.iter().zip(&info.tokens) {
                if total_tokens >= 10 {
                    return Ok(details);
                }

                // Convert bytes to UTF-8 string
                let text = String::from_utf8_lossy(token_bytes).to_string();
                details.push((*token_id, text));
                total_tokens += 1;
            }
        }

        Ok(details)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_initialization() {
        let tokenizer = GeminiTokenizer::new("gemini-2.5-flash");
        if let Err(e) = &tokenizer {
            eprintln!("Tokenizer initialization error: {:?}", e);
        }
        assert!(tokenizer.is_ok());
    }

    #[test]
    fn test_count_tokens() {
        let tokenizer = GeminiTokenizer::new("gemini-2.5-flash").unwrap();
        let count = tokenizer.count_tokens("Hello, Gemini!").unwrap();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_empty_string() {
        let tokenizer = GeminiTokenizer::new("gemini-2.5-flash").unwrap();
        let count = tokenizer.count_tokens("").unwrap();
        assert_eq!(count, 0);
    }
}
