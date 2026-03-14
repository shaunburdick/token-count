//! Interactive consent prompt for API calls
//!
//! This module provides a reusable consent mechanism for prompting users
//! before sending data to external APIs. It includes TTY detection to
//! prevent hanging in non-interactive environments.

use crate::error::TokenError;
use std::io::{self, IsTerminal, Write};

/// Configuration for API consent prompt
pub struct ConsentPrompt {
    /// Provider name (e.g., "Anthropic", "OpenAI")
    pub provider: &'static str,

    /// API endpoint URL (for transparency)
    pub api_endpoint: &'static str,
}

impl ConsentPrompt {
    /// Ask user for consent (interactive mode only)
    ///
    /// Returns `Ok(true)` if user consents, `Ok(false)` if user declines.
    /// Returns `Err` if running in non-interactive mode (stdin not a TTY).
    pub fn ask(&self) -> Result<bool, TokenError> {
        // Check if stdin is a TTY (terminal)
        if !io::stdin().is_terminal() {
            return Err(TokenError::NonInteractiveWithoutYes {
                model: "claude".to_string(), // Default model for error message
            });
        }

        // Display prompt on stderr (don't pollute stdout)
        eprintln!();
        eprintln!(
            "This will send your input to {}'s API for accurate token counting.",
            self.provider
        );
        eprintln!("Your input will be transmitted over HTTPS to: {}", self.api_endpoint);
        eprintln!();
        eprint!("Proceed with API call? (y/N): ");

        // Flush stderr to ensure prompt is visible
        io::stderr().flush().map_err(|e| {
            TokenError::Io(io::Error::other(format!("Failed to flush stderr: {}", e)))
        })?;

        // Read user response from stdin
        let mut response = String::new();
        io::stdin().read_line(&mut response).map_err(TokenError::Io)?;

        // Accept: y, Y, yes, YES (case-insensitive)
        // Reject: n, N, no, NO, empty, anything else
        let normalized = response.trim().to_lowercase();
        Ok(normalized == "y" || normalized == "yes")
    }
}
