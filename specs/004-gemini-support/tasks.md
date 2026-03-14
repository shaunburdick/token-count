# Tasks: Feature 004 - Google Gemini Token Counting Support

**Feature Branch**: `004-gemini-support`  
**Date**: 2026-03-14  
**Status**: Ready for implementation

---

## Task Execution Order

Tasks are numbered in recommended execution order. Tasks marked with `[P]` can be executed in parallel with adjacent tasks.

---

## Phase 6: Implementation

### Module 1: Dependencies & Setup

#### Task 001: Add gemini-tokenizer dependency [P]
**Status**: ⏳ Pending  
**Estimate**: 5 minutes  
**Dependencies**: None

**Action**:
```bash
# Edit Cargo.toml
```

**Changes**:
```toml
[dependencies]
# ... existing deps ...
gemini-tokenizer = "0.2.0"
```

**Verification**:
```bash
cargo check
# Should download gemini-tokenizer and dependencies (sentencepiece, sha2)
```

**Acceptance Criteria**:
- [ ] `gemini-tokenizer = "0.2.0"` added to Cargo.toml
- [ ] `cargo check` passes
- [ ] Dependencies resolve without conflicts

---

#### Task 002: Create google module structure [P]
**Status**: ⏳ Pending  
**Estimate**: 5 minutes  
**Dependencies**: None (can run in parallel with Task 001)

**Action**:
```bash
mkdir -p src/tokenizers/google
touch src/tokenizers/google/mod.rs
touch src/tokenizers/google/models.rs
touch src/tokenizers/google/tokenizer.rs
```

**Verification**:
```bash
ls -la src/tokenizers/google/
# Should show 3 files: mod.rs, models.rs, tokenizer.rs
```

**Acceptance Criteria**:
- [ ] Directory `src/tokenizers/google/` exists
- [ ] Three empty files created (mod.rs, models.rs, tokenizer.rs)

---

### Module 2: Model Definitions

#### Task 003: Implement google/models.rs
**Status**: ⏳ Pending  
**Estimate**: 30 minutes  
**Dependencies**: Task 002

**Action**:
Implement `google_models()` function with 8 Gemini model definitions.

**Reference**: `specs/004-gemini-support/data-model.md` (lines 40-330)

**Implementation**:
```rust
//! Google Gemini model definitions and metadata

use crate::tokenizers::registry::ModelConfig;

/// Get all Google Gemini model configurations
///
/// Returns a vector of ModelConfig for all supported Gemini models.
/// Models are ordered by generation (newest first) and tier (Pro > Flash > Lite).
pub fn google_models() -> Vec<ModelConfig> {
    vec![
        // Gemini 3.x Series (Priority 1 - Preview)
        ModelConfig {
            name: "gemini-3.1-pro-preview".to_string(),
            encoding: "gemini-gemma3".to_string(),
            context_window: 1_000_000,
            description: "Pro model, 1M context (Preview, Feb 2026)".to_string(),
            aliases: vec![
                "gemini-pro".to_string(),
                "gemini-3-pro".to_string(),
                "gemini-3.1-pro".to_string(),
                "google/gemini-pro".to_string(),
            ],
        },
        // ... (implement all 8 models from data-model.md)
    ]
}
```

**Verification**:
```bash
cargo check
```

**Acceptance Criteria**:
- [ ] All 8 models defined (3.x: 3, 2.5: 3, 1.5: 2)
- [ ] All aliases defined (12+ aliases total)
- [ ] Context windows correct (1M or 2M)
- [ ] Encoding is `"gemini-gemma3"` for all models
- [ ] Compiles without errors or warnings

