# Research: Claude Tokenization Options

**Feature**: 003-claude-support  
**Research Date**: 2026-03-14  
**Researcher**: Spec-Driven Planner Agent

---

## Executive Summary

Anthropic Claude 4.6 models (Opus, Sonnet, Haiku) represent a major upgrade released February 2026. Unlike OpenAI, Anthropic does **not** provide an open-source tokenizer. We have three viable approaches: estimation-only (offline), API-only (accurate), or hybrid (both). **Recommendation: Hybrid approach** to balance constitutional principles.

---

## Current Claude Model Landscape

### Latest Generation (Claude 4.6 - Feb 2026)

| Model | API ID | Context | Pricing (Input/Output per MTok) | Use Case |
|-------|--------|---------|--------------------------------|----------|
| **Opus 4.6** | `claude-opus-4-6` | 1M tokens | $5 / $25 | Coding, agents, complex reasoning |
| **Sonnet 4.6** | `claude-sonnet-4-6` | 1M tokens | $3 / $15 | Balanced performance (new default) |
| **Haiku 4.5** | `claude-haiku-4-5-20251001` | 200K tokens | $1 / $5 | Speed, lightweight tasks |

### Legacy Models (Still Active)

| Model | API ID | Context | Status |
|-------|--------|---------|--------|
| Sonnet 4.5 | `claude-sonnet-4-5-20250929` | 200K-1M | Active |
| Opus 4.5 | `claude-opus-4-5-20251101` | 200K | Active |
| Opus 4.1 | `claude-opus-4-1-20250805` | 200K | Active |
| Sonnet 4.0 | `claude-sonnet-4-20250514` | 200K-1M | Active |
| Opus 4.0 | `claude-opus-4-20250514` | 200K | Active |
| Haiku 3 | `claude-3-haiku-20240307` | 200K | **Deprecated** (EOL April 19, 2026) |

**Source**: https://docs.anthropic.com/en/docs/about-claude/models/overview

---

## Tokenization Technical Options

### Option 1: Anthropic Token Counting API ✅

**Endpoint**: `POST https://api.anthropic.com/v1/messages/count_tokens`

**Accuracy**: Exact (official)

**Example**:
```bash
curl https://api.anthropic.com/v1/messages/count_tokens \
  --header "x-api-key: $ANTHROPIC_API_KEY" \
  --header "anthropic-version: 2023-06-01" \
  --data '{
    "model": "claude-opus-4-6",
    "messages": [{"role": "user", "content": "Hello, Claude"}]
  }'
# Response: {"input_tokens": 14}
```

**Pros**:
- 100% accurate (official API)
- Supports all model features (tools, images, PDFs, thinking blocks)
- Free to use (no billing, but rate-limited)
- Handles all current and future models

**Cons**:
- Requires network connection (violates Constitution Principle III: Zero Runtime Dependencies)
- Rate limits: 100-8,000 requests/min based on usage tier
- Adds latency (~50-200ms per request)
- Requires API key setup

**Rate Limits** (from Anthropic docs):
| Usage Tier | Requests Per Minute |
|------------|---------------------|
| 1 (free)   | 100 RPM             |
| 2-4 (paid) | 2,000-8,000 RPM     |

**Documentation**: https://docs.anthropic.com/en/docs/build-with-claude/token-counting

---

### Option 2: claude-tokenizer Crate ⚠️

**Crate**: `claude-tokenizer` v0.3.0  
**Repository**: https://github.com/Jellyfishboy/claude-tokenizer  
**Last Updated**: September 2024

**Dependencies**:
```toml
[dependencies]
anyhow = "1.0.89"
tokenizers = "0.20.0"  # HuggingFace tokenizers library
```

**Pros**:
- Offline capability (embeds JSON tokenizer data)
- Fast (local computation)
- MIT licensed

**Cons**:
- ⚠️ **Unofficial** - Not from Anthropic, no guarantee of accuracy
- ⚠️ **Outdated** - Last updated Sept 2024 (before Claude 4.6 release Feb 2026)
- ⚠️ **Low adoption** - Only 2 GitHub stars, 13,982 total downloads, 2 reverse deps
- ⚠️ **Unverified accuracy** - No published benchmarks against Anthropic API
- ⚠️ **Model coverage unclear** - Doesn't mention Claude 4.x support
- ⚠️ **Uses HuggingFace tokenizers** - May not match Anthropic's internal tokenizer

