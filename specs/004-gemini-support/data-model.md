# Data Model: Google Gemini Models

**Feature**: 004-gemini-support  
**Date**: 2026-03-14

---

## Overview

This document defines the data structures for Google Gemini model support in token-count.

**Key Characteristics**:
- 8 Gemini models across 3 generations (3.x, 2.5, 1.5)
- All models use the same tokenizer (Gemma 3 SentencePiece)
- 2 context window sizes: 1M tokens (most), 2M tokens (gemini-1.5-pro only)
- 12+ aliases for user convenience

---

## Entity: Gemini Model

### Description
Represents a specific Google Gemini model with its configuration metadata.

### Fields

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| `name` | String | Yes | Canonical name | Official model name (e.g., `gemini-3-flash-preview`) |
| `encoding` | String | Yes | `"gemini-gemma3"` | Tokenizer encoding name (same for all models) |
| `context_window` | usize | Yes | 1000000 or 2000000 | Maximum context window in tokens |
| `description` | String | Yes | Max 100 chars | Human-readable description |
| `aliases` | Vec\<String\> | Yes | Non-empty | List of alternative names |

### Validation Rules

1. **Name**: Must match pattern `gemini-{version}-{size}[-preview]`
   - Examples: `gemini-3-flash-preview`, `gemini-2.5-pro`, `gemini-1.5-flash`

2. **Encoding**: Must be `"gemini-gemma3"` for all models
   - Rationale: All Gemini models use identical tokenizer

3. **Context Window**: Must be `1000000` or `2000000`
   - Most models: 1M tokens
   - Special case: `gemini-1.5-pro` has 2M tokens

4. **Description**: Should include generation, size, and context window
   - Format: `"{tier} model, {context}M context ({status})"`
   - Example: `"Flash model, 1M context (Preview)"`

5. **Aliases**: Must be unique across all models (no collisions)
   - Case-insensitive matching
   - Include short names (`gemini`, `gemini-pro`, `gemini-flash`)
   - Include provider format (`google/gemini`)

### Relationships

**Model → Registry**: Many-to-One
- Models are registered in `ModelRegistry`
- Registry provides model lookup by name or alias

**Model → Tokenizer**: Many-to-One
- All models use the same `GoogleTokenizer` instance
- Tokenizer is shared (stateless, deterministic)

---

## Model Definitions

### Gemini 3.x Series (Priority 1 - Preview)

#### gemini-3.1-pro-preview

```rust
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
}
```

**Notes**:
- Replaces deprecated `gemini-3-pro-preview` (shut down Mar 9, 2026)
- Most capable 3.x model
- Released Feb 19, 2026

---

#### gemini-3-flash-preview (DEFAULT)

```rust
ModelConfig {
    name: "gemini-3-flash-preview".to_string(),
    encoding: "gemini-gemma3".to_string(),
    context_window: 1_000_000,
    description: "Flash model, 1M context (Preview, default)".to_string(),
    aliases: vec![
        "gemini".to_string(),           // Default alias
        "gemini-flash".to_string(),
        "gemini-3-flash".to_string(),
        "google/gemini".to_string(),    // Provider default
        "google/gemini-flash".to_string(),
    ],
}
```

**Notes**:
- Default model for `--model gemini`
- Fast, balanced performance
- Released Dec 17, 2025
- Forward-looking choice (2.5 deprecated June 2026)

---

#### gemini-3.1-flash-lite-preview

```rust
ModelConfig {
    name: "gemini-3.1-flash-lite-preview".to_string(),
    encoding: "gemini-gemma3".to_string(),
    context_window: 1_000_000,
    description: "Flash Lite model, 1M context (Preview, fastest)".to_string(),
    aliases: vec![
        "gemini-lite".to_string(),
        "gemini-3-lite".to_string(),
        "gemini-3.1-lite".to_string(),
        "gemini-3.1-flash-lite".to_string(),
        "google/gemini-lite".to_string(),
    ],
}
```

**Notes**:
- Most cost-efficient, fastest
- Released Mar 3, 2026
- Good for simple use cases

---

### Gemini 2.5 Series (Priority 2 - Deprecated June 2026)

#### gemini-2.5-pro

