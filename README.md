# token-count

> A fast, accurate CLI tool for counting tokens in LLM model inputs

[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-177%20passing-brightgreen.svg)](tests/)

## Overview

`token-count` is a POSIX-style command-line tool that counts tokens for various LLM models. It supports exact tokenization for OpenAI and Google Gemini models (offline), and adaptive estimation for Claude models (with optional API mode for exact counts). Pipe any text in, get token counts out—fast, offline, and accurate.

```bash
# OpenAI models (exact, offline)
echo "Hello world" | token-count --model gpt-4
3

# Google Gemini models (exact, offline)
echo 'Hello, Gemini!' | token-count --model gemini
5

# Claude models (estimation, offline)
echo 'Hello, Claude!' | token-count --model claude
4

# From file
token-count --model gpt-4 < document.txt
1842

# With context info
cat prompt.txt | token-count --model claude-sonnet-4-6 -v
Model: claude-sonnet-4-6 (anthropic-claude)
Tokens: 142
Context window: 1000000 tokens (0.0142% used)
```

## Features

✅ **Accurate** - Exact tokenization for OpenAI and Google Gemini, adaptive estimation for Claude  
✅ **Fast** - Optimized for speed with embedded tokenizers  
✅ **Offline** - Zero runtime dependencies for OpenAI and Gemini; optional API for Claude  
✅ **Simple** - POSIX-style interface, works like `wc` or `grep`

## Installation

### Quick Install (Recommended)

**Linux / macOS:**
```bash
curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

**Homebrew (macOS / Linux):**
```bash
brew install shaunburdick/tap/token-count
```

**Cargo (All Platforms):**
```bash
cargo install token-count
```

**Manual Download:**  
Download pre-built binaries from [GitHub Releases](https://github.com/shaunburdick/token-count/releases).

For detailed installation instructions, troubleshooting, and platform-specific guidance, see [INSTALL.md](INSTALL.md).

### System Requirements

- **Platform**: Linux x86_64, macOS (Intel/Apple Silicon), Windows x86_64
- **Runtime**: No dependencies (static binary)
- **Build from source**: Rust 1.85.0 or later, CMake 3.10+ (for gemini-tokenizer SentencePiece dependency)

## Usage

### Basic Usage

```bash
# Default model (gpt-3.5-turbo)
echo "Hello world" | token-count
3

# Specific model
echo "Hello world" | token-count --model gpt-4
3

# From file
token-count --model gpt-4 < input.txt
1842

# Piped from another command
cat README.md | token-count --model gpt-4o
3521
```

### Model Selection

```bash
# Use canonical name
token-count --model gpt-4 < input.txt

# Use alias (case-insensitive)
token-count --model gpt4 < input.txt
token-count --model GPT-4 < input.txt

# With provider prefix
token-count --model openai/gpt-4 < input.txt
```

### Verbosity Levels

```bash
# Simple output (default) - just the number
echo "test" | token-count
2

# Verbose (-v) - model info and context usage
echo "test" | token-count -v
Model: gpt-3.5-turbo (cl100k_base)
Tokens: 2
Context window: 16385 tokens (0.0122% used)

# Debug (-vvv) - for troubleshooting
echo "test" | token-count -vvv
Model: gpt-3.5-turbo (cl100k_base)
Tokens: 2
Context window: 16385 tokens

[Debug mode: Token IDs and decoding require tokenizer access]
[Full implementation in Phase 6]
```

### Model Information

```bash
# List all supported models
token-count --list-models

# Output:
# Supported models:
#
#   gpt-3.5-turbo
#     Encoding: cl100k_base
#     Context window: 16385 tokens
#     Aliases: gpt-3.5, gpt35, gpt-35-turbo, openai/gpt-3.5-turbo
#
#   gpt-4
#     Encoding: cl100k_base
#     Context window: 128000 tokens
#     Aliases: gpt4, openai/gpt-4
# ...
```

### Help and Version

```bash
# Show help
token-count --help

