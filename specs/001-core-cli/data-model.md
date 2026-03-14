# Data Model: Core CLI Token Counting

**Date**: 2026-03-13 | **Feature**: 001-core-cli | **Plan**: [plan.md](./plan.md)

## Overview

This document defines all data structures, enums, traits, and their relationships for the token-count CLI tool. Each entity includes field definitions, validation rules, and usage context.

---

## CLI Layer

### Cli (Command-Line Arguments)

**Purpose**: Parsed command-line arguments using clap derive macros.

**Definition**:
```rust
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "token-count",
    version,
    about = "Count tokens for LLM models using exact tokenization",
    long_about = "A POSIX-style CLI tool that counts tokens for LLM models.
Pipe text via stdin and get accurate token counts for GPT-4, Claude, Gemini, and more."
)]
pub struct Cli {
    /// Model to use for tokenization
    #[arg(
        short = 'm',
        long = "model",
        default_value = "gpt-3.5-turbo",
        help = "Model name or alias (e.g., gpt-4, claude-sonnet)"
    )]
    pub model: String,

    /// Increase output verbosity (can be repeated: -v, -vv, -vvv)
    #[arg(
        short = 'v',
        long = "verbose",
        action = clap::ArgAction::Count,
        help = "Verbosity level: 0=count only, 1=model info, 2=context %, 3=token IDs"
    )]
    pub verbosity: u8,

    /// List all supported models
    #[arg(
        long = "list-models",
        help = "Display all supported models and their aliases"
    )]
    pub list_models: bool,
}
```

**Fields**:
- `model`: String - Model name or alias (default: "gpt-3.5-turbo")
- `verbosity`: u8 - Output detail level (0-3, capped at 3)
- `list_models`: bool - Flag to display supported models

**Validation**:
- `model` must be non-empty (clap ensures this)
- `verbosity` > 3 is treated as 3 (capped in code)
- `list_models` is mutually exclusive with stdin processing (checked in main)

**Usage**:
```rust
fn main() {
    let args = Cli::parse();
    
    if args.list_models {
        print_supported_models();
        return;
    }
    
    // Process stdin with args.model and args.verbosity
}
```

---

## Model Layer

### ModelConfig (Model Metadata)

**Purpose**: Configuration for a supported LLM model.

**Definition**:
```rust
#[derive(Debug, Clone)]
pub struct ModelConfig {
    /// Canonical model name (e.g., "gpt-4")
    pub name: &'static str,
    
    /// Tokenizer encoding name (e.g., "cl100k_base")
    pub encoding: &'static str,
    
    /// Maximum context window size in tokens
    pub context_window: usize,
    
    /// List of recognized aliases (e.g., ["gpt4", "openai/gpt-4"])
    pub aliases: &'static [&'static str],
    
    /// Provider name (e.g., "OpenAI")
    pub provider: &'static str,
}
```

**Fields**:
- `name`: &'static str - Canonical name (used in output)
- `encoding`: &'static str - Tokenizer encoding (for tiktoken-rs)
- `context_window`: usize - Max tokens (for percentage calculation)
- `aliases`: &'static [&'static str] - Alternative names
- `provider`: &'static str - Provider name (for `--list-models`)

**Constants** (OpenAI Models):
```rust
pub const GPT_35_TURBO: ModelConfig = ModelConfig {
    name: "gpt-3.5-turbo",
    encoding: "cl100k_base",
    context_window: 16_385,
    aliases: &["gpt35", "gpt3.5", "openai/gpt-3.5-turbo"],
    provider: "OpenAI",
};

pub const GPT_4: ModelConfig = ModelConfig {
    name: "gpt-4",
    encoding: "cl100k_base",
    context_window: 8_192,
    aliases: &["gpt4", "openai/gpt-4"],
    provider: "OpenAI",
};

pub const GPT_4_TURBO: ModelConfig = ModelConfig {
    name: "gpt-4-turbo",
    encoding: "cl100k_base",
    context_window: 128_000,
    aliases: &["gpt4-turbo", "openai/gpt-4-turbo"],
    provider: "OpenAI",
};

pub const GPT_4O: ModelConfig = ModelConfig {
    name: "gpt-4o",
    encoding: "o200k_base",
    context_window: 128_000,
    aliases: &["gpt4o", "openai/gpt-4o"],
    provider: "OpenAI",
};
```

