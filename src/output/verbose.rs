//! Verbose formatter - outputs detailed information

use crate::output::OutputFormatter;
use crate::tokenizers::TokenizationResult;

/// Verbose formatter that outputs model info and context window percentage
pub struct VerboseFormatter;

impl OutputFormatter for VerboseFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        // Safe conversion with overflow and division-by-zero protection
        let percentage = if result.model_info.context_window == 0 {
            0.0 // Prevent division by zero
        } else {
            let token_count_f64 = result.token_count as f64;
            let context_window_f64 = result.model_info.context_window as f64;

            // Calculate percentage with overflow check
            let raw_percentage = (token_count_f64 / context_window_f64) * 100.0;
            if raw_percentage.is_finite() {
                raw_percentage
            } else {
                f64::MAX // Saturate to maximum representable value
            }
        };

        format!(
            "Model: {} ({})\nTokens: {}\nContext window: {} tokens ({:.4}% used)",
            result.model_info.name,
            result.model_info.encoding,
            result.token_count,
            result.model_info.context_window,
            percentage
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizers::ModelInfo;

    #[test]
    fn test_verbose_formatter() {
        let formatter = VerboseFormatter;
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
        assert!(output.contains("Context window: 128000"));
        assert!(output.contains("%"));
    }

    #[test]
    fn test_percentage_calculation() {
        let formatter = VerboseFormatter;
        let result = TokenizationResult {
            token_count: 64000,
            model_info: ModelInfo {
                name: "gpt-4".to_string(),
                encoding: "cl100k_base".to_string(),
                context_window: 128000,
                description: "GPT-4".to_string(),
            },
        };

        let output = formatter.format(&result);
        assert!(output.contains("50.0000%"));
    }
}
