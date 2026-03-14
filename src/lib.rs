//! Token counting library for LLM models
//!
//! This library provides exact tokenization for various LLM models using their official tokenizers.
//!
//! # Features
//!
//! - **Exact tokenization** for OpenAI models (GPT-3.5, GPT-4, GPT-4 Turbo, GPT-4o)
//! - **Model aliases** with case-insensitive matching
//! - **Fuzzy suggestions** for typos and unknown models
//! - **Zero runtime dependencies** - all tokenizers embedded
//! - **Fast and efficient** - ~2.7µs for small inputs
//!
//! # Quick Start
//!
//! ```
//! use token_count::count_tokens;
//!
//! // Count tokens for a specific model
//! let result = count_tokens("Hello world", "gpt-4", false).unwrap();
//! assert_eq!(result.token_count, 2);
//! println!("Tokens: {}", result.token_count);
//! println!("Model: {}", result.model_info.name);
//! ```
//!
//! # Supported Models
//!
//! - `gpt-3.5-turbo` - GPT-3.5 Turbo (16K context)
//! - `gpt-4` - GPT-4 (128K context)
//! - `gpt-4-turbo` - GPT-4 Turbo (128K context)
//! - `gpt-4o` - GPT-4o (128K context)
//!
//! All models support aliases (e.g., `gpt4`, `GPT-4`, `openai/gpt-4`).
//!
//! # Error Handling
//!
//! ```
//! use token_count::{count_tokens, TokenError};
//!
//! // Unknown model returns an error with suggestions
//! match count_tokens("test", "gpt-5", false) {
//!     Ok(_) => panic!("Should have failed"),
//!     Err(TokenError::UnknownModel { model, suggestion }) => {
//!         assert_eq!(model, "gpt-5");
//!         assert!(suggestion.contains("Did you mean"));
//!     }
//!     Err(_) => panic!("Wrong error type"),
//! }
//! ```
//!
//! # Architecture
//!
//! The library is organized into several modules:
//!
//! - [`tokenizers`] - Core tokenization engine and model registry
//! - [`output`] - Output formatting (simple, verbose, debug)
//! - [`cli`] - Command-line interface components
//! - [`error`] - Error types and handling
//! - [`api`] - API integration utilities (consent prompts, etc.)
//!
//! The main entry point is the [`count_tokens`] function, which takes text and a model name
//! and returns a [`TokenizationResult`] with the token count and model information.

pub mod api;
pub mod cli;
pub mod error;
pub mod output;
pub mod tokenizers;

pub use error::TokenError;
pub use output::{select_formatter, OutputFormatter};
pub use tokenizers::{ModelInfo, TokenizationResult, Tokenizer};

/// Count tokens in the given text using the specified model
///
/// # Arguments
///
/// * `text` - The text to tokenize
/// * `model_name` - The model to use for tokenization (e.g., "gpt-4", "claude-sonnet-4-6")
/// * `accurate` - Whether to use accurate mode for models that support it (Claude API)
///
/// # Returns
///
/// A `Result` containing the token count and model information
///
/// # Errors
///
/// Returns `TokenError` if:
/// - The model is not supported
/// - The tokenizer fails to initialize
/// - Token counting fails
///
/// # Example
///
/// ```
/// use token_count::count_tokens;
///
/// let result = count_tokens("Hello world", "gpt-4", false).unwrap();
/// assert_eq!(result.token_count, 2);
/// ```
pub fn count_tokens(
    text: &str,
    model_name: &str,
    accurate: bool,
) -> Result<TokenizationResult, TokenError> {
    use tokenizers::registry::ModelRegistry;

    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer(model_name, accurate)?;

    let token_count =
        tokenizer.count_tokens(text).map_err(|e| TokenError::Tokenization(e.to_string()))?;
    let model_info = tokenizer.get_model_info();

    Ok(TokenizationResult { token_count, model_info })
}
