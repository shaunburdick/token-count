# Implementation Plan: Core CLI Token Counting

**Branch**: `001-core-cli` | **Date**: 2026-03-13 | **Spec**: [001-core-cli.md](../../.specify/features/001-core-cli.md)

## Summary

Build a POSIX-style CLI tool that counts tokens for LLM models (starting with OpenAI GPT models) using exact tokenization via tiktoken-rs. The tool accepts text from stdin, processes it with zero external runtime dependencies, and outputs token counts with configurable verbosity levels. MVP focuses on accuracy, cross-platform support, and trivial installation.

## Technical Context

**Language/Version**: Rust 1.85.0+ (stable channel)  
**Primary Dependencies**: 
- `clap` 4.6.0+ (CLI parsing with derive macros)
- `tiktoken-rs` 0.9.1+ (OpenAI tokenization)
- `anyhow` 1.0.102+ (error handling)

**Storage**: N/A (stateless CLI, no persistence)  
**Testing**: `cargo test` with criterion for benchmarks (Linux-only for MVP)  
**Target Platform**: 
- **MVP**: Linux x64 (Ubuntu 22.04+)
- **Post-MVP**: macOS x64/ARM64, Windows x64, Linux musl (Alpine)
  
**Project Type**: CLI tool with library-first architecture  
**Performance Goals**: 
- <10ms latency for small inputs (<10KB)
- <100ms for medium inputs (1MB)
- <5s for large inputs (100MB)
- Streaming support for >1GB inputs

**Constraints**: 
- Binary size: Best effort <50MB (accuracy takes precedence per Amendment 1.3.0)
- Memory usage ≤500MB for any input size
- Zero external runtime dependencies (offline-capable)
- No network calls
- UTF-8 text only

**Scale/Scope**: 
- MVP: OpenAI models only (4 model families, ~10 variants)
- 9 user stories, 24 functional requirements
- Single-binary distribution

## Constitution Check

✅ **POSIX Simplicity**: Tool accepts stdin, outputs to stdout, errors to stderr. Single purpose: count tokens.

✅ **Accuracy Over Speed**: Using tiktoken-rs for exact OpenAI tokenization. No estimation in MVP.

✅ **Zero External Dependencies**: All tokenizers embedded in binary via tiktoken-rs. Works offline.

✅ **Cross-Platform First-Class**: **MVP focuses on Linux-only**. Platform-agnostic Rust stdlib used (macOS/Windows expansion post-MVP).

✅ **Fail Fast with Clear Errors**: Comprehensive error handling with exit codes (0/1/2) and helpful messages.

✅ **Installation Should Be Trivial**: Single binary for MVP (Linux). **Post-MVP**: Homebrew, curl|bash, GitHub Releases for all platforms.

✅ **Semantic Versioning**: Starting at v0.1.0, following SemVer strictly.

**MVP Strategy**: Linux-only for v0.1.0 (faster iteration, reduced testing surface). Cross-platform expansion in v0.2.0+.

## Project Structure

### Documentation (this feature)

```text
specs/001-core-cli/
├── plan.md              # This file (technical architecture & approach)
├── research.md          # Library evaluation, design decisions, alternatives
├── data-model.md        # CLI args, model definitions, output formats
├── quickstart.md        # Key validation scenarios & testing guide
├── contracts/           # API contracts (internal module interfaces)
│   ├── tokenizer.rs     # Tokenizer trait definition
│   ├── model.rs         # Model registry interface
│   └── output.rs        # Output formatter interface
└── tasks.md             # Task breakdown (created after planning)
```

### Source Code (repository root)

