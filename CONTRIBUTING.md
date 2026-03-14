# Contributing to token-count

Thank you for your interest in contributing to `token-count`! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Code Quality Standards](#code-quality-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Project Structure](#project-structure)

## Code of Conduct

This project follows the Rust Code of Conduct. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- **Rust**: 1.85.0 or later
- **Git**: For version control
- **Linux**: Ubuntu 22.04+ (macOS/Windows support coming in v0.2.0)

### Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/token-count
cd token-count
```

## Development Setup

### 1. Install Rust

```bash
# Install rustup if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ensure you have Rust 1.85.0+
rustup update
rustc --version  # Should be 1.85.0 or later
```

### 2. Build the Project

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test suite
cargo test --test model_aliases
```

### 4. Run Benchmarks

```bash
# Run performance benchmarks
cargo bench

# View results in target/criterion/report/index.html
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Changes

Follow the [Code Quality Standards](#code-quality-standards) below.

### 3. Run Quality Checks

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter (must have zero warnings)
cargo clippy -- -D warnings

# Run tests
cargo test

# Security audit
cargo install cargo-audit  # First time only
cargo audit
```

### 4. Commit Changes

```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "feat: add support for new model"
# or
git commit -m "fix: correct token count for edge case"
```

**Commit message format**:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions or changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Build process or tooling changes

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Code Quality Standards

### Non-Negotiable Rules

#### 1. No Disabled Linting Rules ❌

**NEVER** add these to your code:
- `#[allow(clippy::...)]`
- `// eslint-disable`
- `@ts-ignore`
- `@ts-nocheck`

**Instead**: Refactor your code to comply with the lint rule.

**Example - Wrong vs Right:**

```rust
// ❌ WRONG - Disabling the rule
#[allow(clippy::needless_return)]
fn calculate(x: i32) -> i32 {
    return x * 2;
}

// ✅ RIGHT - Following the rule
fn calculate(x: i32) -> i32 {
    x * 2
}
```

#### 2. 100% Type Safety ✅

- No `any` types in TypeScript (if we add TS tooling)
- Proper Rust types, no excessive `unwrap()` in library code
- Use `Result<T, E>` for fallible operations
- Use `Option<T>` for optional values

#### 3. Documentation Required 📝

All public APIs must have doc comments with examples:

```rust
/// Count tokens in the given text using the specified model
///
/// # Arguments
///
/// * `text` - The input text to tokenize
/// * `model_name` - The name of the model to use (canonical name or alias)
///
/// # Returns
///
/// Returns a `TokenizationResult` containing the token count and model information
///
/// # Errors
///
/// Returns an error if:
/// - The model name is unknown
/// - Tokenization fails
///
/// # Example
///
/// ```
/// use token_count::count_tokens;
///
/// let result = count_tokens("Hello world", "gpt-4").unwrap();
/// assert_eq!(result.token_count, 2);
/// ```
pub fn count_tokens(text: &str, model_name: &str) -> Result<TokenizationResult, TokenError> {
    // Implementation
}
```

#### 4. Test Coverage ✅

- All new features must have tests
- All bug fixes must have regression tests
- Integration tests for user-facing functionality
- Unit tests for internal logic

### Verification Protocol (Before Committing)

**⚠️ Complete this checklist before EVERY commit:**

- [ ] `cargo test` - All tests pass
- [ ] `cargo clippy -- -D warnings` - Zero warnings
- [ ] `cargo fmt --check` - Code is formatted
- [ ] `cargo build --release` - Release build succeeds
- [ ] Manually test the affected functionality
- [ ] Update documentation if behavior changed
- [ ] Add tests for new functionality

## Testing

### Test Structure

```
tests/
├── fixtures/              # Test data files
│   ├── ascii.txt
│   ├── unicode.txt
│   ├── large.txt
│   └── invalid_utf8.bin
├── model_aliases.rs       # Model resolution tests
├── verbosity.rs           # Output format tests
├── performance.rs         # Performance tests
├── error_handling.rs      # Error scenario tests
├── end_to_end.rs          # E2E integration tests
└── ...
```

### Writing Tests

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_new_feature() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--model")
        .arg("gpt-4")
        .write_stdin("test input");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("expected output"));
}
```

### Running Specific Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test model_aliases

# Run specific test function
cargo test test_fuzzy_suggestions

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Submitting Changes

### Pull Request Process

1. **Ensure all checks pass**
   - All tests pass
   - Zero clippy warnings
   - Code is formatted
   - Security audit clean

2. **Update documentation**
   - Update README.md if user-facing changes
   - Update CHANGELOG.md with your changes
   - Add/update doc comments

3. **Write a clear PR description**
   - What does this change do?
   - Why is this change needed?
   - How was it tested?
   - Any breaking changes?

4. **Request review**
   - Tag relevant maintainers
   - Respond to feedback promptly
   - Make requested changes

5. **Wait for CI**
   - GitHub Actions will run all tests
   - All checks must pass before merge

### PR Title Format

Use conventional commits format:

```
feat: add support for Claude models
fix: correct token count for emoji
docs: update README with new examples
test: add integration tests for file input
refactor: simplify error handling
perf: optimize large file processing
```

## Project Structure

```
token-count/
├── .github/
│   └── workflows/
│       └── ci.yml          # GitHub Actions CI
├── .specify/               # Specification documents
│   ├── memory/
│   │   └── constitution.md
│   └── features/
│       └── 001-core-cli.md
├── benches/                # Performance benchmarks
│   └── tokenization.rs
├── scripts/                # Development scripts
│   └── generate_fixtures.py
├── src/
│   ├── lib.rs             # Public library API
│   ├── main.rs            # Binary entry point
│   ├── cli/               # CLI argument parsing
│   │   ├── args.rs        # Clap definitions
│   │   ├── input.rs       # Stdin reading
│   │   └── mod.rs
│   ├── tokenizers/        # Tokenization engine
│   │   ├── openai.rs      # OpenAI tokenizer
│   │   ├── registry.rs    # Model registry
│   │   └── mod.rs
│   ├── output/            # Output formatters
│   │   ├── simple.rs      # Simple formatter
│   │   ├── verbose.rs     # Verbose formatter
│   │   ├── debug.rs       # Debug formatter
│   │   └── mod.rs
│   └── error.rs           # Error types
├── specs/                 # Implementation plans
│   └── 001-core-cli/
│       ├── plan.md
│       ├── research.md
│       ├── data-model.md
│       └── tasks.md
├── tests/                 # Integration tests
│   ├── fixtures/          # Test data
│   └── *.rs               # Test files
├── Cargo.toml             # Rust project manifest
├── README.md              # User documentation
├── CHANGELOG.md           # Version history
├── CONTRIBUTING.md        # This file
└── LICENSE                # MIT license
```

## Release Process

This section is for maintainers who have permission to create releases.

### Pre-Release Checklist

Before creating a release, ensure:

- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] CHANGELOG.md is updated with version and date
- [ ] README.md reflects current features
- [ ] All documentation is up to date