**Validation**:
- `name` must be unique across all models
- `encoding` must be valid tiktoken encoding
- `context_window` must be > 0
- `aliases` must not conflict with other model names

**Relationships**:
- One `ModelConfig` → One `Tokenizer` implementation
- Multiple `aliases` → One canonical `ModelConfig`

---

### ModelRegistry (Model Lookup)

**Purpose**: Central registry for model resolution and listing.

**Definition**:
```rust
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub struct ModelRegistry {
    models: HashMap<&'static str, &'static ModelConfig>,
    aliases: HashMap<&'static str, &'static str>, // alias → canonical name
}

pub static REGISTRY: Lazy<ModelRegistry> = Lazy::new(|| {
    let mut registry = ModelRegistry {
        models: HashMap::new(),
        aliases: HashMap::new(),
    };
    
    // Register OpenAI models
    registry.register(&GPT_35_TURBO);
    registry.register(&GPT_4);
    registry.register(&GPT_4_TURBO);
    registry.register(&GPT_4O);
    
    registry
});

impl ModelRegistry {
    fn register(&mut self, config: &'static ModelConfig) {
        self.models.insert(config.name, config);
        
        for alias in config.aliases {
            self.aliases.insert(alias, config.name);
        }
    }
    
    pub fn resolve(&self, input: &str) -> Result<&'static ModelConfig, TokenError> {
        let normalized = input.to_lowercase().trim();
        
        // Check exact match
        if let Some(config) = self.models.get(normalized.as_str()) {
            return Ok(config);
        }
        
        // Check alias
        if let Some(canonical) = self.aliases.get(normalized.as_str()) {
            return Ok(self.models[canonical]);
        }
        
        // Not found - return error with suggestions
        Err(TokenError::UnknownModel {
            model: input.to_string(),
            suggestions: find_similar_models(normalized.as_str()),
        })
    }
    
    pub fn list_models(&self) -> Vec<&'static ModelConfig> {
        let mut models: Vec<_> = self.models.values().copied().collect();
        models.sort_by_key(|m| m.name);
        models
    }
}
```

**Methods**:
- `register(&mut self, config: &'static ModelConfig)` - Add model to registry
- `resolve(&self, input: &str) -> Result<&'static ModelConfig>` - Find model by name/alias
- `list_models(&self) -> Vec<&'static ModelConfig>` - Get all models sorted by name

**Validation**:
- Case-insensitive matching (normalize to lowercase)
- Exact match prioritized over aliases
- Returns error with suggestions if not found

---

## Tokenization Layer

### Tokenizer (Trait)

**Purpose**: Abstract interface for tokenization across providers.

**Definition**:
```rust
pub trait Tokenizer: Send + Sync {
    /// Encode text into token IDs
    fn encode(&self, text: &str) -> Result<Vec<u32>, TokenError>;
    
    /// Count tokens without returning IDs (potentially faster)
    fn count_tokens(&self, text: &str) -> Result<usize, TokenError> {
        Ok(self.encode(text)?.len())
    }
    
    /// Decode token IDs back to text
    fn decode(&self, tokens: &[u32]) -> Result<String, TokenError>;
    
    /// Get model configuration
    fn model_config(&self) -> &ModelConfig;
}
```

**Methods**:
- `encode(&self, text: &str) -> Result<Vec<u32>>` - Encode to token IDs
- `count_tokens(&self, text: &str) -> Result<usize>` - Count tokens (default impl)
- `decode(&self, tokens: &[u32]) -> Result<String>` - Decode token IDs
- `model_config(&self) -> &ModelConfig` - Get model metadata

**Implementations**:
- `OpenAITokenizer` (MVP) - Uses tiktoken-rs
- `ClaudeTokenizer` (Phase 2) - TBD (estimation or official tokenizer)
- `GeminiTokenizer` (Phase 2) - TBD (llm-tokenizer or estimation)

