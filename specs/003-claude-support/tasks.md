# Implementation Tasks: Claude Model Support

**Feature**: 003-claude-support  
**Branch**: `003-claude-support`  
**Estimated Effort**: 5 days  
**Date Created**: 2026-03-14

This document breaks down the implementation into concrete, ordered tasks following TDD principles.

---

## Task Key

- `[ ]` - Not started
- `[~]` - In progress
- `[✓]` - Complete
- `[P]` - Parallel-safe (can be done alongside other [P] tasks)
- `→` - Depends on previous task

---

## Phase 1: Foundation (Days 1-2)

### Task 1.1: Extend Error Types for Claude
**File**: `src/error.rs`  
**Depends on**: None  
**Status**: `[ ]`

Add Claude-specific error variants to `TokenError`:

```rust
#[error("Accurate mode requires ANTHROPIC_API_KEY environment variable...")]
MissingApiKey { model: String },

#[error("Invalid ANTHROPIC_API_KEY...")]
InvalidApiKey,

#[error("Anthropic API rate limit exceeded...")]
RateLimited,

#[error("Anthropic API server error (HTTP {0})...")]
ApiServerError(u16),

#[error("Anthropic API error: {0}")]
ApiError(String),

#[error("API call requires consent. Running in non-interactive mode...")]
NonInteractiveWithoutYes { model: String },
```

**Validation**:
- [x] Compiles without errors
- [x] Error messages include helpful context
- [x] All error variants implement Display trait

---

### Task 1.2: Add CLI Flags
**File**: `src/cli/args.rs`  
**Depends on**: None  
**Status**: `[ ]` `[P]`

Add new flags to `Cli` struct:

```rust
/// Use API for exact token counts (requires ANTHROPIC_API_KEY for Claude models)
#[arg(long)]
pub accurate: bool,

/// Skip API consent prompt (for scripting/automation, requires --accurate)
#[arg(short = 'y', long)]
pub yes: bool,
```

**Validation**:
- [x] `--accurate` flag recognized
- [x] `-y/--yes` flag recognized
- [x] Flags work in combination: `--accurate -y`
- [x] Help text displays correctly: `token-count --help`

**Tests**:
```rust
#[test]
fn test_cli_accurate_flag() {
    let args = Cli::parse_from(&["token-count", "--model", "claude", "--accurate"]);
    assert!(args.accurate);
}

#[test]
fn test_cli_yes_flag() {
    let args = Cli::parse_from(&["token-count", "--model", "claude", "--accurate", "-y"]);
    assert!(args.yes);
}
```

---

### Task 1.3: Create TokenCount Enum
**File**: `src/tokenizers/mod.rs`  
**Depends on**: None  
**Status**: `[ ]` `[P]`

Create enum to distinguish estimated vs. exact counts:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenCount {
    Estimated(usize),
    Exact(usize),
}

impl TokenCount {
    pub fn value(&self) -> usize { /* ... */ }
    pub fn is_estimated(&self) -> bool { /* ... */ }
    pub fn is_exact(&self) -> bool { /* ... */ }
}

impl Display for TokenCount { /* ... */ }
```

**Validation**:
- [x] Enum compiles without errors
- [x] Display trait shows `~42` for Estimated
- [x] Display trait shows `42` for Exact
- [x] Helper methods work correctly

**Tests**:
```rust
#[test]
fn test_token_count_display() {
    assert_eq!(format!("{}", TokenCount::Estimated(42)), "~42");
    assert_eq!(format!("{}", TokenCount::Exact(42)), "42");
}

#[test]
fn test_token_count_value() {
    assert_eq!(TokenCount::Estimated(42).value(), 42);
    assert_eq!(TokenCount::Exact(42).value(), 42);
}
```

---

### Task 1.4: Create API Module Structure
**Files**: `src/api/mod.rs`, `src/api/consent.rs`  
**Depends on**: Task 1.1 (error types)  
**Status**: `[ ]`

Create API utilities module for consent prompt:

**src/api/mod.rs**:
```rust
pub mod consent;
pub use consent::ConsentPrompt;
```

**src/api/consent.rs** (stub):
```rust
pub struct ConsentPrompt {
    pub provider: &'static str,
    pub api_endpoint: &'static str,
}

