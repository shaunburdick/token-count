# Token Counter CLI Constitution

## Core Principles

### I. POSIX Simplicity
The tool follows POSIX CLI conventions and behaves like standard utilities (wc, grep, cat). Single responsibility: count tokens accurately and report results clearly. No feature creep—if it doesn't serve token counting, it doesn't belong.

**Rationale**: Users should be able to pipe text and get immediate, predictable results without learning complex interfaces.

### II. Accuracy Over Speed
For supported models, tokenization must be exact using proper tokenizers. We never sacrifice correctness for performance. If we cannot accurately tokenize a model, we do not support it—no guessing.

**Rationale**: Developers rely on accurate token counts for API cost prediction and context window management. Inaccurate counts break trust.

### III. Zero External Dependencies at Runtime
The binary must be standalone with all tokenizers embedded. No network calls, no config downloads, no external files required. Works offline, works anywhere.

**Rationale**: CLI tools should be reliable in any environment—CI/CD pipelines, air-gapped systems, flaky networks.

### IV. Cross-Platform First-Class Support
Windows, Linux, and macOS are equally important. Every feature works identically across platforms. Binary releases for all three platforms on every release.

**Rationale**: Developers use different platforms. The tool must work seamlessly everywhere.

### V. Fail Fast with Clear Errors
Invalid input, unsupported models, or binary data cause immediate exit with helpful error messages including suggestions. No silent failures, no partial results without warnings.

**Rationale**: Ambiguous errors waste developer time. Clear, actionable messages enable quick fixes.

### VI. Installation Should Be Trivial
Users can install via `curl | bash`, Homebrew, or Cargo with a single command. No manual steps, no configuration required. Binary releases on GitHub are the source of truth.

**Rationale**: Friction in installation prevents adoption. One-command install is table stakes for modern CLI tools.

### VII. Semantic Versioning Strictly Enforced
Breaking changes require major version bump. New models/features are minor versions. Bug fixes are patches. Changelog documents every change with examples.

**Rationale**: Users need predictable upgrade paths. Scripts break when APIs change unexpectedly.

## Technical Decisions

### Language & Toolchain
- **Language**: Rust (stable channel, MSRV: 1.75.0+)
- **Rationale**: Memory safety, cross-compilation, single-binary output, excellent CLI ecosystem

### Core Dependencies
| Crate | Version | Purpose | Justification |
|-------|---------|---------|---------------|
| `clap` | 4.6.0+ | CLI argument parsing | Industry standard, derive macros, excellent error messages |
| `tiktoken-rs` | 0.9.1+ | OpenAI tokenization | Most mature, 4.5M+ downloads, supports all OpenAI models |
| `llm-tokenizer` | 1.3.0+ | Multi-provider tokenization | Supports HuggingFace + tiktoken, good for Gemini/Llama |
| `anyhow` | 1.0.102+ | Error handling | Ergonomic error propagation, context support |
| `serde` + `serde_json` | 1.0.149+ | Structured output (future JSON mode) | Standard serialization library |

**Dependency Update Policy**: Check for updates quarterly, update within 2 weeks unless breaking changes require major refactor.

### Architecture
```
token-count (binary)
├── cli/           # Argument parsing, stdin handling
├── tokenizers/    # Model-specific tokenization logic
│   ├── openai.rs  # GPT-2/3.5/4/o1 via tiktoken-rs
│   ├── anthropic.rs # Claude via estimation/heuristics
│   ├── google.rs  # Gemini via llm-tokenizer
│   └── meta.rs    # Llama via llm-tokenizer (P2)
├── models/        # Model definitions & aliases
├── output/        # Formatting (simple/verbose)
└── lib.rs         # Core token counting API
```

**Design Principles**:
- Library-first: Core tokenization logic usable as a Rust library
- Provider abstraction: Each provider is a trait implementation
- Streaming: Process large inputs without loading entire file into memory

### Model Support Strategy
**Phase 1 (MVP - v0.1.0)**:
- OpenAI: All GPT models (exact tokenization via tiktoken-rs)
- Default model: `gpt-3.5-turbo`

**Phase 2 (v0.2.0)**:
- Anthropic: Claude 3/3.5/4 (smart estimation via character-based heuristics + safety margin)
- Google: Gemini 1.5/2.0 (via llm-tokenizer or estimation)

**Phase 3 (v0.3.0)**:
- Meta: Llama 2/3/4 (via llm-tokenizer with SentencePiece)
- Mistral: Mistral/Codestral (via llm-tokenizer)

**Not Supported**: DeepSeek, Qwen (P3, insufficient Rust library support)

### Model Name Resolution
Support three formats simultaneously:
1. **Exact names**: `gpt-4-turbo-2024-04-09` (precise)
2. **Smart aliases**: `gpt4` → latest GPT-4 variant, `claude-sonnet` → Claude 3.5 Sonnet
3. **Provider format**: `openai/gpt-4`, `anthropic/claude-sonnet`

Alias resolution rules:
- Case-insensitive matching
- Partial match suggests alternatives (e.g., `gpt` → "Did you mean: gpt-3.5-turbo, gpt-4, gpt-4o?")
- Unknown model exits with error code 2 and suggestions

### Output Formats
**Default (verbosity 0)**: Number only
```
142
```

**Verbose (-v)**: Model info + count
```
Model: gpt-4 (cl100k_base)
Tokens: 142
```

**Very Verbose (-vv)**: Context window percentage
```
Model: gpt-4 (cl100k_base)
Tokens: 142
Context Window: 8,192 tokens
Usage: 1.73%
```

