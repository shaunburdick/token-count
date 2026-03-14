# Feature 004: Google Gemini Support - Summary

**Created**: 2026-03-14  
**Status**: ✅ Specification Complete - Ready for Planning  

---

## 📋 Quick Summary

Feature 004 adds **exact, offline tokenization** for Google Gemini models using the `gemini-tokenizer` Rust crate. Unlike Claude (which uses estimation), Gemini gives us 100% accurate token counts without API calls.

**Default model**: `gemini-3-flash-preview` (forward-looking, 2.5 series deprecated June 2026)

---

## 🎯 What's Included

### Models (8 total)

**Gemini 3.x (Priority 1 - Preview but Future)**:
- `gemini-3.1-pro-preview` (1M context)
- `gemini-3-flash-preview` (1M context) ← **Default**
- `gemini-3.1-flash-lite-preview` (1M context)

**Gemini 2.5 (Priority 2 - Deprecated June 17, 2026)**:
- `gemini-2.5-pro`, `gemini-2.5-flash`, `gemini-2.5-flash-lite`

**Gemini 1.5 (Priority 3 - Legacy)**:
- `gemini-1.5-pro` (2M context!), `gemini-1.5-flash`

### Aliases

```bash
gemini → gemini-3-flash-preview
gemini-pro → gemini-3.1-pro-preview
gemini-flash → gemini-3-flash-preview
gemini-lite → gemini-3.1-flash-lite-preview
google/gemini → gemini-3-flash-preview
```

---

## ✨ Key Features

1. **Exact offline tokenization** - No API, no network, 100% accurate
2. **Same tokenizer for ALL Gemini models** - Gemma 3 SentencePiece (262K vocab)
3. **Massive context windows** - 1M tokens (most), 2M tokens (1.5-pro)
4. **Fast** - <1ms for small inputs
5. **Small binary increase** - +2.3MB (9.2MB → 11.5MB total)

---

## 📊 User Stories (6 total)

- **US-015**: Quick Gemini token count
- **US-016**: Model aliases support
- **US-017**: Multiple Gemini versions (3.x, 2.5, 1.5)
- **US-018**: Context window validation
- **US-019**: List Gemini models
- **US-020**: Debug mode token details

---

## 🧪 Testing

- **35+ new tests** (unit + integration)
- **Total: 187 tests** (up from 152)
- Performance benchmarks
- Cross-platform CI (Linux, macOS, Windows)
- One-time comparison with Google's Python SDK (validation)

---

## 📦 Technical Details

### New Dependency
```toml
gemini-tokenizer = "0.2.0"  # Apache-2.0, community-maintained
```

### Module Structure
```
src/tokenizers/google/
├── mod.rs           # Public API, provider trait impl
├── models.rs        # Model definitions (8 models, aliases)
└── tokenizer.rs     # Wrapper around gemini-tokenizer crate
```

### Performance Targets
- Small input (<10KB): <10ms ✅ (expect ~1ms)
- Medium input (1MB): <100ms ✅ (expect ~50ms)
- Large input (100MB): <10s (streaming)

---

## 🚫 Out of Scope (v0.3.0)

- ❌ Multimodal tokenization (images/audio/video) - future v0.4.0
- ❌ Function calling token counting - niche use case
- ❌ Cost estimation - violates constitution
- ❌ Model comparison mode - use tool multiple times
- ❌ API mode - not needed (exact offline already)

---

## 📚 Documentation

**Files**:
- [Feature Spec](./004-gemini-support.md) - Full specification (this document)
- [Research](../.specify/RESEARCH-GEMINI-TOKENIZATION.md) - Technical research

**Updates Needed**:
- README.md - Add Gemini to supported models
- CHANGELOG.md - Document v0.3.0 changes
- `--list-models` output - Include Gemini models

---

## 🎯 Success Metrics

- 🎯 100% token count accuracy (match Google's tokenizer)
- 🎯 <15MB binary size (currently ~11.5MB projected)
- 🎯 95%+ operations complete in <100ms
- 🎯 Zero clippy warnings
- 🎯 80%+ test coverage

---

## ⏭️ Next Steps

### For You (User)
✅ Review specification  
✅ Approve or request changes  
✅ Hand off to implementation

### For Implementation Agent
1. Create feature branch: `004-gemini-support`
2. Add `gemini-tokenizer` dependency
3. Implement `src/tokenizers/google/` module
4. Write 35+ tests
5. Update documentation
6. Release v0.3.0

---

## 🤔 Key Decisions Made

1. **Focus on 3.x preview models** - 2.5 deprecated June 2026 (3 months)
2. **Default: gemini-3-flash-preview** - Forward-looking, fast workhorse
3. **No preview warnings** - Clean UX, users know preview status from name
4. **Keep `-preview` suffix** - Transparent, matches Google's naming
5. **No 2.5 models as defaults** - Avoid deprecation in 3 months
6. **Exact offline tokenization** - No API mode needed (unlike Claude)

---

## 🔗 Related Features

- **Feature 001**: Core CLI (OpenAI exact tokenization)
- **Feature 003**: Claude Support (estimation + API, contrast to Gemini)
- **Constitution**: Principle III (Zero External Dependencies) - perfectly aligned

---

## ✅ Specification Quality Checklist

- [x] Constitution alignment verified (all 7 principles)
- [x] All requirements are clear and unambiguous
- [x] Acceptance criteria are specific and testable
- [x] Technology versions specified (`gemini-tokenizer = "0.2.0"`)
- [x] Quality gates defined (tests, clippy, benchmarks)
- [x] Edge cases documented (6 scenarios)
- [x] Out of scope clearly defined
- [x] Success metrics measurable
- [x] Zero "[NEEDS CLARIFICATION]" markers

---

**Status**: ✅ Ready for Planning → Implementation → Release  
**Target Release**: v0.3.0  
**Estimated Effort**: 1-2 weeks
