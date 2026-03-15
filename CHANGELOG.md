# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **MSRV Bump**: Updated Minimum Supported Rust Version from 1.85.0 to 1.86.0
  - Required for `io::Error::other()` API (used in `src/api/consent.rs`)
  - Provides cleaner, more idiomatic error handling
  - Impact: Users building from source need Rust 1.86.0+ (Feb 2025)
  - Pre-built binaries unaffected

## [0.4.0] - 2026-03-14

### 🐛 Debug Mode with Token ID Display

The fourth release adds comprehensive debug mode functionality (`-vvv`) that displays individual token IDs and their decoded text representations. Also differentiates all verbosity levels and includes security hardening against DoS attacks.

### Added

#### Debug Mode Features
- **Token ID Display** (`-vvv` flag): Shows token IDs for the first 10 tokens
- **Decoded Token Text**: Displays the text representation of each token
- **Differentiated Verbosity Levels**:
  - Level 0 (default): Just the token count
  - Level 1 (`-v`): Model name + encoding + token count
  - Level 2 (`-vv`): Add context window size + percentage
  - Level 3 (`-vvv`): Add token IDs + decoded tokens
- **Multi-Tokenizer Support**: Works with OpenAI (tiktoken-rs) and Gemini models
- **Claude Handling**: Shows "estimation-based" message (no real token IDs)

#### Security Hardening
- **DoS Prevention**: 50KB input size limit for debug mode to prevent stack overflow
- **Graceful Degradation**: Large inputs show warning and fall back to token-count-only mode
- **No Application Crashes**: Prevents unrecoverable panic from tiktoken-rs recursion issues

#### Testing
- **4 new tests** (regression tests for debug mode security)
- **Total: 181 tests** (increased from 178)
- All tests passing with zero clippy warnings

### Changed

- **Test count badge**: Updated from 178 to 181 passing tests
- **Verbosity behavior**: `-v` and `-vv` now show different output (previously identical)
- **Binary size**: Increased from ~11.5MB to ~16.8MB (includes debug token decoding)
- **README**: Updated with debug mode examples and security documentation

### Fixed

- **Stack Overflow Protection**: tiktoken-rs can crash with large inputs in debug mode; now limited to 50KB with graceful degradation
- **Verbosity Differentiation**: All 4 levels now produce distinct output as originally specified

### Technical Details

#### New/Modified Modules
- `src/output/basic.rs` - New `BasicFormatter` for level 1 (`-v`)
- `src/tokenizers/mod.rs` - Added `TokenDetail` struct with `PartialEq`/`Eq` derives
- `src/tokenizers/openai.rs` - Implemented `encode_with_details()` with 50KB safety limit
- `src/tokenizers/google/mod.rs` - Implemented `encode_with_details()` via `compute_tokens()`
- `src/tokenizers/claude/mod.rs` - Returns `None` for estimation-based tokenizer
- `src/output/debug.rs` - Updated to display token IDs and decoded text
- `src/lib.rs` - Updated `count_tokens()` signature to accept `verbosity: u8`

#### Security Improvements
- **Input Size Limit**: `MAX_DEBUG_INPUT_SIZE` constant (50KB) in OpenAI tokenizer
- **Graceful Error Handling**: User-friendly warning message for oversized inputs
- **Regression Tests**: Added tests for both large (60KB) and normal inputs

### Limitations

- **Debug mode input limit**: 50KB maximum for token ID display (normal tokenization supports 100MB)
- **Token display limit**: Shows first 10 tokens only to avoid overwhelming output
- **Claude models**: No token IDs available (estimation-based, not exact tokenization)

### References