impl ConsentPrompt {
    pub fn ask(&self) -> Result<bool> {
        // TODO: Implement in Task 1.5
        unimplemented!()
    }
}
```

**Validation**:
- [x] Module structure compiles
- [x] `use crate::api::ConsentPrompt` works from other files

---

### Task 1.5: Implement Consent Prompt
**File**: `src/api/consent.rs`  
**Depends on**: Task 1.4  
**Status**: `[ ]`

Implement interactive consent prompt with TTY detection:

```rust
impl ConsentPrompt {
    pub fn ask(&self) -> Result<bool> {
        // Check TTY
        if !std::io::stdin().is_terminal() {
            return Err(TokenError::NonInteractiveWithoutYes { 
                model: "claude".to_string() 
            });
        }
        
        // Display prompt on stderr
        eprintln!("\nThis will send your input to {}'s API...", self.provider);
        eprintln!("Your input will be transmitted over HTTPS to: {}", self.api_endpoint);
        eprintln!();
        eprint!("Proceed with API call? (y/N): ");
        std::io::stderr().flush()?;
        
        // Read response
        let mut response = String::new();
        std::io::stdin().read_line(&mut response)?;
        
        let normalized = response.trim().to_lowercase();
        Ok(normalized == "y" || normalized == "yes")
    }
}
```

**Validation**:
- [x] Interactive mode shows prompt on stderr
- [x] Accepts 'y', 'Y', 'yes', 'YES'
- [x] Rejects 'n', 'N', 'no', 'NO', empty string
- [x] Non-interactive (piped stdin) returns error

**Tests**:
```rust
#[test]
fn test_consent_prompt_accept() {
    // Mock stdin with 'y\n'
    // Assert returns Ok(true)
}

#[test]
fn test_consent_prompt_reject() {
    // Mock stdin with 'n\n'
    // Assert returns Ok(false)
}

#[test]
fn test_consent_prompt_non_interactive() {
    // Mock non-TTY stdin
    // Assert returns Err(NonInteractiveWithoutYes)
}
```

---

## Phase 2: Estimation Algorithm (Day 2)

### Task 2.1: Create Claude Module Structure
**Files**: `src/tokenizers/claude/mod.rs`, `src/tokenizers/claude/estimation.rs`, `src/tokenizers/claude/models.rs`  
**Depends on**: Task 1.3 (TokenCount enum)  
**Status**: `[ ]`

Create Claude tokenizer module:

**src/tokenizers/claude/mod.rs**:
```rust
mod estimation;
mod models;

pub use estimation::{estimate_tokens, detect_content_type, ContentType};
pub use models::CLAUDE_MODELS;

pub struct ClaudeTokenizer {
    config: ModelConfig,
    use_accurate: bool,
}

impl ClaudeTokenizer {
    pub fn new(config: ModelConfig, use_accurate: bool) -> Result<Self> {
        Ok(Self { config, use_accurate })
    }
}
```

**Validation**:
- [x] Module compiles
- [x] Exports are accessible from parent

---

### Task 2.2: Implement ContentType Detection
**File**: `src/tokenizers/claude/estimation.rs`  
**Depends on**: Task 2.1  
**Status**: `[ ]`

Implement content type classification:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Code,   // >15% code indicators
    Prose,  // <5% code indicators
    Mixed,  // 5-15% code indicators
}

impl ContentType {
    pub fn chars_per_token(&self) -> f64 {
        match self {
            Self::Code => 3.0,
            Self::Prose => 4.5,
            Self::Mixed => 3.75,
        }
    }
}

pub fn detect_content_type(text: &str) -> ContentType {
    let total_chars = text.chars().count();
    if total_chars == 0 {
        return ContentType::Prose;
    }
    
    let code_indicators = count_code_indicators(text);
    let ratio = code_indicators as f64 / total_chars as f64;
    
    if ratio > 0.15 {
        ContentType::Code
    } else if ratio > 0.05 {
        ContentType::Mixed
    } else {
        ContentType::Prose
    }
}

fn count_code_indicators(text: &str) -> usize {
    // Count: {, }, [, ], (, ), ;, //, fn, def, const, let, var, etc.
    // TODO: Implement indicator counting
}
```

**Validation**:
- [x] Pure code input → ContentType::Code
- [x] Pure prose input → ContentType::Prose
- [x] Mixed content → ContentType::Mixed
- [x] Empty string handled gracefully

