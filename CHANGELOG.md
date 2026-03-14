# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-13

### 🎉 Initial Release - Linux MVP

The first release of `token-count` provides accurate token counting for OpenAI models on Linux.

### Added

#### Core Features
- **Token counting** for OpenAI models using exact tiktoken tokenization
- **4 supported models**: gpt-3.5-turbo, gpt-4, gpt-4-turbo, gpt-4o
- **12+ model aliases** with case-insensitive matching (e.g., `gpt4`, `GPT-4`, `openai/gpt-4`)
- **Fuzzy model suggestions** for typos using Levenshtein distance (≤3 edits)
- **3 verbosity levels**:
  - Level 0 (default): Simple token count
  - Level 1-2 (`-v`, `-vv`): Verbose output with model info and context percentage
  - Level 3+ (`-vvv`): Debug mode with diagnostic information
- **Stdin input support** for piping and file redirection
- **UTF-8 validation** with byte offset error reporting
- **Exit code mapping**: 0 (success), 1 (I/O/UTF-8 error), 2 (unknown model)

#### CLI Interface
- `--model <MODEL>` - Select tokenization model (default: gpt-3.5-turbo)
- `-v, --verbose` - Increase output verbosity (repeatable)
- `--list-models` - List all supported models with details
- `--help` - Display help information
- `--version` - Display version information

#### Documentation
- Comprehensive README with usage examples
- API documentation with examples
- 91 tests covering all functionality
- Performance benchmarks
- CHANGELOG following Keep a Changelog format

### Performance

#### Benchmarks (Release Build)
- **Small input** (100 bytes): ~2.7µs (3,700x faster than 10ms target)
- **Medium input** (1KB): ~54µs (1,850x faster than 100ms target)
- **Large input** (10KB): ~534µs

#### Resource Usage
- **Memory**: 57 MB for 12MB file (8.8x under 500MB limit)
- **Binary size**: 9.2 MB (5.4x under 50MB target)
- **Processing**: 0.76s for 12MB file

### Technical Details

#### Stack
- **Rust**: 1.85.0 MSRV (Minimum Supported Rust Version)
- **Dependencies**:
  - clap 4.6+ (CLI parsing)
  - tiktoken-rs 0.9.1+ (OpenAI tokenization)
  - anyhow 1.0.102+ (error handling)
  - thiserror 1.0+ (error types)
  - strsim 0.11+ (fuzzy matching)

#### Architecture
- Library-first design with thin binary wrapper
- Trait-based tokenizer abstraction
- Strategy pattern for output formatting
- Registry pattern for model configuration
- Zero runtime dependencies (all tokenizers embedded)

#### Quality Assurance
- **91 tests passing** (13 test suites)
- **Zero clippy warnings** (strict linting)
- **Zero security vulnerabilities** (cargo audit)
- **100% type safety** (no suppressions)
- **CI/CD**: Automated testing on ubuntu-22.04

### Platform Support

#### Supported
- ✅ **Linux**: Ubuntu 22.04+ (tested and validated)

#### Not Yet Supported
- ❌ **macOS**: Planned for v0.2.0
- ❌ **Windows**: Planned for v0.2.0

### Known Limitations

1. **Linux-only MVP**: macOS and Windows support coming in v0.2.0
2. **OpenAI models only**: Other providers (Claude, Gemini, Llama) planned for v0.3.0
3. **Debug mode placeholder**: Full token ID display requires tokenizer API enhancements
4. **No streaming output**: Counts only (not individual tokens)

These are intentional MVP scope limitations, not bugs.

### Installation

```bash
# From source (requires Rust 1.85.0+)
cargo install --git https://github.com/shaunburdick/token-count

# Manual build
git clone https://github.com/shaunburdick/token-count
cd token-count
cargo build --release
```

### Examples

```bash
# Basic usage
echo "Hello world" | token-count --model gpt-4
2

# Verbose output
cat document.txt | token-count --model gpt-4 -v
Model: gpt-4 (cl100k_base)
Tokens: 1842
Context window: 128000 tokens (1.4391% used)

# List models
token-count --list-models

# Error handling
echo "test" | token-count --model gpt5
Error: Unknown model: 'gpt5'. Did you mean: gpt-4, gpt-4o?
```

### Migration Notes

This is the initial release. No migration required.

### Contributors

- Shaun Burdick ([@shaunburdick](https://github.com/shaunburdick)) - Initial implementation

---

## [Unreleased]

### Planned for v0.2.0
- macOS support (Intel + Apple Silicon)
- Windows support
- Homebrew tap for easy installation
- GitHub release binaries
- Installation script

### Planned for v0.3.0
- Anthropic Claude model support
- Google Gemini model support
- Meta Llama model support
- Mistral model support

### Planned for v1.0.0
- Stable library API
- Token ID output in debug mode
- Batch processing mode
- Configuration file support

---

[0.1.0]: https://github.com/shaunburdick/token-count/releases/tag/v0.1.0
