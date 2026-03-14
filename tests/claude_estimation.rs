//! Integration tests for Claude estimation mode
//!
//! Tests the offline token estimation algorithm for Claude models without requiring API keys.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_claude_estimation_simple() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude-sonnet-4-6").write_stdin("Hello, world!");

    cmd.assert().success();

    // Should output a number (estimated count, no ~ prefix since we changed to exact-looking output)
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let tokens: usize = stdout.trim().parse().unwrap();

    // "Hello, world!" should be roughly 3-4 tokens
    assert!(tokens >= 2 && tokens <= 6, "Expected 2-6 tokens, got {}", tokens);
}

#[test]
fn test_claude_estimation_verbose() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").arg("-v").write_stdin("Hello, world!");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Model: claude-sonnet-4-6"))
        .stdout(predicate::str::contains("Tokens:"))
        .stdout(predicate::str::contains("Context window: 1000000"));
}

#[test]
fn test_claude_estimation_empty_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").write_stdin("");

    cmd.assert().success().stdout("0\n");
}

#[test]
fn test_claude_estimation_prose() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let prose = "This is a longer piece of prose text without any code. \
                 It should be detected as natural language content and use \
                 the prose character-to-token ratio of approximately 4.5 characters per token.";

    cmd.arg("--model").arg("claude").write_stdin(prose);

    cmd.assert().success();

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let tokens: usize = stdout.trim().parse().unwrap();

    // 195 chars ÷ 4.5 ≈ 43 tokens
    assert!(tokens >= 38 && tokens <= 48, "Expected ~43 tokens for prose, got {}", tokens);
}

#[test]
fn test_claude_estimation_code() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let code = r#"{
  "name": "test",
  "version": "1.0.0",
  "dependencies": {
    "tokio": "1.0",
    "serde": "1.0"
  }
}"#;

    cmd.arg("--model").arg("claude").write_stdin(code);

    cmd.assert().success();

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let tokens: usize = stdout.trim().parse().unwrap();

    // JSON has lots of punctuation ({}:,"), should be detected as code
    // Using 3.0 chars/token ratio
    assert!(tokens >= 25 && tokens <= 45, "Expected ~35 tokens for JSON code, got {}", tokens);
}

#[test]
fn test_claude_estimation_mixed_content() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let mixed = r#"# Example Code

Here's a simple function:

```rust
fn hello() {
    println!("Hello, world!");
}
```

This demonstrates basic Rust syntax."#;

    cmd.arg("--model").arg("claude").write_stdin(mixed);

    cmd.assert().success();

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let tokens: usize = stdout.trim().parse().unwrap();

    // Mixed content should use 3.75 chars/token ratio
    // Should be between code (3.0) and prose (4.5) estimates
    assert!(tokens > 0, "Expected positive token count");
}

#[test]
fn test_claude_all_models() {
    let models = vec!["claude-opus-4-6", "claude-sonnet-4-6", "claude-haiku-4-5"];

    for model in models {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(model).write_stdin("test");

        cmd.assert().success();
    }
}

#[test]
fn test_claude_model_aliases() {
    let aliases = vec![
        "claude",                      // Default alias
        "sonnet",                      // Short alias
        "sonnet-4-6",                  // Version with hyphen
        "sonnet-4.6",                  // Version with dot
        "anthropic/claude-sonnet-4-6", // Provider prefix with full name
    ];

    for alias in aliases {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(alias).write_stdin("test");

        cmd.assert().success();
    }
}

#[test]
fn test_claude_unicode() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").write_stdin("Hello 世界 🌍");

    cmd.assert().success();
}

#[test]
fn test_claude_multiline() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").write_stdin("Line 1\nLine 2\nLine 3");

    cmd.assert().success();
}

#[test]
fn test_claude_large_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let large_input = "a ".repeat(1000);
    cmd.arg("--model").arg("claude").write_stdin(large_input);

    cmd.assert().success();
}

#[test]
fn test_claude_case_insensitive() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("CLAUDE").write_stdin("test");

    cmd.assert().success();
}