**Tests**:
```rust
#[test]
fn test_detect_code() {
    let code = "fn main() { println!(\"test\"); }";
    assert_eq!(detect_content_type(code), ContentType::Code);
}

#[test]
fn test_detect_prose() {
    let prose = "The quick brown fox jumps over the lazy dog.";
    assert_eq!(detect_content_type(prose), ContentType::Prose);
}

#[test]
fn test_detect_mixed() {
    let mixed = "## Title\n\n```rust\nfn test() {}\n```\n\nSome text.";
    assert_eq!(detect_content_type(mixed), ContentType::Mixed);
}
```

---

### Task 2.3: Implement Token Estimation
**File**: `src/tokenizers/claude/estimation.rs`  
**Depends on**: Task 2.2  
**Status**: `[ ]`

Implement adaptive token estimation:

```rust
pub fn estimate_tokens(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    
    let content_type = detect_content_type(text);
    let char_count = text.chars().count();
    let ratio = content_type.chars_per_token();
    
    (char_count as f64 / ratio).ceil() as usize
}
```

**Validation**:
- [x] Empty string → 0 tokens
- [x] Single character → 1 token
- [x] Code uses 3.0 ratio
- [x] Prose uses 4.5 ratio
- [x] Mixed uses 3.75 ratio

**Tests**:
```rust
#[test]
fn test_estimate_empty() {
    assert_eq!(estimate_tokens(""), 0);
}

#[test]
fn test_estimate_code() {
    let code = "fn main() {}";  // 12 chars ÷ 3.0 = 4 tokens
    let tokens = estimate_tokens(code);
    assert_eq!(tokens, 4);
}

#[test]
fn test_estimate_prose() {
    let prose = "Hello world!";  // 12 chars ÷ 4.5 = 3 tokens (rounded up)
    let tokens = estimate_tokens(prose);
    assert_eq!(tokens, 3);
}
```

---

### Task 2.4: Add ClaudeTokenizer Trait Implementation (Estimation Only)
**File**: `src/tokenizers/claude/mod.rs`  
**Depends on**: Task 2.3  
**Status**: `[ ]`

Implement Tokenizer trait for Claude (estimation mode only):

```rust
impl Tokenizer for ClaudeTokenizer {
    fn count_tokens(&self, text: &str) -> Result<usize> {
        // For now, only estimation (API client added in Phase 3)
        Ok(estimate_tokens(text))
    }
    
    fn get_model_info(&self) -> ModelInfo {
        ModelInfo {
            name: self.config.name.clone(),
            encoding: "anthropic-claude".to_string(),
            context_window: self.config.context_window,
            description: self.config.description.clone(),
        }
    }
}
```

**Validation**:
- [x] Implements Tokenizer trait
- [x] Returns estimated token count
- [x] Model info correct

---

## Phase 3: API Client (Day 3)

### Task 3.1: Add Dependencies
**File**: `Cargo.toml`  
**Depends on**: None  
**Status**: `[ ]`

Add new dependencies:

```toml
[dependencies]
# New for Claude API
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
tokio = { version = "1", features = ["rt", "macros"] }
serde = { version = "1.0.149", features = ["derive"] }
serde_json = "1.0.149"

[dev-dependencies]
mockito = "1.0"
```

**Validation**:
- [x] `cargo build` succeeds
- [x] Dependencies resolve without conflicts

---

### Task 3.2: Create API Client Structure
**File**: `src/tokenizers/claude/api_client.rs`  
**Depends on**: Task 3.1, Task 1.1 (error types)  
**Status**: `[ ]`

Create API client stub:

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct CountTokensRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct CountTokensResponse {
    input_tokens: usize,
}

pub struct ClaudeApiClient {
    client: Client,
    api_key: String,
}

impl ClaudeApiClient {
    const API_ENDPOINT: &'static str = "https://api.anthropic.com/v1/messages/count_tokens";
    const API_VERSION: &'static str = "2023-06-01";
    const TIMEOUT_SECS: u64 = 30;
    
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(Self::TIMEOUT_SECS))
            .user_agent(format!("token-count/{}", env!("CARGO_PKG_VERSION")))
            .build()?;
        
        Ok(Self { client, api_key })
    }
}
```

**Validation**:
- [x] Compiles without errors
- [x] Structs serialize/deserialize correctly

**Tests**:
```rust
#[test]
fn test_serialize_request() {
    let req = CountTokensRequest {
        model: "claude-sonnet-4-6".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }],
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("claude-sonnet-4-6"));
}

