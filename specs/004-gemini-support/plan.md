# Implementation Plan: Feature 004 - Google Gemini Token Counting Support

**Branch**: `004-gemini-support` | **Date**: 2026-03-14 | **Spec**: [.specify/features/004-gemini-support.md](../../.specify/features/004-gemini-support.md)  
**Input**: Feature specification from spec-driven-planner agent

## Summary

Add exact, offline tokenization for Google Gemini models (3.x, 2.5, 1.5) using the `gemini-tokenizer` Rust crate (v0.2.0). This provides 100% accurate token counts that match Google's official tokenizer, with zero network dependencies. All 8 Gemini models use the same Gemma 3 SentencePiece tokenizer (262K vocab), enabling consistent tokenization across all versions.

**Default model**: `gemini-3-flash-preview` (forward-looking, as 2.5 series deprecated June 2026)

## Technical Context

**Language/Version**: Rust 1.85.0+ (stable channel, MSRV defined in constitution)  
**Primary Dependencies**: 
- `gemini-tokenizer = "0.2.0"` - Local SentencePiece tokenizer for Gemini
- `sentencepiece = "^0.11"` - SentencePiece bindings (transitive via gemini-tokenizer)
- `sha2 = "^0.10"` - SHA-256 hash verification (transitive via gemini-tokenizer)
- Existing: `clap 4.6`, `tiktoken-rs 0.9.1`, `anyhow 1.0.102`, `strsim 0.11`

**Storage**: N/A (stateless CLI tool, embedded tokenizer model ~2MB in binary)  
**Testing**: `cargo test` (152 tests currently, target: 187+ tests after this feature)  
**Target Platform**: Cross-platform CLI (Linux x86_64, macOS x86_64/aarch64, Windows x86_64)  
**Project Type**: Command-line utility (CLI) with library API (`lib.rs`)  
**Performance Goals**:
- Small input (<10KB): <10ms latency (expect ~1ms)
- Medium input (1MB): <100ms latency (expect ~50ms)
- Large input (100MB): <10s with streaming
- Binary size: <50MB (current: 9.2MB, projected: 11.5MB)

