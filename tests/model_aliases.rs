//! Integration tests for model aliases and resolution

use assert_cmd::Command;

/// Test all canonical model names
#[test]
fn test_canonical_model_names() {
    let models = vec!["gpt-3.5-turbo", "gpt-4", "gpt-4-turbo", "gpt-4o"];

    for model in models {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(model).write_stdin("test");

        cmd.assert().success();
    }
}

/// Test gpt-4 aliases
#[test]
fn test_gpt4_aliases() {
    let aliases = vec!["gpt4", "GPT-4", "GPT4", "openai/gpt-4"];

    for alias in aliases {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(alias).write_stdin("test");

        cmd.assert().success();
    }
}

/// Test gpt-3.5-turbo aliases
#[test]
fn test_gpt35_aliases() {
    let aliases = vec!["gpt-3.5", "gpt35", "gpt-35-turbo", "GPT-3.5-TURBO", "openai/gpt-3.5-turbo"];

    for alias in aliases {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(alias).write_stdin("test");

        cmd.assert().success();
    }
}

/// Test gpt-4-turbo aliases
#[test]
fn test_gpt4_turbo_aliases() {
    let aliases = vec!["gpt4-turbo", "gpt-4turbo", "GPT-4-TURBO", "openai/gpt-4-turbo"];

    for alias in aliases {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(alias).write_stdin("test");

        cmd.assert().success();
    }
}

/// Test gpt-4o aliases
#[test]
fn test_gpt4o_aliases() {
    let aliases = vec!["gpt4o", "GPT-4O", "openai/gpt-4o"];

    for alias in aliases {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(alias).write_stdin("test");

        cmd.assert().success();
    }
}

/// Test case-insensitive model resolution
#[test]
fn test_case_insensitive() {
    let variations = vec![
        ("gpt-4", "GPT-4"),
        ("gpt-4", "Gpt-4"),
        ("gpt-4", "gPt-4"),
        ("gpt-3.5-turbo", "GPT-3.5-TURBO"),
        ("gpt-4o", "GPT-4O"),
    ];

    for (_canonical, variant) in variations {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(variant).write_stdin("test");

        cmd.assert().success();
    }
}

/// Test that different aliases for the same model produce the same token count
#[test]
fn test_alias_consistency() {
    let test_input = "Hello world";

    // Test gpt-4 aliases produce same count
    let mut cmd1 = Command::cargo_bin("token-count").unwrap();
    let output1 = cmd1.arg("--model").arg("gpt-4").write_stdin(test_input).output().unwrap();

    let mut cmd2 = Command::cargo_bin("token-count").unwrap();
    let output2 = cmd2.arg("--model").arg("gpt4").write_stdin(test_input).output().unwrap();

    let mut cmd3 = Command::cargo_bin("token-count").unwrap();
    let output3 = cmd3.arg("--model").arg("openai/gpt-4").write_stdin(test_input).output().unwrap();

    assert_eq!(output1.stdout, output2.stdout, "gpt-4 and gpt4 should produce same count");
    assert_eq!(output1.stdout, output3.stdout, "gpt-4 and openai/gpt-4 should produce same count");
}

/// Test openai/ prefix works for all models
#[test]
fn test_openai_prefix() {
    let models =
        vec!["openai/gpt-3.5-turbo", "openai/gpt-4", "openai/gpt-4-turbo", "openai/gpt-4o"];

    for model in models {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.arg("--model").arg(model).write_stdin("test");

        cmd.assert().success();
    }
}
