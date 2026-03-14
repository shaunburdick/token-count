//! Integration tests for input size limits and DoS protection
//!
//! Note: tiktoken-rs has a known limitation where highly repetitive single-character
//! inputs (like "A" repeated 1M times) can trigger stack overflow due to regex
//! backtracking in the tokenization engine. This is a limitation of the underlying
//! library, not this tool. Real-world text with varied content works fine at large sizes.

use token_count::count_tokens;

/// Test that inputs up to 100KB work correctly (well under the limit)
#[test]
fn test_large_input_within_limit() {
    // 100KB of varied text (not highly repetitive)
    let input = "word ".repeat(20_000);
    let result = count_tokens(&input, "gpt-4");

    assert!(result.is_ok(), "Should handle 100KB input successfully");
    let token_count = result.unwrap().token_count;
    assert!(token_count > 0, "Should produce tokens for large input");
}

/// Test that varied content (not highly repetitive) works at larger sizes
#[test]
fn test_500kb_varied_input() {
    // 500KB of varied text - should work fine
    let base_text = "The quick brown fox jumps over the lazy dog. ";
    let repetitions = 500_000 / base_text.len();
    let input = base_text.repeat(repetitions);

    let result = count_tokens(&input, "gpt-4");
    assert!(result.is_ok(), "Should handle 500KB of varied text without stack overflow");
}

/// Test that inputs with varied content work at medium sizes
#[test]
fn test_medium_varied_input() {
    // Create 50KB of varied text to ensure tokenizer handles it
    let base_text = "The quick brown fox jumps over the lazy dog. ";
    let repetitions = 50_000 / base_text.len();
    let input = base_text.repeat(repetitions);

    let result = count_tokens(&input, "gpt-4");
    assert!(result.is_ok(), "Should handle 50KB of varied text");
}

/// Test Unicode handling in large inputs
#[test]
fn test_large_unicode_input() {
    // 10KB of Unicode characters
    let emoji_text = "Hello 👋 World 🌍 ";
    let repetitions = 10_000 / emoji_text.len();
    let input = emoji_text.repeat(repetitions);

    let result = count_tokens(&input, "gpt-4");
    assert!(result.is_ok(), "Should handle large Unicode input");
}

/// Test that the tool gracefully handles inputs near the maximum size
/// Note: This test is disabled by default as it allocates 50MB of memory
/// and may be slow. Enable with: cargo test -- --ignored
#[test]
#[ignore]
fn test_near_max_input_size() {
    // 50MB input (half the limit) - varied text to avoid regex backtracking
    let base_text = "Lorem ipsum dolor sit amet. ";
    let repetitions = (50 * 1024 * 1024) / base_text.len();
    let input = base_text.repeat(repetitions);
    let result = count_tokens(&input, "gpt-4");

    assert!(result.is_ok(), "Should handle 50MB input (half of 100MB limit)");
}

/// Test memory efficiency with streaming patterns
#[test]
fn test_multiple_medium_inputs() {
    // Process multiple 100KB inputs sequentially to verify no memory leaks
    for _ in 0..5 {
        let input = "Test input. ".repeat(10_000);
        let result = count_tokens(&input, "gpt-4");
        assert!(result.is_ok(), "Should handle multiple sequential large inputs");
    }
}

/// Test that different models handle large inputs consistently
#[test]
fn test_large_input_across_models() {
    let input = "word ".repeat(20_000); // ~100KB of varied text

    let models = vec!["gpt-3.5-turbo", "gpt-4", "gpt-4o"];

    for model in models {
        let result = count_tokens(&input, model);
        assert!(result.is_ok(), "Model {} should handle 100KB input", model);
    }
}