**Risk Assessment**: **HIGH RISK** for production use. Cannot verify this produces accurate counts for Claude 4.6 models.

---

### Option 3: Character-Based Estimation 🔄

**Approach**: Use heuristics based on character/word counts with safety margins.

**Algorithm** (from community research):
```
tokens_estimate = (char_count / 4.0).ceil()  # Conservative baseline
tokens_upper_bound = (char_count / 3.0).ceil()  # Safety margin
```

**Accuracy** (based on reverse engineering research):
- English text: ~4-5 characters per token
- Code: ~3-4 characters per token (more tokens due to symbols)
- Expected accuracy: ±15-20% from true count

**Pros**:
- Fully offline (no network dependency)
- Fast (millisecond-level computation)
- No external dependencies
- Works in air-gapped environments

**Cons**:
- Not exact (estimation only)
- Accuracy varies by content type (prose vs. code vs. mixed)
- Cannot account for special tokens, system prompts, tool definitions
- May mislead users into thinking counts are exact

**Mitigation**:
- Clearly label as "estimated" in output
- Document accuracy range in help text
- Provide example comparison to Anthropic API

**Research Sources**:
- https://grohan.co/2026/02/10/ctoc/ (reverse-engineered ~96% accuracy with 36K vocab)
- Community discussions on Claude token estimation

---

## Recommended Approach: Hybrid Strategy

**Proposal**: Implement **both estimation and API** with clear UX distinction.

### Design

**Default Behavior (Offline)**:
```bash
echo "Hello, Claude" | token-count --model claude-sonnet-4-6
~4

# With verbose
echo "Hello, Claude" | token-count --model claude-sonnet-4-6 -v
Model: claude-sonnet-4-6
Tokens: ~4 (estimated, ±15-20% accuracy)
Estimation method: Character-based heuristic (4 chars/token)
For exact count, use: --accurate (requires ANTHROPIC_API_KEY)
```

**Accurate Mode (Online)**:
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "Hello, Claude" | token-count --model claude-sonnet-4-6 --accurate
3

# With verbose
echo "Hello, Claude" | token-count --model claude-sonnet-4-6 --accurate -v
Model: claude-sonnet-4-6
Tokens: 3 (exact via Anthropic API)
Context window: 1000000 tokens (0.0003% used)
```

**Error Handling** (API unavailable):
```bash
echo "Hello" | token-count --model claude-sonnet-4-6 --accurate
Error: Accurate mode requires ANTHROPIC_API_KEY environment variable
Tip: For offline estimation, omit --accurate flag

# Or, if API key set but network down:
Warning: Anthropic API unreachable, falling back to estimation
Model: claude-sonnet-4-6
Tokens: ~2 (estimated, API unavailable)
```

### Implementation Strategy

1. **Estimation Module** (`src/tokenizers/claude_estimation.rs`):
   - Character-based heuristic (4 chars/token baseline)
   - Return `TokenCount::Estimated(u32)` with metadata

2. **API Client Module** (`src/tokenizers/claude_api.rs`):
   - HTTP client using `reqwest` crate (already have network deps via other features)
   - Caching to avoid repeated API calls for same input
   - Rate limit handling (exponential backoff)
   - Return `TokenCount::Exact(u32)` with metadata

3. **Output Formatting**:
   - Simple mode: prefix `~` for estimates
   - Verbose mode: explain estimation method
   - Error messages guide users to alternate modes

### Alignment with Constitution

| Principle | Alignment |
|-----------|-----------|
| **I. POSIX Simplicity** | ✅ Default offline behavior preserves simplicity |
| **II. Accuracy Over Speed** | ✅ `--accurate` flag provides exact counts when needed |
| **III. Zero Runtime Dependencies** | ✅ Default offline mode has no network deps |
| **IV. Cross-Platform Support** | ✅ Works on all platforms (API client is cross-platform) |
| **V. Fail Fast with Clear Errors** | ✅ Clear messaging about estimation vs. exact |
| **VI. Trivial Installation** | ✅ No API key required for basic usage |

---

## Alternative Architectures Considered

### Architecture A: API-Only (Rejected)
**Why rejected**: Violates Principle III (Zero Runtime Dependencies). Users in air-gapped environments or without API keys cannot use Claude support.

### Architecture B: Estimation-Only (Viable but suboptimal)
**Why suboptimal**: Violates Principle II (Accuracy Over Speed). Users who need exact counts for billing/planning cannot get them.

### Architecture C: Download Tokenizer on First Use (Rejected)
**Why rejected**: 
- Anthropic doesn't provide downloadable tokenizer
- Would require network dependency anyway
- Adds complexity for installation

---

## Implementation Dependencies

### New Crate Dependencies

```toml
[dependencies]
# Existing (already in project)
anyhow = "1.0.102+"
thiserror = "1.0+"
clap = "4.6.0+"

