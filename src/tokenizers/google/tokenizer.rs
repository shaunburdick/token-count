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
