//! Output formatting for different verbosity levels
//!
//! This module provides output formatters that display tokenization results
//! in different formats based on user preferences.
//!
//! # Formatters
//!
//! Three formatters are available via the [`OutputFormatter`] trait:
//!
//! - [`SimpleFormatter`] - Verbosity 0 (default): Just the token count
//! - [`VerboseFormatter`] - Verbosity 1-2: Model info and context window usage
//! - [`DebugFormatter`] - Verbosity 3+: Debug information for troubleshooting
//!
//! # Example
//!
//! ```
//! use token_count::output::{select_formatter, OutputFormatter};
//! use token_count::tokenizers::{TokenizationResult, ModelInfo};
//!
//! let result = TokenizationResult {
//!     token_count: 2,
//!     model_info: ModelInfo {
//!         name: "gpt-4".to_string(),
//!         encoding: "cl100k_base".to_string(),
//!         context_window: 128000,
//!         description: "GPT-4".to_string(),
//!     },
//!     token_details: None,
//! };
//!
//! // Simple output (verbosity 0)
//! let simple = select_formatter(0);
//! assert_eq!(simple.format(&result), "2");
//!
//! // Verbose output (verbosity 1)
//! let verbose = select_formatter(1);
//! let output = verbose.format(&result);
//! assert!(output.contains("Model: gpt-4"));
//! assert!(output.contains("Tokens: 2"));
//! ```
//!
//! # Strategy Pattern
//!
//! The formatters use the Strategy pattern, allowing easy extension with
//! new output formats without modifying existing code. To add a new formatter:
//!
//! 1. Create a new struct (e.g., `JsonFormatter`)
//! 2. Implement the [`OutputFormatter`] trait
//! 3. Update [`select_formatter`] to return it for the appropriate verbosity
//!
//! # Verbosity Levels
//!
//! | Level | Flag | Formatter | Output |
//! |-------|------|-----------|--------|
//! | 0 | (default) | Simple | `2` |
//! | 1-2 | `-v`, `-vv` | Verbose | Model info + context % |
//! | 3+ | `-vvv` | Debug | Diagnostic information |

pub mod debug;
pub mod simple;
pub mod verbose;

pub use debug::DebugFormatter;
pub use simple::SimpleFormatter;
pub use verbose::VerboseFormatter;

use crate::tokenizers::{ModelInfo, TokenizationResult};

/// Trait for formatting tokenization output
pub trait OutputFormatter {
    /// Format the tokenization result as a string
    fn format(&self, result: &TokenizationResult) -> String;
}

/// Select the appropriate formatter based on verbosity level
///
/// - 0: Simple (number only)
/// - 1-2: Verbose (model info, context window percentage)
/// - 3+: Debug (token IDs, decoded tokens)
pub fn select_formatter(verbosity: u8) -> Box<dyn OutputFormatter> {
    match verbosity {
        0 => Box::new(SimpleFormatter),
        1 | 2 => Box::new(VerboseFormatter),
        _ => Box::new(DebugFormatter),
    }
}

/// Extended tokenization result with token details for debug mode
///
/// Reserved for future enhancement in v0.2.0 when full token ID
/// decoding will be implemented in the debug formatter.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DetailedResult {
    pub token_count: usize,
    pub model_info: ModelInfo,
    pub token_ids: Vec<usize>,
    pub sample_tokens: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_selection() {
        let f0 = select_formatter(0);
        let f1 = select_formatter(1);
        let f2 = select_formatter(2);
        let f3 = select_formatter(3);

        // Just ensure they're different types (compilation test)
        drop(f0);
        drop(f1);
        drop(f2);
        drop(f3);
    }
}