```rust
ModelConfig {
    name: "gemini-2.5-pro".to_string(),
    encoding: "gemini-gemma3".to_string(),
    context_window: 1_000_000,
    description: "Pro model, 1M context (GA, deprecated June 2026)".to_string(),
    aliases: vec![
        "gemini-2-pro".to_string(),
        "gemini-2.5".to_string(),
    ],
}
```

**Notes**:
- GA model (stable)
- Deprecated June 17, 2026
- Users should migrate to 3.1-pro-preview

---

#### gemini-2.5-flash

```rust
ModelConfig {
    name: "gemini-2.5-flash".to_string(),
    encoding: "gemini-gemma3".to_string(),
    context_window: 1_000_000,
    description: "Flash model, 1M context (GA, deprecated June 2026)".to_string(),
    aliases: vec![
        "gemini-2-flash".to_string(),
    ],
}
```

**Notes**:
- GA model (stable)
- Deprecated June 17, 2026
- Users should migrate to 3-flash-preview

---

#### gemini-2.5-flash-lite

```rust
ModelConfig {
    name: "gemini-2.5-flash-lite".to_string(),
    encoding: "gemini-gemma3".to_string(),
    context_window: 1_000_000,
    description: "Flash Lite model, 1M context (GA, deprecated June 2026)".to_string(),
    aliases: vec![
        "gemini-2-lite".to_string(),
        "gemini-2.5-lite".to_string(),
    ],
}
```

**Notes**:
- GA model (stable)
- Deprecated June 17, 2026
- Users should migrate to 3.1-flash-lite-preview

---

### Gemini 1.5 Series (Priority 3 - Legacy)

#### gemini-1.5-pro

```rust
ModelConfig {
    name: "gemini-1.5-pro".to_string(),
    encoding: "gemini-gemma3".to_string(),
    context_window: 2_000_000,  // ⚠️ Only model with 2M context
    description: "Pro model, 2M context (Legacy, largest context)".to_string(),
    aliases: vec![
        "gemini-1-pro".to_string(),
        "gemini-1.5".to_string(),
    ],
}
```

**Notes**:
- **Largest context window**: 2M tokens (unique)
- Legacy model (still supported)
- Good for very large documents

---

#### gemini-1.5-flash

```rust
ModelConfig {
    name: "gemini-1.5-flash".to_string(),
    encoding: "gemini-gemma3".to_string(),
    context_window: 1_000_000,
    description: "Flash model, 1M context (Legacy)".to_string(),
    aliases: vec![
        "gemini-1-flash".to_string(),
    ],
}
```

**Notes**:
- Legacy model (still supported)
- Users should migrate to 3-flash-preview

---

## Alias Resolution Table

| Alias | Resolves To | Type |
|-------|-------------|------|
| `gemini` | `gemini-3-flash-preview` | Default |
| `gemini-pro` | `gemini-3.1-pro-preview` | Short name |
| `gemini-flash` | `gemini-3-flash-preview` | Short name |
| `gemini-lite` | `gemini-3.1-flash-lite-preview` | Short name |
| `gemini-3-pro` | `gemini-3.1-pro-preview` | Version alias |
| `gemini-3-flash` | `gemini-3-flash-preview` | Version alias |
| `gemini-3-lite` | `gemini-3.1-flash-lite-preview` | Version alias |
| `gemini-3.1-pro` | `gemini-3.1-pro-preview` | Version without suffix |
| `gemini-3.1-lite` | `gemini-3.1-flash-lite-preview` | Version without suffix |
| `gemini-2-pro` | `gemini-2.5-pro` | Legacy alias |
| `gemini-2-flash` | `gemini-2.5-flash` | Legacy alias |
| `gemini-2-lite` | `gemini-2.5-flash-lite` | Legacy alias |
| `gemini-2.5` | `gemini-2.5-pro` | Version default |
| `gemini-2.5-lite` | `gemini-2.5-flash-lite` | Short name |
| `gemini-1-pro` | `gemini-1.5-pro` | Legacy alias |
| `gemini-1-flash` | `gemini-1.5-flash` | Legacy alias |
| `gemini-1.5` | `gemini-1.5-pro` | Version default |
| `google/gemini` | `gemini-3-flash-preview` | Provider default |
| `google/gemini-pro` | `gemini-3.1-pro-preview` | Provider format |
| `google/gemini-flash` | `gemini-3-flash-preview` | Provider format |
| `google/gemini-lite` | `gemini-3.1-flash-lite-preview` | Provider format |

