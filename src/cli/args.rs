//! Command-line argument parsing

use clap::Parser;

/// Count tokens for LLM models using exact tokenization
#[derive(Parser, Debug)]
#[command(name = "token-count")]
#[command(version, about)]
#[command(after_help = "\
EXAMPLES:
    echo \"Hello world\" | token-count --model gpt-4
    token-count --model gpt-4 < file.txt
    token-count --list-models
")]
pub struct Cli {
    /// Model to use for tokenization (use --list-models to see all)
    #[arg(short, long, default_value = "gpt-3.5-turbo")]
    pub model: String,

    /// Increase output verbosity (-v, -vv, -vvv for debug)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// List all supported models and exit
    #[arg(long)]
    pub list_models: bool,
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Normalize model name (lowercase, trim whitespace)
    pub fn normalized_model(&self) -> String {
        self.model.trim().to_lowercase()
    }

    /// Get verbosity level (0-3)
    pub fn verbosity_level(&self) -> u8 {
        self.verbose.min(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_model() {
        let cli = Cli::parse_from(["token-count"]);
        assert_eq!(cli.model, "gpt-3.5-turbo");
        assert_eq!(cli.verbose, 0);
        assert!(!cli.list_models);
    }

    #[test]
    fn test_model_normalization() {
        let cli = Cli::parse_from(["token-count", "--model", "  GPT-4  "]);
        assert_eq!(cli.normalized_model(), "gpt-4");
    }

    #[test]
    fn test_verbosity_levels() {
        let cli1 = Cli::parse_from(["token-count", "-v"]);
        assert_eq!(cli1.verbosity_level(), 1);

        let cli2 = Cli::parse_from(["token-count", "-vv"]);
        assert_eq!(cli2.verbosity_level(), 2);

        let cli3 = Cli::parse_from(["token-count", "-vvv"]);
        assert_eq!(cli3.verbosity_level(), 3);

        let cli4 = Cli::parse_from(["token-count", "-vvvv"]);
        assert_eq!(cli4.verbosity_level(), 3); // Capped at 3
    }

    #[test]
    fn test_list_models_flag() {
        let cli = Cli::parse_from(["token-count", "--list-models"]);
        assert!(cli.list_models);
    }
}
