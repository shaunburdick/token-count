# Implementation Plan: Claude Model Support

**Branch**: `003-claude-support` | **Date**: 2026-03-14 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `.specify/features/003-claude-support.md`

## Summary

Add support for Anthropic Claude models (4.6, 4.5, 4.1, 4.0 series) using a **hybrid tokenization strategy**:

1. **Default: Offline Estimation** - Adaptive character-based heuristics with content-type detection (code vs. prose) targeting ±10% accuracy
2. **Optional: Accurate API Mode** - Anthropic's official count_tokens API for exact counts (requires `ANTHROPIC_API_KEY` and user consent)

This approach balances Constitutional Principles:
- **Principle III** (Zero Runtime Dependencies) - Offline estimation works anywhere
- **Principle II** (Accuracy Over Speed) - Optional API mode for exact counts when needed
- **Principle V** (Fail Fast with Clear Errors) - Clear consent prompts and error handling

**Key Technical Decisions**:
- NO caching (short-lived CLI doesn't benefit from in-memory cache)
- Adaptive estimation algorithm with code/prose detection (±10% target vs. ±20% simple approach)
- Interactive consent prompt pattern (reusable for future OpenAI, Gemini API integrations)
- TTY detection for non-interactive environments (require `-y` flag)

---

## Technical Context

**Language/Version**: Rust 1.85.0+ (stable channel)  
**Primary Dependencies**: 
- Existing: `clap 4.6.0+`, `tiktoken-rs 0.9.1+`, `anyhow 1.0.102+`, `thiserror 1.0+`, `strsim 0.11+`
- New: `reqwest 0.12+` (JSON + rustls-tls), `serde 1.0.149+`, `serde_json 1.0.149+`

**Storage**: N/A (no persistent cache, stateless CLI)  
**Testing**: `cargo test` with unit + integration tests, mock API responses  
**Target Platform**: Linux x64, macOS x64/ARM64, Windows x64 (cross-platform CLI)  
**Project Type**: CLI tool with library API  
**Performance Goals**: 
- Estimation: <10ms for 1KB, <100ms for 1MB
- API mode: <500ms total (includes network latency)
- Memory: <50MB for 10MB input (no cache overhead)

**Constraints**:
- Offline-capable by default (no network calls without explicit `--accurate` flag)
- User consent required before API calls (interactive prompt or `-y` flag)
- Binary size: <15MB increase (reqwest + rustls adds ~2-3MB)
- Cross-platform identical behavior

**Scale/Scope**: 
- 8 Claude models (Opus/Sonnet/Haiku across 4.6/4.5/4.1/4.0)
- ~15 aliases (short names, version variants, provider prefixes)
- 2 tokenization modes (estimation vs. API)

---

## Constitution Check

### ✅ Aligned Principles

**I. POSIX Simplicity**
- Maintains single responsibility: count tokens, report results
- Stdin → stdout pattern unchanged
- No new complexity in core workflow

**II. Accuracy Over Speed**
- Offline estimation: ±10% target (better than ±20% simple approach)
- API mode: Exact counts from official Anthropic API
- Clear distinction in output (`~42` estimated vs. `42` exact)

**III. Zero External Dependencies at Runtime**
- ✅ Default behavior (estimation) requires no network
- ✅ API mode is opt-in via `--accurate` flag
- ✅ Works fully offline unless user explicitly requests API mode

**V. Fail Fast with Clear Errors**
- Missing API key → helpful error with setup instructions
- Network failure → automatic fallback to estimation with warning
- Non-interactive without `-y` → clear error with examples

**VII. Semantic Versioning**
- This is v0.2.0 (minor version) - adds new models/features
- No breaking changes to existing OpenAI functionality

### ⚠️ Principle Tension (Resolved)

**Principle II (Accuracy) vs. Principle III (Zero Dependencies)**

**Tension**: Claude lacks open-source tokenizer (unlike OpenAI's tiktoken). Must choose between:
- A) Estimation only (offline, ±10-20% accuracy)
- B) API only (exact, requires network)
- C) Hybrid (estimation default, API opt-in)