#[test]
fn test_deserialize_response() {
    let json = r#"{"input_tokens": 42}"#;
    let resp: CountTokensResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.input_tokens, 42);
}
```

---

### Task 3.3: Implement API Call with Retry Logic
**File**: `src/tokenizers/claude/api_client.rs`  
**Depends on**: Task 3.2  
**Status**: `[ ]`

Implement token counting with retry:

```rust
impl ClaudeApiClient {
    const MAX_RETRIES: u32 = 3;
    
    pub async fn count_tokens(&self, model: &str, text: &str) -> Result<usize> {
        let mut attempts = 0;
        
        while attempts < Self::MAX_RETRIES {
            match self.try_count_tokens(model, text).await {
                Ok(count) => return Ok(count),
                Err(e) if attempts < Self::MAX_RETRIES - 1 => {
                    let backoff_ms = 2u64.pow(attempts) * 1000; // 2s, 4s, 8s
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    attempts += 1;
                }
                Err(e) => return Err(e),
            }
        }
        
        unreachable!()
    }
    
    async fn try_count_tokens(&self, model: &str, text: &str) -> Result<usize> {
        let request = CountTokensRequest {
            model: model.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: text.to_string(),
            }],
        };
        
        let response = self.client
            .post(Self::API_ENDPOINT)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", Self::API_VERSION)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(Self::parse_api_error(response).await);
        }
        
        let body: CountTokensResponse = response.json().await?;
        Ok(body.input_tokens)
    }
}
```

**Validation**:
- [x] Successful response returns token count
- [x] Network error triggers retry
- [x] 3 retries max with exponential backoff
- [x] Timeout after 30 seconds

---

### Task 3.4: Implement API Error Parsing
**File**: `src/tokenizers/claude/api_client.rs`  
**Depends on**: Task 3.3  
**Status**: `[ ]`

Parse API error responses:

```rust
impl ClaudeApiClient {
    async fn parse_api_error(response: reqwest::Response) -> TokenError {
        let status = response.status();
        
        match status.as_u16() {
            401 => TokenError::InvalidApiKey,
            429 => TokenError::RateLimited,
            500..=599 => TokenError::ApiServerError(status.as_u16()),
            _ => TokenError::ApiError(format!("HTTP {}", status)),
        }
    }
}
```

**Validation**:
- [x] 401 → InvalidApiKey
- [x] 429 → RateLimited
- [x] 5xx → ApiServerError
- [x] Other → ApiError

**Tests**:
```rust
#[test]
fn test_parse_401() {
    // Mock response with 401 status
    // Assert returns InvalidApiKey
}

#[test]
fn test_parse_429() {
    // Mock response with 429 status
    // Assert returns RateLimited
}
```

---

### Task 3.5: Add API Client to ClaudeTokenizer
**File**: `src/tokenizers/claude/mod.rs`  
**Depends on**: Task 3.4, Task 2.4  
**Status**: `[ ]`

Integrate API client with tokenizer:

```rust
pub struct ClaudeTokenizer {
    config: ModelConfig,
    api_client: Option<ClaudeApiClient>,
    use_accurate: bool,
}

impl ClaudeTokenizer {
    pub fn new(config: ModelConfig, use_accurate: bool) -> Result<Self> {
        let api_client = if use_accurate {
            match env::var("ANTHROPIC_API_KEY") {
                Ok(key) if !key.is_empty() => Some(ClaudeApiClient::new(key)?),
                _ => return Err(TokenError::MissingApiKey { 
                    model: config.name.clone() 
                }),
            }
        } else {
            None
        };
        
        Ok(Self { config, api_client, use_accurate })
    }
    
    pub async fn count_tokens_async(&self, text: &str) -> Result<TokenCount> {
        if let Some(client) = &self.api_client {
            // Try API, fall back to estimation on error
            match client.count_tokens(&self.config.name, text).await {
                Ok(count) => Ok(TokenCount::Exact(count)),
                Err(e) => {
                    eprintln!("Warning: API call failed ({}), falling back to estimation", e);
                    Ok(TokenCount::Estimated(estimate_tokens(text)))
                }
            }
        } else {
            Ok(TokenCount::Estimated(estimate_tokens(text)))
        }
    }
}
```

**Validation**:
- [x] `use_accurate=false` → estimation only
- [x] `use_accurate=true` + no API key → error
- [x] `use_accurate=true` + API key → creates client
- [x] API error → falls back to estimation

---

## Phase 4: Integration (Day 4)

### Task 4.1: Define Claude Models
**File**: `src/tokenizers/claude/models.rs`  
**Depends on**: None  
**Status**: `[ ]` `[P]`

Define all Claude model configurations:

```rust
use crate::tokenizers::registry::ModelConfig;

