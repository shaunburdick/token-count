# Feature 004: Google Gemini Token Counting Support

**Feature ID**: 004  
**Priority**: P2 (High Value)  
**Status**: Specification Complete  
**Version**: 1.0  
**Last Updated**: 2026-03-14  
**Dependencies**: None (standalone feature)

---

## Problem Statement

Users working with Google Gemini models need accurate token counting for:
- **Cost estimation**: Understanding API usage costs before making requests
- **Context window management**: Ensuring prompts fit within 1M-2M token limits
- **Prompt optimization**: Iterating on prompts to maximize information density

Currently, token-count supports OpenAI (exact, offline) and Claude (estimation + optional API). Adding Gemini completes the "Big 3" LLM providers and serves a large user base (Gemini powers Google Workspace, Android, Chrome, and enterprise products).

### Why This Matters

Google Gemini has massive context windows (1M-2M tokens) compared to GPT-4 (128K), making token counting even more critical for:
- Processing large documents (entire codebases, research papers, books)
- Multi-turn conversations with extensive history
- RAG applications with many retrieved chunks

---

## Solution Overview

Add exact, offline tokenization for Google Gemini models using the `gemini-tokenizer` Rust crate (v0.2.0+). This provides 100% accurate token counts that match Google's official tokenizer, with zero network dependencies.

### Key Benefits

✅ **Exact tokenization** - Same SentencePiece model as Google (262,144 vocab)  
✅ **Fully offline** - No API keys, no network calls, works anywhere  
✅ **Fast** - <1ms for small inputs (meets our <10ms target)  
✅ **Future-proof** - All Gemini models (1.5, 2.x, 3.x) use the same tokenizer  
✅ **Constitution-aligned** - Satisfies all 7 principles (especially Principle III: Zero External Dependencies)

### What Makes Gemini Different

Unlike Claude (which required estimation), Gemini has an **official tokenizer** that Google open-sourced:
- Based on SentencePiece (Gemma 3 model)
- Embedded vocabulary (~2MB)
- Identical to Google's Python SDK
- No guessing, no estimation needed

---

## User Stories

### US-015: Quick Gemini Token Count
**As a** developer using Gemini  
**I want to** pipe text to token-count and get accurate token counts  
**So that** I can estimate API costs and manage context windows  

**Acceptance Criteria**:
- Command: `echo "Hello Gemini" | token-count --model gemini`
- Output: Token count (e.g., `2`)
- Processing time: <10ms for <10KB input
- No API key required
- No network calls
- Exit code 0 on success

**Example**:
```bash
$ echo "Hello, Gemini!" | token-count --model gemini
3

$ echo "Hello, Gemini!" | token-count --model gemini -v
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 3
Context window: 1000000 tokens (0.0003% used)
```

---

### US-016: Gemini Model Aliases
**As a** user  
**I want to** use short, intuitive model names  
**So that** I don't have to remember full model names like `gemini-3-flash-preview`  

**Acceptance Criteria**:
- Short aliases work: `gemini`, `gemini-pro`, `gemini-flash`, `gemini-lite`
- Provider format works: `google/gemini`, `google/gemini-pro`
- Case-insensitive: `GEMINI`, `Gemini`, `gemini` all work
- Unknown model shows suggestions: `gemini-4` → "Did you mean: gemini-3-flash-preview, gemini-3.1-pro-preview?"

**Examples**:
```bash
$ token-count --model gemini < file.txt
412

$ token-count --model gemini-pro < file.txt
412

$ token-count --model google/gemini < file.txt
412

$ token-count --model GEMINI < file.txt
412
```

---

### US-017: Multiple Gemini Model Versions
**As a** developer  
**I want to** count tokens for different Gemini model versions (3.x, 2.5, 1.5)  
**So that** I can work with the model version my application uses  