**Resolution**: **Option C (Hybrid)** approved by user
- Default mode satisfies Principle III (offline, zero dependencies)
- Optional mode satisfies Principle II (accuracy when needed)
- User explicitly opts into API mode via `--accurate` flag
- Constitution allows opt-in network calls for accuracy

**Justification**: 
1. Anthropic does not provide open tokenizer (research confirmed)
2. Adaptive estimation (±10% target) is acceptable for most use cases
3. Power users can opt into exact counts via API
4. Pattern enables future API integrations (OpenAI streaming, Gemini)

---

## Project Structure

### Documentation (this feature)

```text
specs/003-claude-support/
├── plan.md              # This file (implementation plan)
├── research.md          # Technology research (already exists)
├── data-model.md        # Data structures and types
├── quickstart.md        # Validation scenarios
├── contracts/           # API schemas and interfaces
│   ├── anthropic-api.yaml    # Anthropic API request/response
│   └── consent-prompt.yaml   # Consent interface (reusable)
└── tasks.md             # Task breakdown (created next phase)
```

### Source Code (repository root)

```text
src/
├── tokenizers/
│   ├── mod.rs           # [MODIFY] Add claude module export
│   ├── registry.rs      # [MODIFY] Add Claude models to registry
│   ├── openai.rs        # [NO CHANGE] Existing OpenAI tokenizer
│   └── claude/          # [NEW] Claude tokenization module
│       ├── mod.rs           # Public API, trait implementation
│       ├── estimation.rs    # Adaptive estimation algorithm
│       ├── api_client.rs    # Anthropic API HTTP client
│       └── models.rs        # Claude model definitions
│
├── api/                 # [NEW] API integration utilities
│   ├── mod.rs               # Module exports
│   └── consent.rs           # Reusable consent prompt mechanism
│
├── cli/
│   ├── mod.rs           # [NO CHANGE] Module exports
│   ├── args.rs          # [MODIFY] Add --accurate, -y flags
│   └── input.rs         # [NO CHANGE] Stdin handling
│
├── output/
│   ├── mod.rs           # [NO CHANGE] Module exports
│   ├── simple.rs        # [MODIFY] Handle ~ prefix for estimates
│   ├── verbose.rs       # [MODIFY] Show estimation method
│   └── debug.rs         # [NO CHANGE] Token ID debugging
│
├── error.rs             # [MODIFY] Add Claude-specific errors
├── lib.rs               # [MODIFY] Export claude module
└── main.rs              # [MODIFY] Handle --accurate + consent flow

tests/
├── integration/         # [NEW] Claude integration tests
│   ├── claude_estimation.rs    # Test estimation mode
│   ├── claude_api.rs           # Test API mode (mocked)
│   └── consent_prompt.rs       # Test TTY detection + consent
│
└── unit/                # [NEW] Claude unit tests
    ├── adaptive_estimation.rs  # Test content-type detection
    ├── model_aliases.rs        # Test Claude alias resolution
    └── api_errors.rs           # Test error handling

scripts/
└── validate-claude-accuracy.sh # [NEW] Compare estimation vs. API
```

**Structure Decision**: Single Rust project (existing structure)
- Follows existing `src/tokenizers/{provider}` pattern established by OpenAI
- New `src/api/` for cross-provider API utilities (consent, error handling)
- Claude gets its own submodule: `src/tokenizers/claude/` (3 files)
- Tests organized by test type (unit vs. integration)

---

## Architecture Decisions

### 1. Adaptive Estimation Algorithm

**Decision**: Implement content-type detection (code vs. prose) with different token ratios

**Rationale**:
- Code has more symbols/operators → more tokens per character (~3.0 chars/token)
- Prose has longer words → fewer tokens per character (~4.5 chars/token)
- Mixed content uses balanced ratio (~3.75 chars/token)
- Improves accuracy from ±20% (simple) to ±10% (adaptive)

