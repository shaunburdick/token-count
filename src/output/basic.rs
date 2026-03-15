//! Basic formatter - outputs model info and token count without percentage

use crate::output::OutputFormatter;
use crate::tokenizers::TokenizationResult;

/// Basic formatter that outputs model info and token count (no percentage)
pub struct BasicFormatter;

impl OutputFormatter for BasicFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        format!(
            "Model: {} ({})\nTokens: {}",
            result.model_info.name, result.model_info.encoding, result.token_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizers::ModelInfo;

    #[test]
    fn test_basic_formatter() {
        let formatter = BasicFormatter;
        let result = TokenizationResult {
            token_count: 2,
            model_info: ModelInfo {
                name: "gpt-4".to_string(),
                encoding: "cl100k_base".to_string(),
                context_window: 128000,
                description: "GPT-4".to_string(),
            },
            token_details: None,
        };

        let output = formatter.format(&result);
        assert!(output.contains("Model: gpt-4"));
        assert!(output.contains("Tokens: 2"));
        assert!(!output.contains("%"), "Should not contain percentage");
        assert!(!output.contains("Context window"), "Should not contain context window");
    }
}
