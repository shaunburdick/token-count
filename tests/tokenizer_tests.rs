use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use token_count::tokenizers::registry::ModelRegistry;

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    input: String,
    expected_tokens: usize,
}

type Fixtures = HashMap<String, Vec<TestCase>>;

fn load_fixtures() -> Fixtures {
    let fixture_data = include_str!("fixtures/tokenization_reference.json");
    serde_json::from_str(fixture_data).expect("Failed to parse fixtures")
}

#[test]
fn test_cl100k_base_fixtures() {
    let fixtures = load_fixtures();
    let registry = ModelRegistry::global();

    let cl100k_cases = fixtures.get("cl100k_base").expect("cl100k_base fixtures not found");

    // Test with gpt-4 (uses cl100k_base)
    let tokenizer = registry.get_tokenizer("gpt-4", false).unwrap();

    for (i, test_case) in cl100k_cases.iter().enumerate() {
        let count = tokenizer
            .count_tokens(&test_case.input)
            .unwrap_or_else(|e| panic!("Test case {}: tokenization failed: {}", i, e));

        assert_eq!(
            count, test_case.expected_tokens,
            "Test case {} failed: input='{}', expected={}, got={}",
            i, test_case.input, test_case.expected_tokens, count
        );
    }
}

#[test]
fn test_o200k_base_fixtures() {
    let fixtures = load_fixtures();
    let registry = ModelRegistry::global();

    let o200k_cases = fixtures.get("o200k_base").expect("o200k_base fixtures not found");

    // Test with gpt-4o (uses o200k_base)
    let tokenizer = registry.get_tokenizer("gpt-4o", false).unwrap();

    for (i, test_case) in o200k_cases.iter().enumerate() {
        let count = tokenizer
            .count_tokens(&test_case.input)
            .unwrap_or_else(|e| panic!("Test case {}: tokenization failed: {}", i, e));

        assert_eq!(
            count, test_case.expected_tokens,
            "Test case {} failed: input='{}', expected={}, got={}",
            i, test_case.input, test_case.expected_tokens, count
        );
    }
}

#[test]
fn test_unicode_handling() {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gpt-4", false).unwrap();

    // From fixtures: "Hello 世界 🌍" should be 8 tokens
    let count = tokenizer.count_tokens("Hello 世界 🌍").unwrap();
    assert_eq!(count, 8, "Unicode tokenization failed");
}

#[test]
fn test_empty_string() {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gpt-4", false).unwrap();

    let count = tokenizer.count_tokens("").unwrap();
    assert_eq!(count, 0, "Empty string should have 0 tokens");
}

#[test]
fn test_all_models() {
    let registry = ModelRegistry::global();
    let models = ["gpt-3.5-turbo", "gpt-4", "gpt-4-turbo", "gpt-4o"];

    for model in &models {
        let tokenizer = registry
            .get_tokenizer(model, false)
            .unwrap_or_else(|e| panic!("Failed to get tokenizer for {}: {}", model, e));

        let count = tokenizer
            .count_tokens("Hello world")
            .unwrap_or_else(|e| panic!("Tokenization failed for {}: {}", model, e));

        assert_eq!(count, 2, "Model {} should tokenize 'Hello world' as 2 tokens", model);
    }
}
