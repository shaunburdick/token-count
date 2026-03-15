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
}