```text
token-count/
├── Cargo.toml           # Package manifest
├── Cargo.lock           # Dependency lock file
├── README.md            # User-facing documentation
├── CHANGELOG.md         # Version history
├── LICENSE              # MIT license
│
├── src/
│   ├── main.rs          # Binary entry point, argument parsing
│   ├── lib.rs           # Library API (count_tokens function)
│   │
│   ├── cli/
│   │   ├── mod.rs       # CLI module exports
│   │   ├── args.rs      # Clap argument definitions
│   │   └── input.rs     # Stdin reader with streaming support
│   │
│   ├── tokenizers/
│   │   ├── mod.rs       # Tokenizer trait & factory
│   │   ├── openai.rs    # OpenAI tokenization via tiktoken-rs
│   │   └── registry.rs  # Model registry & name resolution
│   │
│   ├── models/
│   │   ├── mod.rs       # Model definitions & metadata
│   │   ├── openai.rs    # OpenAI model configs (GPT-3.5, GPT-4, etc.)
│   │   └── aliases.rs   # Alias mappings (gpt4 → gpt-4, etc.)
│   │
│   ├── output/
│   │   ├── mod.rs       # Output formatter trait
│   │   ├── simple.rs    # Verbosity 0 (number only)
│   │   ├── verbose.rs   # Verbosity 1-2 (model info, context window)
│   │   └── debug.rs     # Verbosity 3 (token IDs, decoded tokens)
│   │
│   └── error.rs         # Error types & exit code mapping
│
├── tests/
│   ├── integration/
│   │   ├── cli_basic.rs      # Basic stdin piping tests (US-001)
│   │   ├── model_aliases.rs  # Alias resolution tests (US-002, US-003)
│   │   ├── verbosity.rs      # Output format tests (US-004)
│   │   ├── file_input.rs     # File redirection tests (US-005)
│   │   ├── error_handling.rs # Error scenarios (US-006, US-007, US-008)
│   │   └── help_version.rs   # Help/version flags (US-009)
│   │
│   ├── unit/
│   │   ├── tokenizer_tests.rs  # Tokenization accuracy tests
│   │   ├── model_registry_tests.rs  # Model resolution tests
│   │   └── input_streaming_tests.rs # Large input handling
│   │
│   └── fixtures/
│       ├── tokenization_reference.json  # Pre-generated token counts (tiktoken)
│       ├── ascii.txt          # Simple ASCII text
│       ├── unicode.txt        # Unicode characters (emoji, CJK)
│       ├── large.txt          # Large file (>10MB) for streaming tests
│       └── invalid_utf8.bin   # Invalid UTF-8 sequence
│
├── scripts/
│   └── generate_fixtures.py  # One-time fixture generation (requires Python tiktoken)
│
├── benches/
│   └── tokenization.rs  # Criterion benchmarks (small/medium/large inputs)
│
└── .github/
    └── workflows/
        ├── ci.yml        # PR checks (test, lint, build)
        └── release.yml   # Release pipeline (build binaries, publish)
```

**Structure Decision**: Single project layout chosen because:
- Single binary output, no frontend/backend split
- Library-first architecture allows reuse via `lib.rs`
- Clear separation of concerns (cli → tokenizers → models → output)
- Tests organized by type (integration vs unit) following Rust conventions

## Architecture Decisions

### 1. Library-First Design

**Decision**: Core tokenization logic in `lib.rs`, binary is thin wrapper in `main.rs`.

**Rationale**:
- Enables reuse of tokenization logic as a Rust library
- Separates CLI concerns from business logic
- Easier to test core functionality without CLI overhead
- Future-proof: can publish as both binary and library crate

**Implementation**:
```rust
// src/lib.rs
pub fn count_tokens(text: &str, model: &str) -> Result<usize, TokenError> { ... }

// src/main.rs
fn main() {
    let args = Cli::parse();
    let input = read_stdin()?;
    let count = token_count::count_tokens(&input, &args.model)?;
    println!("{}", count);
}
```

### 2. Trait-Based Tokenizer Abstraction

**Decision**: Define `Tokenizer` trait with `encode()` and `count_tokens()` methods.

**Rationale**:
- Enables future providers (Claude, Gemini) without changing core logic
- Each provider can optimize its implementation
- Easy to mock for testing
- Clear contract for what a tokenizer must provide

**Interface** (see `contracts/tokenizer.rs`):
```rust
pub trait Tokenizer: Send + Sync {
    fn encode(&self, text: &str) -> Result<Vec<u32>, TokenError>;
    fn count_tokens(&self, text: &str) -> Result<usize, TokenError>;
    fn model_info(&self) -> &ModelInfo;
}
```

### 3. Streaming Input Processing

**Decision**: Read stdin in 64KB chunks, process incrementally for large inputs.

**Rationale**:
- Prevents OOM on large files (>1GB)
- Maintains <500MB memory footprint (per constitution)
- Allows future progress reporting
- tiktoken-rs supports incremental encoding

