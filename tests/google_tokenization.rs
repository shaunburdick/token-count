//! Integration tests for Google Gemini tokenization
//!
//! Tests CLI integration with Google Gemini models using the gemini-tokenizer
//! library for exact, offline token counting.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_gemini_default_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini"]).write_stdin("Hello, Gemini!").assert().success().stdout("4\n");
}

#[test]
fn test_gemini_pro_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini-pro"]).write_stdin("test").assert().success().stdout("1\n");
}

#[test]
fn test_all_gemini_models() {
    // Test all 4 supported models
    let models =
        vec!["gemini-2.5-pro", "gemini-2.5-flash", "gemini-2.5-flash-lite", "gemini-3-pro-preview"];

    for model in models {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.args(["--model", model]).write_stdin("test").assert().success().stdout("1\n");
    }
}

#[test]
fn test_gemini_case_insensitive() {
    for variant in &["gemini", "GEMINI", "Gemini", "GeMiNi"] {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.args(["--model", variant]).write_stdin("test").assert().success().stdout("1\n");
    }
}

#[test]
fn test_gemini_provider_format() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "google/gemini"]).write_stdin("test").assert().success().stdout("1\n");
}

#[test]
fn test_gemini_verbose() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini", "-v"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("Model: gemini-2.5-flash"))
        .stdout(predicate::str::contains("gemini-gemma3"))
        .stdout(predicate::str::contains("Tokens: 1"))
        .stdout(predicate::str::contains("Context window: 1000000"));
}

#[test]
fn test_gemini_3_pro_preview() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini-3-pro-preview", "-v"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("Context window: 1000000"));
}

#[test]
fn test_gemini_list_models() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--list-models")
        .assert()
        .success()
        .stdout(predicate::str::contains("gemini-2.5-flash"))
        .stdout(predicate::str::contains("gemini-gemma3"));
}

#[test]
fn test_gemini_unknown_model() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini-4"])
        .write_stdin("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown model: 'gemini-4'"));
}

#[test]
fn test_gemini_empty_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini"]).write_stdin("").assert().success().stdout("0\n");
}

#[test]
fn test_gemini_multiline() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini"])
        .write_stdin("Line 1\nLine 2\nLine 3")
        .assert()
        .success()
        .stdout("11\n");
}

#[test]
fn test_gemini_unicode() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini"]).write_stdin("Hello 世界 🌍").assert().success();
    // Note: exact count may vary, just verify it succeeds
}

#[test]
fn test_gemini_long_text() {
    let long_text = "The quick brown fox jumps over the lazy dog. ".repeat(100);
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini"]).write_stdin(long_text.as_str()).assert().success();
    // Should succeed with reasonable token count
}

#[test]
fn test_gemini_flash_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini-flash"]).write_stdin("test").assert().success().stdout("1\n");
}

#[test]
fn test_gemini_lite_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini-lite"]).write_stdin("test").assert().success().stdout("1\n");
}

#[test]
fn test_gemini_3_pro_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(["--model", "gemini-3-pro"]).write_stdin("test").assert().success().stdout("1\n");
}

#[test]
fn test_gemini_comparison_openai() {
    // Verify Gemini and OpenAI can both tokenize the same text
    let test_text = "Hello world";

    let mut gemini_cmd = Command::cargo_bin("token-count").unwrap();
    let gemini_output =
        gemini_cmd.args(["--model", "gemini"]).write_stdin(test_text).output().unwrap();

    let mut openai_cmd = Command::cargo_bin("token-count").unwrap();
    let openai_output =
        openai_cmd.args(["--model", "gpt-4"]).write_stdin(test_text).output().unwrap();

    // Both should succeed
    assert!(gemini_output.status.success());
    assert!(openai_output.status.success());

    // Both should produce valid token counts (may differ)
    let gemini_count = String::from_utf8(gemini_output.stdout).unwrap();
    let openai_count = String::from_utf8(openai_output.stdout).unwrap();

    assert!(!gemini_count.trim().is_empty());
    assert!(!openai_count.trim().is_empty());
}
