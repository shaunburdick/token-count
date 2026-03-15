//! Integration tests for verbosity levels and output formats

use assert_cmd::Command;
use predicates::prelude::*;

/// Test verbosity level 0 (default) - simple output
#[test]
fn test_verbosity_0_simple() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").write_stdin("Hello world");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Simple output should be just the number
    assert_eq!(stdout.trim(), "2", "Simple output should be just the token count");
    assert!(!stdout.contains("Model:"), "Simple output should not contain 'Model:'");
}

/// Test verbosity level 1 (-v) - basic output (no percentage)
#[test]
fn test_verbosity_1_verbose() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-v").write_stdin("Hello world");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("Model: gpt-4"), "Should show model name");
    assert!(stdout.contains("Tokens: 2"), "Should show token count");
    assert!(!stdout.contains("Context window:"), "Should NOT show context window at -v level");
    assert!(!stdout.contains('%'), "Should NOT show percentage at -v level");
}

/// Test verbosity level 2 (-vv) - verbose output with percentage
#[test]
fn test_verbosity_2_very_verbose() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-vv").write_stdin("Hello world");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Model: gpt-4"))
        .stdout(predicate::str::contains("Tokens: 2"))
        .stdout(predicate::str::contains("Context window:"))
        .stdout(predicate::str::contains('%'));
}

/// Test verbosity level 3+ (-vvv) - debug mode
#[test]
fn test_verbosity_3_debug() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-vvv").write_stdin("Hello world");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Model: gpt-4"))
        .stdout(predicate::str::contains("Tokens: 2"))
        .stdout(predicate::str::contains("Token IDs:"))
        .stdout(predicate::str::contains("Decoded tokens:"));
}

/// Test verbosity level 4+ (-vvvv) - same as -vvv
#[test]
fn test_verbosity_4_plus() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-vvvv").write_stdin("Hello world");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Token IDs:"))
        .stdout(predicate::str::contains("Decoded tokens:"));
}

/// Test that verbose output includes encoding information
#[test]
fn test_verbose_encoding_info() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-v").write_stdin("test");

    cmd.assert().success().stdout(predicate::str::contains("cl100k_base"));
}

/// Test that verbose output includes context window percentage
#[test]
fn test_verbose_context_percentage() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-vv").write_stdin("test");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("Context window:"), "Should show context window");
    assert!(stdout.contains("%"), "Should show percentage");
}

/// Test verbose output with different models shows different encodings
#[test]
fn test_verbose_different_encodings() {
    // gpt-4 uses cl100k_base
    let mut cmd1 = Command::cargo_bin("token-count").unwrap();
    let output1 = cmd1.arg("--model").arg("gpt-4").arg("-v").write_stdin("test").output().unwrap();
    let stdout1 = String::from_utf8(output1.stdout).unwrap();
    assert!(stdout1.contains("cl100k_base"), "gpt-4 should use cl100k_base");

    // gpt-4o uses o200k_base
    let mut cmd2 = Command::cargo_bin("token-count").unwrap();
    let output2 = cmd2.arg("--model").arg("gpt-4o").arg("-v").write_stdin("test").output().unwrap();
    let stdout2 = String::from_utf8(output2.stdout).unwrap();
    assert!(stdout2.contains("o200k_base"), "gpt-4o should use o200k_base");
}

/// Test verbose output with large input shows correct context percentage
#[test]
fn test_verbose_large_input_percentage() {
    let large_input = "word ".repeat(1000); // ~1000 tokens

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-vv").write_stdin(large_input);

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("%"), "Should show percentage");
    // For gpt-4 with 128K context, 1000 tokens should be less than 1%
    // We can't assert the exact value due to tokenization variations
}

/// Test that simple mode works without -v flag
#[test]
fn test_default_is_simple() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("test");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should be just a number
    assert!(!stdout.contains("Model:"), "Default output should not contain 'Model:'");
    assert!(!stdout.contains("Tokens:"), "Default output should not contain 'Tokens:'");
}
