//! Tokenizer implementations for various LLM models
//!
//! This module provides the core tokenization functionality for supported LLM models.
//!
//! # Architecture
//!
//! The tokenization system uses a trait-based design for extensibility:
//!
//! - [`Tokenizer`] - Trait for all tokenizer implementations
//! - [`openai::OpenAITokenizer`] - OpenAI model tokenizer using tiktoken
//! - [`registry::ModelRegistry`] - Registry of supported models with lazy initialization
//!
//! # Example
//!
//! ```
//! use token_count::tokenizers::registry::ModelRegistry;
//!
//! // Get the global model registry
//! let registry = ModelRegistry::global();
//!
//! // Get a tokenizer for a specific model
//! let tokenizer = registry.get_tokenizer("gpt-4", false).unwrap();
//!
//! // Count tokens
//! let count = tokenizer.count_tokens("Hello world").unwrap();
//! assert_eq!(count, 2);
//!
//! // Get model information
//! let info = tokenizer.get_model_info();
//! assert_eq!(info.name, "gpt-4");
//! assert_eq!(info.encoding, "cl100k_base");
//! ```
//!
//! # Supported Models
//!
//! Currently supports:
//! - OpenAI models: GPT-3.5 Turbo, GPT-4, GPT-4 Turbo, GPT-4o
//! - Claude models: Claude 4.0-4.6 (Opus, Sonnet, Haiku variants)
//!
//! See [`registry::ModelRegistry`] for model configuration and aliases.

pub mod claude;
pub mod google;
pub mod openai;
pub mod registry;

use std::fmt;

/// Result of token counting, indicating whether count is estimated or exact
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenCount {
    /// Estimated count using heuristics (displays with ~ prefix)
    Estimated(usize),

    /// Exact count from official API (displays without prefix)
    Exact(usize),
}

impl TokenCount {
    /// Get the numeric value regardless of estimation status
    pub fn value(&self) -> usize {
        match self {
            Self::Estimated(n) | Self::Exact(n) => *n,
        }
    }

    /// Check if this count is estimated
    pub fn is_estimated(&self) -> bool {
        matches!(self, Self::Estimated(_))
    }

    /// Check if this count is exact
    pub fn is_exact(&self) -> bool {
        matches!(self, Self::Exact(_))
    }
}

impl fmt::Display for TokenCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Estimated(n) => write!(f, "~{}", n),
            Self::Exact(n) => write!(f, "{}", n),
        }
    }
}

/// Detailed information about a single token
#[derive(Debug, Clone)]
pub struct TokenDetail {
    /// The token ID
    pub id: u32,
    /// The decoded text representation of this token
    pub text: String,
}

/// Trait for tokenizing text with a specific model
pub trait Tokenizer: Send + Sync {
    /// Count the number of tokens in the given text
    fn count_tokens(&self, text: &str) -> anyhow::Result<usize>;

    /// Get information about the model
    fn get_model_info(&self) -> ModelInfo;

    /// Encode text and return detailed token information (optional, for debug mode)
    ///
    /// Returns `None` if the tokenizer doesn't support detailed tokenization.
    /// This is used for debug output (`-vvv` flag).
    fn encode_with_details(&self, _text: &str) -> anyhow::Result<Option<Vec<TokenDetail>>> {
        Ok(None)
    }
}

/// Information about a tokenization model
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub encoding: String,
    pub context_window: usize,
    pub description: String,
}

impl fmt::Display for ModelInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.encoding)
    }
}

/// Result of tokenization operation
#[derive(Debug, Clone)]
pub struct TokenizationResult {
    pub token_count: usize,
    pub model_info: ModelInfo,
    /// Optional detailed token information (for debug mode)
    pub token_details: Option<Vec<TokenDetail>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_count_display_estimated() {
        let count = TokenCount::Estimated(42);
        assert_eq!(format!("{}", count), "~42");
    }

    #[test]
    fn test_token_count_display_exact() {
        let count = TokenCount::Exact(42);
        assert_eq!(format!("{}", count), "42");
    }

    #[test]
    fn test_token_count_value() {
        assert_eq!(TokenCount::Estimated(42).value(), 42);
        assert_eq!(TokenCount::Exact(42).value(), 42);
    }

    #[test]
    fn test_token_count_is_estimated() {
        assert!(TokenCount::Estimated(42).is_estimated());
        assert!(!TokenCount::Exact(42).is_estimated());
    }

    #[test]
    fn test_token_count_is_exact() {
        assert!(!TokenCount::Estimated(42).is_exact());
        assert!(TokenCount::Exact(42).is_exact());
    }

    #[test]
    fn test_token_count_equality() {
        assert_eq!(TokenCount::Estimated(42), TokenCount::Estimated(42));
        assert_eq!(TokenCount::Exact(42), TokenCount::Exact(42));
        assert_ne!(TokenCount::Estimated(42), TokenCount::Exact(42));
    }
}