# New for Claude API client
reqwest = { version = "0.12+", features = ["json", "rustls-tls"], optional = true }
serde = { version = "1.0.149+", features = ["derive"] }
serde_json = "1.0.149+"

[features]
default = ["claude-estimation"]
claude-estimation = []  # Offline estimation (always enabled)
claude-api = ["reqwest"]  # API client (optional, for --accurate mode)
```

**Size Impact**:
- Estimation only: +10KB (negligible)
- API client: +2-3MB (reqwest + TLS dependencies)

**Mitigation**: Make API client optional via feature flag. Users who only need estimation don't download network deps.

---

## Testing Strategy

### Validation Tests

1. **Estimation Accuracy Benchmark**:
   - Test against 100 diverse inputs (prose, code, mixed)
   - Compare estimates to Anthropic API ground truth
   - Target: ±20% accuracy, document actual results

2. **API Client Tests**:
   - Unit tests with mocked HTTP responses
   - Integration tests with real API (requires API key in CI)
   - Error handling tests (network down, invalid key, rate limits)

3. **Regression Tests**:
   - Known inputs with verified token counts from Anthropic
   - Ensure estimation algorithm doesn't drift over time

### Example Test Cases

```rust
#[test]
fn test_estimation_prose() {
    let input = "Hello, world!";
    let estimated = estimate_tokens(input);
    assert!(estimated >= 2 && estimated <= 4); // Known range: 3 tokens actual
}

#[test]
fn test_api_client_success() {
    // Mock Anthropic API response
    let response = r#"{"input_tokens": 14}"#;
    let count = parse_api_response(response).unwrap();
    assert_eq!(count, 14);
}

#[test]
fn test_api_fallback_on_error() {
    // Simulate API failure
    let result = count_tokens_with_fallback("Hello", true);
    assert!(matches!(result, TokenCount::Estimated(_)));
}
```

---

## Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Estimation accuracy degrades over time (Anthropic changes tokenizer) | Medium | Document accuracy range, provide --accurate flag, monitor via tests |
| API rate limits block users | Low | Implement exponential backoff, cache results, clear error messages |
| API key exposure in CI/CD logs | Medium | Use secret management, sanitize logs, document best practices |
| Users misinterpret estimates as exact | Medium | Prefix estimates with `~`, verbose mode explains clearly |
| reqwest dependency bloats binary | Low | Make API client optional feature flag |

---

## Open Questions for User

1. **Default behavior preference**:
   - Option A: Default to estimation (offline-first)
   - Option B: Default to API if key present, else estimate (accuracy-first)
   - **Recommendation**: Option A (offline-first) aligns with POSIX philosophy

2. **Model coverage**:
   - Support all legacy models (Claude 3.x, 4.0, 4.1, 4.5) or just latest (4.6)?
   - **Recommendation**: Support all active models for backward compatibility

3. **API key management**:
   - Env var only (`ANTHROPIC_API_KEY`) or config file support?
   - **Recommendation**: Env var only (simpler, POSIX-style)

4. **Caching strategy**:
   - Cache API results for repeated inputs (saves API calls)?
   - **Recommendation**: Yes, in-memory cache with 1000-entry LRU (improves UX in scripts)

---

## Next Steps

1. ✅ Research complete
2. ⏭️ Create feature specification (003-claude-support.md)
3. ⏭️ Get user approval on hybrid approach
4. ⏭️ Create implementation plan with data models and API contracts
5. ⏭️ Break down into tasks
6. ⏭️ Implement with TDD approach

---

## References

- **Anthropic Models Docs**: https://docs.anthropic.com/en/docs/about-claude/models/overview
- **Token Counting API**: https://docs.anthropic.com/en/docs/build-with-claude/token-counting
- **claude-tokenizer crate**: https://crates.io/crates/claude-tokenizer
- **Reverse engineering research**: https://grohan.co/2026/02/10/ctoc/
- **Anthropic API SDKs**: https://github.com/anthropics/anthropic-sdk-rust

---

**Status**: Research complete, awaiting user decision on hybrid approach  
**Last Updated**: 2026-03-14
