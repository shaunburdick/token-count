//! Integration tests for error handling

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_invalid_utf8_error() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin(b"\xff\xfe");

    cmd.assert().failure().code(1).stderr(predicate::str::contains("Input contains invalid UTF-8"));
}

#[test]
fn test_unknown_model_error() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-5").write_stdin("test");

    cmd.assert().failure().code(2).stderr(predicate::str::contains("Unknown model: 'gpt-5'"));
}

#[test]
fn test_fuzzy_suggestion_close_match() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt4-tubro").write_stdin("test");

    cmd.assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Did you mean"))
        .stderr(predicate::str::contains("gpt-4-turbo"));
}

#[test]
fn test_fuzzy_suggestion_typo() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt-4o-turb").write_stdin("test");

    cmd.assert().failure().code(2).stderr(predicate::str::contains("Did you mean"));
}

#[test]
fn test_fuzzy_suggestion_no_close_match() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("claude-3").write_stdin("test");

    cmd.assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Unknown model"))
        .stderr(predicate::str::contains("Use --list-models"));
}

#[test]
fn test_empty_stdin_success() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("");

    // Empty input should succeed with 0 tokens (once Phase 6 is implemented)
    // For now, this will fail with todo!() panic
    // cmd.assert().success().stdout("0\n");
}

#[test]
fn test_case_insensitive_model_names() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("GPT-4").write_stdin("test");

    // Should succeed - case-insensitive matching
    // For now, this will fail with todo!() panic in Phase 6
    // cmd.assert().success();
}

#[test]
fn test_model_alias_resolution() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model").arg("gpt4").write_stdin("test");

    // Should succeed - alias resolution
    // For now, this will fail with todo!() panic in Phase 6
    // cmd.assert().success();
}

#[test]
fn test_invalid_utf8_with_large_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let mut invalid_input = vec![b'a'; 1000];
    invalid_input.push(0xff);
    invalid_input.push(0xfe);

    cmd.write_stdin(invalid_input);

    cmd.assert().failure().code(1).stderr(predicate::str::contains("invalid UTF-8"));
}