**Implementation**:
```rust
const CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks

pub fn read_stdin_streaming<F>(mut process: F) -> Result<(), IoError>
where
    F: FnMut(&str) -> Result<(), TokenError>,
{
    let stdin = io::stdin();
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, stdin.lock());
    let mut buffer = String::with_capacity(CHUNK_SIZE);
    
    loop {
        buffer.clear();
        let bytes_read = reader.read_to_string(&mut buffer)?;
        if bytes_read == 0 { break; }
        
        process(&buffer)?;
    }
    
    Ok(())
}
```

### 4. Model Registry Pattern

**Decision**: Central registry for model metadata, aliases, and tokenizer factories.

**Rationale**:
- Single source of truth for supported models
- Simplifies alias resolution logic
- Easy to add new models without touching multiple files
- Enables `--list-models` implementation

**Structure**:
```rust
pub struct ModelRegistry {
    models: HashMap<String, ModelConfig>,
    aliases: HashMap<String, String>, // alias → canonical name
}

pub struct ModelConfig {
    pub name: String,
    pub encoding: String,
    pub context_window: usize,
    pub tokenizer_factory: fn() -> Box<dyn Tokenizer>,
}
```

### 5. Error Handling Strategy

**Decision**: Use `anyhow::Result` for application code, custom `TokenError` for library API.

**Rationale**:
- `anyhow` provides great error context for CLI (chain of causes)
- Custom `TokenError` enum for library users (explicit error types)
- Exit code mapping in main.rs based on error type
- All errors go to stderr, never stdout

**Exit Code Mapping**:
```rust
match result {
    Ok(_) => exit(0),
    Err(e) if e.is::<InvalidUtf8Error>() => {
        eprintln!("Error: {}", e);
        exit(1);
    }
    Err(e) if e.is::<UnknownModelError>() => {
        eprintln!("Error: {}", e);
        eprintln!("\nUse --list-models to see supported models");
        exit(2);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
        exit(1);
    }
}
```

### 6. Output Formatter Strategy

**Decision**: Strategy pattern for output formatting based on verbosity level.

**Rationale**:
- Clean separation of concerns (tokenization vs formatting)
- Easy to add new output formats (future JSON mode)
- Each formatter is independently testable
- Verbosity level selects appropriate formatter at runtime

**Interface**:
```rust
pub trait OutputFormatter {
    fn format(&self, result: &TokenizationResult) -> String;
}

pub struct SimpleFormatter;   // Verbosity 0: "142"
pub struct VerboseFormatter;  // Verbosity 1-2: Multi-line with model info
pub struct DebugFormatter;    // Verbosity 3: Token IDs + decoded tokens
```

### 7. Zero-Copy String Handling Where Possible

**Decision**: Use string slices (`&str`) in tokenizer interfaces, avoid cloning large inputs.

**Rationale**:
- Reduces memory allocations for large inputs
- tiktoken-rs accepts `&str` natively
- Streaming approach already limits memory usage
- Performance critical for <10ms latency goal

### 8. Embedded Tokenizer Data

**Decision**: tiktoken-rs embeds BPE vocabulary files at compile time.

**Rationale**:
- No network calls at runtime (offline-capable)
- No external files to distribute
- Increases binary size (~15-20MB) but stays within 30MB budget
- tiktoken-rs handles this automatically via `include_bytes!`

## Implementation Phases

### Phase 0: Project Setup (Foundation)
**Goal**: Initialize Rust project with dependencies and basic structure.

**Tasks**:
1. Initialize Cargo project with proper metadata
2. Add dependencies (clap, tiktoken-rs, anyhow, criterion)
3. Create module structure (cli, tokenizers, models, output, error)
4. Set up CI/CD workflows (GitHub Actions for test + lint)
5. Configure rustfmt and clippy with strict settings

**Validation**: `cargo build` succeeds, CI runs successfully on PR.

---

### Phase 1: Core Tokenization Logic (Heart of the Tool)
**Goal**: Implement OpenAI tokenization with accuracy verification.