**Acceptance Criteria**:
- Support Gemini 3.x models: `gemini-3.1-pro-preview`, `gemini-3-flash-preview`, `gemini-3.1-flash-lite-preview`
- Support Gemini 2.5 models: `gemini-2.5-pro`, `gemini-2.5-flash`, `gemini-2.5-flash-lite`
- Support Gemini 1.5 models: `gemini-1.5-pro`, `gemini-1.5-flash`
- All models produce identical token counts (same tokenizer)
- Context window sizes are accurate for each model

**Examples**:
```bash
$ echo "test" | token-count --model gemini-3-flash-preview
1

$ echo "test" | token-count --model gemini-2.5-flash
1

$ echo "test" | token-count --model gemini-1.5-pro
1
```

---

### US-018: Gemini Context Window Validation
**As a** user  
**I want to** see context window usage percentage  
**So that** I know if my prompt fits within Gemini's limits  

**Acceptance Criteria**:
- Verbose mode (`-v`) shows context window size
- Context windows are accurate:
  - Gemini 1.5 Pro: 2M tokens
  - All other models: 1M tokens
- Usage percentage calculated correctly
- Format matches existing verbose output

**Example**:
```bash
$ cat large-document.txt | token-count --model gemini -v
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 142857
Context window: 1000000 tokens (14.29% used)

$ cat large-document.txt | token-count --model gemini-1.5-pro -v
Model: gemini-1.5-pro (gemini-gemma3)
Tokens: 142857
Context window: 2000000 tokens (7.14% used)
```

---

### US-019: List Gemini Models
**As a** user  
**I want to** see all supported Gemini models  
**So that** I can discover available options  

**Acceptance Criteria**:
- `--list-models` shows Gemini models grouped by version
- Each model shows: name, encoding, context window, aliases
- Models are sorted logically (3.x first, then 2.5, then 1.5)
- Output is readable and well-formatted

**Example**:
```bash
$ token-count --list-models | grep -A 10 "Gemini"

Google Gemini models:

  gemini-3.1-pro-preview
    Encoding: gemini-gemma3
    Context window: 1000000 tokens
    Aliases: gemini-pro, gemini-3-pro, google/gemini-pro

  gemini-3-flash-preview (default)
    Encoding: gemini-gemma3
    Context window: 1000000 tokens
    Aliases: gemini, gemini-flash, gemini-3-flash, google/gemini
```

---

### US-020: Debug Mode Token Details
**As a** developer debugging tokenization  
**I want to** see individual token IDs and decoded tokens  
**So that** I can understand how Gemini tokenizes my input  

**Acceptance Criteria**:
- Debug mode (`-vvv`) shows token IDs
- Shows decoded token strings
- Limits output for large inputs (first 20 tokens)
- Format is readable and informative

**Example**:
```bash
$ echo "Hello, world!" | token-count --model gemini -vvv
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 4

Token details (showing first 4):
  [8699] "Hello"
  [235269] ","
  [2134] " world"
  [235341] "!"

Note: Use compute_tokens() API for programmatic access to all tokens.
```

---

## Functional Requirements

### FR-020: Gemini Tokenizer Integration
**Description**: Integrate `gemini-tokenizer` crate (v0.2.0+) for exact offline tokenization.

**Implementation**:
- Add dependency: `gemini-tokenizer = "0.2.0"`
- Create module: `src/tokenizers/google/`
- Initialize tokenizer once at startup (cache for performance)
- Use `LocalTokenizer::count_tokens()` for counting
- Use `LocalTokenizer::compute_tokens()` for debug mode

**Edge Cases**:
- Empty input → return 0 tokens
- Very large input (>100MB) → stream processing (if supported, otherwise fail gracefully)
- Invalid UTF-8 → return clear error (same as OpenAI/Claude)
- Tokenizer initialization failure → return helpful error

**Acceptance Criteria**:
- ✅ Token counts match Google's official tokenizer 100%
- ✅ Tokenization completes in <10ms for <10KB input
- ✅ No network calls (fully offline)
- ✅ No API key required