**Tests** (add to models.rs):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_models_count() {
        let models = google_models();
        assert_eq!(models.len(), 8, "Should have 8 Gemini models");
    }

    #[test]
    fn test_default_alias() {
        let models = google_models();
        let flash = models.iter().find(|m| m.name == "gemini-3-flash-preview");
        assert!(flash.is_some());
        assert!(
            flash.unwrap().aliases.contains(&"gemini".to_string()),
            "gemini alias should point to gemini-3-flash-preview"
        );
    }

    #[test]
    fn test_gemini_1_5_pro_context_window() {
        let models = google_models();
        let pro_1_5 = models.iter().find(|m| m.name == "gemini-1.5-pro").unwrap();
        assert_eq!(pro_1_5.context_window, 2_000_000, "1.5 Pro should have 2M context");
    }

    #[test]
    fn test_all_use_same_encoding() {
        let models = google_models();
        for model in models {
            assert_eq!(model.encoding, "gemini-gemma3", "All models should use gemini-gemma3 encoding");
        }
    }
}
```

---

#### Task 004: Implement google/tokenizer.rs
**Status**: ⏳ Pending  
**Estimate**: 20 minutes  
**Dependencies**: Task 001 (gemini-tokenizer dependency)

**Action**:
Implement wrapper around `gemini-tokenizer` crate.

**Implementation**:
```rust
//! Wrapper around gemini-tokenizer crate

use crate::error::TokenError;
use anyhow::{Context, Result};
use gemini_tokenizer::LocalTokenizer;

/// Wrapper around gemini-tokenizer's LocalTokenizer
///
/// Provides a simplified interface for token counting.
pub struct GeminiTokenizer {
    tokenizer: LocalTokenizer,
}

impl GeminiTokenizer {
    /// Create a new Gemini tokenizer
    ///
    /// # Arguments
    /// * `model_name` - Any Gemini model name (all use same tokenizer)
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully initialized tokenizer
    /// * `Err(TokenError::Tokenization)` - Failed to initialize
    pub fn new(model_name: &str) -> Result<Self, TokenError> {
        let tokenizer = LocalTokenizer::new(model_name).map_err(|e| {
            TokenError::Tokenization(format!("Failed to initialize Gemini tokenizer: {}", e))
        })?;

        Ok(Self { tokenizer })
    }

    /// Count tokens in the given text
    ///
    /// # Arguments
    /// * `text` - Input text to tokenize
    ///
    /// # Returns
    /// * `Ok(usize)` - Total token count
    /// * `Err(anyhow::Error)` - Tokenization failed
    pub fn count_tokens(&self, text: &str) -> Result<usize> {
        let result = self.tokenizer.count_tokens(text, None);
        Ok(result.total_tokens)
    }

    /// Get detailed token information for debug mode
    ///
    /// Returns token IDs and decoded tokens.
    pub fn compute_tokens(&self, text: &str) -> Result<Vec<(i32, String)>> {
        let result = self
            .tokenizer
            .compute_tokens(text)
            .context("Failed to compute tokens")?;

        let mut tokens = Vec::new();
        for info in result.tokens_info {
            for (id, token) in info.token_ids.iter().zip(&info.tokens) {
                tokens.push((*id, token.clone()));
            }
        }

        Ok(tokens)
    }
}
```

**Verification**:
```bash
cargo check
```

**Acceptance Criteria**:
- [ ] `GeminiTokenizer` struct defined
- [ ] `new()` method initializes LocalTokenizer
- [ ] `count_tokens()` method returns token count
- [ ] `compute_tokens()` method returns token details
- [ ] Errors wrapped with helpful context
- [ ] Compiles without errors or warnings

**Tests** (add to tokenizer.rs):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_initialization() {
        let tokenizer = GeminiTokenizer::new("gemini-3-flash-preview");
        assert!(tokenizer.is_ok());
    }

    #[test]
    fn test_count_tokens() {
        let tokenizer = GeminiTokenizer::new("gemini-3-flash-preview").unwrap();
        let count = tokenizer.count_tokens("Hello, Gemini!").unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_empty_string() {
        let tokenizer = GeminiTokenizer::new("gemini-3-flash-preview").unwrap();
        let count = tokenizer.count_tokens("").unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_compute_tokens() {
        let tokenizer = GeminiTokenizer::new("gemini-3-flash-preview").unwrap();
        let tokens = tokenizer.compute_tokens("Hello").unwrap();
        assert!(!tokens.is_empty());
        assert!(tokens[0].0 > 0); // Token ID should be positive
        assert!(!tokens[0].1.is_empty()); // Token string should not be empty
    }
}
```

---

