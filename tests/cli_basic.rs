use assert_cmd::Command;

#[test]
fn test_stdin_pipe_basic() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("Hello world");

    // For now, we're in Phase 3, so we just verify it doesn't crash
    // Full functionality will be tested in Phase 6 after integration
    cmd.assert().success();
}

#[test]
fn test_empty_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("");

    cmd.assert().success();
}

#[test]
fn test_unicode_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("Hello 世界 🌍");

    cmd.assert().success();
}

#[test]
fn test_multiline_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.write_stdin("Line 1\nLine 2\nLine 3");

    cmd.assert().success();
}