**Debug (-vvv)**: Token IDs + decoded tokens (first 10 only for long inputs)
```
Model: gpt-4 (cl100k_base)
Tokens: 142
Token IDs: [15339, 1917, 11, 1268, 527, ...]
Sample Decoded: ["Hello", " world", ",", " how", " are", ...]
```

### Binary Size Budget
- **Target**: <20MB uncompressed
- **Maximum**: 30MB uncompressed
- **Rationale**: Balances embedded tokenizers with reasonable download size

### Error Handling
| Scenario | Behavior | Exit Code |
|----------|----------|-----------|
| Empty input | Print `0` | 0 |
| Binary/invalid UTF-8 | Error: "Input contains invalid UTF-8" | 1 |
| Unknown model | Error + suggestions | 2 |
| Large input (>1GB) | Stream processing | 0 |
| Missing --model | Use default (`gpt-3.5-turbo`) | 0 |
| Unsupported model | Error: "Model not supported" + suggestions | 2 |

## Quality Standards

### Testing Requirements
- **Unit test coverage**: ≥80% for all tokenization logic
- **Integration tests**: Test CLI with piped input, file input, model aliases
- **Cross-platform tests**: CI runs on Linux, macOS, Windows
- **Regression tests**: Every bug fix gets a test case

### Performance Standards
- **Small input (<10KB)**: <10ms latency
- **Medium input (1MB)**: <100ms latency
- **Large input (100MB)**: Streaming with <500MB memory footprint
- **Binary size**: <30MB uncompressed

### Code Quality
- **Linting**: `cargo clippy` with zero warnings
- **Formatting**: `rustfmt` with default settings
- **Documentation**: Public API functions have doc comments with examples
- **MSRV**: Support Rust 1.75.0+ (check compatibility in CI)

### CI/CD Pipeline
**On Pull Request**:
1. `cargo fmt --check`
2. `cargo clippy -- -D warnings`
3. `cargo test` (all platforms)
4. `cargo build --release` (all platforms)

**On Release Tag**:
1. Run all PR checks
2. Build release binaries (Linux x64, macOS x64/ARM64, Windows x64)
3. Generate SHA256 checksums
4. Create GitHub Release with binaries
5. Update Homebrew formula (automated PR)

### Release Process
1. Update version in `Cargo.toml`
2. Update CHANGELOG.md with all changes since last release
3. Tag commit: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions builds and publishes release
6. Verify installation works via `curl | bash` and Homebrew

## Anti-Patterns to Avoid

### ❌ Guessing Token Counts
**Never** use rough estimates like "~4 chars per token" for models we claim to support. If we can't tokenize accurately, we don't list the model.

**Why**: Inaccurate counts break production systems. Users will lose trust and switch tools.

### ❌ Feature Creep
Do not add:
- Cost estimation (pricing changes too frequently)
- Model comparison mode (users can call tool multiple times)
- Interactive REPL mode (use shell history instead)
- Configuration files (unnecessary complexity)

**Why**: Focus preserves simplicity. Each feature adds maintenance burden and complexity.

### ❌ Silent Warnings
Do not continue processing when encountering errors. Fail fast with clear messages.

**Why**: Silent failures lead to incorrect results being used in production.

### ❌ Implicit Behaviors
Do not auto-detect models from input, change behavior based on environment variables, or use "smart" defaults that aren't documented.

**Why**: Implicit behavior is unpredictable. Explicit flags make scripts self-documenting.

### ❌ Network Dependencies
Do not fetch tokenizer files, model configs, or pricing data from the internet at runtime.

**Why**: Network calls break offline environments and add latency/failure modes.

## Success Metrics

### Product Metrics
- **Adoption**: 1,000+ GitHub stars within 6 months
- **Installation**: 10,000+ installs via Homebrew/Cargo within 1 year
- **Reliability**: <1% error rate from user feedback
- **Performance**: 95% of operations complete in <100ms

### Technical Metrics
- **Test coverage**: Maintain ≥80% coverage
- **Build success rate**: ≥99% on CI/CD pipeline
- **Release cadence**: Monthly releases for first 6 months
- **Issue response time**: <48 hours for bug reports

### Operational Metrics
- **Binary size**: Stay <25MB average across platforms
- **Memory usage**: <100MB for typical usage
- **Cross-platform parity**: All tests pass on Windows/Linux/macOS

## Governance

### Amendment Process
1. Propose change in GitHub Discussion with rationale
2. Allow 1 week for feedback
3. Update constitution with version bump (major = breaking principle, minor = new principle, patch = clarification)
4. Document change in "Constitutional Amendments" section at bottom

### Conflict Resolution
If feature request conflicts with constitution:
1. Constitution takes precedence
2. If feature is valuable, propose constitutional amendment first
3. Document decision rationale in issue/PR

### Compliance Verification
Every PR must:
- [ ] Align with core principles
- [ ] Meet quality standards
- [ ] Include tests
- [ ] Update CHANGELOG if user-facing

**Version**: 1.1.0 | **Ratified**: 2026-03-13 | **Last Amended**: 2026-03-13

---

## Constitutional Amendments

### Amendment 1.1.0 (2026-03-13): Package Naming Correction
**Type**: Technical Decision Update (Non-Breaking)

**Change**: Updated project name from `token-counter` to `token-count`

**Rationale**:
- `token-counter` already exists on crates.io (HuggingFace-based CLI)
- `token-count` is available and follows Rust CLI naming conventions
- Hyphenated names are common in Cargo packages (cargo-watch, git-cliff)
- Aligns with Homebrew formula naming best practices

**Impact**:
- All documentation updated
- Binary name: `token-count`
- Cargo package: `token-count`
- Homebrew formula: `Formula/token-count.rb`
- No implementation impact (caught during specification phase)

**Approved**: 2026-03-13 (User decision: Option A)