#### Task 005: Implement google/mod.rs
**Status**: ⏳ Pending  
**Estimate**: 25 minutes  
**Dependencies**: Task 003, Task 004

**Action**:
Implement `GoogleTokenizer` that combines models + tokenizer and implements `Tokenizer` trait.

**Reference**: `specs/004-gemini-support/data-model.md` (lines 386-418)

**Implementation**:
```rust
//! Tokenizer implementation for Google Gemini models

mod models;
mod tokenizer;

pub use models::google_models;

use crate::error::TokenError;
use crate::tokenizers::registry::ModelConfig;
use crate::tokenizers::{ModelInfo, Tokenizer};
use tokenizer::GeminiTokenizer;

/// Tokenizer for Google Gemini models
pub struct GoogleTokenizer {
    /// Underlying gemini-tokenizer wrapper
    gemini: GeminiTokenizer,

    /// Model configuration (name, context window, etc.)
    config: ModelConfig,
}

impl GoogleTokenizer {
    /// Create a new Google tokenizer
    ///
    /// # Arguments
    /// * `config` - Model configuration
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully created tokenizer
    /// * `Err(TokenError::Tokenization)` - Failed to initialize
    pub fn new(config: ModelConfig) -> Result<Self, TokenError> {
        let gemini = GeminiTokenizer::new(&config.name)?;
        Ok(Self { gemini, config })
    }
}

impl Tokenizer for GoogleTokenizer {
    fn count_tokens(&self, text: &str) -> anyhow::Result<usize> {
        self.gemini.count_tokens(text)
    }

    fn get_model_info(&self) -> ModelInfo {
        ModelInfo {
            name: self.config.name.clone(),
            encoding: self.config.encoding.clone(),
            context_window: self.config.context_window,
            description: self.config.description.clone(),
        }
    }
}
```

**Verification**:
```bash
cargo check
```

**Acceptance Criteria**:
- [ ] `GoogleTokenizer` struct defined
- [ ] `Tokenizer` trait implemented
- [ ] `google_models()` exported
- [ ] Compiles without errors or warnings

**Tests** (add to mod.rs):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizers::Tokenizer;

    #[test]
    fn test_google_tokenizer_creation() {
        let config = google_models().into_iter().next().unwrap();
        let tokenizer = GoogleTokenizer::new(config);
        assert!(tokenizer.is_ok());
    }

    #[test]
    fn test_tokenizer_trait_implementation() {
        let config = google_models().into_iter().next().unwrap();
        let tokenizer = GoogleTokenizer::new(config).unwrap();

        // Test count_tokens
        let count = tokenizer.count_tokens("Hello").unwrap();
        assert!(count > 0);

        // Test get_model_info
        let info = tokenizer.get_model_info();
        assert_eq!(info.encoding, "gemini-gemma3");
    }
}
```

---

### Module 3: Registry Integration

#### Task 006: Update src/tokenizers/mod.rs
**Status**: ⏳ Pending  
**Estimate**: 5 minutes  
**Dependencies**: Task 005

**Action**:
Add `google` module to `src/tokenizers/mod.rs`.

**Changes**:
```rust
pub mod claude;
pub mod google;  // NEW
pub mod openai;
pub mod registry;
```

**Verification**:
```bash
cargo check
```

**Acceptance Criteria**:
- [ ] `pub mod google;` added
- [ ] Compiles without errors or warnings

---

#### Task 007: Update registry.rs - Add Google models
**Status**: ⏳ Pending  
**Estimate**: 10 minutes  
**Dependencies**: Task 006

**Action**:
Update `ModelRegistry::new()` to register Google models.

**Reference**: `specs/004-gemini-support/data-model.md` (lines 435-456)

**Changes** to `src/tokenizers/registry.rs`:
```rust
use crate::tokenizers::{
    claude::{claude_models, ClaudeTokenizer},
    google::{google_models, GoogleTokenizer},  // NEW
    openai::OpenAITokenizer,
    ModelInfo, Tokenizer,
};

