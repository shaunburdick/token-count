//! End-to-end integration tests for Phase 6

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_basic_tokenization() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").write_stdin("Hello world");

    // "Hello world" = 2 tokens (write_stdin does not add newline)
    cmd.assert().success().stdout("2\n");
}

#[test]
fn test_model_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt4").write_stdin("test");

    cmd.assert().success();
}

#[test]
fn test_case_insensitive_model() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("GPT-4").write_stdin("test");

    cmd.assert().success();
}

#[test]
fn test_verbose_output() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-vv").write_stdin("test");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Model: gpt-4"))
        .stdout(predicate::str::contains("Tokens:"))
        .stdout(predicate::str::contains("Context window:"));
}

#[test]
fn test_debug_output() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").arg("-vvv").write_stdin("test");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Model: gpt-4"))
        .stdout(predicate::str::contains("Tokens:"))
        .stdout(predicate::str::contains("Token IDs:"))
        .stdout(predicate::str::contains("Decoded tokens:"));
}

#[test]
fn test_all_models() {
    let models = vec!["gpt-3.5-turbo", "gpt-4", "gpt-4-turbo", "gpt-4o"];

    for model in models {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(model).write_stdin("test");

        cmd.assert().success();
    }
}

#[test]
fn test_empty_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("");

    cmd.assert().success().stdout("0\n");
}

#[test]
fn test_unicode_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").write_stdin("Hello 世界 🌍");

    cmd.assert().success();
}

#[test]
fn test_large_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let large_input = "a ".repeat(1000);
    cmd.arg("--model").arg("gpt-4").write_stdin(large_input);

    cmd.assert().success();
}

#[test]
fn test_multiline_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4").write_stdin("Line 1\nLine 2\nLine 3");

    cmd.assert().success();
}

#[test]
fn test_default_model() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("test");

    // Should use gpt-3.5-turbo by default
    cmd.assert().success();
}

#[test]
fn test_exit_code_success() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("test");

    cmd.assert().code(0);
}

#[test]
fn test_exit_code_unknown_model() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("invalid").write_stdin("test");

    cmd.assert().code(2);
}

#[test]
fn test_exit_code_invalid_utf8() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin(b"\xff\xfe");

    cmd.assert().code(1);
}

#[test]
fn test_openai_prefix_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("openai/gpt-4").write_stdin("test");

    cmd.assert().success();
}