**Constraints**:
- Fully offline (no network calls, Constitution Principle III)
- Exact tokenization (100% match with Google's tokenizer, Constitution Principle II)
- Cross-platform first-class support (Constitution Principle IV)
- No API key required (zero runtime dependencies)

**Scale/Scope**: 8 Gemini models, 35+ new tests, ~500 lines of code, ~2.3MB binary size increase

## Constitution Check

*GATE: Must pass before implementation. Re-check after code review.*

✅ **Principle I: POSIX Simplicity** - No new CLI flags, follows existing `--model gemini` pattern  
✅ **Principle II: Accuracy Over Speed** - Exact tokenization using official SentencePiece model  
✅ **Principle III: Zero External Dependencies** - Fully offline, model embedded in binary  
✅ **Principle IV: Cross-Platform** - gemini-tokenizer compiles on all target platforms  
✅ **Principle V: Fail Fast** - Clear errors for unknown models, initialization failures  
✅ **Principle VI: Installation Trivial** - No changes to installation (built-in)  
✅ **Principle VII: Semantic Versioning** - New feature = minor bump (v0.2.2 → v0.3.0)

**Quality Standards**:
- ✅ Test coverage: 80%+ (35+ tests for Gemini module)
- ✅ Performance: <10ms for small inputs (SentencePiece is fast)
- ✅ Binary size: 11.5MB projected (well under 50MB budget)
- ✅ Code quality: Zero clippy warnings, rustfmt compliance

**Violations**: None

## Project Structure

### Documentation (this feature)

```text
specs/004-gemini-support/
├── spec.md              # Feature specification (from .specify/features/)
├── plan.md              # This file (implementation plan)
├── research.md          # Technology research and decisions
├── data-model.md        # Model definitions and registry structure
├── quickstart.md        # Validation scenarios and testing guide
└── tasks.md             # Ordered task list (Phase 5)
```

### Source Code (repository root)

```text
src/
├── tokenizers/
│   ├── mod.rs           # Trait definitions, TokenCount enum (existing)
│   ├── registry.rs      # Model registry, get_tokenizer() (UPDATE)
│   ├── openai.rs        # OpenAI tokenizer (existing, no changes)
│   ├── claude/          # Claude tokenizer (existing, no changes)
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── api_client.rs
│   │   └── estimation.rs
│   └── google/          # NEW MODULE for Gemini
│       ├── mod.rs       # GoogleTokenizer struct, Tokenizer trait impl
│       ├── models.rs    # google_models() function, ModelConfig definitions
│       └── tokenizer.rs # Wrapper around gemini-tokenizer crate
├── cli/                 # CLI argument parsing (no changes needed)
├── output/              # Output formatters (no changes needed)
├── error.rs             # TokenError enum (existing)
├── lib.rs               # Library API (no changes needed)
└── main.rs              # Binary entry point (no changes needed)

tests/
├── openai_tokenization.rs      # Existing OpenAI tests
├── claude_tokenization.rs      # Existing Claude tests
└── google_tokenization.rs      # NEW: Gemini integration tests

benches/
├── tokenization.rs             # Existing benchmarks (UPDATE: add Gemini)
```

**Structure Decision**: Single project structure (Option 1). Following established pattern from Features 001 (OpenAI) and 003 (Claude). New `src/tokenizers/google/` module mirrors `claude/` structure for consistency.

## Architecture Decisions

### AD-001: Module Structure Follows Claude Pattern

**Decision**: Create `src/tokenizers/google/` directory with `mod.rs`, `models.rs`, `tokenizer.rs`

**Rationale**:
- Consistency with existing `claude/` module structure
- Separation of concerns: models (data), tokenizer (logic), mod (public API)
- Easy to extend with future features (multimodal, function calling)

**Alternatives Rejected**:
- Single file `google.rs` - Would work for MVP, but harder to extend and inconsistent with Claude
- Flat structure in `tokenizers/` - Would clutter top-level namespace

### AD-002: Use `gemini-tokenizer` Crate (Not Google API)

**Decision**: Local tokenization with `gemini-tokenizer = "0.2.0"` crate

**Rationale**:
- **Accuracy**: 100% match with Google's official tokenizer (same SentencePiece model)
- **Offline**: Aligns with Constitution Principle III (Zero External Dependencies)
- **Speed**: <1ms vs 50-200ms API latency
- **Cost**: Free (no API billing)
- **Reliability**: No network failures, works in air-gapped environments

**Alternatives Rejected**:
- Google CountTokens API - Violates Constitution Principle III, requires API key
- Custom estimation - Violates Constitution Principle II (Accuracy Over Speed)
- Different Rust crate - gemini-tokenizer is most mature, actively maintained

**Trade-offs**:
- ✅ Perfect alignment with constitution
- ✅ Fast, reliable, offline
- ❌ Community-maintained (not official Google crate) - Risk: Low (based on official SDK)
- ❌ Binary size increase (+2.3MB) - Acceptable (well under budget)

### AD-003: Default Model is `gemini-3-flash-preview`

**Decision**: `--model gemini` resolves to `gemini-3-flash-preview` (not `gemini-2.5-flash`)

**Rationale**:
- Gemini 2.5 series deprecated June 17, 2026 (3 months away)
- Gemini 3.x is the future (will go GA before June)
- Avoids needing to change default in 3 months
- "Flash" models are fast, suitable for CLI use cases
- User explicitly chose "lean into 3.x" strategy in specification phase

**Alternatives Rejected**:
- Default to `gemini-2.5-flash` (GA) - Would require changing default in 3 months
- No default (require explicit version) - Violates POSIX simplicity

**Trade-offs**:
- ✅ Forward-looking, avoids deprecation churn
- ⚠️ Preview model as default - Mitigated: Will go GA before 2.5 deprecation

### AD-004: Keep `-preview` Suffix in Canonical Names

**Decision**: Use `gemini-3-flash-preview` as canonical name (not `gemini-3-flash`)

**Rationale**:
- Transparency: Users know they're using preview models
- Matches Google's official naming convention
- When 3.x goes GA, we can add `gemini-3-flash` as new model (backward compatible)
- No silent behavior change when GA releases

**Alternatives Rejected**:
- Strip `-preview` suffix - Less transparent, confusing when GA releases
- Show preview warnings - Clutters UX, users already know from name

### AD-005: All Models Share Same Encoding Name

**Decision**: All Gemini models use encoding name `"gemini-gemma3"`

**Rationale**:
- All Gemini models (1.5, 2.x, 3.x) use the same tokenizer (Gemma 3 SentencePiece)
- Consistent with OpenAI pattern (multiple models, same encoding: `cl100k_base`)
- Simplifies implementation (single tokenizer instance for all models)

**Alternatives Rejected**:
- Per-version encoding names (`gemini-1.5`, `gemini-2.5`, `gemini-3.x`) - Misleading, they're identical
- Model name as encoding - Inconsistent with OpenAI/Claude patterns

## Data Model

See `specs/004-gemini-support/data-model.md` for detailed entity definitions.

**Summary**:
- **8 Gemini models** across 3 generations (3.x, 2.5, 1.5)
- **12+ aliases** for user convenience (`gemini`, `gemini-pro`, `gemini-flash`, `gemini-lite`, `google/*`)
- **2 context window sizes**: 1M tokens (most models), 2M tokens (gemini-1.5-pro only)
- **1 encoding**: `gemini-gemma3` (shared across all models)

## API Contracts

No external API contracts (offline tokenization). Internal Rust API follows existing `Tokenizer` trait:

```rust
pub trait Tokenizer: Send + Sync {
    fn count_tokens(&self, text: &str) -> anyhow::Result<usize>;
    fn get_model_info(&self) -> ModelInfo;
}
```

See `specs/004-gemini-support/data-model.md` for detailed struct definitions.

## Implementation Phases

### Phase 4: Planning (Current)
- ✅ Create `plan.md` (this file)
- ⏳ Create `research.md` (document technology choices)
- ⏳ Create `data-model.md` (model definitions)
- ⏳ Create `quickstart.md` (validation guide)

### Phase 5: Tasking
- ⏳ Create `tasks.md` (ordered task list with dependencies)

### Phase 6: Implementation
- ⏳ Add `gemini-tokenizer` dependency
- ⏳ Implement `src/tokenizers/google/` module
- ⏳ Update `src/tokenizers/registry.rs` with Google models
- ⏳ Write 35+ tests (unit + integration)
- ⏳ Add performance benchmarks
- ⏳ Update documentation (README, CHANGELOG)
- ⏳ Run full verification (tests, lint, build, benchmarks)

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `gemini-tokenizer` crate abandoned | Low | Medium | Fork if needed, implementation is simple wrapper around sentencepiece |
| Binary size exceeds budget | Very Low | Low | Currently 11.5MB projected (budget: 50MB), 38.5MB headroom |
| Gemini 3.x stays in preview past June | Medium | Low | No breaking change, 2.5 models still work, just deprecated |
| Cross-platform compilation issues | Low | Medium | sentencepiece is mature, tested on all platforms |
| Performance regression | Very Low | Low | SentencePiece is C++ library, very fast (<1ms expected) |

## Success Criteria

**Before merge**:
- [ ] All 187+ tests pass (152 existing + 35+ new)
- [ ] Zero clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt --check`)
- [ ] Release build succeeds (`cargo build --release`)
- [ ] Binary size <15MB (target: 11.5MB)
- [ ] Benchmarks meet targets (<10ms for small inputs)
- [ ] Documentation updated (README, CHANGELOG)

**Post-release validation** (v0.3.0):
- [ ] Token counts match Google's Python SDK (100% accuracy in manual validation)
- [ ] Works on all platforms (Linux, macOS Intel/ARM, Windows)
- [ ] No installation issues reported
- [ ] Performance meets targets (95th percentile <100ms for typical usage)

## Next Steps

1. **Complete Phase 4** (Planning):
   - Create `research.md` documenting technology choices
   - Create `data-model.md` with model definitions
   - Create `quickstart.md` with validation scenarios

2. **Move to Phase 5** (Tasking):
   - Break down implementation into ordered tasks
   - Mark parallel-safe tasks with `[P]`
   - Create `tasks.md`

3. **Execute Phase 6** (Implementation):
   - Follow TDD approach (write tests first)
   - Implement module following established patterns
   - Verify all acceptance criteria before committing