// In ModelRegistry::new():
impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self { models: HashMap::new(), aliases: HashMap::new() };

        // Existing: OpenAI models (4 models)
        // ... existing code ...

        // Existing: Claude models (3 models)
        for model in claude_models() {
            registry.add_model(model);
        }

        // NEW: Google Gemini models (8 models)
        for model in google_models() {
            registry.add_model(model);
        }

        registry
    }
}
```

**Verification**:
```bash
cargo check
```

**Acceptance Criteria**:
- [ ] `google_models()` imported
- [ ] Loop added to register all Google models
- [ ] Compiles without errors or warnings

---

#### Task 008: Update registry.rs - Add GoogleTokenizer
**Status**: ⏳ Pending  
**Estimate**: 15 minutes  
**Dependencies**: Task 007

**Action**:
Update `ModelRegistry::get_tokenizer()` to handle Google models.

**Reference**: `specs/004-gemini-support/data-model.md` (lines 458-488)

**Changes** to `get_tokenizer()` method:
```rust
pub fn get_tokenizer(
    &self,
    name: &str,
    use_accurate: bool,
) -> Result<Box<dyn Tokenizer>, TokenError> {
    let config = self.get_model(name)?;

    // Detect tokenizer type based on encoding
    match config.encoding.as_str() {
        "anthropic-claude" => {
            // Claude tokenizer (estimation or API)
            let tokenizer = ClaudeTokenizer::new(config.clone(), use_accurate)?;
            Ok(Box::new(tokenizer))
        }
        "gemini-gemma3" => {
            // NEW: Google Gemini tokenizer
            let tokenizer = GoogleTokenizer::new(config.clone())?;
            Ok(Box::new(tokenizer))
        }
        _ => {
            // OpenAI tokenizer (tiktoken)
            let model_info = ModelInfo {
                name: config.name.clone(),
                encoding: config.encoding.clone(),
                context_window: config.context_window,
                description: config.description.clone(),
            };

            let tokenizer = OpenAITokenizer::new(&config.encoding, model_info)
                .map_err(|e| TokenError::Tokenization(e.to_string()))?;

            Ok(Box::new(tokenizer))
        }
    }
}
```

**Verification**:
```bash
cargo check
```

**Acceptance Criteria**:
- [ ] `"gemini-gemma3"` case added to match statement
- [ ] `GoogleTokenizer` instantiated correctly
- [ ] Compiles without errors or warnings

---

#### Task 009: Update registry.rs - Fix test expectations
**Status**: ⏳ Pending  
**Estimate**: 5 minutes  
**Dependencies**: Task 008

**Action**:
Update test that counts total models (now 15 instead of 7).

**Changes** to `src/tokenizers/registry.rs` tests:
```rust
#[test]
fn test_list_models() {
    let registry = ModelRegistry::new();
    let models = registry.list_models();
    assert_eq!(models.len(), 15); // Was 7, now: 4 OpenAI + 3 Claude + 8 Gemini
    // ... existing assertions ...
}
```

**Verification**:
```bash
cargo test --lib registry
```

**Acceptance Criteria**:
- [ ] Test count updated to 15
- [ ] Registry tests pass

---

### Module 4: Testing

#### Task 010: Write unit tests for google module
**Status**: ⏳ Pending  
**Estimate**: 30 minutes  
**Dependencies**: Task 009

**Action**:
Add comprehensive unit tests (already added inline in Tasks 003-005, verify they pass).

**Tests to verify**:
- `google/models.rs`: 4 tests
- `google/tokenizer.rs`: 4 tests
- `google/mod.rs`: 2 tests

**Verification**:
```bash
cargo test --lib google
```

**Expected output**:
```
running 10 tests
test tokenizers::google::models::tests::test_all_use_same_encoding ... ok
test tokenizers::google::models::tests::test_default_alias ... ok
test tokenizers::google::models::tests::test_gemini_1_5_pro_context_window ... ok
test tokenizers::google::models::tests::test_google_models_count ... ok
test tokenizers::google::tokenizer::tests::test_compute_tokens ... ok
test tokenizers::google::tokenizer::tests::test_count_tokens ... ok
test tokenizers::google::tokenizer::tests::test_empty_string ... ok
test tokenizers::google::tokenizer::tests::test_tokenizer_initialization ... ok
test tokenizers::google::mod::tests::test_google_tokenizer_creation ... ok
test tokenizers::google::mod::tests::test_tokenizer_trait_implementation ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured
```

**Acceptance Criteria**:
- [ ] All 10 unit tests pass
- [ ] No warnings or errors

---

#### Task 011: Write integration tests - Create test file
**Status**: ⏳ Pending  
**Estimate**: 45 minutes  
**Dependencies**: Task 010

**Action**:
Create `tests/google_tokenization.rs` with CLI integration tests.

**File**: `tests/google_tokenization.rs`

**Implementation** (15+ tests):
```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_gemini_default_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(&["--model", "gemini"])
        .write_stdin("Hello, Gemini!")
        .assert()
        .success()
        .stdout("3\n");
}