pub fn claude_models() -> Vec<ModelConfig> {
    vec![
        // Claude 4.6
        ModelConfig {
            name: "claude-opus-4-6".to_string(),
            encoding: "anthropic-claude".to_string(),
            context_window: 1_000_000,
            description: "Claude Opus 4.6 (1M context)".to_string(),
            aliases: vec![
                "opus-4-6".to_string(),
                "opus".to_string(),
                "anthropic/claude-opus-4-6".to_string(),
            ],
        },
        ModelConfig {
            name: "claude-sonnet-4-6".to_string(),
            encoding: "anthropic-claude".to_string(),
            context_window: 1_000_000,
            description: "Claude Sonnet 4.6 (1M context)".to_string(),
            aliases: vec![
                "sonnet-4-6".to_string(),
                "sonnet".to_string(),
                "claude".to_string(),
                "anthropic/claude-sonnet-4-6".to_string(),
            ],
        },
        // ... more models (4.5, 4.1, 4.0)
    ]
}
```

**Validation**:
- [x] All 8 models defined
- [x] Aliases correct
- [x] Context windows match Anthropic docs

---

### Task 4.2: Register Claude Models
**File**: `src/tokenizers/registry.rs`  
**Depends on**: Task 4.1  
**Status**: `[ ]`

Add Claude models to ModelRegistry:

```rust
impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();
        
        // [Existing OpenAI models...]
        
        // Add Claude models
        for model in claude_models() {
            registry.add_model(model);
        }
        
        registry
    }
}
```

**Validation**:
- [x] `--list-models` includes Claude models
- [x] Alias resolution works: `claude` → `claude-sonnet-4-6`
- [x] Fuzzy matching works: `claude-sonet` → suggestions

**Tests**:
```rust
#[test]
fn test_claude_alias_resolution() {
    let registry = ModelRegistry::new();
    assert_eq!(
        registry.resolve_model_name("claude").unwrap(),
        "claude-sonnet-4-6"
    );
}

#[test]
fn test_claude_fuzzy_match() {
    let registry = ModelRegistry::new();
    let result = registry.resolve_model_name("claude-sonet");
    assert!(result.is_err());
    // Should suggest "claude-sonnet-4-6"
}
```

---

### Task 4.3: Update Tokenizer Factory
**File**: `src/tokenizers/registry.rs`  
**Depends on**: Task 3.5, Task 4.2  
**Status**: `[ ]`

Extend `get_tokenizer()` to create ClaudeTokenizer:

```rust
impl ModelRegistry {
    pub fn get_tokenizer(&self, model_name: &str, use_accurate: bool) -> Result<Box<dyn Tokenizer>> {
        let canonical = self.resolve_model_name(model_name)?;
        let config = self.get_config(&canonical)?;
        
        match config.encoding.as_str() {
            "cl100k_base" | "o200k_base" => {
                Ok(Box::new(OpenAITokenizer::new(&config.encoding)?))
            }
            "anthropic-claude" => {
                Ok(Box::new(ClaudeTokenizer::new(config.clone(), use_accurate)?))
            }
            _ => Err(TokenError::UnsupportedEncoding(config.encoding.clone())),
        }
    }
}
```

**Validation**:
- [x] OpenAI models still work
- [x] Claude models create ClaudeTokenizer
- [x] Unknown encoding returns error

---

### Task 4.4: Update Main CLI Flow
**File**: `src/main.rs`  
**Depends on**: Task 1.5 (consent), Task 4.3  
**Status**: `[ ]`

Integrate consent prompt and accurate mode:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    
    // Check if API consent needed
    if args.accurate && !args.yes {
        // Check if this is a Claude model (requires consent)
        let is_claude = /* check model encoding */;
        
        if is_claude {
            let consent = ConsentPrompt {
                provider: "Anthropic",
                api_endpoint: "https://api.anthropic.com",
            };
            
            if !consent.ask()? {
                eprintln!("Falling back to estimation (API call cancelled by user)\n");
                args.accurate = false; // Use estimation instead
            }
        }
    }
    
    // Get tokenizer
    let tokenizer = registry.get_tokenizer(&args.model, args.accurate)?;
    
    // Read input
    let input = read_stdin()?;
    
    // Count tokens (async if API mode)
    let count = if args.accurate {
        tokenizer.count_tokens_async(&input).await?
    } else {
        TokenCount::Estimated(tokenizer.count_tokens(&input)?)
    };
    
    // Output
    println!("{}", count);
    
    Ok(())
}
```