**Tasks**:
1. Define `Tokenizer` trait and `ModelInfo` struct
2. Implement `OpenAITokenizer` using tiktoken-rs
3. Create model registry with GPT-3.5, GPT-4, GPT-4o configs
4. Add alias resolution (gpt4 → gpt-4, etc.)
5. Generate reference token counts using Python tiktoken (one-time fixture generation)
6. Write unit tests using hardcoded test fixtures (no runtime Python dependency)
7. Add tokenization benchmarks (small/medium/large inputs)

**Test Fixture Strategy**: Use Python tiktoken **once** to generate reference counts for ~50 test cases covering:
- Basic ASCII text ("Hello world" → 2 tokens for cl100k_base)
- Unicode characters ("Hello 世界 🌍" → 6 tokens)
- Edge cases (empty string, single char, whitespace variations)
- Large text samples (1KB, 10KB, 100KB, 1MB)
- All 4 OpenAI encodings (gpt2, p50k_base, cl100k_base, o200k_base)

Store fixtures in `tests/fixtures/tokenization_reference.json`:
```json
{
  "cl100k_base": [
    {"input": "Hello world", "expected_tokens": 2},
    {"input": "Hello 世界 🌍", "expected_tokens": 6}
  ]
}
```

**Validation**: Token counts match pre-generated reference fixtures for all test cases. Python tiktoken only used during fixture generation (manual step, not in CI).

---

### Phase 2: CLI Argument Parsing (User Interface)
**Goal**: Parse command-line arguments with clap, handle edge cases.

**Tasks**:
1. Define `Cli` struct with clap derive macros
2. Implement `--model`, `--verbose`, `--list-models`, `--help`, `--version`
3. Add model name normalization (lowercase, trim)
4. Implement `--list-models` command (print registry contents)
5. Write tests for argument parsing (valid/invalid inputs)

**Validation**: All CLI flags work as specified in feature spec.

---

### Phase 3: Input Processing (Stdin Handling)
**Goal**: Read stdin with streaming support, handle UTF-8 validation.

**Tasks**:
1. Implement buffered stdin reader with 64KB chunks
2. Add UTF-8 validation with clear error messages
3. Handle empty input (return 0)
4. Add streaming tests with large files (>100MB)
5. Verify memory usage stays <500MB

**Validation**: Processes 1GB file without OOM, <500MB memory.

---

### Phase 4: Output Formatting (Display Results)
**Goal**: Format output according to verbosity level.

**Tasks**:
1. Implement `OutputFormatter` trait
2. Create `SimpleFormatter` (number only)
3. Create `VerboseFormatter` (model info, context window percentage)
4. Create `DebugFormatter` (token IDs, decoded tokens - first 10)
5. Write tests for each verbosity level

**Validation**: Output matches examples in feature spec exactly.

---

### Phase 5: Error Handling (Robustness)
**Goal**: Handle all error scenarios gracefully with helpful messages.

**Tasks**:
1. Define `TokenError` enum (InvalidUtf8, UnknownModel, IoError)
2. Implement error-to-exit-code mapping
3. Add fuzzy model name suggestions (Levenshtein distance ≤ 2)
4. Write error message templates with hints
5. Test all error scenarios (invalid UTF-8, unknown model, empty input)

**Validation**: All error messages match spec, exit codes correct.

---

### Phase 6: Integration & Testing (Quality Assurance)
**Goal**: End-to-end tests for all user stories on Linux (MVP platform).

**MVP Tasks** (Linux-only):
1. Write integration tests for each user story (US-001 through US-009)
2. Set up CI on Linux (Ubuntu 22.04)
3. Create test fixtures (ASCII, Unicode, large files, invalid UTF-8)
4. Run performance benchmarks on Linux, verify targets met
5. Track binary size (informational, no hard limit per Amendment 1.3.0)
6. Add `cargo audit` to CI (security scanning)

**Post-MVP Tasks** (deferred):
- Cross-platform CI (macOS x64/ARM64, Windows x64)
- Cross-platform binary builds
- Platform-specific tests (CRLF on Windows, etc.)

**Validation**: All tests pass on Linux, benchmarks meet targets, security audit clean.

---

### Phase 7: Documentation & Polish (User Experience)
**Goal**: Complete user-facing documentation and examples.

**Tasks**:
1. Write README with installation instructions and examples
2. Create CHANGELOG with v0.1.0 release notes
3. Add comprehensive help text to `--help` output
4. Write quickstart guide with common usage patterns
5. Add inline doc comments to all public API functions