# Show version
token-count --version
```

## Supported Models

### OpenAI Models (Exact Tokenization - Offline)

| Model | Encoding | Context Window | Aliases |
|-------|----------|----------------|---------|
| gpt-3.5-turbo | cl100k_base | 16,385 | gpt-3.5, gpt35, gpt-35-turbo |
| gpt-4 | cl100k_base | 128,000 | gpt4 |
| gpt-4-turbo | cl100k_base | 128,000 | gpt4-turbo, gpt-4turbo |
| gpt-4o | o200k_base | 128,000 | gpt4o |

### Anthropic Claude Models (Adaptive Estimation - Offline by Default)

| Model | Context Window | Aliases | Estimation Mode |
|-------|----------------|---------|-----------------|
| claude-opus-4-6 | 1,000,000 | opus, opus-4-6, opus-4.6 | ±10% accuracy |
| claude-sonnet-4-6 | 1,000,000 | claude, sonnet, sonnet-4-6, sonnet-4.6 | ±10% accuracy |
| claude-haiku-4-5 | 200,000 | haiku, haiku-4-5, haiku-4.5 | ±10% accuracy |

### Google Gemini Models (Exact Tokenization - Offline)

| Model | Encoding | Context Window | Aliases |
|-------|----------|----------------|---------|
| gemini-2.5-pro | gemini-gemma3 | 1,000,000 | gemini-pro, gemini-2-pro, gemini-2.5 |
| gemini-2.5-flash | gemini-gemma3 | 1,000,000 | gemini, gemini-flash, gemini-2-flash |
| gemini-2.5-flash-lite | gemini-gemma3 | 1,000,000 | gemini-lite, gemini-2-lite, gemini-2.5-lite |
| gemini-3-pro-preview | gemini-gemma3 | 1,000,000 | gemini-3-pro, gemini-3 |

**Note**: The `gemini` alias defaults to `gemini-2.5-flash`, the recommended general-purpose model.

**Claude Tokenization Modes:**

**Offline Estimation (Default)** - No API key needed:
```bash
# Fast offline estimation using adaptive content-type detection
echo 'Hello, Claude!' | token-count --model claude
4
```

**Exact API Mode (Optional)** - Requires `ANTHROPIC_API_KEY`:
```bash
# Exact count via Anthropic API (requires consent)
export ANTHROPIC_API_KEY="sk-ant-..."
echo 'Hello, Claude!' | token-count --model claude --accurate
# Prompts: "This will send your input to Anthropic's API... Proceed? (y/N)"
# Output: 8

# Skip prompt for automation
cat file.txt | token-count --model claude --accurate -y
```

**How Claude Estimation Works:**
- Detects content type (code vs. prose) using punctuation and keyword analysis
- **Code**: 3.0 chars/token (lots of `{}[]();` and keywords)
- **Prose**: 4.5 chars/token (natural language)
- **Mixed**: 3.75 chars/token (markdown + code blocks)
- Target: ±10% accuracy for typical inputs

All models support:
- Case-insensitive names (e.g., `GPT-4`, `gpt-4`, `Gpt-4`, `GEMINI`)
- Provider prefix (e.g., `openai/gpt-4`, `anthropic/claude-sonnet-4-6`, `google/gemini`)

## Error Handling

`token-count` provides helpful error messages with suggestions:

```bash
# Unknown model with fuzzy suggestions
$ echo "test" | token-count --model gpt5
Error: Unknown model: 'gpt5'. Did you mean: gpt-4, gpt-4o?

# Typo correction
$ echo "test" | token-count --model gpt4-tubro
Error: Unknown model: 'gpt4-tubro'. Did you mean: gpt-4-turbo?

# Invalid UTF-8
$ token-count < invalid.bin
Error: Input contains invalid UTF-8 at byte 0
```

### Exit Codes

- `0` - Success
- `1` - I/O error or invalid UTF-8
- `2` - Unknown model name

## Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/shaunburdick/token-count
cd token-count

# Run tests
cargo test

# Run benchmarks
cargo bench

# Build release binary
cargo build --release

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check

# Security audit
cargo audit
```

### Running Tests

```bash
# All tests (100 tests)
cargo test

# Specific test suite
cargo test --test model_aliases
cargo test --test verbosity
cargo test --test performance

# With output
cargo test -- --nocapture
```

### Project Structure

```
token-count/
├── src/
│   ├── lib.rs              # Public library API
│   ├── main.rs             # Binary entry point
│   ├── cli/                # CLI argument parsing
│   │   ├── args.rs         # Clap definitions
│   │   ├── input.rs        # Stdin reading
│   │   └── mod.rs
│   ├── tokenizers/         # Tokenization engine
│   │   ├── openai.rs       # OpenAI tokenizer (tiktoken)
│   │   ├── claude/         # Claude tokenizer
│   │   │   ├── mod.rs      # Main tokenizer
│   │   │   ├── estimation.rs  # Adaptive estimation
│   │   │   ├── api_client.rs  # Anthropic API
│   │   │   └── models.rs   # Model definitions
│   │   ├── registry.rs     # Model registry
│   │   └── mod.rs
│   ├── api/                # API utilities
│   │   ├── consent.rs      # Interactive consent prompt
│   │   └── mod.rs
│   ├── output/             # Output formatters
│   │   ├── simple.rs       # Simple formatter
│   │   ├── verbose.rs      # Verbose formatter
│   │   ├── debug.rs        # Debug formatter
│   │   └── mod.rs
│   └── error.rs            # Error types
├── tests/                  # Integration tests
│   ├── fixtures/           # Test data
│   ├── model_aliases.rs
│   ├── verbosity.rs
│   ├── performance.rs
│   ├── error_handling.rs
│   ├── end_to_end.rs
│   ├── claude_estimation.rs  # Claude estimation tests
│   ├── claude_api.rs          # Claude API tests
│   └── ...
├── benches/                # Performance benchmarks
│   └── tokenization.rs
    └── .github/
        └── workflows/
            └── ci.yml          # CI configuration
```