**Validation**:
- [x] Consent prompt appears for Claude + `--accurate`
- [x] No prompt with `-y` flag
- [x] Estimation works without API key
- [x] API mode works with key + consent

---

### Task 4.5: Update Simple Output Formatter
**File**: `src/output/simple.rs`  
**Depends on**: Task 1.3 (TokenCount enum)  
**Status**: `[ ]`

Handle `~` prefix for estimates:

```rust
pub fn format_simple(count: TokenCount) -> String {
    // TokenCount::Display already handles ~ prefix
    format!("{}", count)
}
```

**Validation**:
- [x] Estimated → `~42`
- [x] Exact → `42`

**Tests**:
```rust
#[test]
fn test_format_estimated() {
    let output = format_simple(TokenCount::Estimated(42));
    assert_eq!(output, "~42");
}

#[test]
fn test_format_exact() {
    let output = format_simple(TokenCount::Exact(42));
    assert_eq!(output, "42");
}
```

---

### Task 4.6: Update Verbose Output Formatter
**File**: `src/output/verbose.rs`  
**Depends on**: Task 1.3  
**Status**: `[ ]`

Show estimation method in verbose mode:

```rust
pub fn format_verbose(count: TokenCount, model_info: ModelInfo) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("Model: {}\n", model_info.name));
    
    match count {
        TokenCount::Estimated(n) => {
            output.push_str(&format!("Tokens: ~{} (estimated)\n", n));
            output.push_str("Estimation method: Adaptive character-based heuristic\n");
            output.push_str("Accuracy: ±10% target from actual count\n");
            output.push_str(&format!("Context window: {} tokens\n", model_info.context_window));
            output.push_str("\nFor exact count, use: --accurate (requires ANTHROPIC_API_KEY)\n");
        }
        TokenCount::Exact(n) => {
            output.push_str(&format!("Tokens: {} (exact via Anthropic API)\n", n));
            output.push_str(&format!("Context window: {} tokens\n", model_info.context_window));
        }
    }
    
    output
}
```

**Validation**:
- [x] Estimated shows method and accuracy
- [x] Exact shows API confirmation
- [x] Context window always shown

---

## Phase 5: Testing & Validation (Day 5)

### Task 5.1: Integration Test - Estimation Mode
**File**: `tests/integration/claude_estimation.rs`  
**Depends on**: Task 4.4  
**Status**: `[ ]`

Test CLI estimation mode:

```rust
#[test]
fn test_claude_estimation_simple() {
    let output = run_cli(&["--model", "claude-sonnet-4-6"], "Hello, world!");
    assert!(output.starts_with("~"));
    let tokens: usize = output.trim_start_matches('~').parse().unwrap();
    assert!(tokens > 0 && tokens < 10);
}

#[test]
fn test_claude_estimation_verbose() {
    let output = run_cli(&["--model", "claude", "-v"], "Hello, world!");
    assert!(output.contains("estimated"));
    assert!(output.contains("Estimation method"));
}

#[test]
fn test_claude_empty_input() {
    let output = run_cli(&["--model", "claude"], "");
    assert_eq!(output.trim(), "0");
}
```

---

### Task 5.2: Integration Test - API Mode
**File**: `tests/integration/claude_api.rs`  
**Depends on**: Task 4.4  
**Status**: `[ ]`

Test API mode with mocked responses:

```rust
#[tokio::test]
async fn test_api_mode_missing_key() {
    env::remove_var("ANTHROPIC_API_KEY");
    let result = run_cli_expect_error(&["--model", "claude", "--accurate"], "test");
    assert!(result.contains("requires ANTHROPIC_API_KEY"));
}

#[tokio::test]
async fn test_api_mode_with_mock() {
    let mut server = mockito::Server::new_async().await;
    
    let mock = server.mock("POST", "/v1/messages/count_tokens")
        .with_status(200)
        .with_body(r#"{"input_tokens": 3}"#)
        .create_async()
        .await;
    
    env::set_var("ANTHROPIC_API_KEY", "sk-test");
    let output = run_cli(&["--model", "claude", "--accurate", "-y"], "Hello");
    assert_eq!(output.trim(), "3");
    
    mock.assert_async().await;
}
```