**Validation**: Documentation is clear, help text fits in 24 lines.

## Risk Mitigation

### Risk 1: Binary Size Exceeds 50MB (Low Priority)
**Likelihood**: Low | **Impact**: Low (acceptable per Amendment 1.3.0)

**Mitigation**:
- Monitor size during development: `cargo build --release && ls -lh target/release/token-count`
- Use `strip` to remove debug symbols: `strip target/release/token-count`
- Enable LTO (Link-Time Optimization) in Cargo.toml for release builds
- Use `cargo bloat --release` to identify optimization opportunities
- Consider `opt-level = "z"` for minimum size if needed

**Note**: Binary size is no longer a hard constraint per Amendment 1.3.0. Expected size: 40-60MB with embedded tokenizers. Accuracy and offline capability take precedence.

### Risk 2: Streaming Doesn't Work for Large Files
**Likelihood**: Low | **Impact**: High (performance requirement failure)

**Mitigation**:
- Test with 1GB+ files early in Phase 3
- Use `valgrind` or `heaptrack` to verify memory usage
- tiktoken-rs supports chunked encoding, verify this works
- Add progress reporting to stderr for large inputs (future feature)

**Contingency**: Document memory limits in README, fail gracefully if input too large.

### Risk 3: Token Counts Don't Match Official Tiktoken
**Likelihood**: Low | **Impact**: Critical (accuracy requirement failure)

**Mitigation**:
- Test against Python tiktoken library in CI
- Create reference outputs for all test cases
- Use exact same BPE vocabulary files (tiktoken-rs pulls from same source)
- Add regression tests for any discrepancies found

**Contingency**: Report bug to tiktoken-rs maintainer, patch locally if needed.

### Risk 4: Cross-Platform stdin Handling Issues (Post-MVP)
**Likelihood**: Medium | **Impact**: Medium (constitution violation)

**MVP Strategy**: Focus on Linux-only for MVP. Test on macOS/Windows post-MVP.

**Mitigation**:
- Use Rust's standard library (platform-agnostic by design)
- MVP: Test only on Linux (Ubuntu 22.04)
- Post-MVP: Add Windows (CRLF) and macOS (ARM64) testing
- Use `BufReader` which handles line endings correctly

**Contingency**: Use `atty` crate to detect terminal vs pipe, handle differently.

---

### Risk 5: tiktoken-rs Dependency Breaking Changes
**Likelihood**: Low | **Impact**: High (blocks development or runtime failures)

**Issue**: tiktoken-rs is relatively new and could introduce breaking changes, become unmaintained, or have upstream issues.

**Mitigation**:
- Pin exact version in Cargo.lock (0.9.1 for MVP)
- Monitor tiktoken-rs GitHub repo health (last commit, issue response time)
- Verify repo is actively maintained before MVP release
- Test thoroughly with current version (token accuracy tests vs Python)
- Fork tiktoken-rs as backup (MIT licensed, can maintain ourselves)

**Contingency**: 
- Fork and maintain tiktoken-rs if upstream becomes unmaintained
- Binary data (BPE vocabularies) can be extracted and used independently
- Fallback: Minimal Rust BPE implementation using extracted data

---

### Risk 6: Test Flakiness (CI Failures)
**Likelihood**: Medium | **Impact**: Medium (blocks PRs, delays releases)

**Issue**: Tests might be flaky due to timing differences, CI environment changes, or performance assertions.

**MVP Strategy**: Minimize test surface area - Linux-only CI for MVP.

**Mitigation**:
- MVP: Test only on Linux (Ubuntu 22.04) - single platform reduces flake
- Pin GitHub Actions runner version (`ubuntu-22.04`, not `ubuntu-latest`)
- Use relative performance assertions (not absolute: "<10ms")
- Performance tests: Use generous thresholds (2x expected time)
- Add retries for known flaky tests (retry-action: max 3 attempts)
- Separate performance benchmarks from CI (run manually, not on every PR)

**Post-MVP Expansion**:
- Phase 2: Add macOS x64 testing
- Phase 3: Add macOS ARM64, Windows x64 testing
- Phase 4: Add Linux musl (Alpine) testing

