//! Integration tests for Claude API mode
//!
//! Tests the --accurate flag behavior including error handling, consent requirements,
//! and fallback to estimation when API is unavailable.

use assert_cmd::Command;
use predicates::prelude::*;
use std::env;

#[test]
fn test_api_mode_with_invalid_key() {
    // Set an obviously invalid API key
    env::set_var("ANTHROPIC_API_KEY", "invalid-key-123");

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").arg("--accurate").arg("-y").write_stdin("test");

    // With invalid key, API call will fail and fall back to estimation
    // Should succeed with estimation and show warning in stderr
    let output = cmd.output().unwrap();

    // Should output token count (estimation fallback)
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.trim().is_empty(), "Should output token count");

    // Should have warning about API failure and fallback
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("falling back to estimation"),
        "Expected fallback warning, got: {}",
        stderr
    );

    // Clean up
    env::remove_var("ANTHROPIC_API_KEY");
}

#[test]
fn test_api_mode_missing_key_verbose_error() {
    env::remove_var("ANTHROPIC_API_KEY");

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").arg("--accurate").arg("-y").write_stdin("test");

    // Error should include setup instructions
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("export ANTHROPIC_API_KEY="))
        .stderr(predicate::str::contains("omit --accurate flag"));
}

#[test]
fn test_api_mode_non_interactive_without_yes() {
    env::remove_var("ANTHROPIC_API_KEY");

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").arg("--accurate").write_stdin("test");

    // Should fail in non-interactive mode without -y flag
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("non-interactive"))
        .stderr(predicate::str::contains("-y"));
}

#[test]
fn test_api_mode_only_applies_to_claude() {
    // OpenAI models should work with --accurate without needing consent or API key
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("--accurate").write_stdin("test");

    // Should succeed (tiktoken is offline)
    cmd.assert().success();
}

#[test]
fn test_accurate_flag_without_yes_non_interactive() {
    env::set_var("ANTHROPIC_API_KEY", "sk-test-key");

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").arg("--accurate").write_stdin("test");

    // Non-interactive without -y should fail with clear message
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("consent"))
        .stderr(predicate::str::contains("non-interactive"));

    env::remove_var("ANTHROPIC_API_KEY");
}

#[test]
fn test_yes_flag_requires_accurate() {
    // -y flag should work but not change behavior without --accurate
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").arg("-y").write_stdin("test");

    // Should succeed with estimation (no API call)
    cmd.assert().success();
}

#[test]
fn test_estimation_fallback_message() {
    // When API is unavailable, should fall back to estimation with warning
    env::set_var("ANTHROPIC_API_KEY", "sk-invalid");

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").arg("--accurate").arg("-y").write_stdin("test");

    let output = cmd.output().unwrap();

    // Should produce output (estimation fallback)
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.trim().is_empty(), "Should output token count");

    // May have warning in stderr
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !stderr.is_empty() {
        assert!(
            stderr.contains("Falling back") || stderr.contains("API"),
            "Unexpected stderr: {}",
            stderr
        );
    }

    env::remove_var("ANTHROPIC_API_KEY");
}

#[test]
fn test_all_claude_models_require_consent() {
    let models = vec!["claude-opus-4-6", "claude-sonnet-4-6", "claude-haiku-4-5"];

    for model in models {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(model).arg("--accurate").write_stdin("test");

        // All Claude models in accurate mode require consent
        cmd.assert().failure().stderr(predicate::str::contains("non-interactive"));
    }
}

#[test]
fn test_accurate_flag_preserves_exit_code_on_error() {
    env::remove_var("ANTHROPIC_API_KEY");

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude").arg("--accurate").arg("-y").write_stdin("test");

    // Missing API key should cause failure during tokenizer construction
    cmd.assert().failure();
}
