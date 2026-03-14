//! Tokenizer implementation for Google Gemini models

mod models;
mod tokenizer;

pub use models::google_models;

use crate::error::TokenError;
use crate::tokenizers::registry::ModelConfig;
use crate::tokenizers::{ModelInfo, Tokenizer};
use tokenizer::GeminiTokenizer;

/// Tokenizer for Google Gemini models
pub struct GoogleTokenizer {
    /// Underlying gemini-tokenizer wrapper
    gemini: GeminiTokenizer,

    /// Model configuration (name, context window, etc.)
    config: ModelConfig,
}

impl GoogleTokenizer {
    /// Create a new Google tokenizer
    ///
    /// # Arguments
    /// * `config` - Model configuration
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully created tokenizer
    /// * `Err(TokenError::Tokenization)` - Failed to initialize
    pub fn new(config: ModelConfig) -> Result<Self, TokenError> {
        let gemini = GeminiTokenizer::new(&config.name)?;
        Ok(Self { gemini, config })
    }
}

impl Tokenizer for GoogleTokenizer {
    fn count_tokens(&self, text: &str) -> anyhow::Result<usize> {
        self.gemini.count_tokens(text)
    }

    fn get_model_info(&self) -> ModelInfo {
        ModelInfo {
            name: self.config.name.clone(),
            encoding: self.config.encoding.clone(),
            context_window: self.config.context_window,
            description: self.config.description.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizers::Tokenizer;

    #[test]
    fn test_google_tokenizer_creation() {
        let config = google_models().into_iter().next().unwrap();
        let tokenizer = GoogleTokenizer::new(config);
        assert!(tokenizer.is_ok());
    }

    #[test]
    fn test_tokenizer_trait_implementation() {
        let config = google_models().into_iter().next().unwrap();
        let tokenizer = GoogleTokenizer::new(config).unwrap();

        // Test count_tokens
        let count = tokenizer.count_tokens("Hello").unwrap();
        assert!(count > 0);

        // Test get_model_info
        let info = tokenizer.get_model_info();
        assert_eq!(info.encoding, "gemini-gemma3");
    }
}
