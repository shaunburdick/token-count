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
        }
    }
}