---

### FR-021: Gemini Model Registry
**Description**: Define all supported Gemini models with metadata (context window, aliases, encoding).

**Models to Support**:

**Gemini 3.x Series (Priority 1 - Preview)**:
- `gemini-3.1-pro-preview` - 1M context, aliases: `gemini-pro`, `gemini-3-pro`
- `gemini-3-flash-preview` - 1M context, aliases: `gemini`, `gemini-flash`, `gemini-3-flash` (DEFAULT)
- `gemini-3.1-flash-lite-preview` - 1M context, aliases: `gemini-lite`, `gemini-3-lite`

**Gemini 2.5 Series (Priority 2 - Being Deprecated June 2026)**:
- `gemini-2.5-pro` - 1M context
- `gemini-2.5-flash` - 1M context
- `gemini-2.5-flash-lite` - 1M context

**Gemini 1.5 Series (Priority 3 - Legacy)**:
- `gemini-1.5-pro` - 2M context (largest context window)
- `gemini-1.5-flash` - 1M context

**Provider Format**:
- `google/gemini` → `gemini-3-flash-preview`
- `google/gemini-pro` → `gemini-3.1-pro-preview`
- `google/{model}` → `{model}` (passthrough)

**Acceptance Criteria**:
- ✅ All 8 models defined in registry
- ✅ All aliases resolve correctly
- ✅ Context windows are accurate
- ✅ Case-insensitive matching works
- ✅ Unknown models suggest similar Gemini models

---

### FR-022: Default Model Selection
**Description**: Set `gemini-3-flash-preview` as the default Gemini model.

**Rationale**:
- Gemini 2.5 series will be deprecated June 17, 2026 (3 months away)
- Gemini 3.x is the future (will go GA before June)
- "Flash" models are fast, good for CLI use cases
- 1M context window is sufficient for most users
- Avoids needing to change default in 3 months

**Acceptance Criteria**:
- ✅ `--model gemini` resolves to `gemini-3-flash-preview`
- ✅ `--model google/gemini` resolves to `gemini-3-flash-preview`
- ✅ Users can explicitly request 2.5 models if needed
- ✅ Documentation notes that 3.x are preview models

---

### FR-023: Gemini Error Handling
**Description**: Provide clear, actionable errors for Gemini-specific failure modes.

**Error Scenarios**:

1. **Unknown Gemini model**:
   ```
   Error: Unknown model: 'gemini-4'
   
   Did you mean one of these Gemini models?
     - gemini-3-flash-preview
     - gemini-3.1-pro-preview
     - gemini-3.1-flash-lite-preview
   
   See all models: token-count --list-models
   ```

2. **Tokenizer initialization failure**:
   ```
   Error: Failed to initialize Gemini tokenizer
   
   This is likely a bug. Please report it:
     https://github.com/shaunburdick/token-count/issues
   
   Include: OS, architecture, token-count version
   ```

3. **Invalid UTF-8** (same as OpenAI/Claude):
   ```
   Error: Input contains invalid UTF-8 at byte 1234
   ```

**Acceptance Criteria**:
- ✅ All errors are user-friendly (no raw stack traces)
- ✅ Errors suggest next steps
- ✅ Exit codes are consistent (0=success, 1=I/O, 2=unknown model)

---

### FR-024: Output Format Consistency
**Description**: Gemini output follows same format as OpenAI/Claude for all verbosity levels.

**Verbosity Levels**:

**Level 0 (default)**: Number only
```
142
```

**Level 1 (-v)**: Model info + count + context window
```
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 142
Context window: 1000000 tokens (0.0142% used)
```

**Level 2 (-vv)**: Same as level 1 (reserved for future features)
```
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 142
Context window: 1000000 tokens (0.0142% used)
```

**Level 3 (-vvv)**: Debug with token details
```
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 142

Token details (showing first 20 of 142):
  [8699] "Hello"
  [235269] ","
  [2134] " world"
  ...

Note: Use compute_tokens() API for programmatic access to all tokens.
```

