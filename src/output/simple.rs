//! Simple formatter - outputs just the token count

use crate::output::OutputFormatter;
use crate::tokenizers::TokenizationResult;

/// Simple formatter that outputs only the token count
pub struct SimpleFormatter;

impl OutputFormatter for SimpleFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        result.token_count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizers::ModelInfo;

    #[test]
    fn test_simple_formatter() {
        let formatter = SimpleFormatter;
        let result = TokenizationResult {
            token_count: 42,
            model_info: ModelInfo {
                name: "gpt-4".to_string(),
                encoding: "cl100k_base".to_string(),
                context_window: 128000,
                description: "GPT-4".to_string(),
            },
        };

        let output = formatter.format(&result);
        assert_eq!(output, "42");
    }

    #[test]
    fn test_zero_tokens() {
        let formatter = SimpleFormatter;
        let result = TokenizationResult {
            token_count: 0,
            model_info: ModelInfo {
                name: "gpt-4".to_string(),
                encoding: "cl100k_base".to_string(),
                context_window: 128000,
                description: "GPT-4".to_string(),
            },
        };

        let output = formatter.format(&result);
        assert_eq!(output, "0");
    }
}