**Implementation**:
```rust
pub enum ContentType {
    Code,   // High density of code indicators (>, 15%)
    Prose,  // Low density of code indicators (< 5%)
    Mixed,  // Moderate density (5-15%)
}

pub fn detect_content_type(text: &str) -> ContentType {
    // Count code indicators: {}, [], (), ;, //, fn, def, const, etc.
    // Calculate ratio: indicators / total_chars
    // Classify based on thresholds
}

pub fn estimate_tokens(text: &str) -> usize {
    let content_type = detect_content_type(text);
    let char_count = text.chars().count();
    
    match content_type {
        ContentType::Code => (char_count as f64 / 3.0).ceil() as usize,
        ContentType::Prose => (char_count as f64 / 4.5).ceil() as usize,
        ContentType::Mixed => (char_count as f64 / 3.75).ceil() as usize,
    }
}
```

**Validation**: Script compares estimation vs. API across 100+ diverse inputs

---

### 2. API Client with Retry Logic

**Decision**: Use `reqwest` with rustls-tls, 3 retry attempts with exponential backoff

**Rationale**:
- `reqwest` is Rust standard for HTTP clients (used by cargo, etc.)
- `rustls-tls` avoids OpenSSL dependency (cross-platform simplicity)
- Retry logic handles transient network failures gracefully
- 30s timeout prevents hanging on network issues

**Implementation**:
```rust
pub struct ClaudeApiClient {
    client: reqwest::Client,
    api_key: String,
}

impl ClaudeApiClient {
    pub async fn count_tokens(&self, model: &str, text: &str) -> Result<usize> {
        let mut attempts = 0;
        let max_attempts = 3;
        
        while attempts < max_attempts {
            match self.try_count_tokens(model, text).await {
                Ok(count) => return Ok(count),
                Err(e) if attempts < max_attempts - 1 => {
                    let backoff = 2u64.pow(attempts) * 1000; // 2s, 4s, 8s
                    tokio::time::sleep(Duration::from_millis(backoff)).await;
                    attempts += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

**Error Handling**:
- 401 Unauthorized → "Invalid API key"
- 429 Rate Limited → "Rate limit exceeded"
- 5xx Server Error → "Anthropic API error"
- Network timeout → Fall back to estimation

---

### 3. Consent Prompt Pattern (Reusable)

**Decision**: Centralized consent mechanism in `src/api/consent.rs` for all future API integrations

**Rationale**:
- This pattern applies to future OpenAI, Gemini, etc. API calls
- Centralized implementation ensures consistent UX
- TTY detection prevents hanging in CI/CD environments
- `-y` flag enables scripting/automation

**Implementation**:
```rust
pub struct ConsentPrompt {
    provider: &'static str,
    api_endpoint: &'static str,
}

impl ConsentPrompt {
    pub fn ask(&self) -> Result<bool> {
        // Check if stdin is TTY
        if !std::io::stdin().is_terminal() {
            return Err(anyhow!("Non-interactive mode requires -y flag"));
        }
        
        // Display prompt on stderr (don't pollute stdout)
        eprintln!("This will send your input to {}'s API...", self.provider);
        eprintln!("Your input will be transmitted over HTTPS to: {}", self.api_endpoint);
        eprintln!();
        eprint!("Proceed with API call? (y/N): ");
        
        // Read user response
        let mut response = String::new();
        std::io::stdin().read_line(&mut response)?;
        
        // Accept: y, Y, yes, YES
        // Reject: n, N, no, NO, empty/Enter
        Ok(response.trim().to_lowercase() == "y" 
           || response.trim().to_lowercase() == "yes")
    }
}
```

**Usage**:
```rust
// In main.rs
if args.accurate && !args.yes {
    let consent = ConsentPrompt {
        provider: "Anthropic",
        api_endpoint: "https://api.anthropic.com",
    };
    
    if !consent.ask()? {
        eprintln!("Falling back to estimation (API call cancelled by user)");
        // Use estimation instead
    }
}
```

---

### 4. TokenCount Enum (Estimation vs. Exact)

**Decision**: Use enum to distinguish estimated vs. exact counts at type level

**Rationale**:
- Type system prevents mixing estimated and exact counts
- Clear semantics in code (`TokenCount::Estimated(42)` vs. `TokenCount::Exact(42)`)
- Enables correct formatting in output layer (`~42` vs. `42`)

**Implementation**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenCount {
    Estimated(usize),
    Exact(usize),
}

impl TokenCount {
    pub fn value(&self) -> usize {
        match self {
            Self::Estimated(n) | Self::Exact(n) => *n,
        }
    }
    
    pub fn is_estimated(&self) -> bool {
        matches!(self, Self::Estimated(_))
    }
}

impl std::fmt::Display for TokenCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Estimated(n) => write!(f, "~{}", n),
            Self::Exact(n) => write!(f, "{}", n),
        }
    }
}
```