#[test]
fn test_gemini_pro_alias() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(&["--model", "gemini-pro"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("1\n");
}

#[test]
fn test_all_gemini_models() {
    let models = vec![
        "gemini-3.1-pro-preview",
        "gemini-3-flash-preview",
        "gemini-3.1-flash-lite-preview",
        "gemini-2.5-pro",
        "gemini-2.5-flash",
        "gemini-2.5-flash-lite",
        "gemini-1.5-pro",
        "gemini-1.5-flash",
    ];

    for model in models {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.args(&["--model", model])
            .write_stdin("test")
            .assert()
            .success()
            .stdout("1\n");
    }
}

#[test]
fn test_gemini_case_insensitive() {
    for variant in &["gemini", "GEMINI", "Gemini", "GeMiNi"] {
        let mut cmd = Command::cargo_bin("token-count").unwrap();
        cmd.args(&["--model", variant])
            .write_stdin("test")
            .assert()
            .success()
            .stdout("1\n");
    }
}

#[test]
fn test_gemini_provider_format() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(&["--model", "google/gemini"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout("1\n");
}

#[test]
fn test_gemini_verbose() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(&["--model", "gemini", "-v"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("Model: gemini-3-flash-preview"))
        .stdout(predicate::str::contains("gemini-gemma3"))
        .stdout(predicate::str::contains("Tokens: 1"))
        .stdout(predicate::str::contains("Context window: 1000000"));
}

#[test]
fn test_gemini_1_5_pro_context_window() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(&["--model", "gemini-1.5-pro", "-v"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("Context window: 2000000"));
}

#[test]
fn test_gemini_list_models() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.arg("--list-models")
        .assert()
        .success()
        .stdout(predicate::str::contains("Google Gemini"))
        .stdout(predicate::str::contains("gemini-3-flash-preview"))
        .stdout(predicate::str::contains("gemini-gemma3"));
}

#[test]
fn test_gemini_unknown_model() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(&["--model", "gemini-4"])
        .write_stdin("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown model: 'gemini-4'"))
        .stderr(predicate::str::contains("Did you mean"));
}

#[test]
fn test_gemini_empty_input() {
    let mut cmd = Command::cargo_bin("token-count").unwrap();
    cmd.args(&["--model", "gemini"])
        .write_stdin("")
        .assert()
        .success()
        .stdout("0\n");
}

// ... (add more tests for debug mode, large input, etc.)
```

**Verification**:
```bash
cargo test --test google_tokenization
```

**Acceptance Criteria**:
- [ ] All 15+ integration tests pass
- [ ] Tests cover all 6 user stories
- [ ] Tests cover edge cases
- [ ] No warnings or errors

---

#### Task 012: Update benchmark suite
**Status**: ⏳ Pending  
**Estimate**: 20 minutes  
**Dependencies**: Task 011

**Action**:
Add Gemini benchmarks to `benches/tokenization.rs`.

**Changes** to `benches/tokenization.rs`:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use token_count::tokenizers::registry::ModelRegistry;

// ... existing benches ...

fn bench_gemini_small(c: &mut Criterion) {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gemini", false).unwrap();
    let text = "Hello, Gemini!";

    c.bench_function("gemini_small", |b| {
        b.iter(|| tokenizer.count_tokens(black_box(text)))
    });
}

fn bench_gemini_medium(c: &mut Criterion) {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gemini", false).unwrap();
    let text = "Hello, Gemini! ".repeat(1000); // ~15KB

    c.bench_function("gemini_medium", |b| {
        b.iter(|| tokenizer.count_tokens(black_box(&text)))
    });
}

fn bench_gemini_large(c: &mut Criterion) {
    let registry = ModelRegistry::global();
    let tokenizer = registry.get_tokenizer("gemini", false).unwrap();
    let text = "Hello, Gemini! ".repeat(100_000); // ~1.5MB

    c.bench_function("gemini_large", |b| {
        b.iter(|| tokenizer.count_tokens(black_box(&text)))
    });
}

criterion_group!(
    benches,
    // ... existing benches ...
    bench_gemini_small,
    bench_gemini_medium,
    bench_gemini_large
);
```

