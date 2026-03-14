# Research: Gemini Tokenization for Feature 004

**Research Date**: 2026-03-14  
**Researcher**: Spec-Driven Development Architect  
**Purpose**: Investigate the best approach for adding Google Gemini token counting to token-count CLI

---

## Executive Summary

✅ **LOCAL TOKENIZATION IS AVAILABLE AND RECOMMENDED**

- **Primary Method**: Use `gemini-tokenizer` Rust crate (v0.2.0)
- **Status**: Community-maintained, ported from official Google Python SDK v1.6.20
- **Accuracy**: Exact match with Google's official tokenizer (same SentencePiece model)
- **Offline**: Fully offline, no API calls needed
- **Binary Size**: ~2MB embedded model (well within our budget)
- **Recommendation**: This is the best option for token-count's philosophy

### Why Local Tokenization Wins

| Criterion | Local (gemini-tokenizer) | API (CountTokens) |
|-----------|-------------------------|-------------------|
| **Accuracy** | ✅ Exact (same model as Google) | ✅ Exact (official API) |
| **Offline** | ✅ Yes (embeds model) | ❌ No (requires network) |
| **Speed** | ✅ <1ms (local) | ❌ 50-200ms (network latency) |
| **Cost** | ✅ Free | ⚠️ Billed as API call |
| **Constitution** | ✅ Aligns perfectly | ❌ Violates Principle III |
| **Dependencies** | ✅ Minimal (sentencepiece) | ❌ Requires API key + network |

**Decision**: Use `gemini-tokenizer` crate for local, offline, exact tokenization.

---

## Current Gemini Model Landscape (March 2026)

### ⚠️ Important Context: Transition Period

Google is in an **active transition** from Gemini 2.5 to Gemini 3.x:

- **Gemini 2.5 series** (currently GA) will be **deprecated June 17, 2026**
- **Gemini 3.x series** (currently Preview) has **no GA date announced yet**
- **Developer concern**: Stable models being deprecated before replacements go GA

**Our strategy**: Focus on Gemini 3.x since 2.5 will be gone in 3 months. Accept preview status as temporary.

### Gemini 3.x Series (Current Focus - Preview)

| Model | Context Window | Status | Release Date | Notes |
|-------|---------------|--------|--------------|-------|
| **gemini-3.1-pro-preview** | 1M tokens | Preview | Feb 19, 2026 | Replaces deprecated 3-pro-preview |
| **gemini-3-flash-preview** | 1M tokens | Preview | Dec 17, 2025 | Main workhorse model |
| **gemini-3.1-flash-lite-preview** | 1M tokens | Preview | Mar 3, 2026 | Cost-efficient, fastest |

**Note**: Gemini 3 Pro Preview was shut down March 9, 2026. Replaced by 3.1 Pro Preview.

### Gemini 2.5 Series (Being Deprecated)

| Model | Context Window | Status | Deprecation Date |
|-------|---------------|--------|------------------|
| **gemini-2.5-pro** | 1M tokens | GA → Deprecated | June 17, 2026 |
| **gemini-2.5-flash** | 1M tokens | GA → Deprecated | June 17, 2026 |
| **gemini-2.5-flash-lite** | 1M tokens | GA → Deprecated | June 17, 2026 |

**Migration path**: Google recommends migrating to 3.x preview models.

### Gemini 2.0 Series (Legacy)

| Model | Context Window | Status |
|-------|---------------|--------|
| **gemini-2.0-flash** | 1M tokens | GA (likely deprecated later) |

### Gemini 1.5 Series (Legacy)

| Model | Context Window | Status |
|-------|---------------|--------|
| **gemini-1.5-pro** | 2M tokens | Legacy (still supported) |
| **gemini-1.5-flash** | 1M tokens | Legacy (still supported) |

### Key Observations