---

### 5. No Caching (Simplified Architecture)

**Decision**: Remove all caching logic from original spec

**Rationale**:
- CLI is short-lived (each invocation starts fresh)
- Users rarely count same input twice in single session
- No persistent state between runs
- Simpler architecture, fewer dependencies, less memory

**Impact**:
- No `lru` dependency needed
- No cache invalidation logic
- No concurrent access concerns
- ~50MB less memory usage (no cache overhead)

---

### 6. Model Registry Extension

**Decision**: Extend existing `ModelRegistry` to support Claude models alongside OpenAI

**Rationale**:
- Reuses existing alias resolution, fuzzy matching, list-models functionality
- Single source of truth for all models
- Consistent UX across providers

**Implementation**:
```rust
// In registry.rs
impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self { /* ... */ };
        
        // [Existing OpenAI models...]
        
        // Claude 4.6 models
        registry.add_model(ModelConfig {
            name: "claude-opus-4-6".to_string(),
            encoding: "anthropic-claude".to_string(), // Marker for Claude tokenizer
            context_window: 1_000_000,
            description: "Claude Opus 4.6 (1M context)".to_string(),
            aliases: vec![
                "opus-4-6".to_string(),
                "opus".to_string(),
                "anthropic/claude-opus-4-6".to_string(),
            ],
        });
        
        // [More Claude models...]
        
        registry
    }
}
```

**Tokenizer Selection**:
```rust
pub fn get_tokenizer(&self, model_name: &str) -> Result<Box<dyn Tokenizer>> {
    let canonical_name = self.resolve_model_name(model_name)?;
    let config = self.get_config(&canonical_name)?;
    
    match config.encoding.as_str() {
        "cl100k_base" | "o200k_base" => {
            Ok(Box::new(OpenAITokenizer::new(&config.encoding)?))
        }
        "anthropic-claude" => {
            Ok(Box::new(ClaudeTokenizer::new(config)?))
        }
        _ => Err(TokenError::UnsupportedEncoding(config.encoding.clone())),
    }
}
```

---

## Risks & Mitigations

### Risk 1: Estimation Accuracy Lower Than Target (±10%)

**Likelihood**: Medium  
**Impact**: Medium (users may distrust estimation mode)

**Mitigation**:
1. Validation script tests 100+ diverse inputs against API
2. Document actual measured accuracy in README
3. Clear messaging: "±10% target, actual results may vary"
4. Recommend `--accurate` for production use cases
5. Continuous improvement: adjust ratios based on validation data

---

### Risk 2: Anthropic API Changes

**Likelihood**: Low (API is stable)  
**Impact**: High (breaks API mode completely)

**Mitigation**:
1. Pin API version header: `anthropic-version: 2023-06-01`
2. Graceful fallback to estimation on API errors
3. Clear error messages indicating API issues
4. Monitor Anthropic changelog for breaking changes
5. Integration tests with mocked API responses catch schema changes

---

### Risk 3: Non-Interactive Prompt Hanging

**Likelihood**: Medium (users forget `-y` in scripts)  
**Impact**: High (CI/CD pipelines hang indefinitely)

**Mitigation**:
1. TTY detection with `std::io::stdin().is_terminal()`
2. Non-interactive mode errors immediately (no prompt)
3. Clear error message with `-y` flag example
4. Documentation emphasizes `-y` for scripting
5. Integration tests validate non-interactive behavior

---

### Risk 4: Binary Size Growth

**Likelihood**: Low  
**Impact**: Low (constitution allows growth for accuracy)