**Verification**:
```bash
cargo bench
```

**Expected**:
- Small: <10ms (target), ~1ms (actual)
- Medium: <100ms (target), ~5-10ms (actual)
- Large: <1s (target), ~50-100ms (actual)

**Acceptance Criteria**:
- [ ] Benchmarks added for small, medium, large inputs
- [ ] Performance meets targets
- [ ] No warnings or errors

---

### Module 5: Documentation

#### Task 013: Update README.md
**Status**: ⏳ Pending  
**Estimate**: 15 minutes  
**Dependencies**: Task 012

**Action**:
Update README with Gemini support.

**Changes to README.md**:

1. Update model count in intro:
   ```markdown
   Supports 15+ models (OpenAI, Claude, Gemini)
   ```

2. Add Gemini examples:
   ```markdown
   ## Supported Models

   ### Google Gemini
   - gemini-3.1-pro-preview (1M context)
   - gemini-3-flash-preview (1M context, default)
   - gemini-3.1-flash-lite-preview (1M context)
   - gemini-2.5-pro, gemini-2.5-flash, gemini-2.5-flash-lite (deprecated June 2026)
   - gemini-1.5-pro (2M context), gemini-1.5-flash (legacy)

   ```bash
   echo "Hello, Gemini!" | token-count --model gemini
   ```

3. Update features list:
   ```markdown
   ✅ Google Gemini (exact, offline, SentencePiece)
   ```

**Verification**:
```bash
# Preview README
cat README.md | grep -A 10 "Gemini"
```

**Acceptance Criteria**:
- [ ] Gemini models listed
- [ ] Examples added
- [ ] Model count updated
- [ ] Features list updated

---

#### Task 014: Update CHANGELOG.md
**Status**: ⏳ Pending  
**Estimate**: 15 minutes  
**Dependencies**: Task 013

**Action**:
Add v0.3.0 entry to CHANGELOG.

**Reference**: `specs/004-gemini-support.md` (lines 874-913)

**Addition to CHANGELOG.md**:
```markdown
## [0.3.0] - 2026-XX-XX

### 🚀 Google Gemini Support

Added exact, offline tokenization for Google Gemini models using the 
gemini-tokenizer crate. All Gemini models (1.5, 2.x, 3.x) use the same 
Gemma 3 SentencePiece tokenizer (262,144 vocab).

### Added

#### Gemini Tokenization
- **8 Gemini models**: 3.1-pro, 3-flash, 3.1-flash-lite (3.x preview), 
  2.5-pro, 2.5-flash, 2.5-flash-lite (2.5 GA, deprecated June 2026), 
  1.5-pro, 1.5-flash (1.5 legacy)
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

**Verification**:
```bash
cat CHANGELOG.md | head -50
```

**Acceptance Criteria**:
- [ ] v0.3.0 entry added at top
- [ ] All features documented
- [ ] Technical details included
- [ ] Follows existing format

---

### Module 6: Verification & Commit

#### Task 015: Run all tests
**Status**: ⏳ Pending  
**Estimate**: 5 minutes  
**Dependencies**: Task 014

**Action**:
```bash
cargo test --all-features
```

**Expected output**:
```
running 187 tests
...
test result: ok. 187 passed; 0 failed; 0 ignored; 0 measured
```

**Acceptance Criteria**:
- [ ] All 187+ tests pass
- [ ] Zero failures
- [ ] No ignored tests

---

