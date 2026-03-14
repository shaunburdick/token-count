# token-count

> A fast, accurate CLI tool for counting tokens in LLM model inputs

[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-100%20passing-brightgreen.svg)](tests/)

## Overview

`token-count` is a POSIX-style command-line tool that counts tokens for various LLM models using exact tokenization. Pipe any text in, get accurate token counts out—no browser, no API calls, just a fast offline binary.

```bash
# Quick token count
echo "Hello world" | token-count --model gpt-4
2

# From file
token-count --model gpt-4 < document.txt
1842

# With context info
cat prompt.txt | token-count --model gpt-4 -v
Model: gpt-4 (cl100k_base)
Tokens: 142
Context window: 128000 tokens (0.1109% used)
```

## Features

✅ **Accurate** - Exact tokenization using OpenAI's tiktoken library  
✅ **Fast** - ~2.7µs for small inputs (3,700x faster than 10ms target)  
✅ **Efficient** - 57MB memory for 12MB files (8.8x under 500MB limit)  
✅ **Compact** - 9.2MB binary with all tokenizers embedded  
✅ **Offline** - Zero runtime dependencies, all tokenizers built-in  
✅ **Simple** - POSIX-style interface, works like `wc` or `grep`

## Installation

### From Source (Requires Rust 1.85.0+)

```bash
cargo install --git https://github.com/shaunburdick/token-count
```

### Manual Build

```bash
git clone https://github.com/shaunburdick/token-count
cd token-count
cargo build --release
# Binary at: target/release/token-count
```

### System Requirements

- **Platform**: Linux (Ubuntu 22.04+)
- **Rust**: 1.85.0 or later (for building)
- **Runtime**: No dependencies (static binary)

**Note**: macOS and Windows support coming in future releases.

## Usage

### Basic Usage

```bash
# Default model (gpt-3.5-turbo)
echo "Hello world" | token-count
2

# Specific model
echo "Hello world" | token-count --model gpt-4
2

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
1

# Verbose (-v) - model info and context usage
echo "test" | token-count -v
Model: gpt-3.5-turbo (cl100k_base)
Tokens: 1
Context window: 16385 tokens (0.0061% used)

# Debug (-vvv) - for troubleshooting
echo "test" | token-count -vvv
Model: gpt-3.5-turbo (cl100k_base)
Tokens: 1
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

### OpenAI Models (Exact Tokenization)

| Model | Encoding | Context Window | Aliases |
|-------|----------|----------------|---------|
| gpt-3.5-turbo | cl100k_base | 16,385 | gpt-3.5, gpt35, gpt-35-turbo |
| gpt-4 | cl100k_base | 128,000 | gpt4 |
| gpt-4-turbo | cl100k_base | 128,000 | gpt4-turbo, gpt-4turbo |
| gpt-4o | o200k_base | 128,000 | gpt4o |

All models support:
- Case-insensitive names (e.g., `GPT-4`, `gpt-4`, `Gpt-4`)
- Provider prefix (e.g., `openai/gpt-4`)

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

## Performance

### Benchmarks

Measured on Ubuntu 22.04 with Rust 1.85.0:

| Input Size | Time | Target | Result |
|------------|------|--------|--------|
| 100 bytes | 2.7µs | <10ms | 3,700x faster ⚡ |
| 1 KB | 54µs | <100ms | 1,850x faster ⚡ |
| 10 KB | 534µs | N/A | Excellent |

### Memory Usage

- **12MB file**: 57 MB resident memory (8.8x under 500MB limit)
- **Processing time**: 0.76 seconds for 12MB
- **No memory leaks**: Validated with valgrind

### Binary Size

- **Release binary**: 9.2 MB (5.4x under 50MB target)
- **Includes**: All 4 OpenAI tokenizers embedded
- **Optimizations**: Stripped, LTO enabled

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
│   │   ├── openai.rs       # OpenAI tokenizer
│   │   ├── registry.rs     # Model registry
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

- **Last audit**: 2026-03-13
- **Findings**: 0 critical, 0 high, 0 medium vulnerabilities
- **Dependencies**: 5 direct, all audited with `cargo audit`
- **Binary**: Stripped, no debug symbols, 9.2MB

Run security checks:
```bash
cargo audit                      # Check for known vulnerabilities
cargo clippy -- -D warnings     # Strict linting
```

### Reporting Security Issues

If you discover a security vulnerability, please email security@token-count.dev (or open a private security advisory on GitHub). Do not open public issues for security concerns.

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
- **Tokenization**: tiktoken-rs 0.9.1+ (OpenAI models)
- **Error Handling**: anyhow 1.0.102+, thiserror 1.0+
- **Fuzzy Matching**: strsim 0.11+ (Levenshtein distance)
- **Testing**: 100 tests with criterion benchmarks

### Key Features

- **Library-first design**: Core logic in `lib.rs`, thin binary wrapper
- **Trait-based abstractions**: Extensible for future tokenizers
- **Strategy pattern**: Multiple output formatters
- **Registry pattern**: Model configuration with lazy initialization
- **Streaming support**: 64KB chunks for large inputs

## Roadmap

### v0.1.0 (Current - Linux MVP) ✅

- [x] OpenAI model support (4 models)
- [x] CLI with model selection and verbosity
- [x] Fuzzy model suggestions
- [x] UTF-8 validation with error reporting
- [x] Comprehensive test suite (100 tests)
- [x] Performance benchmarks
- [x] Linux support (Ubuntu 22.04+)

### v0.2.0 (Future - Cross-Platform)

- [ ] macOS support (Intel + Apple Silicon)
- [ ] Windows support
- [ ] Homebrew installation
- [ ] GitHub release binaries
- [ ] Installation script

### v0.3.0 (Future - More Models)

- [ ] Anthropic Claude support
- [ ] Google Gemini support
- [ ] Meta Llama support
- [ ] Mistral support

### v1.0.0 (Future - Stable API)

- [ ] Stable library API for embedding
- [ ] Token ID output (debug mode)
- [ ] Batch processing mode
- [ ] Configuration file support

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

**Status**: ✅ MVP Complete (Linux) | **Version**: 0.1.0  
**Author**: [Shaun Burdick](https://github.com/shaunburdick)
