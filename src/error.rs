//! Error types for token counting operations

use thiserror::Error;

/// Errors that can occur during token counting
#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Input contains invalid UTF-8 at byte {offset}")]
    InvalidUtf8 { offset: usize },

    #[error("Unknown model: '{model}'. {suggestion}")]
    UnknownModel { model: String, suggestion: String },

    #[error("Input size ({size} bytes) exceeds maximum limit ({limit} bytes). Consider processing in smaller chunks.")]
    InputTooLarge { size: usize, limit: usize },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Tokenization error: {0}")]
    Tokenization(String),

    /// API key not found in environment
    #[error(
        "Accurate mode requires ANTHROPIC_API_KEY environment variable.\n\n\
         Get your API key from: https://console.anthropic.com/\n\
         Then set: export ANTHROPIC_API_KEY=\"sk-ant-...\"\n\n\
         For offline estimation (no API key needed), omit --accurate flag:\n  \
         token-count --model {model}"
    )]
    MissingApiKey { model: String },

    /// Invalid API key (401 response)
    #[error(
        "Invalid ANTHROPIC_API_KEY. Please check your API key.\n\n\
         Get a valid key from: https://console.anthropic.com/"
    )]
    InvalidApiKey,

    /// Rate limit exceeded (429 response)
    #[error(
        "Anthropic API rate limit exceeded. Please try again later.\n\n\
         Rate limits: https://docs.anthropic.com/en/api/rate-limits"
    )]
    RateLimited,

    /// API server error (5xx response)
    #[error(
        "Anthropic API server error (HTTP {0}). The service may be temporarily unavailable.\n\n\
         Check status: https://status.anthropic.com/"
    )]
    ApiServerError(u16),

    /// Generic API error
    #[error("Anthropic API error: {0}")]
    ApiError(String),

    /// Non-interactive mode without --yes flag
    #[error(
        "API call requires consent. Running in non-interactive mode (stdin not a TTY).\n\n\
         Options:\n  \
         1. Add -y/--yes flag to skip prompt:\n     \
            cat file.txt | token-count --model {model} --accurate -y\n  \n  \
         2. Use estimation mode (no API call):\n     \
            cat file.txt | token-count --model {model}"
    )]
    NonInteractiveWithoutYes { model: String },
}

impl TokenError {
    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidUtf8 { .. } => 1,
            Self::UnknownModel { .. } => 2,
            Self::InputTooLarge { .. } => 1,
            Self::Io(_) => 1,
            Self::Tokenization(_) => 1,
            Self::MissingApiKey { .. } => 1,
            Self::InvalidApiKey => 1,
            Self::RateLimited => 1,
            Self::ApiServerError(_) => 1,
            Self::ApiError(_) => 1,
            Self::NonInteractiveWithoutYes { .. } => 1,
        }
    }
}
