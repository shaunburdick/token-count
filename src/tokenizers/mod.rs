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
//! let tokenizer = registry.get_tokenizer("gpt-4").unwrap();
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
//! Currently supports OpenAI models:
//! - GPT-3.5 Turbo (cl100k_base encoding)
//! - GPT-4 (cl100k_base encoding)
//! - GPT-4 Turbo (cl100k_base encoding)
//! - GPT-4o (o200k_base encoding)
//!
//! See [`registry::ModelRegistry`] for model configuration and aliases.

pub mod openai;
pub mod registry;

use std::fmt;

/// Trait for tokenizing text with a specific model
pub trait Tokenizer: Send + Sync {
    /// Count the number of tokens in the given text
    fn count_tokens(&self, text: &str) -> anyhow::Result<usize>;

    /// Get information about the model
    fn get_model_info(&self) -> ModelInfo;
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
}
