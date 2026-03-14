//! Tokenizer implementation for Anthropic Claude models
//!
//! This module provides tokenization for Claude models using a hybrid approach:
//! - **Default**: Adaptive estimation based on content type (code vs. prose)
//! - **Optional**: Exact counting via Anthropic API (requires API key and --accurate flag)

mod estimation;
mod models;

pub use estimation::{detect_content_type, estimate_tokens, ContentType};
pub use models::claude_models;

use crate::error::TokenError;
use crate::tokenizers::registry::ModelConfig;
use crate::tokenizers::{ModelInfo, Tokenizer};

/// Tokenizer for Anthropic Claude models
pub struct ClaudeTokenizer {
    /// Model configuration (name, context window, etc.)
    config: ModelConfig,

    /// Whether to use accurate mode (--accurate flag)
    use_accurate: bool,
}

impl ClaudeTokenizer {
    /// Create a new Claude tokenizer
    pub fn new(config: ModelConfig, use_accurate: bool) -> Result<Self, TokenError> {
        // For now, API client will be added in Phase 3
        if use_accurate {
            return Err(TokenError::MissingApiKey { model: config.name.clone() });
        }

        Ok(Self { config, use_accurate })
    }
}

impl Tokenizer for ClaudeTokenizer {
    fn count_tokens(&self, text: &str) -> anyhow::Result<usize> {
        // Phase 2: Only estimation supported
        // Phase 3 will add API client when self.use_accurate == true
        if self.use_accurate {
            // This should never happen due to check in new(), but be defensive
            return Err(TokenError::MissingApiKey { model: self.config.name.clone() }.into());
        }

        Ok(estimate_tokens(text))
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
