//! Debug formatter - outputs token IDs and sample decoded tokens

use crate::output::OutputFormatter;
use crate::tokenizers::TokenizationResult;

/// Debug formatter that outputs token IDs and sample decoded tokens
pub struct DebugFormatter;

impl OutputFormatter for DebugFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        let percentage =
            (result.token_count as f64 / result.model_info.context_window as f64) * 100.0;

        let mut output = format!(
            "Model: {} ({})\nTokens: {}\nContext window: {} tokens ({:.4}% used)\n",
            result.model_info.name,
            result.model_info.encoding,
            result.token_count,
            result.model_info.context_window,
            percentage
        );

        // Add token details if available
        if let Some(details) = &result.token_details {
            if details.is_empty() {
                output.push_str("\nNo tokens to display (empty input)");
            } else {
                // Show token IDs
                let ids: Vec<String> = details.iter().map(|d| d.id.to_string()).collect();
                output.push_str(&format!("\nToken IDs: [{}]", ids.join(", ")));

                // Show decoded tokens
                output.push_str("\nDecoded tokens:");
                for (i, detail) in details.iter().enumerate() {
                    output.push_str(&format!("\n  [{}] {} → {:?}", i, detail.id, detail.text));
                }

                // Note if truncated
                if result.token_count > 10 {
                    output.push_str(&format!(
                        "\n\n(Showing first 10 of {} tokens)",
                        result.token_count
                    ));
                }
            }
        } else {
            output.push_str(
                "\n\nNote: Token IDs not available for this model (estimation-based tokenization)",
            );
        }

        output
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
            token_details: None,
        };

        let output = formatter.format(&result);
        assert!(output.contains("Model: gpt-4"));
        assert!(output.contains("Tokens: 2"));
        assert!(output.contains("Token IDs not available"));
    }
}