---

### Task 5.3: Integration Test - Consent Prompt
**File**: `tests/integration/consent_prompt.rs`  
**Depends on**: Task 4.4  
**Status**: `[ ]`

Test consent prompt behavior:

```rust
#[test]
fn test_consent_non_interactive_without_yes() {
    env::set_var("ANTHROPIC_API_KEY", "sk-test");
    let result = pipe_stdin_expect_error(&["--model", "claude", "--accurate"], "test");
    assert!(result.contains("Non-interactive mode"));
    assert!(result.contains("-y/--yes"));
}

#[test]
fn test_consent_with_yes_flag() {
    // Mock API to return success
    env::set_var("ANTHROPIC_API_KEY", "sk-test");
    let output = run_cli(&["--model", "claude", "--accurate", "-y"], "test");
    // Should not prompt, should call API directly
    assert!(!output.contains("Proceed with API call"));
}
```

---

### Task 5.4: Unit Test - Content Type Detection
**File**: `tests/unit/adaptive_estimation.rs`  
**Depends on**: Task 2.2  
**Status**: `[ ]` `[P]`

Test content type detection accuracy:

```rust
#[test]
fn test_detect_rust_code() {
    let code = r#"
        fn main() {
            println!("Hello");
        }
    "#;
    assert_eq!(detect_content_type(code), ContentType::Code);
}

#[test]
fn test_detect_json() {
    let json = r#"{"key": "value", "count": 42}"#;
    assert_eq!(detect_content_type(json), ContentType::Code);
}

#[test]
fn test_detect_markdown_mixed() {
    let md = "## Title\n\nSome text\n\n```rust\nfn test() {}\n```";
    assert_eq!(detect_content_type(md), ContentType::Mixed);
}

#[test]
fn test_detect_plain_text() {
    let text = "The quick brown fox jumps over the lazy dog.";
    assert_eq!(detect_content_type(text), ContentType::Prose);
}
```

---

### Task 5.5: Unit Test - Model Aliases
**File**: `tests/unit/model_aliases.rs`  
**Depends on**: Task 4.2  
**Status**: `[ ]` `[P]`

Test Claude model alias resolution:

```rust
#[test]
fn test_opus_alias() {
    let registry = ModelRegistry::new();
    assert_eq!(registry.resolve_model_name("opus").unwrap(), "claude-opus-4-6");
}

#[test]
fn test_sonnet_alias() {
    let registry = ModelRegistry::new();
    assert_eq!(registry.resolve_model_name("sonnet").unwrap(), "claude-sonnet-4-6");
}

#[test]
fn test_claude_default_alias() {
    let registry = ModelRegistry::new();
    assert_eq!(registry.resolve_model_name("claude").unwrap(), "claude-sonnet-4-6");
}

#[test]
fn test_provider_prefix() {
    let registry = ModelRegistry::new();
    assert_eq!(
        registry.resolve_model_name("anthropic/claude-opus-4-6").unwrap(),
        "claude-opus-4-6"
    );
}
```

---

### Task 5.6: Create Validation Script
**File**: `scripts/validate-claude-accuracy.sh`  
**Depends on**: Task 4.4  
**Status**: `[ ]`

Create script to compare estimation vs. API:

```bash
#!/bin/bash
# Validates estimation accuracy against Anthropic API
# Requires ANTHROPIC_API_KEY

set -e

if [ -z "$ANTHROPIC_API_KEY" ]; then
    echo "Error: ANTHROPIC_API_KEY required"
    exit 1
fi

BINARY="${1:-./target/release/token-count}"
RESULTS_FILE="validation-results.csv"

# Test inputs (code, prose, mixed)
declare -a INPUTS=(
    "Hello, world!"
    "fn main() { println!(\"test\"); }"
    "The quick brown fox jumps over the lazy dog."
    # ... 100+ more diverse inputs
)

echo "input,estimated,exact,error_pct" > "$RESULTS_FILE"

for input in "${INPUTS[@]}"; do
    estimated=$(echo "$input" | "$BINARY" --model claude-sonnet-4-6 | tr -d '~')
    exact=$(echo "$input" | "$BINARY" --model claude-sonnet-4-6 --accurate -y)
    
    error=$(echo "scale=2; 100 * ($estimated - $exact) / $exact" | bc)
    
    echo "\"$input\",$estimated,$exact,$error" >> "$RESULTS_FILE"
done

# Calculate statistics
echo ""
echo "=== Validation Results ==="
python3 <<EOF
import pandas as pd
df = pd.read_csv('$RESULTS_FILE')
print(f"Mean error: {df['error_pct'].mean():.1f}%")
print(f"Median error: {df['error_pct'].median():.1f}%")
print(f"95th percentile: {df['error_pct'].quantile(0.95):.1f}%")
print(f"Max error: {df['error_pct'].abs().max():.1f}%")
EOF
```

