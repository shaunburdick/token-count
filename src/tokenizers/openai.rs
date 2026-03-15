//! OpenAI tokenization using tiktoken-rs

use crate::tokenizers::{ModelInfo, TokenDetail, Tokenizer};
use anyhow::{Context, Result};
use tiktoken_rs::CoreBPE;

/// OpenAI tokenizer using tiktoken-rs
pub struct OpenAITokenizer {
    bpe: CoreBPE,
    model_info: ModelInfo,
}

impl OpenAITokenizer {
    /// Create a new OpenAI tokenizer for the given encoding
    pub fn new(encoding_name: &str, model_info: ModelInfo) -> Result<Self> {
        let tokenizer_enum = match encoding_name {
            "o200k_base" => tiktoken_rs::tokenizer::Tokenizer::O200kBase,
            "cl100k_base" => tiktoken_rs::tokenizer::Tokenizer::Cl100kBase,
            "p50k_base" => tiktoken_rs::tokenizer::Tokenizer::P50kBase,
            "r50k_base" => tiktoken_rs::tokenizer::Tokenizer::R50kBase,
            "gpt2" => tiktoken_rs::tokenizer::Tokenizer::Gpt2,
            _ => anyhow::bail!("Unsupported encoding: {}", encoding_name),
        };

        let bpe = tiktoken_rs::get_bpe_from_tokenizer(tokenizer_enum)
            .with_context(|| format!("Failed to load encoding: {}", encoding_name))?;

        Ok(Self { bpe, model_info })
    }
}

impl Tokenizer for OpenAITokenizer {
    fn count_tokens(&self, text: &str) -> Result<usize> {
        let tokens = self.bpe.encode_with_special_tokens(text);
        Ok(tokens.len())
    }

    fn get_model_info(&self) -> ModelInfo {
        self.model_info.clone()
    }

    fn encode_with_details(&self, text: &str) -> Result<Option<Vec<TokenDetail>>> {
        // Skip detailed tokenization for inputs >50KB to prevent stack overflow
        // tiktoken-rs has known recursion depth issues with large inputs
        // See: https://github.com/zurawiki/tiktoken-rs/issues/327
        const MAX_DEBUG_INPUT_SIZE: usize = 50 * 1024; // 50KB safety limit

        if text.len() > MAX_DEBUG_INPUT_SIZE {
            eprintln!(
                "Warning: Input size ({} bytes) exceeds debug mode limit ({} bytes). \
                 Showing token count only. For token IDs, provide smaller input.",
                text.len(),
                MAX_DEBUG_INPUT_SIZE
            );
            return Ok(None);
        }

        let token_ids = self.bpe.encode_with_special_tokens(text);

        // Limit to first 10 tokens to avoid overwhelming output
        let mut details = Vec::new();
        for token_id in token_ids.iter().take(10) {
            // Decode individual token
            let decoded = self.bpe.decode(vec![*token_id])?;
            details.push(TokenDetail { id: *token_id, text: decoded });
        }

        Ok(Some(details))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let model_info = ModelInfo {
            name: "gpt-4".to_string(),
            encoding: "cl100k_base".to_string(),
            context_window: 128000,
            description: "GPT-4 model".to_string(),
        };

        let tokenizer = OpenAITokenizer::new("cl100k_base", model_info).unwrap();
        let count = tokenizer.count_tokens("Hello world").unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_empty_string() {
        let model_info = ModelInfo {
            name: "gpt-4".to_string(),
            encoding: "cl100k_base".to_string(),
            context_window: 128000,
            description: "GPT-4 model".to_string(),
        };

        let tokenizer = OpenAITokenizer::new("cl100k_base", model_info).unwrap();
        let count = tokenizer.count_tokens("").unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_encode_with_details_large_input() {
        let model_info = ModelInfo {
            name: "gpt-4".to_string(),
            encoding: "cl100k_base".to_string(),
            context_window: 128000,
            description: "GPT-4 model".to_string(),
        };

        let tokenizer = OpenAITokenizer::new("cl100k_base", model_info).unwrap();

        // Create a 60KB input (exceeds 50KB limit)
        let large_input = "a".repeat(60 * 1024);
        let result = tokenizer.encode_with_details(&large_input);

        // Should return None gracefully, not panic
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_encode_with_details_normal_input() {
        let model_info = ModelInfo {
            name: "gpt-4".to_string(),
            encoding: "cl100k_base".to_string(),
            context_window: 128000,
            description: "GPT-4 model".to_string(),
        };

        let tokenizer = OpenAITokenizer::new("cl100k_base", model_info).unwrap();

        // Normal input should work fine
        let result = tokenizer.encode_with_details("Hello world");
        assert!(result.is_ok());

        let details = result.unwrap();
        assert!(details.is_some());

        let details = details.unwrap();
        assert_eq!(details.len(), 2); // "Hello" + " world"
        assert_eq!(details[0].id, 9906); // "Hello"
        assert_eq!(details[1].id, 1917); // " world"
    }
}