**Contingency**: 
- Mark flaky tests as `#[ignore]` temporarily
- Run flaky tests only on main branch (skip on PRs)
- Document known flaky tests in tests/README.md

---

### Risk 7: Security Audit Findings (Dependencies)
**Likelihood**: Low | **Impact**: Medium (need to fix vulnerabilities)

**Issue**: `cargo audit` might find CVEs in dependencies (tiktoken-rs has transitive deps: fancy-regex, base64, etc.).

**Mitigation**:
- Add `cargo audit` to CI pipeline (fail on HIGH/CRITICAL severity)
- Set up Dependabot for automated security updates
- Review dependency tree regularly: `cargo tree`
- Minimize dependency count (fewer deps = smaller attack surface)
- Pin dependency versions in Cargo.lock (reproducible builds)
- Run `cargo audit` before every release

**Contingency**: 
- If CVE in tiktoken-rs: Fork and patch locally, report upstream
- If CVE in transitive dep: Upgrade parent dependency or find alternative
- Document known vulnerabilities in SECURITY.md if upgrade breaks compatibility
- Worst case: Vendor dependencies and patch manually

## Success Criteria

**Phase 0 Complete**:
- [ ] `cargo build` succeeds
- [ ] CI pipeline runs on PR (test + lint)
- [ ] Project structure matches plan

**Phase 1 Complete**:
- [ ] Token counts match Python tiktoken for 10+ test cases
- [ ] All OpenAI models registered (GPT-3.5, GPT-4, GPT-4o)
- [ ] Alias resolution works (gpt4 → gpt-4)
- [ ] Benchmarks show <10ms for <10KB input

**Phase 2 Complete**:
- [ ] All CLI flags work (--model, --verbose, --list-models, --help, --version)
- [ ] `--help` output fits in 24 lines
- [ ] `--list-models` shows all supported models

**Phase 3 Complete**:
- [ ] Processes 1GB file without OOM
- [ ] Memory usage <500MB during streaming
- [ ] UTF-8 validation works correctly
- [ ] Empty input returns 0

**Phase 4 Complete**:
- [ ] All 4 verbosity levels work as specified
- [ ] Output matches examples in feature spec exactly

**Phase 5 Complete**:
- [ ] All error scenarios tested (invalid UTF-8, unknown model, etc.)
- [ ] Exit codes correct (0 success, 1 runtime error, 2 user error)
- [ ] Error messages include helpful hints

**Phase 6 Complete** (MVP - Linux-only):
- [ ] All integration tests pass on Linux (one per user story)
- [ ] CI pipeline green on Ubuntu 22.04
- [ ] Code coverage ≥80%
- [ ] `cargo clippy` shows zero warnings
- [ ] `cargo audit` shows no HIGH/CRITICAL vulnerabilities
- [ ] Binary size ≤30MB on Linux x64
- [ ] Performance benchmarks meet targets on Linux

**Phase 7 Complete**:
- [ ] README has installation instructions and examples
- [ ] CHANGELOG documents v0.1.0 features
- [ ] All public API functions have doc comments
- [ ] Quickstart guide written

**Feature Complete** (MVP v0.1.0 - Linux-only):
- [ ] All user stories (US-001 through US-009) implemented
- [ ] All functional requirements (FR-001 through FR-009) met
- [ ] All acceptance criteria checked off in feature spec
- [ ] Performance benchmarks meet targets on Linux (FR-007)
- [ ] Security audit clean (cargo audit)
- [ ] Ready for v0.1.0 release (Linux binary)

**Post-MVP Expansion** (v0.2.0+):
- [ ] Cross-platform testing (macOS, Windows)
- [ ] Cross-platform binaries (4 targets: Linux/macOS/Windows)
- [ ] Homebrew formula (macOS)
- [ ] Installation methods for all platforms

## Next Steps

1. **Review this plan** with spec author for alignment with feature spec
2. **Create research.md** documenting library evaluation and alternatives
3. **Create data-model.md** with detailed struct definitions
4. **Create contracts/** with trait definitions and module interfaces
5. **Create quickstart.md** with validation scenarios
6. **Generate tasks.md** breaking down phases into actionable tasks (via `/speckit.tasks`)
7. **Begin Phase 0** implementation

---

**Plan Version**: 1.0 | **Last Updated**: 2026-03-13