**Validation**:
- [x] Script runs without errors
- [x] Outputs CSV with results
- [x] Calculates accuracy metrics
- [x] Mean error ≤15%

---

### Task 5.7: Update Documentation
**File**: `README.md`, `CHANGELOG.md`  
**Depends on**: All previous tasks  
**Status**: `[ ]`

Update README with Claude examples:

```markdown
## Supported Models

### Anthropic Claude
- claude-opus-4-6 (aliases: opus-4-6, opus)
- claude-sonnet-4-6 (aliases: sonnet-4-6, sonnet, claude)
- claude-haiku-4-5 (aliases: haiku-4-5, haiku)
- ... [more models]

### Examples

# Offline estimation (no API key needed)
echo "Hello, Claude!" | token-count --model claude
~4

# Exact count via API (requires ANTHROPIC_API_KEY)
export ANTHROPIC_API_KEY="sk-ant-..."
echo "Hello, Claude!" | token-count --model claude --accurate
3
```

Update CHANGELOG:

```markdown
## [0.2.0] - 2026-03-XX

### Added
- Claude model support (Opus, Sonnet, Haiku 4.6/4.5/4.1/4.0)
- Adaptive token estimation for offline use (±10% accuracy target)
- Optional accurate mode via Anthropic API (--accurate flag)
- Interactive consent prompt for API calls
- Distinction between estimated (~42) and exact (42) counts

### Changed
- Extended model registry with Claude models
- Added --accurate and -y/--yes CLI flags
```

---

## Pre-Commit Verification Checklist

Before committing ANY changes, complete this checklist:

- [ ] `cargo test` - All tests pass
- [ ] `cargo clippy -- -D warnings` - Zero warnings
- [ ] `cargo fmt --check` - Code formatted
- [ ] `cargo build --release` - Release build succeeds
- [ ] Manual test: `echo "test" | ./target/release/token-count --model claude`
- [ ] Manual test: `echo "test" | ./target/release/token-count --model claude-sonnet-4-6 -v`
- [ ] No debug code (println!, dbg!, TODO comments) in committed code
- [ ] All error messages user-friendly and helpful
- [ ] ANTHROPIC_API_KEY never logged or exposed

---

## Success Criteria (Final Review)

### Must Have (MVP)
- [ ] All 8 Claude models supported
- [ ] Offline estimation works without API key
- [ ] `--accurate` mode uses Anthropic API with consent
- [ ] Clear visual distinction: `~42` vs `42`
- [ ] Graceful fallback on API failure
- [ ] Non-interactive mode requires `-y` flag
- [ ] Zero clippy warnings
- [ ] ≥80% test coverage for new code
- [ ] All existing OpenAI tests still pass
- [ ] Documentation updated

### Should Have
- [ ] Validation script with accuracy metrics
- [ ] Fuzzy model name suggestions
- [ ] Comprehensive error messages
- [ ] TTY detection for consent prompt

---

## Estimated Timeline

- **Phase 1 (Foundation)**: 1.5 days - Tasks 1.1-1.5
- **Phase 2 (Estimation)**: 0.5 days - Tasks 2.1-2.4
- **Phase 3 (API Client)**: 1 day - Tasks 3.1-3.5
- **Phase 4 (Integration)**: 1 day - Tasks 4.1-4.6
- **Phase 5 (Testing)**: 1 day - Tasks 5.1-5.7

**Total**: 5 days

---

## Notes

- Tasks marked `[P]` can be done in parallel
- All other tasks should be done sequentially
- TDD approach: Write tests BEFORE implementation
- Commit after each completed phase (not individual tasks)
- Run pre-commit checklist before every commit
- If any test fails, DO NOT commit - fix first

---

**Status**: ⏭️ Ready to begin implementation  
**Next Action**: Start Task 1.1 (Extend Error Types)