**Acceptance Criteria**:
- ✅ Output format matches OpenAI/Claude
- ✅ Encoding name is consistent: `gemini-gemma3`
- ✅ Context window usage is calculated correctly
- ✅ Debug mode shows meaningful token information

---

### FR-025: Performance Standards
**Description**: Gemini tokenization meets performance requirements.

**Targets**:
- Small input (<10KB): <10ms latency
- Medium input (1MB): <100ms latency
- Large input (100MB): <10 seconds, streaming support
- Memory usage: <500MB peak for large files

**Implementation Notes**:
- `gemini-tokenizer` crate uses SentencePiece (fast C++ library)
- Tokenizer initialization is one-time cost (~5ms)
- Cache tokenizer instance across multiple tokenizations
- Stream large inputs if supported by crate

**Acceptance Criteria**:
- ✅ 95th percentile latency meets targets
- ✅ Memory usage stays within budget
- ✅ No memory leaks (validate with valgrind)
- ✅ Performance documented in benchmarks

---

### FR-026: Cross-Platform Compatibility
**Description**: Gemini support works on all platforms (Linux, macOS, Windows).

**Platform Requirements**:
- Linux x86_64 (Ubuntu 20.04+, Debian 10+, Fedora 35+)
- macOS x86_64 (Intel, macOS 10.15+)
- macOS aarch64 (Apple Silicon, macOS 11.0+)
- Windows x86_64 (Windows 10+)

**Build Requirements**:
- `sentencepiece` crate compiles on all platforms
- Embedded model (~2MB) included in binary
- No runtime dependencies (static linking)

**Acceptance Criteria**:
- ✅ CI tests pass on all platforms
- ✅ Release binaries work on all platforms
- ✅ No platform-specific bugs
- ✅ Binary size increase is acceptable (~2.3MB)

---

## Non-Functional Requirements