1. **All Gemini models use the SAME tokenizer** - Gemma 3 SentencePiece (262,144 vocab)
2. **Context windows are massive** - 1M-2M tokens (vs GPT-4's 128K)
3. **Gemini 3.x is the future** - Focus on these despite preview status
4. **2.5 series has 3 months left** - June 17, 2026 deprecation
5. **Naming convention**: `gemini-{version}-{size}[-preview]`

---

## Tokenization Technical Details

### The Universal Tokenizer

**Key Finding**: All Gemini models (1.5, 2.0, 2.5, 3.x) use the **same tokenizer**:

- **Name**: Gemma 3 SentencePiece model
- **Algorithm**: SentencePiece (BPE + Unigram)
- **Vocabulary Size**: 262,144 tokens
- **Model File**: `tokenizer.model` (~2MB binary)
- **Source**: From Gemma 3 open-weights model on HuggingFace

### Token Statistics (from Google docs)

- **Average**: ~4 characters per token
- **100 tokens** ≈ 60-80 English words
- **Multimodal**: Images/audio/video also tokenized (not in scope for MVP)

### How SentencePiece Works

1. **Byte-Pair Encoding (BPE)**: Merges frequent character pairs
2. **Unigram Language Model**: Statistical subword tokenization
3. **Handles Unicode**: Works with any language (CJK, Arabic, emoji)
4. **Reversible**: Can decode tokens back to original text

---

## Rust Crate: `gemini-tokenizer`

### Crate Details

```toml
[dependencies]
gemini-tokenizer = "0.2.0"
```

- **Version**: 0.2.0 (released Feb 7, 2026)
- **License**: Apache-2.0
- **Author**: 5ocworkshop (community-maintained)
- **Downloads**: 362/month (recent release, growing)
- **Rust Version**: 1.70+ (we're on 1.85, fully compatible)
- **Documentation**: 97.5% documented

### Dependencies

| Dependency | Version | Purpose | Audit Status |
|------------|---------|---------|--------------|
| `sentencepiece` | ^0.11 | SentencePiece tokenizer bindings | ✅ Mature (MIT/Apache-2.0) |
| `serde` | ^1 | Serialization | ✅ Already in our deps |
| `serde_json` | ^1 | JSON support | ✅ Already in our deps |
| `sha2` | ^0.10 | Model hash verification | ✅ Cryptographic library |

**Total new dependencies**: 2 (sentencepiece, sha2)

### API Example

```rust
use gemini_tokenizer::LocalTokenizer;

// Initialize tokenizer (any Gemini model name works)
let tokenizer = LocalTokenizer::new("gemini-2.5-pro")
    .expect("failed to load tokenizer");

// Count tokens (simple text)
let result = tokenizer.count_tokens("What is your name?", None);
assert_eq!(result.total_tokens, 5);

// Get individual tokens (for debug mode)
let result = tokenizer.compute_tokens("Hello, world!");
for info in &result.tokens_info {
    for (id, token) in info.token_ids.iter().zip(&info.tokens) {
        println!("Token ID: {}, Text: {:?}", id, token);
    }
}
```

### Features We Need

✅ **Count tokens** - `count_tokens()` method  
✅ **Get token details** - `compute_tokens()` for debug mode  
✅ **Model verification** - SHA-256 hash check for embedded model  
✅ **Structured content** - Supports Gemini API content objects (future: function calling)

### Features We Don't Need (Yet)

- ❌ Multimodal token counting (images/audio/video) - text-only for MVP
- ❌ Function call tokenization - nice-to-have for future
- ❌ Schema tokenization - not needed for CLI use case

---

## Alternative: Google CountTokens API

### API Endpoint

```
POST https://generativelanguage.googleapis.com/v1beta/{model}:countTokens
```

### Pros
- ✅ Official Google API (authoritative)
- ✅ Supports multimodal (images, audio, video)
- ✅ Always up-to-date with latest model changes

### Cons
- ❌ Requires API key (`GOOGLE_API_KEY` env var)
- ❌ Network call (50-200ms latency)
- ❌ Billed as API usage (costs money)
- ❌ Violates Constitution Principle III (Zero External Dependencies)
- ❌ Doesn't work offline (breaks in CI/CD, air-gapped systems)

### Verdict

**DO NOT USE API for MVP**. The local tokenizer is exact and aligns with our principles.

**Future consideration**: Add `--accurate` flag for multimodal inputs (if we add image support in v0.4+).

---

## Comparison with Claude Approach

### Claude (Feature 003)
- **Default**: Estimation (adaptive content-type detection)
- **Accurate mode**: API call with consent prompt
- **Reason**: No official local tokenizer available

### Gemini (Feature 004)
- **Default**: Exact local tokenization (gemini-tokenizer)
- **Accurate mode**: Not needed (already exact)
- **Reason**: Official SentencePiece model is public and embeddable

### Key Difference

Gemini is **better positioned** than Claude:
- Claude forced us to use estimation due to lack of local tokenizer
- Gemini gives us exact counts offline (best of both worlds)
- No need for API mode, consent prompts, or environment variables

---

## Recommended Model Support (MVP)

### Priority 1: Gemini 3.x Series (Preview but Future)

| Model | Alias | Context Window | Status |
|-------|-------|----------------|--------|
| `gemini-3.1-pro-preview` | `gemini-pro`, `gemini-3-pro` | 1M | Preview |
| `gemini-3-flash-preview` | `gemini`, `gemini-flash`, `gemini-3-flash` | 1M | Preview |
| `gemini-3.1-flash-lite-preview` | `gemini-lite`, `gemini-3-lite` | 1M | Preview |

**Rationale**: 
- 2.5 series deprecated in 3 months (June 2026)
- These are the future (will go GA before June)
- Better to support preview now than deprecate models 3 months after launch
- All use same tokenizer, so no accuracy concerns

### Priority 2: Gemini 2.5 Series (GA but Short-Lived)

| Model | Alias | Context Window | Status |
|-------|-------|----------------|--------|
| `gemini-2.5-pro` | `gemini-2.5-pro` | 1M | GA (deprecated June 17) |
| `gemini-2.5-flash` | `gemini-2.5-flash` | 1M | GA (deprecated June 17) |
| `gemini-2.5-flash-lite` | `gemini-2.5-lite` | 1M | GA (deprecated June 17) |

**Rationale**: Support for users who need GA stability now, but don't make these defaults.

### Priority 3: Legacy Models (Optional)

| Model | Alias | Context Window | Status |
|-------|-------|----------------|--------|
| `gemini-1.5-pro` | `gemini-1.5-pro` | 2M | Legacy |
| `gemini-1.5-flash` | `gemini-1.5-flash` | 1M | Legacy |

**Rationale**: Easy to add (same tokenizer), but defer to future if time-constrained.

### Default Model

**Recommendation**: `gemini-3-flash-preview`

**Reasons**:
1. **Forward-looking**: Will become GA (likely before June)
2. **"Flash" = fast**: Good for CLI use case
3. **1M context window**: Plenty for most users
4. **Good balance**: Cost/performance (Google's "workhorse" line)
5. **Avoids immediate deprecation**: Won't need to change default in 3 months

**Alternative**: `gemini-3.1-pro-preview` if we prioritize capability over speed.

**User messaging**: Document that 3.x are preview models, but recommended over 2.5 (which are being deprecated).

---

## Model Alias Strategy

### Format Support

Same as OpenAI/Claude:
1. **Exact names**: `gemini-2.5-flash`
2. **Short aliases**: `gemini`, `gemini-flash`, `gemini-pro`
3. **Provider format**: `google/gemini-2.5-flash`

### Proposed Aliases

```rust
// Primary alias (most common) - points to 3.x preview
"gemini" -> "gemini-3-flash-preview"

// Size-based aliases (3.x series)
"gemini-pro" -> "gemini-3.1-pro-preview"
"gemini-flash" -> "gemini-3-flash-preview"
"gemini-lite" -> "gemini-3.1-flash-lite-preview"

// Version-specific (3.x series)
"gemini-3" -> "gemini-3-flash-preview"
"gemini-3-pro" -> "gemini-3.1-pro-preview"
"gemini-3-flash" -> "gemini-3-flash-preview"
"gemini-3-lite" -> "gemini-3.1-flash-lite-preview"

// Version-specific (2.5 series - being deprecated)
"gemini-2.5" -> "gemini-2.5-flash"
"gemini-2.5-pro" -> "gemini-2.5-pro"
"gemini-2.5-flash" -> "gemini-2.5-flash"
"gemini-2.5-lite" -> "gemini-2.5-flash-lite"

// Provider format (use 3.x as default)
"google/gemini" -> "gemini-3-flash-preview"
"google/gemini-pro" -> "gemini-3.1-pro-preview"
"google/gemini-2.5-flash" -> "gemini-2.5-flash"
```

**Note**: We include `-preview` suffix in canonical names to be transparent about model status.

### Case-Insensitive

All aliases are case-insensitive (like OpenAI/Claude):
- `Gemini`, `GEMINI`, `gemini` all work

---

## Binary Size Impact

### Embedded Model Size
- **SentencePiece model**: ~2MB
- **gemini-tokenizer dependencies**: ~500KB compiled

### Total Binary Size Estimate
- **Current binary**: 9.2 MB (OpenAI + Claude)
- **After adding Gemini**: ~11.5 MB (+2.3 MB)
- **Target budget**: <50 MB (well within limit)

### Verdict

✅ **No concerns**. Binary size increase is minimal and acceptable.

---

## Performance Expectations

### Tokenization Speed

Based on SentencePiece benchmarks:
- **Small input (<10KB)**: <1ms
- **Medium input (1MB)**: ~50ms
- **Large input (10MB)**: ~500ms

### Memory Usage

- **Model loading**: ~5MB (one-time, cached)
- **Processing**: ~2x input size (similar to tiktoken)

### Verdict

✅ **Meets Constitution performance standards** (<10ms for small inputs).

---

## Security & Maintenance Considerations

### Crate Maturity

⚠️ **Community-maintained** (not official Google)

**Risks**:
- No Google official support
- Single maintainer (5ocworkshop)
- Only 362 downloads/month (new crate)

**Mitigations**:
1. **Model verification**: Crate includes SHA-256 hash check (ensures correct model)
2. **Ported from official SDK**: Direct port of Google's Python SDK (v1.6.20)
3. **SentencePiece is mature**: Underlying library is Google's official C++ implementation
4. **Our constitution**: We can fork/maintain if needed (open source, Apache-2.0)

### Dependency Audit

```bash
cargo audit
```

- `sentencepiece`: Mature, bindings to Google's C++ library
- `sha2`: RustCrypto project (well-audited)
- `serde/serde_json`: Already in our deps

### Recommendation

✅ **Accept the risk**. The crate is well-designed and verifiable. If it becomes unmaintained, we can:
1. Fork and maintain ourselves
2. Fall back to API mode (but this violates our principles)
3. Use alternative SentencePiece bindings directly

---

## Alternative Approaches Considered

### Option A: Use llm-tokenizer crate

**Status**: Investigated in constitution (mentioned for Llama/Mistral)

**Finding**: `llm-tokenizer` v1.3.0 does NOT support Gemini/Gemma tokenizer as of March 2026.

**Verdict**: ❌ Not viable for Gemini.

### Option B: Direct SentencePiece bindings

**Approach**: Use `sentencepiece` crate directly + download `tokenizer.model` from HuggingFace

**Pros**:
- More control over implementation
- No third-party crate dependency

**Cons**:
- Need to implement TextAccumulator logic ourselves (complex)
- Need to manage model file embedding
- More code to maintain
- Duplicates work already done by gemini-tokenizer

**Verdict**: ❌ Reinventing the wheel. Use gemini-tokenizer.

### Option C: API-only (like Claude accurate mode)

**Approach**: Always use Google CountTokens API

**Pros**:
- Official Google API (always correct)
- No additional dependencies

**Cons**:
- Violates Constitution Principle III (Zero External Dependencies)
- Requires API key
- Costs money
- Doesn't work offline

**Verdict**: ❌ Violates core principles. Local tokenization available.

---

## Implementation Recommendations

### Architecture

```
src/tokenizers/google/
├── mod.rs           # Main Google tokenizer interface
├── models.rs        # Model definitions (list, aliases, context windows)
└── tokenizer.rs     # Wrapper around gemini-tokenizer crate
```

### Integration Points

1. **CLI args**: Add Gemini models to `--model` flag
2. **Model registry**: Register Gemini provider in `src/tokenizers/registry.rs`
3. **Output formatters**: Reuse existing formatters (simple/verbose/debug)
4. **Error handling**: Handle tokenizer initialization errors

### Testing Strategy

1. **Unit tests**: Each model + alias
2. **Integration tests**: CLI end-to-end
3. **Performance tests**: Verify <10ms for small inputs
4. **Comparison tests**: Verify against Google API (one-time validation)

### Rollout Plan

1. **MVP (v0.3.0)**: Text-only tokenization for 6-8 models
2. **Future (v0.4.0)**: Add multimodal support (images, audio) via API mode

---

## Constitution Alignment Check

| Principle | Gemini Implementation | Alignment |
|-----------|----------------------|-----------|
| I. POSIX Simplicity | Same CLI interface as OpenAI/Claude | ✅ Perfect |
| II. Accuracy Over Speed | Exact tokenization (same as Google) | ✅ Perfect |
| III. Zero External Dependencies | Fully offline, embedded model | ✅ Perfect |
| IV. Cross-Platform | SentencePiece builds on all platforms | ✅ Perfect |
| V. Fail Fast | Clear errors for unknown models | ✅ Perfect |
| VI. Trivial Installation | No extra steps, no API keys | ✅ Perfect |
| VII. Semantic Versioning | Minor version (v0.3.0) for new models | ✅ Perfect |

**Verdict**: ✅ **Gemini implementation perfectly aligns with all 7 principles.**

---

## Risks & Mitigation

### Risk 1: gemini-tokenizer becomes unmaintained

**Likelihood**: Medium (single maintainer)  
**Impact**: Medium (we'd lose updates)

**Mitigation**:
- Fork the crate to our own repo
- Pin to specific version (0.2.0)
- Monitor for updates quarterly
- Consider upstreaming improvements

### Risk 2: Google changes tokenizer for new models

**Likelihood**: Low (Gemma 3 tokenizer is standardized across all Gemini models)  
**Impact**: Medium (token counts would diverge)

**Mitigation**:
- Document which Gemini versions are supported (1.5+, 2.x, 3.x all use same tokenizer)
- Monitor Google announcements for tokenizer changes
- Update crate when Google releases new tokenizer (if ever)
- Add warning for unsupported models

### Risk 3: Gemini 3.x stays in preview past June 2026

**Likelihood**: Low (Google must GA before deprecating 2.5)  
**Impact**: Medium (users concerned about preview models in production)

**Mitigation**:
- Support both 2.5 (GA) and 3.x (preview) models
- Document deprecation timeline clearly
- Add optional warning message for preview models
- Plan to update defaults when 3.x goes GA

### Risk 4: Binary size grows too large

**Likelihood**: Low (2MB addition)  
**Impact**: Low (still under 50MB budget)

**Mitigation**:
- Already accounted for in binary size budget
- Constitution Amendment 1.3.0 relaxed size constraint

### Risk 4: SentencePiece compilation issues

**Likelihood**: Low (mature C++ library)  
**Impact**: High (breaks build on some platforms)

**Mitigation**:
- Test on all platforms in CI (Linux, macOS, Windows)
- Document build requirements in INSTALL.md
- Provide pre-built binaries (already in place)

---

## Recommended Next Steps

### Phase 1: Specification (Current)
1. ✅ Research Gemini tokenization options (this document)
2. ⏳ Write Feature 004 specification
3. ⏳ Get user approval

### Phase 2: Planning
1. Create feature branch: `004-gemini-support`
2. Design module structure
3. Define data models (model list, aliases, context windows)
4. Write implementation plan

### Phase 3: Implementation
1. Add `gemini-tokenizer` dependency
2. Implement `src/tokenizers/google/` module
3. Register models in registry
4. Add integration tests
5. Update documentation (README, CHANGELOG)

### Phase 4: Release
1. Bump version to v0.3.0
2. Create release PR
3. Publish to crates.io
4. Update Homebrew formula

---

## Questions for User

Before proceeding to specification:

1. ✅ **Model selection**: Focus on Gemini 3.x preview models (3.1-pro, 3-flash, 3.1-flash-lite) as primary
2. ✅ **Default model**: Use `gemini-3-flash-preview` (fast workhorse, forward-looking)
3. **Support 2.5 series**: Should we also include 2.5 models (deprecated June 17) for users needing GA stability?
4. **Preview warning**: Should we show a warning when users first use preview models? (e.g., "Note: Using gemini-3-flash-preview (preview model). For GA stability, use gemini-2.5-flash until June 2026.")
5. **Canonical names**: Keep `-preview` suffix in model names for transparency, or strip it for cleaner UX?

---

## References

### Official Google Documentation
- [Gemini API - Understand and Count Tokens](https://ai.google.dev/gemini-api/docs/tokens)
- [Gemini API - Models List](https://ai.google.dev/gemini-api/docs/models)
- [Vertex AI - CountTokens API](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/model-reference/count-tokens)
- [Gemini 3.1 Pro Model Card](https://deepmind.google/models/model-cards/gemini-3-1-pro/)

### Rust Crates
- [gemini-tokenizer v0.2.0](https://crates.io/crates/gemini-tokenizer)
- [gemini-tokenizer docs](https://docs.rs/gemini-tokenizer/0.2.0/gemini_tokenizer/)
- [sentencepiece v0.13.1](https://crates.io/crates/sentencepiece)

### Community Articles
- [Counting Gemini text tokens locally (Medium)](https://medium.com/google-cloud/counting-gemini-text-tokens-locally-78979fea6244)
- [Gemini and Gemma tokenizer in Java](https://glaforge.dev/posts/2024/10/04/a-gemini-and-gemma-tokenizer-in-java/)
- [Gemma 3 Architecture Explained](https://developers.googleblog.com/gemma-explained-whats-new-in-gemma-3/)

---

**Research Complete**: Ready to proceed to Feature 004 specification.