**Constraints**:
- `Send + Sync` required for thread-safety (future parallelization)
- `&str` parameter (zero-copy, no ownership transfer)
- Return `Result` for error handling (invalid input, encoding errors)

---

### OpenAITokenizer (Implementation)

**Purpose**: OpenAI tokenization using tiktoken-rs.

**Definition**:
```rust
use tiktoken_rs::{CoreBPE, get_bpe_from_model};

pub struct OpenAITokenizer {
    bpe: CoreBPE,
    config: &'static ModelConfig,
}

impl OpenAITokenizer {
    pub fn new(config: &'static ModelConfig) -> Result<Self, TokenError> {
        let bpe = get_bpe_from_model(config.name)
            .map_err(|e| TokenError::TokenizerInit(e.to_string()))?;
        
        Ok(Self { bpe, config })
    }
}

impl Tokenizer for OpenAITokenizer {
    fn encode(&self, text: &str) -> Result<Vec<u32>, TokenError> {
        self.bpe.encode_with_special_tokens(text)
            .map(|tokens| tokens.into_iter().map(|t| t as u32).collect())
            .map_err(|e| TokenError::Encoding(e.to_string()))
    }
    
    fn count_tokens(&self, text: &str) -> Result<usize, TokenError> {
        // Optimization: tiktoken-rs count is faster than encode + len
        Ok(self.bpe.encode_with_special_tokens(text)?.len())
    }
    
    fn decode(&self, tokens: &[u32]) -> Result<String, TokenError> {
        let tokens_usize: Vec<usize> = tokens.iter().map(|&t| t as usize).collect();
        self.bpe.decode(tokens_usize)
            .map_err(|e| TokenError::Decoding(e.to_string()))
    }
    
    fn model_config(&self) -> &ModelConfig {
        self.config
    }
}
```

**Fields**:
- `bpe`: CoreBPE - tiktoken-rs BPE encoder (lazy-loaded)
- `config`: &'static ModelConfig - Model metadata

**Initialization**:
- Lazy-load BPE encoder on first use (via `get_bpe_from_model`)
- Cache encoder in struct (reuse for multiple calls)

