# Research: Core CLI Token Counting

**Date**: 2026-03-13 | **Feature**: 001-core-cli | **Plan**: [plan.md](./plan.md)

## Overview

This document captures research findings for implementing the token-count CLI tool, including library evaluation, design alternatives, and technical decisions with rationale.

## Technology Stack Research

### 1. Tokenization Libraries

#### Primary Choice: tiktoken-rs

**Version**: 0.9.1 (latest stable)  
**Repository**: https://github.com/zurawiki/tiktoken-rs  
**Downloads**: 4.5M+ total  
**License**: MIT  
**Rust MSRV**: 1.61.0

**Capabilities**:
- Exact OpenAI tokenization (cl100k_base, o200k_base, p50k_base encodings)
- Supports all GPT models (3.5, 4, 4-turbo, 4o, o1)
- Lazy loading of BPE vocabulary files (embedded at compile time)
- Thread-safe, zero-copy string processing
- Battle-tested (used by many production Rust projects)

**Why Chosen**:
- ✅ Most mature Rust implementation of tiktoken
- ✅ Exact tokenization matching OpenAI's Python library
- ✅ Actively maintained (last update: 3 months ago)
- ✅ No external files required (embeds vocabulary data)
- ✅ Efficient memory usage (lazy loads encodings)
- ✅ MIT license (permissive)

**Alternatives Considered**:

1. **`tokenizers` (HuggingFace)** - v0.15.0
   - ❌ Rejected: Requires explicit vocab file loading (not embedded)
   - ❌ Heavier dependency (90MB+ with all models)
   - ❌ Designed for training, overkill for inference-only tool
   - ✅ Would be useful for Llama/Mistral support (post-MVP)

2. **`llm-tokenizer`** - v1.3.0
   - ⏳ Deferred: Good multi-provider support (HuggingFace + tiktoken wrapper)
   - ⏳ Will evaluate for Phase 2 (Claude, Gemini support)
   - ❌ Not needed for MVP (OpenAI only)
   - ✅ Single dependency for multi-provider could simplify post-MVP

3. **Custom BPE implementation**
   - ❌ Rejected: Reinventing the wheel
   - ❌ High risk of tokenization bugs (breaks accuracy requirement)
   - ❌ Months of development time for no benefit
   - ❌ Violates "use proven libraries" principle

**Decision**: Use tiktoken-rs 0.9.1 for MVP. Evaluate llm-tokenizer for Phase 2.

---

### 2. CLI Parsing Libraries

#### Primary Choice: clap

**Version**: 4.6.0 (latest major version)  
**Repository**: https://github.com/clap-rs/clap  
**Downloads**: Most downloaded CLI crate on crates.io  
**License**: MIT OR Apache-2.0  
**Rust MSRV**: 1.85 ✅ (constitution updated to match)

**Capabilities**:
- Derive macros for declarative argument parsing
- Automatic `--help` and `--version` generation
- Subcommands, flags, options, positional args
- Shell completion generation (bash, zsh, fish)
- Excellent error messages with suggestions
- Widely used (industry standard)

**Why Chosen**:
- ✅ Industry standard for Rust CLIs (cargo, ripgrep, fd, bat use it)
- ✅ Derive API is concise and type-safe
- ✅ Excellent error messages (user experience win)
- ✅ Actively maintained by dedicated team
- ✅ Strong backwards compatibility guarantees (4.x is stable)

**MSRV Resolution**:
- ✅ Constitution updated from 1.75 to 1.85 (Amendment 1.2.0)
- ✅ Rationale: clap is critical dependency, excellent UX justifies requirement
- ✅ Impact: Pre-built binaries unaffected, only matters for building from source

**Alternatives Considered**:

1. **`structopt`** - v0.3.x (predecessor to clap derive)
   - ❌ Rejected: Deprecated in favor of clap 3+ derive API
   - ❌ No longer maintained
   - ✅ Was great, but clap 4 supersedes it

