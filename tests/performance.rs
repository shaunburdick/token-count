//! Performance and memory tests for Phase 7

use assert_cmd::Command;
use std::fs;

/// Test that small inputs are processed quickly
#[test]
fn test_small_input_performance() {
    let small_input = "Hello world";

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let start = std::time::Instant::now();
    cmd.arg("--model").arg("gpt-4").write_stdin(small_input).assert().success();
    let elapsed = start.elapsed();

    // Should complete in reasonable time (accounting for process spawn overhead)
    assert!(elapsed.as_secs() < 2, "Small input took too long: {:?}", elapsed);
}

/// Test that medium-sized inputs are processed quickly
#[test]
fn test_medium_input_performance() {
    let medium_input = "word ".repeat(200); // ~1KB

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let start = std::time::Instant::now();
    cmd.arg("--model").arg("gpt-4").write_stdin(medium_input).assert().success();
    let elapsed = start.elapsed();

    // Should complete in reasonable time (accounting for process spawn overhead)
    assert!(elapsed.as_secs() < 2, "Medium input took too long: {:?}", elapsed);
}

/// Test that large inputs are processed efficiently
#[test]
fn test_large_input_performance() {
    let large_input = fs::read_to_string("tests/fixtures/large.txt").unwrap();

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let start = std::time::Instant::now();
    cmd.arg("--model").arg("gpt-4").write_stdin(large_input).assert().success();
    let elapsed = start.elapsed();

    // 12MB file should be processed in reasonable time (debug build is slower)
    assert!(elapsed.as_secs() < 30, "Large input took too long: {:?}", elapsed);
}

/// Test that empty input is handled efficiently
#[test]
fn test_empty_input_performance() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let start = std::time::Instant::now();
    cmd.write_stdin("").assert().success();
    let elapsed = start.elapsed();

    // Empty input should complete quickly (accounting for process spawn)
    assert!(elapsed.as_secs() < 2, "Empty input took too long: {:?}", elapsed);
}

/// Test that unicode input doesn't significantly impact performance
#[test]
fn test_unicode_input_performance() {
    let unicode_input = fs::read_to_string("tests/fixtures/unicode.txt").unwrap();

    let mut cmd = Command::cargo_bin("token-count").unwrap();
    let start = std::time::Instant::now();
    cmd.arg("--model").arg("gpt-4").write_stdin(unicode_input).assert().success();
    let elapsed = start.elapsed();

    // Unicode should be handled quickly
    assert!(elapsed.as_secs() < 2, "Unicode input took too long: {:?}", elapsed);
}

/// Test all models have similar performance
#[test]
fn test_all_models_performance() {
    let models = vec!["gpt-3.5-turbo", "gpt-4", "gpt-4-turbo", "gpt-4o"];
    let test_input = "Hello world test";

    for model in models {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        let start = std::time::Instant::now();
        cmd.arg("--model").arg(model).write_stdin(test_input).assert().success();
        let elapsed = start.elapsed();

        assert!(elapsed.as_secs() < 2, "Model {} took too long: {:?}", model, elapsed);
    }
}

/// Test that verbose output doesn't significantly impact performance
#[test]
fn test_verbose_output_performance() {
    let test_input = "word ".repeat(100);

    // Test simple output
    let mut cmd1 = Command::cargo_bin("token-count").unwrap();
    let start1 = std::time::Instant::now();
    cmd1.arg("--model").arg("gpt-4").write_stdin(test_input.clone()).assert().success();
    let elapsed1 = start1.elapsed();

    // Test verbose output
    let mut cmd2 = Command::cargo_bin("token-count").unwrap();
    let start2 = std::time::Instant::now();
    cmd2.arg("--model").arg("gpt-4").arg("-v").write_stdin(test_input).assert().success();
    let elapsed2 = start2.elapsed();

    // Verbose should not be significantly slower
    assert!(
        elapsed2.as_millis() < elapsed1.as_millis() * 2,
        "Verbose output significantly slower: simple={:?}, verbose={:?}",
        elapsed1,
        elapsed2
    );
}