**Error Handling**:
- `TokenizerInit` - Failed to load BPE encoder (shouldn't happen with valid models)
- `Encoding` - Failed to encode text (invalid UTF-8 or encoding error)
- `Decoding` - Failed to decode tokens (invalid token IDs)

---

### TokenizationResult (Output Data)

**Purpose**: Encapsulate tokenization results for formatting.

**Definition**:
```rust
#[derive(Debug, Clone)]
pub struct TokenizationResult {
    /// Total number of tokens
    pub token_count: usize,
    
    /// Model configuration used
    pub model_config: &'static ModelConfig,
    
    /// Token IDs (only populated if verbosity >= 3)
    pub token_ids: Option<Vec<u32>>,
    
    /// Decoded tokens (only populated if verbosity >= 3)
    pub decoded_tokens: Option<Vec<String>>,
}

impl TokenizationResult {
    /// Calculate context window usage percentage
    pub fn context_usage_percent(&self) -> f64 {
        (self.token_count as f64 / self.model_config.context_window as f64) * 100.0
    }
    
    /// Get first N decoded tokens (for debug output)
    pub fn sample_tokens(&self, n: usize) -> Option<Vec<String>> {
        self.decoded_tokens.as_ref().map(|tokens| {
            tokens.iter().take(n).cloned().collect()
        })
    }
}
```

**Fields**:
- `token_count`: usize - Total tokens (always populated)
- `model_config`: &'static ModelConfig - Model metadata (always populated)
- `token_ids`: Option<Vec<u32>> - Token IDs (only if verbosity >= 3)
- `decoded_tokens`: Option<Vec<String>> - Decoded tokens (only if verbosity >= 3)

**Methods**:
- `context_usage_percent(&self) -> f64` - Calculate percentage of context window used
- `sample_tokens(&self, n: usize) -> Option<Vec<String>>` - Get first N tokens for display

**Lifecycle**:
1. Created after tokenization with minimal data
2. Enriched with token IDs/decoded tokens if verbosity >= 3
3. Passed to output formatter for display

---

## Output Layer

### OutputFormatter (Trait)

**Purpose**: Format tokenization results based on verbosity level.

**Definition**:
```rust
pub trait OutputFormatter {
    fn format(&self, result: &TokenizationResult) -> String;
}
```

**Implementations**:
- `SimpleFormatter` (verbosity 0) - Number only
- `VerboseFormatter` (verbosity 1-2) - Model info + context usage
- `DebugFormatter` (verbosity 3) - Token IDs + decoded tokens

---

### SimpleFormatter (Verbosity 0)

**Purpose**: Output only the token count (default behavior).

**Definition**:
```rust
pub struct SimpleFormatter;

impl OutputFormatter for SimpleFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        result.token_count.to_string()
    }
}
```

**Example Output**:
```
142
```

---

### VerboseFormatter (Verbosity 1-2)

**Purpose**: Output model info and optionally context usage.

**Definition**:
```rust
pub struct VerboseFormatter {
    pub include_context: bool, // true for verbosity 2, false for verbosity 1
}

impl OutputFormatter for VerboseFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        let config = result.model_config;
        
        let mut output = format!(
            "Model: {} ({} encoding)\nTokens: {}",
            config.name,
            config.encoding,
            result.token_count
        );
        
        if self.include_context {
            output.push_str(&format!(
                "\nContext Window: {:,} tokens\nUsage: {:.2}%",
                config.context_window,
                result.context_usage_percent()
            ));
        }
        
        output
    }
}
```

**Example Output (Verbosity 1)**:
```
Model: gpt-4 (cl100k_base encoding)
Tokens: 142
```

**Example Output (Verbosity 2)**:
```
Model: gpt-4 (cl100k_base encoding)
Tokens: 142
Context Window: 8,192 tokens
Usage: 1.73%
```

---

### DebugFormatter (Verbosity 3)

**Purpose**: Output token IDs and decoded tokens for debugging.

**Definition**:
```rust
pub struct DebugFormatter;

impl OutputFormatter for DebugFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        let config = result.model_config;
        
        let mut output = format!(
            "Model: {} ({} encoding)\nTokens: {}",
            config.name,
            config.encoding,
            result.token_count
        );
        
        // Add token IDs (first 10 only if > 10 tokens)
        if let Some(ids) = &result.token_ids {
            let sample_ids: Vec<String> = ids.iter()
                .take(10)
                .map(|id| id.to_string())
                .collect();
            
            let ids_display = if ids.len() > 10 {
                format!("[{}, ...]", sample_ids.join(", "))
            } else {
                format!("[{}]", sample_ids.join(", "))
            };
            
            output.push_str(&format!("\nToken IDs: {}", ids_display));
        }
        
        // Add decoded tokens (first 10 only)
        if let Some(tokens) = result.sample_tokens(10) {
            let tokens_json: Vec<String> = tokens.iter()
                .map(|t| format!("\"{}\"", t.escape_default()))
                .collect();
            
            let tokens_display = if result.token_count > 10 {
                format!("[{}, ...]", tokens_json.join(", "))
            } else {
                format!("[{}]", tokens_json.join(", "))
            };
            
            output.push_str(&format!("\nDecoded Tokens: {}", tokens_display));
        }
        
        // Add context window info
        output.push_str(&format!(
            "\nContext Window: {:,} tokens\nUsage: {:.2}%",
            config.context_window,
            result.context_usage_percent()
        ));
        
        output
    }
}
```

**Example Output**:
```
Model: gpt-4 (cl100k_base encoding)
Tokens: 2
Token IDs: [15339, 1917]
Decoded Tokens: ["Hello", " world"]
Context Window: 8,192 tokens
Usage: 0.02%
```

---

## Error Handling

### TokenError (Error Enum)

**Purpose**: All errors that can occur during tokenization.

**Definition**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Input contains invalid UTF-8\n\ntoken-count requires valid UTF-8 text input.\nBinary files cannot be tokenized.")]
    InvalidUtf8,
    
    #[error("Unknown model '{model}'\n\nDid you mean one of these?\n{suggestions}\n\nUse --list-models to see all supported models")]
    UnknownModel {
        model: String,
        suggestions: String,
    },
    
    #[error("Failed to initialize tokenizer: {0}")]
    TokenizerInit(String),
    
    #[error("Failed to encode text: {0}")]
    Encoding(String),
    
    #[error("Failed to decode tokens: {0}")]
    Decoding(String),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl TokenError {
    /// Map error to exit code (0 = success, 1 = runtime error, 2 = user error)
    pub fn exit_code(&self) -> i32 {
        match self {
            TokenError::InvalidUtf8 => 1,
            TokenError::UnknownModel { .. } => 2,
            TokenError::TokenizerInit(_) => 1,
            TokenError::Encoding(_) => 1,
            TokenError::Decoding(_) => 1,
            TokenError::Io(_) => 1,
        }
    }
}
```

**Variants**:
- `InvalidUtf8` - Input is not valid UTF-8 (exit code 1)
- `UnknownModel` - Model name not found in registry (exit code 2)
- `TokenizerInit` - Failed to load tokenizer (exit code 1)
- `Encoding` - Failed to encode text (exit code 1)
- `Decoding` - Failed to decode tokens (exit code 1)
- `Io` - I/O error reading stdin (exit code 1)

**Exit Code Mapping**:
- `0` - Success
- `1` - Runtime error (invalid UTF-8, I/O error, encoding error)
- `2` - User error (unknown model)

**Relationships**:
- Returned by all `Tokenizer` methods
- Converted to error messages in `main.rs`
- Exit code used for process termination

---

## Data Flow Diagram

```
[User Input]
     |
     | (stdin)
     v
[main.rs] ────> Parse CLI args (Cli struct)
     |
     ├──> list_models=true ──> Print MODEL_REGISTRY ──> Exit(0)
     |
     └──> list_models=false
          |
          v
     [read_stdin()] ────> Read stdin in 64KB chunks
          |                    |
          |                    v
          |               Validate UTF-8 ──> InvalidUtf8 ──> Exit(1)
          |                    |
          v                    v
     [resolve_model()] ───> ModelRegistry::resolve()
          |                    |
          |                    v
          |               UnknownModel ──> Suggestions ──> Exit(2)
          |                    |
          v                    v
     [create_tokenizer()] ──> OpenAITokenizer::new()
          |                    |
          v                    v
     [tokenize()] ────────> Tokenizer::encode()
          |                    |
          v                    v
     [TokenizationResult] ──> format_output()
          |                    |
          v                    v
     [OutputFormatter] ───> SimpleFormatter / VerboseFormatter / DebugFormatter
          |                    |
          v                    v
     [Print to stdout] ───> Exit(0)
```

---

## Validation Rules Summary

### CLI Arguments
- `model`: Non-empty string (clap validates)
- `verbosity`: 0-255 (u8), capped at 3 in code
- `list_models`: Boolean flag

### Model Registry
- Model names must be unique
- Aliases must not conflict with model names
- Encoding names must be valid tiktoken encodings

### Tokenization
- Input must be valid UTF-8 (BufReader validates)
- Empty input returns 0 tokens (not an error)
- Token IDs must be decodable (tiktoken validates)

### Output
- Token count must be non-negative (usize guarantees)
- Context usage percentage must be 0-100% (calculated)
- Sample tokens limited to first 10 (prevent huge output)

---

## Testing Strategy

### Unit Tests
- `ModelRegistry::resolve()` with exact names, aliases, unknown models
- `OpenAITokenizer::encode()` with ASCII, Unicode, empty input
- `OutputFormatter` implementations with sample data
- `TokenError::exit_code()` mapping

### Integration Tests
- CLI argument parsing (valid/invalid flags)
- Stdin reading with ASCII, Unicode, empty, large files
- Error handling (invalid UTF-8, unknown model)
- Output formatting at all verbosity levels

### Property Tests (Optional)
- Encode then decode should be identity (lossy for some models)
- Token count should equal encode().len()
- Context usage percentage should be 0-100%

---

**Data Model Version**: 1.0 | **Last Updated**: 2026-03-13