- Upstream tiktoken-rs issues: [#327](https://github.com/zurawiki/tiktoken-rs/issues/327), [#245](https://github.com/zurawiki/tiktoken-rs/issues/245), [#400](https://github.com/zurawiki/tiktoken-rs/issues/400)
- Pull Request: [#4](https://github.com/shaunburdick/token-count/pull/4)

---

## [0.3.0] - 2026-03-14

### 🚀 Google Gemini Model Support

The third release adds support for Google Gemini models with exact, offline tokenization using the gemini-tokenizer library. Completes the "Big 3" LLM providers (OpenAI, Anthropic, Google).

### Added

#### Gemini Tokenization
- **4 Gemini models** with exact offline tokenization:
  - `gemini-2.5-pro` - Pro model (GA, 1M context, deprecated June 2026)
  - `gemini-2.5-flash` - Default model (GA, 1M context, deprecated June 2026)
  - `gemini-2.5-flash-lite` - Lite model (GA, 1M context, deprecated June 2026)
  - `gemini-3-pro-preview` - Preview model (1M context)
- **Offline tokenization** using Gemma 3 SentencePiece tokenizer (via `gemini-tokenizer` v0.2.0)
- **Model aliases**:
  - `gemini` → `gemini-2.5-flash` (default)
  - `gemini-pro` → `gemini-2.5-pro`
  - `gemini-flash` → `gemini-2.5-flash`
  - `gemini-lite` → `gemini-2.5-flash-lite`
  - `gemini-3-pro` → `gemini-3-pro-preview`
  - Provider prefix: `google/gemini`, `google/gemini-pro`, etc.
- **Case-insensitive model names**: `GEMINI`, `Gemini`, `gemini` all work

#### Testing
- **27 new tests** (10 unit + 17 integration tests)
- **Total: 178 tests** (increased from 152)
- All tests passing with zero clippy warnings

### Changed

- **Test count badge**: Updated from 152 to 178 passing tests
- **Binary size**: Increased from 9.2MB to ~11.5MB (added SentencePiece tokenizer)
- **Supported models**: Now 11 total models (4 OpenAI + 3 Claude + 4 Gemini)
- **README**: Updated with Gemini examples and CMake build requirement

### Technical Details

#### New Modules
- `src/tokenizers/google/` - Google Gemini tokenizer implementation
  - `mod.rs` - Main tokenizer with `Tokenizer` trait implementation
  - `models.rs` - 4 model definitions with 14+ aliases
  - `tokenizer.rs` - Wrapper around `gemini-tokenizer` crate

#### Dependencies Added
- `gemini-tokenizer` 0.2.0 - Official Google tokenizer (via sentencepiece)
- Transitive: `sentencepiece`, `sentencepiece-sys`, `sha2`

#### Build Requirements
- **CMake 3.10+** now required for building from source (SentencePiece dependency)
- No change to runtime requirements (still zero dependencies for end users)

#### Registry Updates
- Extended model registry to support `gemini-gemma3` encoding
- Added Gemini models to `--list-models` output
- Model count increased from 7 to 11

### Notes

- **Model Scope**: Initial spec planned for 8 Gemini models, but `gemini-tokenizer` v0.2.0 only supports 4 models. Additional models (gemini-1.5-*, gemini-3.1-*) will be added when upstream library adds support.
- **Default Model**: Uses `gemini-2.5-flash` (not `gemini-3-flash-preview`) since it's still GA until June 2026.
- **Build-time vs Runtime**: CMake is a build-time dependency only; pre-built binaries don't require users to have CMake installed.

## [0.2.0] - 2026-03-14

### 🚀 Claude Model Support

The second release adds support for Anthropic Claude models with a hybrid tokenization approach: fast offline estimation by default, with optional exact API counting.

### Added

#### Claude Tokenization
- **3 Claude models**: claude-opus-4-6, claude-sonnet-4-6, claude-haiku-4-5
- **Adaptive token estimation algorithm** with content-type detection:
  - **Code detection**: Identifies code by punctuation density (`{}[]();:,<>`) and keywords → 3.0 chars/token
  - **Prose detection**: Natural language text → 4.5 chars/token  
  - **Mixed content**: Markdown with code blocks → 3.75 chars/token
  - **Target accuracy**: ±10% for typical inputs
- **Optional accurate mode** via Anthropic API (`--accurate` flag)
  - Requires `ANTHROPIC_API_KEY` environment variable
  - Interactive consent prompt before API calls
  - Graceful fallback to estimation on API errors
- **Model aliases**: 
  - `claude` → `claude-sonnet-4-6` (default)
  - Short names: `opus`, `sonnet`, `haiku`
  - Version variants: `opus-4-6`, `opus-4.6`
  - Provider prefix: `anthropic/claude-sonnet-4-6`

#### CLI Enhancements
- `--accurate` flag - Use API for exact token counts (Claude models only)
- `-y, --yes` flag - Skip API consent prompt for automation/scripting
- **Interactive consent prompt** for API calls:
  - Shows provider, API endpoint, and data usage notice
  - TTY detection for interactive vs non-interactive mode
  - Clear error messages with examples when consent required
- **Non-interactive mode handling**:
  - Detects piped input (stdin not a TTY)
  - Requires `-y` flag or returns helpful error
  - Example: `cat file.txt | token-count --model claude --accurate -y`

#### API Integration
- **Anthropic API client** with robust error handling:
  - Exponential backoff retry logic (3 attempts: 2s, 4s, 8s delays)
  - 30-second timeout per request
  - Proper error mapping (rate limits, server errors, invalid API keys)
- **Async runtime** (tokio) for API calls while maintaining sync CLI interface
- **Security features**:
  - API keys never logged or exposed in errors
  - HTTPS-only with certificate verification  
  - No caching of API responses (short-lived CLI sessions)

#### Testing
- **21 new integration tests** for Claude estimation and API modes
- **9 API-specific tests** (error handling, consent, fallback behavior)
- **12 estimation tests** (content types, models, aliases, edge cases)
- **Total: 152 tests** (increased from 131)
- All tests pass sequentially (some env var conflicts in parallel execution)

### Changed

- **Test count badge**: Updated from 100 to 152 passing tests
- **OpenAI accurate mode**: `--accurate` flag now only affects Claude models (OpenAI always uses offline tiktoken)
- **Model registry**: Extended to support multiple tokenization strategies (offline vs API-based)
- **Error messages**: Enhanced with Claude-specific errors (missing API key, invalid key, consent required)
- **Dependencies**: Added reqwest 0.12, tokio 1.0, serde 1.0.149, serde_json 1.0.149

### Technical Details

#### New Modules
- `src/tokenizers/claude/` - Claude tokenizer implementation
  - `mod.rs` - Main tokenizer with hybrid API/estimation logic
  - `estimation.rs` - Adaptive content-type detection and estimation
  - `api_client.rs` - Anthropic API client with retry logic
  - `models.rs` - Model definitions and aliases
- `src/api/` - API utilities
  - `consent.rs` - Interactive consent prompt with TTY detection

#### Architecture
- **Hybrid tokenization strategy**: Estimation by default, API on demand
- **Consent pattern**: Reusable for future API providers (OpenAI, Gemini)
- **Content-type detection**: Code vs prose vs mixed content analysis
- **Graceful degradation**: API failures fall back to estimation with warning

#### Performance
- **Estimation speed**: ~5-10µs for small inputs (similar to tiktoken)
- **API mode**: ~200-500ms for API round-trip (network dependent)
- **Fallback**: Automatic estimation if API unavailable (no user intervention)

### Examples

```bash
# Offline estimation (default, no API key needed)
echo "Hello, Claude!" | token-count --model claude
9

# Model aliases work
token-count --model sonnet < document.txt
412

# Verbose output with context
cat prompt.txt | token-count --model claude-opus-4-6 -v
Model: claude-opus-4-6 (anthropic-claude)
Tokens: 142
Context window: 1000000 tokens (0.0142% used)

# Accurate mode with API (requires ANTHROPIC_API_KEY and consent)
export ANTHROPIC_API_KEY="sk-ant-..."
echo "test" | token-count --model claude --accurate
# Prompts: "This will send your input to Anthropic's API... Proceed? (y/N)"
# y
# 1

# Skip consent for automation
cat file.txt | token-count --model claude --accurate -y
842

# Error handling - missing API key
token-count --model claude --accurate -y < input.txt
Error: Accurate mode requires ANTHROPIC_API_KEY environment variable.

Get your API key from: https://console.anthropic.com/
Then set: export ANTHROPIC_API_KEY="sk-ant-..."

For offline estimation (no API key needed), omit --accurate flag:
  token-count --model claude-sonnet-4-6

# Error handling - non-interactive without -y
cat file.txt | token-count --model claude --accurate
Error: API call requires consent. Running in non-interactive mode (stdin not a TTY).

Options:
  1. Add -y/--yes flag to skip prompt:
     cat file.txt | token-count --model claude --accurate -y
  
  2. Use estimation mode (no API call):
     cat file.txt | token-count --model claude
```

### Migration Notes

**For users upgrading from v0.1.0:**
- No breaking changes - all existing commands work identically
- New `--accurate` flag is optional and only affects Claude models
- OpenAI models continue using offline tiktoken (no API calls)

**For library users:**
- `count_tokens()` signature changed: added `accurate: bool` parameter
- Update calls: `count_tokens(text, model, false)` for existing behavior

### Known Limitations

1. **Claude estimation accuracy**: ±10% for typical inputs; may vary for unusual content
2. **API rate limits**: Anthropic rate limits apply when using `--accurate` mode
3. **Test environment variables**: Some tests conflict in parallel execution due to shared env vars (all pass sequentially)
4. **No Claude token caching**: API responses not cached (short CLI session doesn't benefit)

### Contributors

- Shaun Burdick ([@shaunburdick](https://github.com/shaunburdick)) - Claude implementation

---

## [0.1.0] - 2026-03-14

### 🎉 Initial Release

The first release of `token-count` provides accurate token counting for OpenAI models across Linux, macOS, and Windows with multiple installation methods.

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

#### Installation Methods
- **Install script** (`install.sh`) - One-line installation for Linux/macOS with checksum verification
- **Homebrew tap** - `brew install shaunburdick/tap/token-count` (macOS/Linux)
- **Cargo** - `cargo install token-count` (published to crates.io)
- **Pre-built binaries** - GitHub Releases with 4 platform targets
- **Manual build** - From source with Rust 1.85.0+

#### Distribution & Automation
- **GitHub Actions release workflow** with multi-platform builds
- **Automated Homebrew formula updates** on each release
- **SHA256 checksums** for all binary downloads
- **Signed releases** on GitHub with detailed release notes

#### Documentation
- Comprehensive README with updated installation section
- Detailed INSTALL.md with platform-specific instructions
- SECURITY.md with vulnerability reporting process
- API documentation with examples
- 100 tests covering all functionality
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
- **100 tests passing** (integration + unit tests)
- **Zero clippy warnings** (strict linting)
- **Zero security vulnerabilities** (cargo audit)
- **100% type safety** (no suppressions)
- **CI/CD**: Automated testing on ubuntu-22.04

### Platform Support

#### Fully Supported
- ✅ **Linux x86_64**: Ubuntu 22.04+ (tested and validated)
- ✅ **macOS Intel (x86_64)**: macOS 10.15+ (Catalina)
- ✅ **macOS Apple Silicon (aarch64)**: macOS 11.0+ (Big Sur)
- ✅ **Windows x86_64**: Windows 10+ (tested)

#### Planned
- ⏳ **Linux ARM64**: Build from source available
- ⏳ **Windows ARM64**: Future consideration

### Known Limitations

1. **OpenAI models only**: Other providers (Claude, Gemini, Llama) planned for v0.2.0
2. **Debug mode placeholder**: Full token ID display requires tokenizer API enhancements
3. **No streaming output**: Counts only (not individual tokens)
4. **Stack overflow with pathological inputs**: Highly repetitive single-character inputs (1MB+) can cause stack overflow due to regex backtracking in tiktoken-rs

These are intentional MVP scope limitations, not bugs.

### Installation

```bash
# Quick install (Linux/macOS)
curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash

# Homebrew (macOS/Linux)
brew install shaunburdick/tap/token-count

# Cargo (all platforms)
cargo install token-count

# Manual download
# Visit: https://github.com/shaunburdick/token-count/releases
```

See [INSTALL.md](INSTALL.md) for detailed instructions.

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

### Planned for v0.3.0
- Google Gemini model support
- Meta Llama model support
- Mistral model support

### Planned for v0.4.0
- Stable library API
- Token ID output in debug mode
- Batch processing mode
- Configuration file support

---

[0.4.0]: https://github.com/shaunburdick/token-count/releases/tag/v0.4.0
[0.3.0]: https://github.com/shaunburdick/token-count/releases/tag/v0.3.0
[0.2.0]: https://github.com/shaunburdick/token-count/releases/tag/v0.2.0
[0.1.0]: https://github.com/shaunburdick/token-count/releases/tag/v0.1.0