## Security

### Resource Limits

- **Maximum input size**: 100MB per invocation
- **Memory usage**: Typically <100MB, peaks at ~2x input size
- **CPU usage**: Single-threaded, 100% of one core during processing

### Known Limitations

**Stack Overflow with Highly Repetitive Inputs**: The underlying tiktoken-rs library can experience stack overflow when processing highly repetitive single-character inputs (e.g., 1MB+ of the same character). This is due to regex backtracking in the tokenization engine. Real-world text with varied content works fine at large sizes.

- **Workaround**: Break extremely large repetitive inputs into smaller chunks
- **Impact**: Minimal - real documents rarely exhibit this pathological pattern
- **Status**: Tracked upstream in tiktoken-rs

### Best Practices

**For CI/CD Pipelines**:
```bash
# Limit concurrent processes to avoid resource exhaustion
ulimit -n 1024                    # Limit file descriptors
ulimit -v $((500 * 1024))        # Limit virtual memory to 500MB
echo "text" | token-count --model gpt-4
```

**For Untrusted Input**:
```bash
# Use timeout to prevent hangs
timeout 30s token-count --model gpt-4 < input.txt
```

**For Large Files**:
```bash
# Monitor memory usage
/usr/bin/time -v token-count --model gpt-4 < large-file.txt
```

### Security Audit

- **Last audit**: 2026-03-14
- **Findings**: 0 critical, 0 high, 0 medium vulnerabilities
- **Dependencies**: Audited with `cargo audit`

Run security checks:
```bash
cargo audit                      # Check for known vulnerabilities
cargo clippy -- -D warnings     # Strict linting
```

### Reporting Security Issues

If you discover a security vulnerability, please email hello@burdick.dev (or open a private security advisory on GitHub). Do not open public issues for security concerns.

## Architecture


### Design Principles

From our [Constitution](.specify/memory/constitution.md):

1. **POSIX Simplicity** - Behaves like standard Unix utilities
2. **Accuracy Over Speed** - Exact tokenization for supported models
3. **Zero Runtime Dependencies** - Single offline binary
4. **Fail Fast with Clear Errors** - No silent failures
5. **Semantic Versioning** - Predictable upgrade paths

### Technical Stack

- **Language**: Rust 1.85.0+ (stable)
- **CLI Parsing**: clap 4.6.0+ (derive API)
- **Tokenization**: 
  - tiktoken-rs 0.9.1+ (OpenAI models - offline)
  - Adaptive estimation algorithm (Claude models - offline)
  - Anthropic API via reqwest 0.12+ (Claude accurate mode - optional)
- **Async Runtime**: tokio 1.0+ (for API calls)
- **Error Handling**: anyhow 1.0.102+, thiserror 1.0+
- **Fuzzy Matching**: strsim 0.11+ (Levenshtein distance)
- **Testing**: 177 tests with criterion benchmarks

### Key Features

- **Library-first design**: Core logic in `lib.rs`, thin binary wrapper
- **Trait-based abstractions**: Extensible for future tokenizers
- **Strategy pattern**: Multiple output formatters
- **Registry pattern**: Model configuration with lazy initialization
- **Streaming support**: 64KB chunks for large inputs

## Contributing

Contributions are welcome! This project follows specification-driven development.

### Development Setup

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed instructions.

Quick start:
```bash
git clone https://github.com/shaunburdick/token-count
cd token-count
cargo test
cargo clippy
```

### Code Quality Standards

- **No disabled lint rules** - Fix code to comply, don't silence warnings
- **100% type safety** - No `any` types or suppressions
- **All public APIs documented** - With examples
- **Test coverage** - All user stories covered
- **Zero clippy warnings** - Strict linting enforced

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

Built with:
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) - Rust tiktoken implementation
- [clap](https://github.com/clap-rs/clap) - Command line argument parser
- [spec-kit](https://github.com/github/spec-kit) - Specification-driven development

Special thanks to:
- OpenAI for open-sourcing tiktoken
- The Rust community for excellent tooling

---

**Status**: ✅ v0.3.0 Complete (Gemini Support) | **Version**: 0.3.0  
**Author**: [Shaun Burdick](https://github.com/shaunburdick)