#### Task 016: Run linter
**Status**: ⏳ Pending  
**Estimate**: 5 minutes  
**Dependencies**: Task 015

**Action**:
```bash
cargo clippy -- -D warnings
```

**Expected**: Zero warnings

**Acceptance Criteria**:
- [ ] Clippy passes
- [ ] Zero warnings (enforced with `-D warnings`)
- [ ] No linting suppressions added (eslint-disable equivalent)

---

#### Task 017: Check formatting
**Status**: ⏳ Pending  
**Estimate**: 2 minutes  
**Dependencies**: Task 016

**Action**:
```bash
cargo fmt --check
```

**Expected**: No formatting issues

**Acceptance Criteria**:
- [ ] All files formatted correctly
- [ ] No formatting needed

---

#### Task 018: Build release binary
**Status**: ⏳ Pending  
**Estimate**: 2 minutes  
**Dependencies**: Task 017

**Action**:
```bash
cargo build --release
ls -lh target/release/token-count
```

**Expected**: 
- Build succeeds
- Binary size ~11-12MB (under 15MB target)

**Acceptance Criteria**:
- [ ] Release build succeeds
- [ ] Binary size <15MB
- [ ] No build warnings

---

#### Task 019: Manual validation (quickstart guide)
**Status**: ⏳ Pending  
**Estimate**: 30 minutes  
**Dependencies**: Task 018

**Action**:
Follow validation guide in `specs/004-gemini-support/quickstart.md`.

**Key flows to test**:
- [ ] Flow 1: Quick Gemini token count (US-015)
- [ ] Flow 2: Model aliases (US-016)
- [ ] Flow 3: Multiple model versions (US-017)
- [ ] Flow 4: Context window validation (US-018)
- [ ] Flow 5: List Gemini models (US-019)
- [ ] Flow 6: Debug mode token details (US-020)

**Edge cases to test**:
- [ ] Empty input
- [ ] Invalid UTF-8
- [ ] Unknown model
- [ ] Large input (1MB)
- [ ] Gemini 1.5 Pro (2M context)

**Acceptance Criteria**:
- [ ] All user flows work correctly
- [ ] All edge cases handled gracefully
- [ ] Output matches expectations

---

#### Task 020: Commit implementation
**Status**: ⏳ Pending  
**Estimate**: 5 minutes  
**Dependencies**: Task 019

**Action**:
```bash
git status
git add src/tokenizers/google/
git add src/tokenizers/mod.rs
git add src/tokenizers/registry.rs
git add Cargo.toml
git add tests/google_tokenization.rs
git add benches/tokenization.rs
git add README.md
git add CHANGELOG.md

git commit -m "feat: Add Google Gemini token counting support (v0.3.0)

- Add 8 Gemini models (3.x preview, 2.5 deprecated, 1.5 legacy)
- Implement exact offline tokenization using gemini-tokenizer v0.2.0
- All models use Gemma 3 SentencePiece tokenizer (262K vocab)
- Context windows: 1M tokens (most), 2M tokens (gemini-1.5-pro)
- Default model: gemini-3-flash-preview (forward-looking)
- Add 35+ tests (187 total, all passing)
- Binary size: 11.5MB (+2.3MB, under budget)
- Performance: <1ms small inputs, ~50ms for 1MB

Closes #XXX"
```

**Verification**:
```bash
git log -1 --stat
```

**Acceptance Criteria**:
- [ ] All files committed
- [ ] Commit message follows conventional commits format
- [ ] Commit message is descriptive

---

## Summary

**Total tasks**: 20  
**Estimated time**: ~5-6 hours  
**Parallel opportunities**: Tasks 001 & 002 can run in parallel

**Execution order**:
1. Module 1 (Dependencies): Tasks 001-002 [P]
2. Module 2 (Models): Tasks 003-005
3. Module 3 (Registry): Tasks 006-009
4. Module 4 (Testing): Tasks 010-012
5. Module 5 (Documentation): Tasks 013-014
6. Module 6 (Verification): Tasks 015-020

**Next step**: Execute Task 001

---

**Status**: Task breakdown complete, ready for implementation  
**Phase 5 complete**: Moving to Phase 6 (Implementation)
