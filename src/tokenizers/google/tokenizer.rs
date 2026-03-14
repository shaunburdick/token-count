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

    /// Get detailed token information for debug mode
    ///
    /// Returns token IDs and decoded tokens.
    #[allow(dead_code)]
    pub fn compute_tokens(&self, text: &str) -> Result<Vec<(u32, String)>> {
        let result = self.tokenizer.compute_tokens(text);

        let mut tokens = Vec::new();
        for info in result.tokens_info {
            for (id, token) in info.token_ids.iter().zip(&info.tokens) {
                // Tokens are Vec<u8>, convert to String (lossy for invalid UTF-8)
                let token_str = String::from_utf8_lossy(token).to_string();
                tokens.push((*id, token_str));
            }
        }

        Ok(tokens)
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

    #[test]
    fn test_compute_tokens() {
        let tokenizer = GeminiTokenizer::new("gemini-2.5-flash").unwrap();
        let tokens = tokenizer.compute_tokens("Hello").unwrap();
        assert!(!tokens.is_empty());
        assert!(tokens[0].0 > 0); // Token ID should be positive
        assert!(!tokens[0].1.is_empty()); // Token string should not be empty
    }
}