**Case Sensitivity**: All aliases are case-insensitive (normalized to lowercase)

---

## Implementation Structures

### Rust Struct: GoogleTokenizer

```rust
/// Tokenizer for Google Gemini models
pub struct GoogleTokenizer {
    /// Underlying gemini-tokenizer instance
    tokenizer: LocalTokenizer,
    
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
        let tokenizer = LocalTokenizer::new(&config.name)
            .map_err(|e| TokenError::Tokenization(format!(
                "Failed to initialize Gemini tokenizer: {}",
                e
            )))?;
        
        Ok(Self { tokenizer, config })
    }
}

impl Tokenizer for GoogleTokenizer {
    fn count_tokens(&self, text: &str) -> anyhow::Result<usize> {
        let result = self.tokenizer.count_tokens(text, None);
        Ok(result.total_tokens)
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

---

### Function: google_models()

```rust
/// Get all Google Gemini model configurations
///
/// Returns a vector of ModelConfig for all supported Gemini models.
/// Models are ordered by generation (newest first) and tier (Pro > Flash > Lite).
pub fn google_models() -> Vec<ModelConfig> {
    vec![
        // Gemini 3.x Series (Priority 1 - Preview)
        // ... model definitions from above ...
        
        // Gemini 2.5 Series (Priority 2 - Deprecated June 2026)
        // ... model definitions from above ...
        
        // Gemini 1.5 Series (Priority 3 - Legacy)
        // ... model definitions from above ...
    ]
}
```

---

## Registry Integration

### Update: ModelRegistry::new()

```rust
impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self { models: HashMap::new(), aliases: HashMap::new() };
        
        // Existing: OpenAI models (4 models)
        // ... gpt-3.5-turbo, gpt-4, gpt-4-turbo, gpt-4o ...
        
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

### Update: ModelRegistry::get_tokenizer()

```rust
pub fn get_tokenizer(&self, name: &str, use_accurate: bool) -> Result<Box<dyn Tokenizer>, TokenError> {
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

---

## State Transitions

Not applicable (models are stateless configurations).

---

## Indexes / Lookup Tables

### Primary Index: Canonical Name

- Key: `String` (canonical model name)
- Value: `ModelConfig`
- Purpose: Fast lookup by exact name
- Storage: `HashMap<String, ModelConfig>` in `ModelRegistry`

### Secondary Index: Aliases

- Key: `String` (alias, lowercase)
- Value: `String` (canonical name)
- Purpose: Resolve aliases to canonical names
- Storage: `HashMap<String, String>` in `ModelRegistry`

**Example**:
```rust
aliases.get("gemini")          → Some("gemini-3-flash-preview")
aliases.get("gemini-pro")      → Some("gemini-3.1-pro-preview")
aliases.get("google/gemini")   → Some("gemini-3-flash-preview")
```

---

## Testing Scenarios

### Unit Tests

1. **Model count**: Verify 8 models returned by `google_models()`
2. **Default alias**: `gemini` resolves to `gemini-3-flash-preview`
3. **Pro alias**: `gemini-pro` resolves to `gemini-3.1-pro-preview`
4. **Context window**: `gemini-1.5-pro` has 2M, others have 1M
5. **Case-insensitive**: `GEMINI`, `Gemini`, `gemini` all resolve
6. **Provider format**: `google/gemini` resolves correctly
7. **Unique aliases**: No collisions across all models
8. **Encoding**: All models have `gemini-gemma3` encoding

### Integration Tests

1. **Registry integration**: All 8 models appear in `--list-models`
2. **Tokenizer creation**: Can create tokenizer for each model
3. **Token counting**: Token counts are identical across all models (same tokenizer)

---

## Summary Statistics

- **Total models**: 8
- **Gemini 3.x (Preview)**: 3 models
- **Gemini 2.5 (Deprecated)**: 3 models
- **Gemini 1.5 (Legacy)**: 2 models
- **Total aliases**: 21+ (including canonical names)
- **Context window sizes**: 2 (1M, 2M)
- **Encodings**: 1 (`gemini-gemma3`)

---

**Status**: Data model complete  
**Next**: Create `quickstart.md` validation guide
