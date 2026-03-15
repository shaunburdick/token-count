use token_count::output::{
    BasicFormatter, DebugFormatter, OutputFormatter, SimpleFormatter, VerboseFormatter,
};
use token_count::tokenizers::{ModelInfo, TokenizationResult};

fn create_test_result() -> TokenizationResult {
    TokenizationResult {
        token_count: 2,
        model_info: ModelInfo {
            name: "gpt-4".to_string(),
            encoding: "cl100k_base".to_string(),
            context_window: 128000,
            description: "GPT-4 model".to_string(),
        },
        token_details: None,
    }
}

#[test]
fn test_simple_formatter_output() {
    let formatter = SimpleFormatter;
    let result = create_test_result();
    let output = formatter.format(&result);

    assert_eq!(output, "2");
    assert!(!output.contains('\n'));
}

#[test]
fn test_basic_formatter_output() {
    let formatter = BasicFormatter;
    let result = create_test_result();
    let output = formatter.format(&result);

    assert!(output.contains("Model: gpt-4"));
    assert!(output.contains("cl100k_base"));
    assert!(output.contains("Tokens: 2"));
    assert!(!output.contains("Context window"), "Should not contain context window");
    assert!(!output.contains('%'), "Should not contain percentage");

    // Should be multi-line
    assert!(output.contains('\n'));
}

#[test]
fn test_verbose_formatter_output() {
    let formatter = VerboseFormatter;
    let result = create_test_result();
    let output = formatter.format(&result);

    assert!(output.contains("Model: gpt-4"));
    assert!(output.contains("cl100k_base"));
    assert!(output.contains("Tokens: 2"));
    assert!(output.contains("Context window: 128000"));
    assert!(output.contains('%'));

    // Should be multi-line
    assert!(output.contains('\n'));
}

#[test]
fn test_debug_formatter_output() {
    let formatter = DebugFormatter;
    let result = create_test_result();
    let output = formatter.format(&result);

    assert!(output.contains("Model: gpt-4"));
    assert!(output.contains("Tokens: 2"));
    assert!(output.contains("Token IDs not available"));

    // Should be multi-line
    assert!(output.contains('\n'));
}

#[test]
fn test_formatter_selection() {
    let result = create_test_result();

    // Verbosity 0 -> Simple
    let f0 = token_count::output::select_formatter(0);
    let out0 = f0.format(&result);
    assert_eq!(out0, "2");

    // Verbosity 1 -> Basic (no percentage)
    let f1 = token_count::output::select_formatter(1);
    let out1 = f1.format(&result);
    assert!(out1.contains("Model:"));
    assert!(!out1.contains('%'), "Level 1 should not have percentage");

    // Verbosity 2 -> Verbose (with percentage)
    let f2 = token_count::output::select_formatter(2);
    let out2 = f2.format(&result);
    assert!(out2.contains("Model:"));
    assert!(out2.contains('%'), "Level 2 should have percentage");

    // Verbosity 3+ -> Debug
    let f3 = token_count::output::select_formatter(3);
    let out3 = f3.format(&result);
    assert!(out3.contains("Token IDs not available"));
}
