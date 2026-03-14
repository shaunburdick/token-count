use assert_cmd::Command;
use std::fs;

#[test]
fn test_file_redirect() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let input = fs::read_to_string("tests/fixtures/ascii.txt").unwrap();
    cmd.write_stdin(input);

    cmd.assert().success();
}

#[test]
fn test_unicode_file() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let input = fs::read_to_string("tests/fixtures/unicode.txt").unwrap();
    cmd.write_stdin(input);

    cmd.assert().success();
}

#[test]
fn test_invalid_utf8() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let input = fs::read("tests/fixtures/invalid_utf8.bin").unwrap();
    cmd.write_stdin(input);

    // Should fail with UTF-8 error (exit code 1)
    // For now (Phase 3), we're just testing input handling
    // Full error handling will be verified in Phase 5
    cmd.assert().failure();
}
