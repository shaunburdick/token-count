//! Debug formatter - outputs token IDs and sample decoded tokens

use crate::output::OutputFormatter;
use crate::tokenizers::TokenizationResult;

/// Debug formatter that outputs token IDs and sample decoded tokens
pub struct DebugFormatter;

impl OutputFormatter for DebugFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        format!(
            "Model: {} ({})\nTokens: {}\nContext window: {} tokens\n\nNote: Detailed token ID output will be available in v0.2.0",
            result.model_info.name,
            result.model_info.encoding,
            result.token_count,
            result.model_info.context_window
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizers::ModelInfo;

    #[test]
    fn test_debug_formatter() {
        let formatter = DebugFormatter;
        let result = TokenizationResult {
            token_count: 2,
            model_info: ModelInfo {
                name: "gpt-4".to_string(),
                encoding: "cl100k_base".to_string(),
                context_window: 128000,
                description: "GPT-4".to_string(),
            },
        };

        let output = formatter.format(&result);
        assert!(output.contains("Model: gpt-4"));
        assert!(output.contains("Tokens: 2"));
        assert!(output.contains("v0.2.0"));
    }
}