### Creating a Release

1. **Update version in `Cargo.toml`**:
   ```toml
   version = "0.1.0"  # Increment according to semver
   ```

2. **Update CHANGELOG.md**:
   ```markdown
   ## [0.1.0] - 2026-03-14
   
   ### Added
   - Feature descriptions...
   ```

3. **Commit version bump**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: bump version to 0.1.0"
   git push origin main
   ```

4. **Create and push tag**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

5. **Monitor GitHub Actions**:
   - Go to: https://github.com/shaunburdick/token-count/actions
   - Verify the release workflow completes successfully
   - Check that all 4 platform binaries are built
   - Verify checksums.txt is generated

6. **Verify draft release**:
   - Go to: https://github.com/shaunburdick/token-count/releases
   - Review the auto-generated draft release
   - Check that all assets are attached:
     - token-count-VERSION-x86_64-unknown-linux-gnu.tar.gz
     - token-count-VERSION-x86_64-apple-darwin.tar.gz
     - token-count-VERSION-aarch64-apple-darwin.tar.gz
     - token-count-VERSION-x86_64-pc-windows-msvc.zip
     - checksums.txt
   - Edit release notes if needed
   - **Publish the release** (remove draft status)

7. **Verify automated updates**:
   - **Homebrew**: Check that homebrew-tap repository received an automatic commit updating the formula
   - **crates.io**: Verify package is published at https://crates.io/crates/token-count

8. **Test all installation methods**:
   ```bash
   # Install script
   curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
   token-count --version  # Should show new version
   
   # Homebrew (wait ~5 minutes for formula update)
   brew update
   brew upgrade token-count
   token-count --version
   
   # Cargo
   cargo install token-count --force
   token-count --version
   
   # Manual download
   # Download from releases page and verify checksum
   ```

### Hotfix Releases

For critical bug fixes:

1. Create hotfix branch from the release tag:
   ```bash
   git checkout -b hotfix/v0.1.1 v0.1.0
   ```

2. Make the fix and test thoroughly

3. Follow the release process above with the new version

4. Merge hotfix back to main:
   ```bash
   git checkout main
   git merge hotfix/v0.1.1
   git push origin main
   ```

### Rollback Procedure

If a release has critical issues:

1. **Delete the GitHub release** (not the tag):
   - Go to releases page
   - Click "Delete" on the problematic release

2. **Notify users**:
   - Create a GitHub issue explaining the problem
   - Update CHANGELOG.md with a note

3. **Create a fixed release** following the process above

### Troubleshooting Releases

**Release workflow fails:**
- Check GitHub Actions logs for errors
- Common issues:
  - Compilation errors on specific platforms
  - Missing secrets (HOMEBREW_TAP_TOKEN, CARGO_REGISTRY_TOKEN)
  - Network issues downloading dependencies

**Homebrew formula doesn't update:**
- Verify HOMEBREW_TAP_TOKEN secret is valid
- Check that the tag is an official release (no `-` in version)
- Manually update formula as fallback (see specs/002-installation/HOMEBREW-SETUP.md)

**crates.io publish fails:**
- Verify CARGO_REGISTRY_TOKEN is valid
- Check that version doesn't already exist
- Ensure Cargo.toml metadata is complete

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/shaunburdick/token-count/issues)
- **Discussions**: [GitHub Discussions](https://github.com/shaunburdick/token-count/discussions)
- **Email**: hello@burdick.dev

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to `token-count`! 🎉