### NFR-012: Accuracy
**Target**: 100% accuracy (exact match with Google's tokenizer)

**Validation**:
- Compare token counts with Google's official Python SDK
- Test suite includes comparison tests (run once during development)
- Document tokenizer version (Gemma 3 SentencePiece)
- SHA-256 verification of embedded model

**Acceptance**: Token counts match Google's tokenizer for 100 diverse test cases.

---

### NFR-013: Offline Operation
**Target**: No network calls, works in air-gapped environments

**Implementation**:
- Tokenizer model embedded in binary (~2MB)
- No API key required
- No runtime downloads
- No phone-home telemetry

**Acceptance**: Works with network disabled (`sudo ifconfig en0 down`), passes CI in isolated environment.

---

### NFR-014: Binary Size
**Target**: <50MB total binary size (including all tokenizers)

**Current State**:
- Before Gemini: 9.2MB (OpenAI + Claude)
- After Gemini: ~11.5MB (+2.3MB)
- Budget remaining: 38.5MB

**Mitigation**:
- Use release optimizations (LTO, strip, opt-level=3)
- Embedded model is compressed SentencePiece format (~2MB)
- No unnecessary dependencies

**Acceptance**: Release binary is <15MB on all platforms.

---

### NFR-015: Dependency Audit
**Target**: All new dependencies are secure and well-maintained

**New Dependencies**:
- `gemini-tokenizer = "0.2.0"` - Apache-2.0, community-maintained
- `sentencepiece = "^0.11"` - MIT/Apache-2.0, bindings to Google's C++ lib
- `sha2 = "^0.10"` - MIT/Apache-2.0, RustCrypto project

**Audit Process**:
1. Run `cargo audit` - no known vulnerabilities
2. Check crate download stats - sentencepiece is mature
3. Review crate source code - gemini-tokenizer is clean
4. Verify licenses - all compatible with MIT
5. Document maintainer status - community-maintained (acceptable risk)

**Acceptance**: `cargo audit` passes, licenses documented, risks mitigated.

---

### NFR-016: Maintainability
**Target**: Code is easy to maintain and extend

**Code Structure**:
```
src/tokenizers/google/
├── mod.rs           # Public API, provider trait impl
├── models.rs        # Model definitions (list, aliases, context windows)
└── tokenizer.rs     # Wrapper around gemini-tokenizer crate
```

**Best Practices**:
- Reuse existing patterns (similar to OpenAI/Claude modules)
- Comprehensive unit tests (each model, alias, edge case)
- Integration tests (CLI end-to-end)
- Document why decisions were made (code comments)
- Follow Rust conventions (clippy, rustfmt)

**Acceptance**: Code review passes, no clippy warnings, 80%+ test coverage.

---

## Testing Strategy

### Unit Tests

**Test Coverage**:
- ✅ Tokenizer initialization (success, failure modes)
- ✅ Token counting (empty, small, large inputs)
- ✅ Model registry (all models, all aliases, case-insensitivity)
- ✅ Default model resolution (`gemini` → `gemini-3-flash-preview`)
- ✅ Provider format (`google/gemini` → `gemini-3-flash-preview`)
- ✅ Context window calculations (1M vs 2M tokens)
- ✅ Error messages (unknown model, init failure, invalid UTF-8)

**Test Cases** (20+ tests):
```rust
#[test]
fn test_gemini_flash_tokenization() { ... }

#[test]
fn test_gemini_pro_alias() { ... }

#[test]
fn test_gemini_default_model() { ... }

#[test]
fn test_gemini_unknown_model_suggestion() { ... }

#[test]
fn test_gemini_context_window_1_5_pro() { ... }

#[test]
fn test_gemini_case_insensitive() { ... }
```

---

### Integration Tests

**Test Coverage**:
- ✅ CLI with `--model gemini`
- ✅ CLI with all model aliases
- ✅ Piped input (`echo "text" | token-count --model gemini`)
- ✅ File input (`token-count --model gemini < file.txt`)
- ✅ Verbosity levels (`-v`, `-vv`, `-vvv`)
- ✅ `--list-models` output
- ✅ Error handling (unknown model, invalid UTF-8)

**Test Cases** (15+ tests):
```bash
# tests/gemini_tokenization.rs
#[test]
fn test_gemini_cli_basic() { ... }

#[test]
fn test_gemini_all_models() { ... }

#[test]
fn test_gemini_verbose_output() { ... }

#[test]
fn test_gemini_list_models() { ... }
```

---

### Performance Tests

**Benchmarks**:
- Small input (100 bytes): Target <10ms, Expected ~1ms
- Medium input (10KB): Target <100ms, Expected ~5ms
- Large input (1MB): Target <1s, Expected ~50ms

**Benchmark Suite**:
```rust
// benches/gemini_tokenization.rs
fn bench_gemini_small(c: &mut Criterion) { ... }
fn bench_gemini_medium(c: &mut Criterion) { ... }
fn bench_gemini_large(c: &mut Criterion) { ... }
```

**Acceptance**: 95th percentile meets targets, documented in CHANGELOG.

---

### Comparison Tests (One-Time Validation)

**Purpose**: Verify token counts match Google's official tokenizer.

**Approach**:
1. Generate 100 diverse test cases (code, prose, multilingual, edge cases)
2. Count tokens with `gemini-tokenizer` (Rust)
3. Count tokens with Google's Python SDK
4. Compare results (must match 100%)

**Test Cases**:
- English prose (various lengths)
- Code snippets (Python, Rust, JavaScript)
- Multilingual text (CJK, Arabic, emoji)
- Edge cases (empty, single char, repeated chars)
- Special characters (Unicode, control chars)

**Acceptance**: 100% match rate across all test cases.

---

### Cross-Platform Tests

**CI Matrix**:
- Ubuntu 22.04 (x86_64)
- macOS 12 (Intel x86_64)
- macOS 13 (Apple Silicon aarch64)
- Windows 2022 (x86_64)

**Test Commands**:
```bash
cargo test --all-features
cargo clippy -- -D warnings
cargo fmt --check
cargo build --release
./target/release/token-count --model gemini < test.txt
```

**Acceptance**: All tests pass on all platforms.

---

## Edge Cases

### Edge Case 1: Empty Input
**Scenario**: User pipes empty string  
**Expected**: Output `0`, exit code 0  
**Rationale**: Consistent with OpenAI/Claude behavior

```bash
$ echo -n "" | token-count --model gemini
0
```

---

### Edge Case 2: Very Large Input (>100MB)
**Scenario**: User provides massive file  
**Expected**: Process successfully (streaming) or fail gracefully with helpful error  
**Rationale**: SentencePiece may have limits, handle gracefully

```bash
$ token-count --model gemini < 500mb-file.txt
# If supported: token count
# If not: Error: Input too large (500MB). Maximum supported: 100MB.
```

---

### Edge Case 3: Invalid UTF-8
**Scenario**: User provides binary data  
**Expected**: Clear error message, exit code 1  
**Rationale**: Consistent with OpenAI/Claude behavior

```bash
$ token-count --model gemini < binary.bin
Error: Input contains invalid UTF-8 at byte 0
```

---

### Edge Case 4: Model Name Typo
**Scenario**: User types `gemini-4` (doesn't exist)  
**Expected**: Suggest similar Gemini models, exit code 2  
**Rationale**: Fuzzy matching helps users discover models

```bash
$ echo "test" | token-count --model gemini-4
Error: Unknown model: 'gemini-4'

Did you mean one of these Gemini models?
  - gemini-3-flash-preview
  - gemini-3.1-pro-preview
  - gemini-3.1-flash-lite-preview

See all models: token-count --list-models
```

---

### Edge Case 5: Gemini 1.5 Pro Context Window
**Scenario**: User tokenizes with `gemini-1.5-pro` (2M context)  
**Expected**: Correct context window displayed (2M, not 1M)  
**Rationale**: 1.5 Pro is the only Gemini model with 2M context

```bash
$ cat file.txt | token-count --model gemini-1.5-pro -v
Model: gemini-1.5-pro (gemini-gemma3)
Tokens: 500000
Context window: 2000000 tokens (25.00% used)
```

---

### Edge Case 6: Tokenizer Initialization Failure
**Scenario**: Embedded model is corrupted or missing  
**Expected**: Clear error message with bug report link  
**Rationale**: Should never happen, but handle gracefully

```bash
$ token-count --model gemini < file.txt
Error: Failed to initialize Gemini tokenizer

This is likely a bug. Please report it:
  https://github.com/shaunburdick/token-count/issues

Include: OS, architecture, token-count version
```

---

## Out of Scope

### ❌ Multimodal Token Counting (Images, Audio, Video)
**Reason**: Text-only for MVP (v0.3.0). Multimodal requires API calls (images/audio are tokenized differently).  
**Future**: Consider for v0.4.0 with `--accurate` flag (API mode).

### ❌ Function Calling Token Counting
**Reason**: Complex structured content, niche use case.  
**Future**: Consider for v0.4.0 if `gemini-tokenizer` supports it.

### ❌ Google AI Studio Integration
**Reason**: CLI tool, not a web service.  
**Future**: Not planned.

### ❌ Cost Estimation
**Reason**: Violates Constitution (pricing changes frequently, adds maintenance burden).  
**Future**: Not planned (users can calculate: tokens × price-per-token).

### ❌ Model Comparison Mode
**Reason**: Users can run token-count multiple times.  
**Future**: Not planned.

### ❌ Gemini-Specific Flags
**Reason**: Keep CLI simple and consistent.  
**Future**: Not planned.

---

## Success Metrics

### Product Metrics (6 months post-launch)
- 🎯 20% of token-count usage is Gemini models (based on GitHub stars/downloads)
- 🎯 Zero bug reports about Gemini token count accuracy
- 🎯 <5 feature requests for Gemini improvements (means it's complete)

### Technical Metrics
- 🎯 100% test coverage for Gemini module
- 🎯 Zero clippy warnings
- 🎯 Binary size <15MB (currently ~11.5MB projected)
- 🎯 Performance targets met (benchmarks)

### User Experience Metrics
- 🎯 95%+ of Gemini tokenizations complete in <100ms
- 🎯 Zero "how do I use Gemini" support questions (means UX is intuitive)
- 🎯 Positive feedback on offline capability (vs API-based tools)

---

## Implementation Notes

### Module Structure
```rust
// src/tokenizers/google/mod.rs
pub struct GoogleTokenizer {
    tokenizer: LocalTokenizer,
}

impl GoogleTokenizer {
    pub fn new() -> Result<Self> {
        let tokenizer = LocalTokenizer::new("gemini-3-flash-preview")
            .context("Failed to initialize Gemini tokenizer")?;
        Ok(Self { tokenizer })
    }
    
    pub fn count_tokens(&self, text: &str) -> Result<usize> {
        let result = self.tokenizer.count_tokens(text, None);
        Ok(result.total_tokens)
    }
    
    pub fn compute_tokens(&self, text: &str) -> Result<TokenDetails> {
        let result = self.tokenizer.compute_tokens(text);
        // Map to our TokenDetails type
        Ok(TokenDetails { ... })
    }
}

// src/tokenizers/google/models.rs
pub const GEMINI_MODELS: &[ModelDef] = &[
    ModelDef {
        name: "gemini-3.1-pro-preview",
        encoding: "gemini-gemma3",
        context_window: 1_000_000,
        aliases: &["gemini-pro", "gemini-3-pro"],
    },
    ModelDef {
        name: "gemini-3-flash-preview",
        encoding: "gemini-gemma3",
        context_window: 1_000_000,
        aliases: &["gemini", "gemini-flash", "gemini-3-flash"],
    },
    // ... other models
];
```

---

### Error Handling Strategy
```rust
// Use anyhow for error propagation
use anyhow::{Context, Result};

// Wrap gemini-tokenizer errors with context
let tokenizer = LocalTokenizer::new(model)
    .context("Failed to initialize Gemini tokenizer")?;

// Handle invalid UTF-8 consistently
let text = std::str::from_utf8(&input)
    .context("Input contains invalid UTF-8")?;
```

---

### Testing Strategy
```rust
// Unit test: Basic tokenization
#[test]
fn test_gemini_tokenization() {
    let tokenizer = GoogleTokenizer::new().unwrap();
    let count = tokenizer.count_tokens("Hello, Gemini!").unwrap();
    assert_eq!(count, 4); // Verified against Google's tokenizer
}

// Integration test: CLI end-to-end
#[test]
fn test_gemini_cli() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(&["--model", "gemini"])
        .write_stdin("Hello, Gemini!")
        .assert()
        .success()
        .stdout("4\n");
}
```

---

## Documentation Requirements

### README.md Updates
- Add Gemini to "Supported Models" section
- Update feature list (OpenAI + Claude + **Gemini**)
- Add Gemini examples to usage section
- Update model count badge (4 OpenAI + 3 Claude + **8 Gemini** = 15 total)

### CHANGELOG.md Entry
```markdown
## [0.3.0] - 2026-XX-XX

### 🚀 Google Gemini Support

Added exact, offline tokenization for Google Gemini models using the 
gemini-tokenizer crate. All Gemini models (1.5, 2.x, 3.x) use the same 
Gemma 3 SentencePiece tokenizer (262,144 vocab).

### Added

#### Gemini Tokenization
- **8 Gemini models**: 3.1-pro, 3-flash, 3.1-flash-lite (3.x preview), 
  2.5-pro, 2.5-flash, 2.5-flash-lite (2.5 GA), 1.5-pro, 1.5-flash (1.5 legacy)
- **Exact offline tokenization** using gemini-tokenizer crate (v0.2.0+)
- **Model aliases**: 
  - `gemini` → `gemini-3-flash-preview` (default)
  - Short names: `gemini-pro`, `gemini-flash`, `gemini-lite`
  - Provider prefix: `google/gemini`, `google/gemini-pro`
- **Context windows**: 1M tokens (most models), 2M tokens (gemini-1.5-pro)

#### CLI Enhancements
- Same CLI interface as OpenAI/Claude (no special flags)
- All verbosity levels supported (`-v`, `-vv`, `-vvv`)
- `--list-models` shows Gemini models with aliases

#### Testing
- **35+ tests** for Gemini support (unit + integration)
- **Total: 187 tests** (increased from 152)
- All tests passing

### Changed
- **Binary size**: Increased from 9.2MB to ~11.5MB (+2.3MB for embedded tokenizer)
- **Dependencies**: Added gemini-tokenizer 0.2.0, sentencepiece ^0.11, sha2 ^0.10

### Technical Details
- **Tokenizer**: Gemma 3 SentencePiece model (262,144 vocab, ~2MB embedded)
- **Performance**: <1ms for small inputs, ~50ms for 1MB input
- **Architecture**: Fully offline, no API calls, no API key required
```

### INSTALL.md Updates
- No changes needed (Gemini support is built-in)

---

## Rollout Plan

### Phase 1: Implementation (Week 1)
1. Create feature branch: `004-gemini-support`
2. Add `gemini-tokenizer` dependency to `Cargo.toml`
3. Implement `src/tokenizers/google/` module
4. Register models in `src/tokenizers/registry.rs`
5. Update CLI to support Gemini models

### Phase 2: Testing (Week 1-2)
1. Write unit tests (35+ tests)
2. Write integration tests (CLI end-to-end)
3. Run comparison tests vs Google's Python SDK (one-time)
4. Performance benchmarks
5. Cross-platform CI tests

### Phase 3: Documentation (Week 2)
1. Update README.md
2. Update CHANGELOG.md
3. Update `--list-models` output
4. Add code comments

### Phase 4: Release (Week 2-3)
1. Code review (ensure quality standards)
2. Merge to `main`
3. Bump version to v0.3.0
4. Create release tag
5. Publish to crates.io
6. Update Homebrew formula
7. Announce on GitHub, Twitter, Reddit

---

## Migration Notes

### For Users Upgrading from v0.2.x
- No breaking changes
- New `--model gemini` option available
- OpenAI and Claude continue to work identically

### For Library Users (Future)
```rust
// Using token-count as a library
use token_count::Tokenizer;

let tokenizer = Tokenizer::new("gemini")?;
let count = tokenizer.count_tokens("Hello, Gemini!")?;
println!("Tokens: {}", count);
```

---

## Related Documents

- [Research: Gemini Tokenization](../.specify/RESEARCH-GEMINI-TOKENIZATION.md) - Technical research and alternatives
- [Constitution](../.specify/memory/constitution.md) - Principles guiding this feature
- [Feature 001: Core CLI](001-core-cli.md) - OpenAI tokenization (similar approach)
- [Feature 003: Claude Support](003-claude-support.md) - Claude tokenization (contrast: estimation vs exact)

---

## Open Questions

None. All questions resolved during specification phase.

---

## Version History

### v1.0 (2026-03-14)
- Initial specification
- Focus on Gemini 3.x preview models (2.5 deprecated June 2026)
- Default model: `gemini-3-flash-preview`
- No preview warnings (clean UX)
- Include `-preview` suffix in canonical names (transparent)

---

**Specification Status**: ✅ Complete and ready for planning phase  
**Next Step**: Hand off to `modern-architect-engineer` for implementation planning
