# Research: Technology Choices for Feature 004

**Date**: 2026-03-14  
**Feature**: Google Gemini Token Counting Support  
**Branch**: `004-gemini-support`

---

## Research Summary

This document records technology choices, alternatives considered, and rationale for Feature 004 (Google Gemini support).

**Primary Research Source**: `.specify/RESEARCH-GEMINI-TOKENIZATION.md` (comprehensive 610-line research document created during specification phase)

---

## Key Technology Choices

### Choice 1: Local Tokenization with `gemini-tokenizer` Crate

**Selected**: `gemini-tokenizer = "0.2.0"` (local, offline tokenization)

**Rationale**:
- **Accuracy**: 100% match with Google's official tokenizer (uses same Gemma 3 SentencePiece model)
- **Offline**: Perfectly aligns with Constitution Principle III (Zero External Dependencies)
- **Speed**: <1ms for small inputs (vs 50-200ms API latency)
- **Cost**: Free (no API billing)
- **Reliability**: Works in air-gapped environments, CI/CD pipelines, no network failures
- **Binary size**: +2.3MB embedded model (well within 50MB budget)

**Alternatives Considered**:

1. **Google CountTokens API** - Rejected
   - ❌ Requires API key (`GOOGLE_API_KEY` env var)
   - ❌ Network call (50-200ms latency)
   - ❌ Billed as API usage
   - ❌ Violates Constitution Principle III
   - ❌ Doesn't work offline
   - ✅ Supports multimodal (images/audio) - but not needed for MVP

2. **Custom Estimation (like Claude)** - Rejected
   - ❌ Violates Constitution Principle II (Accuracy Over Speed)
   - ❌ Would be inaccurate for SentencePiece tokenization
   - ✅ No binary size increase
   - Note: Not needed since exact tokenization is available

**Supporting Evidence**:
- `gemini-tokenizer` is ported from official Google Python SDK v1.6.20
- Uses official Gemma 3 model from HuggingFace
- SHA-256 verification ensures model integrity
- Community-maintained but stable (v0.2.0 released Feb 2026)
- 362+ downloads/month and growing

**Trade-offs Accepted**:
- Community-maintained (not official Google crate) - Risk: Low, based on official SDK
- Binary size increase (+2.3MB) - Acceptable, well under budget (38.5MB headroom)

---

### Choice 2: Default Model is `gemini-3-flash-preview`

**Selected**: `--model gemini` → `gemini-3-flash-preview`

**Rationale**:
- **Forward-looking**: Gemini 2.5 series deprecated June 17, 2026 (3 months away)
- **Future-proof**: Gemini 3.x is the future, will go GA before June 2026
- **Avoids churn**: No need to change default in 3 months
- **Fast**: "Flash" models optimized for speed, suitable for CLI
- **User choice**: User explicitly chose "lean into 3.x" strategy during spec phase

**Alternatives Considered**:

1. **Default to `gemini-2.5-flash` (GA)** - Rejected
   - ❌ Would be deprecated in 3 months
   - ❌ Requires changing default in June 2026 (churn)
   - ✅ GA status (not preview)

2. **No default (require explicit version)** - Rejected
   - ❌ Violates POSIX simplicity
   - ❌ Poor user experience
   - ✅ Most explicit/transparent

**Supporting Evidence**:
- Google's deprecation notice: Gemini 2.5 EOL June 17, 2026
- Google recommends migrating to 3.x preview models
- Developer feedback: Frustration with stable models being deprecated before replacements go GA
- Strategic decision: Accept preview status as temporary (3 months max)

**Trade-offs Accepted**:
- Preview model as default - Mitigated: Will go GA before 2.5 deprecation (high confidence)
- User awareness - Mitigated: `-preview` suffix in name makes status clear

---

### Choice 3: Keep `-preview` Suffix in Canonical Names

**Selected**: Use `gemini-3-flash-preview` (not `gemini-3-flash`)

**Rationale**:
- **Transparency**: Users know they're using preview models
- **Consistency**: Matches Google's official naming convention
- **Future-compatible**: When GA, can add `gemini-3-flash` as new model (backward compatible)
- **No surprises**: Explicit naming prevents silent behavior changes

**Alternatives Considered**:

1. **Strip `-preview` suffix** - Rejected
   - ❌ Less transparent
   - ❌ Confusing when GA releases (are they the same?)
   - ✅ Cleaner names

2. **Show preview warnings in output** - Rejected
   - ❌ Clutters UX
   - ❌ Users already know from name
   - ✅ More explicit

**User Decision**: User chose "Option A" (keep suffix) during clarification phase.

---

### Choice 4: Support 8 Gemini Models Across 3 Generations

**Selected**: 3.x (priority 1), 2.5 (priority 2), 1.5 (priority 3)

**Models**:

**Gemini 3.x Series (Priority 1 - Preview)**:
- `gemini-3.1-pro-preview` (1M context)
- `gemini-3-flash-preview` (1M context) ← Default
- `gemini-3.1-flash-lite-preview` (1M context)

**Gemini 2.5 Series (Priority 2 - Deprecated June 2026)**:
- `gemini-2.5-pro` (1M context)
- `gemini-2.5-flash` (1M context)
- `gemini-2.5-flash-lite` (1M context)

**Gemini 1.5 Series (Priority 3 - Legacy)**:
- `gemini-1.5-pro` (2M context)
- `gemini-1.5-flash` (1M context)

**Rationale**:
- **Comprehensive**: Covers all active Gemini versions
- **Forward-looking**: 3.x models listed first (priority)
- **Backward-compatible**: Supports users on 2.5 and 1.5
- **No maintenance**: All use same tokenizer (no per-model logic)

**Alternatives Considered**:

1. **Only support 3.x (3 models)** - Rejected
   - ❌ Breaks compatibility for users on 2.5/1.5
   - ✅ Simpler, less code

2. **Support all models + 2.0 series (9 models)** - Rejected
   - ❌ Gemini 2.0 is redundant (deprecated after 2.5)
   - ❌ More models to document/test for little benefit

**Supporting Evidence**:
- All Gemini models (1.5, 2.x, 3.x) use same tokenizer (Gemma 3 SentencePiece)
- Zero implementation complexity to support multiple versions
- User benefit: Works with whatever version they're using

---

### Choice 5: Unified Encoding Name `gemini-gemma3`

**Selected**: All models use encoding `"gemini-gemma3"`

**Rationale**:
- **Accuracy**: All Gemini models use identical tokenizer (Gemma 3 SentencePiece)
- **Consistency**: Follows OpenAI pattern (multiple models, same encoding: `cl100k_base`)
- **Simplicity**: Single tokenizer instance for all models
- **Clear naming**: "gemini" prefix + "gemma3" identifies source model

**Alternatives Considered**:

1. **Per-version encoding names** (`gemini-1.5`, `gemini-2.5`, `gemini-3.x`) - Rejected
   - ❌ Misleading (they're identical tokenizers)
   - ❌ Suggests implementation differences (there are none)

2. **Model name as encoding** (`gemini-3-flash-preview`) - Rejected
   - ❌ Inconsistent with OpenAI/Claude patterns
   - ❌ Redundant (model name already shown separately)

---

## Dependencies Audit

### New Direct Dependencies

**`gemini-tokenizer = "0.2.0"`**
- **License**: Apache-2.0 (compatible with MIT)
- **Author**: 5ocworkshop (community-maintained)
- **Status**: Active (v0.2.0 released Feb 7, 2026)
- **Downloads**: 362/month (recent release, growing)
- **MSRV**: Rust 1.70+ (we're on 1.85.0, fully compatible)
- **Documentation**: 97.5% documented
- **Security**: No known vulnerabilities (`cargo audit`)
- **Source**: Based on official Google Python SDK v1.6.20

**Risk Assessment**: Low
- Community-maintained is acceptable (Constitution allows for well-vetted dependencies)
- Implementation is straightforward wrapper around sentencepiece
- If abandoned, easy to fork or replace (simple API surface)

### New Transitive Dependencies

**`sentencepiece = "^0.11"`**
- **License**: MIT/Apache-2.0 (compatible)
- **Purpose**: Rust bindings to Google's SentencePiece C++ library
- **Status**: Mature, widely used (bindings to official library)
- **Security**: No known vulnerabilities
- **Cross-platform**: Works on all target platforms (Linux, macOS, Windows)

**`sha2 = "^0.10"`**
- **License**: MIT/Apache-2.0 (compatible)
- **Purpose**: SHA-256 hash verification for embedded model
- **Status**: Part of RustCrypto project (very mature)
- **Security**: Cryptographic library, audited

**Total new dependencies**: 3 (gemini-tokenizer + 2 transitive)

---

## Performance Characteristics

### Expected Performance (from research)

**Tokenization Speed**:
- Small input (100 bytes): ~1ms (target: <10ms) ✅
- Medium input (10KB): ~5ms (target: <100ms) ✅
- Large input (1MB): ~50ms (target: <1s) ✅

**Memory Usage**:
- Tokenizer initialization: ~2MB (embedded model)
- Runtime overhead: <10MB
- Large file processing: <500MB peak

**Binary Size**:
- Before: 9.2MB (OpenAI + Claude)
- After: 11.5MB (+2.3MB)
- Budget: 50MB (38.5MB headroom)

**Comparison to Alternatives**:
- Local (gemini-tokenizer): ~1ms, no network
- Google API: ~50-200ms (network latency), requires API key
- Estimation: ~1ms, but inaccurate (violates constitution)

**Bottleneck Analysis**:
- Initialization: 5-10ms one-time cost (acceptable for CLI)
- SentencePiece C++ library: Well-optimized, used by Google in production
- I/O: Dominant factor for large files (expected, unavoidable)

---

## Best Practices Investigated

### Rust Pattern: Provider Trait Implementation

**Pattern**: Follow established `claude/` module structure
- `mod.rs`: Public API, `Tokenizer` trait implementation
- `models.rs`: Model definitions (8 models, aliases, context windows)
- `tokenizer.rs`: Wrapper around `gemini-tokenizer` crate

**Rationale**:
- Consistency with existing codebase (Feature 003: Claude)
- Separation of concerns (data vs logic vs public API)
- Easy to extend (multimodal, function calling in future)

**Reference Implementation**: `src/tokenizers/claude/` (similar structure)

---

### Error Handling Strategy

**Pattern**: Use `anyhow::Result` for error propagation, `TokenError` enum for user-facing errors

**Error Scenarios**:
1. Unknown model → `TokenError::UnknownModel` (exit code 2)
2. Tokenizer initialization failure → `TokenError::Tokenization` (exit code 1)
3. Invalid UTF-8 → `anyhow::Context` with helpful message (exit code 1)

**Best Practice**: Wrap gemini-tokenizer errors with context
```rust
let tokenizer = LocalTokenizer::new(model)
    .context("Failed to initialize Gemini tokenizer")?;
```

---

### Testing Strategy

**Approach**: Unit tests + integration tests + one-time comparison

**Unit Tests** (20+ tests):
- Tokenizer initialization (success, failure modes)
- Token counting (empty, small, large inputs)
- Model registry (all models, all aliases, case-insensitivity)
- Default model resolution
- Provider format (`google/gemini`)
- Context window calculations
- Error messages

**Integration Tests** (15+ tests):
- CLI with `--model gemini`
- All model aliases
- Piped input
- File input
- Verbosity levels (`-v`, `-vv`, `-vvv`)
- `--list-models` output
- Error handling

**Comparison Tests** (one-time validation):
- Generate 100 diverse test cases
- Compare gemini-tokenizer (Rust) vs Google Python SDK
- Must match 100% (exact tokenization requirement)

---

## Configuration Decisions

### Model Aliases

**Primary aliases**:
- `gemini` → `gemini-3-flash-preview` (default)
- `gemini-pro` → `gemini-3.1-pro-preview`
- `gemini-flash` → `gemini-3-flash-preview`
- `gemini-lite` → `gemini-3.1-flash-lite-preview`

**Provider format**:
- `google/gemini` → `gemini-3-flash-preview`
- `google/gemini-pro` → `gemini-3.1-pro-preview`
- `google/{model}` → `{model}` (passthrough)

**Rationale**:
- Short aliases for common use cases
- Provider format for consistency with OpenAI/Claude
- Case-insensitive (POSIX simplicity)

---

## References

### Primary Research
- [RESEARCH-GEMINI-TOKENIZATION.md](../../.specify/RESEARCH-GEMINI-TOKENIZATION.md) - Comprehensive research (610 lines)

### Specifications
- [004-gemini-support.md](../../.specify/features/004-gemini-support.md) - Feature specification (998 lines)
- [004-gemini-support-SUMMARY.md](../../.specify/features/004-gemini-support-SUMMARY.md) - Executive summary

### External Documentation
- [gemini-tokenizer crate](https://crates.io/crates/gemini-tokenizer) - v0.2.0
- [SentencePiece](https://github.com/google/sentencepiece) - Google's tokenization library
- [Gemma 3 model](https://huggingface.co/google/gemma-3-tokenizer) - HuggingFace

### Google Documentation
- [Gemini API: Count Tokens](https://ai.google.dev/api/rest/v1beta/models/countTokens) - Official API
- [Gemini models](https://ai.google.dev/gemini-api/docs/models/gemini) - Model documentation
- [Token counting](https://ai.google.dev/gemini-api/docs/tokens) - Tokenization overview

---

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-03-14 | Use `gemini-tokenizer` crate | Exact, offline, aligns with constitution |
| 2026-03-14 | Default to `gemini-3-flash-preview` | Forward-looking, 2.5 deprecated June 2026 |
| 2026-03-14 | Keep `-preview` suffix | Transparency, matches Google naming |
| 2026-03-14 | Support 8 models (3.x, 2.5, 1.5) | Comprehensive coverage, no implementation cost |
| 2026-03-14 | Unified encoding `gemini-gemma3` | All models use same tokenizer |
| 2026-03-14 | Follow `claude/` module structure | Consistency, separation of concerns |

---

**Status**: Research complete, ready for implementation  
**Next**: Create `data-model.md` and `quickstart.md`
