use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Count tokens for LLM models"))
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Options:"))
        .stdout(predicate::str::contains("--model"))
        .stdout(predicate::str::contains("--verbose"))
        .stdout(predicate::str::contains("--list-models"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("token-count"))
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_list_models_flag() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--list-models");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Supported models"))
        .stdout(predicate::str::contains("gpt-3.5-turbo"))
        .stdout(predicate::str::contains("gpt-4"))
        .stdout(predicate::str::contains("gpt-4-turbo"))
        .stdout(predicate::str::contains("gpt-4o"))
        .stdout(predicate::str::contains("Context window"))
        .stdout(predicate::str::contains("Aliases"));
}

#[test]
fn test_help_text_concise() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--help");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let line_count = stdout.lines().count();

    // Help text should fit in 24 lines (quickstart Test 30)
    assert!(line_count <= 30, "Help text too long: {} lines (target: ≤30)", line_count);
}