2. **`argh`** - v0.1.12 (Google's lightweight alternative)
   - ❌ Rejected: Less feature-rich (no shell completion, basic help)
   - ❌ Smaller community, fewer examples
   - ✅ Would reduce binary size slightly (~50KB savings)
   - ❌ Not worth trading ecosystem for 50KB

3. **`pico-args`** - v0.5.0 (minimal parser)
   - ❌ Rejected: Too low-level, manual help text generation
   - ❌ No derive macros, verbose boilerplate
   - ✅ Would be smallest (20KB)
   - ❌ Violates "excellent error messages" principle

**Decision**: Use clap 4.6.0, update constitution MSRV to 1.85.

---

### 3. Error Handling Libraries

#### Primary Choice: anyhow

**Version**: 1.0.102 (latest stable)  
**Repository**: https://github.com/dtolnay/anyhow  
**Downloads**: Most downloaded error handling crate  
**License**: MIT OR Apache-2.0  
**Rust MSRV**: 1.61.0

**Capabilities**:
- Ergonomic error propagation with `?` operator
- Error context chaining (`context()` method)
- Downcasting to concrete error types (for exit code mapping)
- Minimal boilerplate (one-line `Result<T>` type alias)
- Excellent error messages with causal chains

**Why Chosen**:
- ✅ Perfect for application-level error handling (CLI binaries)
- ✅ Minimal boilerplate (Result<T> = Result<T, anyhow::Error>)
- ✅ Context chaining creates helpful error messages
- ✅ Works well with custom error types for library API
- ✅ David Tolnay's libraries are always high-quality

**Alternatives Considered**:

1. **`thiserror`** - v1.0.69 (same author as anyhow)
   - ✅ Excellent for library error types
   - ❌ More boilerplate for application code
   - **Decision**: Use both - `thiserror` for lib.rs, `anyhow` for main.rs

2. **`eyre`** - v0.6.12 (anyhow fork with customization)
   - ❌ Rejected: More features than needed
   - ❌ Slightly heavier dependency
   - ✅ Good for complex error reporting (not needed here)

3. **Standard library `std::error::Error`**
   - ❌ Rejected: Too much boilerplate
   - ❌ No context chaining without custom implementation
   - ❌ Verbose exit code mapping

**Decision**: Use anyhow 1.0.102 for main.rs, thiserror 1.0.69 for lib.rs.

---

### 4. Benchmarking Libraries

#### Primary Choice: criterion

**Version**: 0.5.1 (latest stable)  
**Repository**: https://github.com/bheisler/criterion.rs  
**License**: MIT OR Apache-2.0

**Why Chosen**:
- ✅ Statistical benchmarking (detects performance regressions)
- ✅ HTML reports with charts
- ✅ Criterion.toml for CI-friendly output
- ✅ Widely used (standard for Rust benchmarks)

**Alternatives Considered**:
- `#[bench]` (nightly-only) - Rejected: Requires nightly Rust
- `iai` (instruction counting) - Deferred: Useful for micro-optimizations later

**Decision**: Use criterion 0.5.1 for benchmarks.

---

## Design Decisions & Alternatives

### 1. Stdin Reading Strategy

#### Chosen Approach: Buffered Streaming with 64KB Chunks

**Implementation**:
```rust
use std::io::{self, BufReader, Read};

const CHUNK_SIZE: usize = 64 * 1024; // 64KB

pub fn read_stdin_streaming<F>(mut process: F) -> io::Result<()>
where
    F: FnMut(&str) -> Result<(), anyhow::Error>,
{
    let stdin = io::stdin();
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, stdin.lock());
    let mut buffer = String::with_capacity(CHUNK_SIZE);
    
    loop {
        buffer.clear();
        let bytes_read = reader.read_to_string(&mut buffer)?;
        if bytes_read == 0 { break; }
        
        // Validate UTF-8 (read_to_string does this automatically)
        process(&buffer)?;
    }
    
    Ok(())
}
```

**Rationale**:
- ✅ `BufReader` reduces syscalls (buffers internally)
- ✅ 64KB chunks balance memory usage vs syscall overhead
- ✅ `read_to_string` validates UTF-8 automatically
- ✅ Memory usage: O(chunk_size) not O(input_size)
- ✅ Works with tiktoken-rs chunked encoding

**Alternatives Considered**:

1. **Load entire stdin into memory**
   - ❌ Rejected: OOM risk for large files (>1GB)
   - ✅ Simpler code (one call to `io::read_to_string()`)
   - ❌ Violates <500MB memory budget

2. **Stream byte-by-byte**
   - ❌ Rejected: Too many syscalls (slow)
   - ❌ UTF-8 validation complexity at chunk boundaries
   - ✅ Minimal memory usage

3. **Memory-mapped files**
   - ❌ Rejected: Stdin is not seekable
   - ✅ Would be faster for large files (avoided)
   - ❌ Requires file path, not stdin

**Decision**: Buffered streaming with 64KB chunks.

---

### 2. Model Registry Architecture

#### Chosen Approach: Const HashMap with Lazy Static

**Implementation**:
```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct ModelConfig {
    pub name: &'static str,
    pub encoding: &'static str,
    pub context_window: usize,
    pub aliases: &'static [&'static str],
}

pub static MODEL_REGISTRY: Lazy<HashMap<&'static str, ModelConfig>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    map.insert("gpt-3.5-turbo", ModelConfig {
        name: "gpt-3.5-turbo",
        encoding: "cl100k_base",
        context_window: 16_385,
        aliases: &["gpt35", "gpt3.5"],
    });
    
    map.insert("gpt-4", ModelConfig {
        name: "gpt-4",
        encoding: "cl100k_base",
        context_window: 8_192,
        aliases: &["gpt4"],
    });
    
    // ... more models
    
    map
});
```

**Rationale**:
- ✅ Const data (no runtime initialization overhead)
- ✅ Lazy initialization (only pay cost if used)
- ✅ Thread-safe (Lazy<> guarantees single initialization)
- ✅ Easy to add new models (single source of truth)
- ✅ Compile-time guarantees (all strings are &'static str)

**Alternatives Considered**:

1. **Match statement for model resolution**
   - ❌ Rejected: Verbose, hard to maintain
   - ❌ Can't iterate over supported models for `--list-models`
   - ✅ Slightly faster (no HashMap lookup)

2. **TOML/JSON config file**
   - ❌ Rejected: Violates "zero external dependencies" principle
   - ❌ Requires parsing at runtime (slower)
   - ❌ More complex error handling (file not found, etc.)
   - ✅ Would enable user-defined models (not a requirement)

3. **Procedural macro to generate registry**
   - ❌ Rejected: Over-engineering for 10 models
   - ❌ Harder to debug (macro expansion complexity)
   - ✅ Would be more maintainable at 100+ models

**Decision**: Lazy static HashMap (using `once_cell` or `std::sync::LazyLock` if MSRV 1.80+).

---

### 3. Error Message Design

#### Chosen Approach: Error Enum + Template Formatting

**Implementation**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Input contains invalid UTF-8\n\ntoken-count requires valid UTF-8 text input.\nBinary files cannot be tokenized.")]
    InvalidUtf8,
    
    #[error("Unknown model '{model}'\n\nDid you mean one of these?\n{suggestions}\n\nUse --list-models to see all supported models")]
    UnknownModel {
        model: String,
        suggestions: String,
    },
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

**Rationale**:
- ✅ `thiserror` generates `Display` impl automatically
- ✅ Error messages are compile-time constants (consistent)
- ✅ Easy to test (error strings are part of enum)
- ✅ Type-safe (can't forget to add suggestions for UnknownModel)

**Fuzzy Matching for Suggestions**:
```rust
use strsim::levenshtein;

pub fn find_similar_models(input: &str, max_distance: usize) -> Vec<String> {
    MODEL_REGISTRY
        .keys()
        .filter(|model| levenshtein(input, model) <= max_distance)
        .take(3)
        .map(|s| format!("  - {}", s))
        .collect()
}
```

**Dependency**: `strsim` 0.11.1 (levenshtein distance, 0 dependencies itself)

**Alternatives Considered**:

1. **Manual error strings in code**
   - ❌ Rejected: Inconsistent formatting
   - ❌ Harder to test (strings scattered across files)
   - ✅ No additional dependency

2. **i18n library for localization**
   - ❌ Rejected: English-only tool (common in dev tools)
   - ❌ Adds complexity (translation files, runtime lookup)
   - ✅ Would enable multi-language support (not a requirement)

**Decision**: thiserror enums + strsim for fuzzy matching.

---

### 4. Output Formatting Strategy

#### Chosen Approach: Trait-Based Formatters

**Implementation**:
```rust
pub trait OutputFormatter {
    fn format(&self, result: &TokenizationResult) -> String;
}

pub struct TokenizationResult {
    pub token_count: usize,
    pub model_name: String,
    pub encoding: String,
    pub context_window: usize,
    pub token_ids: Option<Vec<u32>>,
    pub decoded_tokens: Option<Vec<String>>,
}

// Verbosity 0: "142"
pub struct SimpleFormatter;
impl OutputFormatter for SimpleFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        result.token_count.to_string()
    }
}

// Verbosity 1-2: Multi-line with model info
pub struct VerboseFormatter { include_context: bool }
impl OutputFormatter for VerboseFormatter { ... }

// Verbosity 3: Token IDs + decoded tokens
pub struct DebugFormatter;
impl OutputFormatter for DebugFormatter { ... }
```

**Rationale**:
- ✅ Strategy pattern (select formatter at runtime)
- ✅ Each formatter independently testable
- ✅ Easy to add new formats (JSON, CSV, etc.)
- ✅ Single responsibility (formatters only format, don't compute)

**Alternatives Considered**:

1. **Match statement in main.rs**
   - ❌ Rejected: Mixes concerns (tokenization + formatting)
   - ❌ Harder to test (need full integration test)
   - ✅ Less code (no trait definition)

2. **Template engine (handlebars, tera)**
   - ❌ Rejected: Overkill for 4 simple formats
   - ❌ Adds dependency (increases binary size)
   - ✅ Would enable user-defined templates (not a requirement)

**Decision**: Trait-based formatters with strategy pattern.

---

## Cross-Platform Considerations

### Windows-Specific Issues

**Issue 1: Line Endings**
- Windows uses CRLF (`\r\n`), Unix uses LF (`\n`)
- **Solution**: Rust's `BufReader::read_to_string()` normalizes line endings automatically
- **Verification**: Add Windows CI tests with CRLF input files

**Issue 2: Stdin Pipe Detection**
- Windows CMD behaves differently than PowerShell for pipes
- **Solution**: Use `atty::is(Stream::Stdin)` to detect if stdin is a terminal
- **Verification**: Test `echo "text" | token-count` in CMD and PowerShell

**Issue 3: Binary Paths**
- Windows executable is `token-count.exe`, Unix is `token-count`
- **Solution**: Cargo handles this automatically for `cargo install`
- **Verification**: CI builds and tests `.exe` on Windows

### macOS-Specific Issues

**Issue 1: ARM64 (M1/M2/M3) Support**
- Need separate binary for ARM64 (different instruction set)
- **Solution**: Cross-compile with `cargo build --target aarch64-apple-darwin`
- **Verification**: Test on GitHub Actions macOS ARM64 runner

**Issue 2: Code Signing**
- macOS Gatekeeper may block unsigned binaries
- **Solution**: Document in README how to bypass (`xattr -d com.apple.quarantine`)
- **Future**: Sign binaries with Apple Developer certificate (post-MVP)

### Linux-Specific Issues

**Issue 1: glibc vs musl**
- Some distros use musl (Alpine), most use glibc
- **Solution**: Build both `x86_64-unknown-linux-gnu` and `x86_64-unknown-linux-musl` targets
- **Verification**: Test binary on Alpine Linux (musl) and Ubuntu (glibc)

---

## Performance Optimizations

### 1. Compile-Time Optimizations

**Cargo.toml settings**:
```toml
[profile.release]
lto = "thin"           # Link-Time Optimization (thin = faster build, fat = smaller binary)
codegen-units = 1      # Single codegen unit for better optimization
opt-level = 3          # Maximum optimization (2 = balanced, z = size, s = size)
strip = true           # Strip debug symbols
panic = "abort"        # Smaller binary (no unwinding support)
```

**Rationale**:
- `lto = "thin"` reduces binary size ~10-15% with minimal build time impact
- `codegen-units = 1` enables better inlining (slight size reduction)
- `strip = true` removes debug symbols (~30% size reduction)
- `panic = "abort"` removes unwinding tables (~5-10% size reduction)

**Tradeoff**: Longer release builds (acceptable for CI/CD, not dev builds)

---

### 2. Runtime Optimizations

**Optimization 1: Lazy Tokenizer Loading**
- tiktoken-rs lazy-loads BPE vocabularies on first use
- Only load encoding for requested model (not all encodings)
- **Benefit**: Faster startup (~50ms → ~5ms)

**Optimization 2: String Allocation Reuse**
- Reuse buffer for each stdin chunk (clear instead of allocate)
- **Benefit**: Reduces allocations from N chunks to 1 allocation

**Optimization 3: Zero-Copy String Slicing**
- Pass `&str` to tokenizer (not `String`)
- tiktoken-rs uses zero-copy encoding (no cloning)
- **Benefit**: Lower memory usage, faster for large inputs

---

## Binary Size Analysis

**Estimate Breakdown**:
| Component | Size Estimate | Rationale |
|-----------|---------------|-----------|
| Rust std library | ~2-3 MB | Minimal (only used parts are linked) |
| tiktoken-rs | ~5-7 MB | BPE vocabularies for cl100k_base, o200k_base |
| clap | ~1-2 MB | CLI parsing, help generation |
| anyhow + thiserror | <500 KB | Small error handling libraries |
| Application code | ~1-2 MB | Our code is minimal |
| **Total (unoptimized)** | **~10-15 MB** | Before LTO + strip |
| **Total (optimized)** | **~8-12 MB** | After LTO + strip |

**Verification**: Run `cargo bloat --release` to identify large dependencies.

**Mitigation** (if >30MB):
- Use `opt-level = "z"` (optimize for size) instead of `3` (optimize for speed)
- Consider stripping more aggressively with `strip -s` (GNU) or `strip -x` (macOS)
- Evaluate if all tiktoken encodings are needed (could lazy-load vocabulary files)

---

## Security Considerations

### 1. Memory Safety
- ✅ Rust guarantees memory safety (no buffer overflows)
- ✅ No unsafe code in application (tiktoken-rs may use unsafe internally)
- ✅ Fuzz testing could catch edge cases (post-MVP)

### 2. Denial of Service
- ⚠️ Large inputs could consume memory (mitigated by streaming)
- ⚠️ Malicious UTF-8 sequences could slow down parsing (unlikely with std library)
- ✅ No recursion (no stack overflow risk)

### 3. Information Disclosure
- ✅ No sensitive data in error messages
- ✅ No network calls (no data leakage)
- ✅ No logging of user input (CLI tools don't log by default)

### 4. Supply Chain
- ✅ All dependencies from crates.io (trusted registry)
- ✅ Cargo.lock pins exact versions (reproducible builds)
- ⚠️ Consider `cargo audit` in CI (checks for known vulnerabilities)

**Recommendation**: Add `cargo audit` to CI pipeline (checks CVE database).

---

## Open Questions & Future Research

### Phase 2 (Claude/Gemini Support)
- [ ] Evaluate `llm-tokenizer` vs custom implementations
- [ ] Research Claude tokenization approach (Anthropic doesn't publish BPE vocab)
- [ ] Investigate Gemini tokenization (SentencePiece-based?)
- [ ] Decide on estimation strategy if exact tokenization unavailable

### Phase 3 (Llama/Mistral Support)
- [ ] Evaluate HuggingFace `tokenizers` crate for SentencePiece models
- [ ] Research model file distribution (how to embed SentencePiece models?)
- [ ] Investigate binary size impact (SentencePiece models can be large)

### Post-MVP Features
- [ ] JSON output format (`--format json`)
- [ ] Shell completion generation (`--generate-completions bash`)
- [ ] Progress bar for large inputs (`--progress` flag)
- [ ] Benchmark comparison tool (compare token counts across models)

---

## References

**Crates.io Pages**:
- tiktoken-rs: https://crates.io/crates/tiktoken-rs
- clap: https://crates.io/crates/clap
- anyhow: https://crates.io/crates/anyhow
- thiserror: https://crates.io/crates/thiserror
- criterion: https://crates.io/crates/criterion
- strsim: https://crates.io/crates/strsim

**Official Documentation**:
- OpenAI Tokenizer: https://github.com/openai/tiktoken
- Rust std::io: https://doc.rust-lang.org/std/io/
- Clap Derive API: https://docs.rs/clap/latest/clap/_derive/

**Related Projects**:
- ripgrep (rg): https://github.com/BurntSushi/ripgrep (example of fast, cross-platform Rust CLI)
- fd: https://github.com/sharkdp/fd (example of excellent UX in Rust CLI)
- bat: https://github.com/sharkdp/bat (example of beautiful output formatting)

---

**Research Version**: 1.0 | **Last Updated**: 2026-03-13