**Mitigation**:
1. `reqwest` with `rustls-tls` adds only ~2-3MB
2. No large vocabularies needed (estimation is algorithmic)
3. Release profile optimizations (LTO, strip) minimize growth
4. Expected total: 45-50MB (within constitution's 50MB target)

---

### Risk 5: Rate Limiting on API Mode

**Likelihood**: Medium (100 RPM limit on free tier)  
**Impact**: Low (estimation fallback works)

**Mitigation**:
1. Retry logic respects `retry-after` header
2. Automatic fallback to estimation with warning
3. Clear error messages explaining rate limits
4. No silent failures (always indicate estimation vs. exact)

---

## Testing Strategy

### Unit Tests

**Coverage Target**: ≥80% for new code

**Key Test Cases**:
1. **Adaptive Estimation**:
   - Pure code input (expect ~3.0 chars/token)
   - Pure prose input (expect ~4.5 chars/token)
   - Mixed content input (expect ~3.75 chars/token)
   - Empty string (expect 0 tokens)
   - Emoji-heavy text (expect correct char counting)
   - CJK characters (expect correct UTF-8 handling)

2. **Model Registry**:
   - Claude alias resolution (`claude` → `claude-sonnet-4-6`)
   - Case-insensitive matching (`CLAUDE-OPUS-4-6`)
   - Fuzzy suggestions on typos (`claude-sonet` → suggestions)
   - Provider prefix (`anthropic/claude-sonnet-4-6`)

3. **API Error Handling**:
   - Parse 401 Unauthorized response
   - Parse 429 Rate Limited response
   - Parse 5xx server error response
   - Handle network timeout
   - Handle malformed JSON response

4. **Consent Prompt**:
   - TTY detection (interactive vs. non-interactive)
   - Accept responses (`y`, `Y`, `yes`)
   - Reject responses (`n`, `N`, `no`, empty)
   - Non-interactive error message

### Integration Tests

**Test Scenarios**:
1. **CLI - Estimation Mode**:
   ```rust
   #[test]
   fn test_claude_estimation_stdin() {
       let output = run_cli(&["--model", "claude-sonnet-4-6"], "Hello, world!");
       assert!(output.starts_with("~")); // Estimated count
       assert!(output.trim().parse::<usize>().is_ok()); // Valid number
   }
   ```

2. **CLI - Accurate Mode (No API Key)**:
   ```rust
   #[test]
   fn test_accurate_without_api_key() {
       env::remove_var("ANTHROPIC_API_KEY");
       let result = run_cli_expect_error(&["--model", "claude", "--accurate"], "test");
       assert!(result.contains("requires ANTHROPIC_API_KEY"));
   }
   ```

3. **CLI - Non-Interactive Without -y**:
   ```rust
   #[test]
   fn test_accurate_non_interactive_without_yes() {
       env::set_var("ANTHROPIC_API_KEY", "sk-test");
       let result = pipe_stdin_expect_error(&["--model", "claude", "--accurate"], "test");
       assert!(result.contains("Non-interactive mode"));
       assert!(result.contains("-y/--yes"));
   }
   ```

4. **API Client (Mocked)**:
   ```rust
   #[tokio::test]
   async fn test_api_success() {
       let mock_server = MockServer::start().await;
       // Mock Anthropic API response
       // Verify request format
       // Assert token count returned
   }
   ```

### Manual Testing (Validation Script)

**File**: `scripts/validate-claude-accuracy.sh`

```bash
#!/bin/bash
# Compare estimation vs. API for diverse inputs
# Requires ANTHROPIC_API_KEY

test_inputs=(
    "Hello, world!"
    "def fibonacci(n): return n if n < 2 else fibonacci(n-1) + fibonacci(n-2)"
    "The quick brown fox jumps over the lazy dog."
    # ... 100+ more diverse inputs
)

for input in "${test_inputs[@]}"; do
    estimated=$(echo "$input" | token-count --model claude-sonnet-4-6)
    exact=$(echo "$input" | token-count --model claude-sonnet-4-6 --accurate -y)
    
    # Calculate error percentage
    # Log results to CSV
done

# Output summary statistics:
# - Mean error
# - Median error
# - 95th percentile error
# - Max error
```

---

## Dependencies Update

**Cargo.toml Changes**:

```toml
[dependencies]
# Existing (no changes)
clap = { version = "4.6", features = ["derive"] }
tiktoken-rs = "0.9.1"
anyhow = "1.0.102"
thiserror = "1.0"
strsim = "0.11"

# NEW: HTTP client for Anthropic API
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
tokio = { version = "1", features = ["rt", "macros"] }

# NEW: JSON serialization
serde = { version = "1.0.149", features = ["derive"] }
serde_json = "1.0.149"

[dev-dependencies]
# Existing
criterion = { version = "0.5", features = ["html_reports"] }
tempfile = "3.0"
assert_cmd = "2.0"
predicates = "3.0"

# NEW: Mock HTTP server for API tests
mockito = "1.0"
```

**Binary Size Impact**:
- `reqwest` + `rustls`: +2-3MB
- `tokio` runtime: +1MB (async support)
- `serde` + `serde_json`: +0.5MB
- **Total increase**: ~3-4MB
- **Expected total binary**: 45-50MB (within 50MB constitution target)

---

## Implementation Order

### Phase 1: Foundation (Days 1-2)
1. Add CLI flags (`--accurate`, `-y`) to `args.rs`
2. Extend error types in `error.rs` for Claude errors
3. Create `TokenCount` enum in `tokenizers/mod.rs`
4. Create consent prompt in `api/consent.rs` with TTY detection
5. Write unit tests for consent prompt

### Phase 2: Estimation (Day 2)
6. Implement `ContentType` enum and `detect_content_type()` in `claude/estimation.rs`
7. Implement `estimate_tokens()` function
8. Write comprehensive unit tests for estimation (code/prose/mixed/edge cases)
9. Create `ClaudeTokenizer` struct in `claude/mod.rs`

### Phase 3: API Client (Day 3)
10. Implement `ClaudeApiClient` in `claude/api_client.rs` with retry logic
11. Create request/response structs with serde
12. Implement error handling (401, 429, 5xx, timeout)
13. Write unit tests for API error parsing
14. Write integration tests with mocked HTTP server

### Phase 4: Integration (Day 4)
15. Add Claude models to `ModelRegistry` in `registry.rs`
16. Update `get_tokenizer()` to instantiate `ClaudeTokenizer`
17. Modify `main.rs` to handle `--accurate` flag + consent flow
18. Update `output/simple.rs` to format `TokenCount` enum (~ prefix)
19. Update `output/verbose.rs` to show estimation method

### Phase 5: Testing & Validation (Day 5)
20. Write end-to-end integration tests for CLI
21. Create validation script `scripts/validate-claude-accuracy.sh`
22. Run validation against 100+ inputs (requires API key)
23. Document actual accuracy metrics in README
24. Update CHANGELOG.md

---

## Success Criteria

### Must Have (MVP)
- ✅ 8 Claude models supported (Opus/Sonnet/Haiku 4.6/4.5/4.1/4.0)
- ✅ Offline estimation mode works without API key
- ✅ `--accurate` mode uses Anthropic API with consent prompt
- ✅ Clear visual distinction: `~42` (estimated) vs. `42` (exact)
- ✅ Graceful fallback to estimation on API failure
- ✅ Non-interactive mode requires `-y` flag (no hanging)
- ✅ Zero clippy warnings (`cargo clippy -- -D warnings`)
- ✅ ≥80% test coverage for new code
- ✅ All existing tests still pass (no regression)

### Should Have (Post-MVP)
- ✅ Validation script with accuracy metrics
- ✅ Fuzzy model name suggestions on typos
- ✅ Comprehensive error messages with examples
- ✅ Documentation in README with examples

### Could Have (Future v0.3.0)
- [ ] JSON output mode (`--format json`)
- [ ] Batch mode (multiple files)
- [ ] Configuration file for default model

---

## Next Steps

1. ✅ Planning complete - Review this plan
2. ⏭️ Create `data-model.md` - Define all structs, enums, types
3. ⏭️ Create `contracts/` - API schemas and interfaces
4. ⏭️ Create `quickstart.md` - Validation scenarios
5. ⏭️ Update agent context with new patterns
6. ⏭️ Break down into tasks in `tasks.md`
7. ⏭️ Begin TDD implementation (Phase 1: Foundation)

---

**Status**: ✅ Planning Phase Complete  
**Estimated Effort**: 5 days (implementation + testing + validation)  
**Risk Level**: Low (clear architecture, graceful fallback, good test coverage)
