# Data Model: Claude Model Support

**Feature**: 003-claude-support  
**Date**: 2026-03-14

This document defines all data structures, types, and their relationships for Claude tokenization support.

---

## Core Entities

### 1. TokenCount (Enum)

**Purpose**: Distinguish between estimated and exact token counts at the type level

**Location**: `src/tokenizers/mod.rs`

```rust
/// Result of token counting, indicating whether count is estimated or exact
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenCount {
    /// Estimated count using heuristics (displays with ~ prefix)
    Estimated(usize),
    
    /// Exact count from official API (displays without prefix)
    Exact(usize),
}

impl TokenCount {
    /// Get the numeric value regardless of estimation status
    pub fn value(&self) -> usize {
        match self {
            Self::Estimated(n) | Self::Exact(n) => *n,
        }
    }
    
    /// Check if this count is estimated
    pub fn is_estimated(&self) -> bool {
        matches!(self, Self::Estimated(_))
    }
    
    /// Check if this count is exact
    pub fn is_exact(&self) -> bool {
        matches!(self, Self::Exact(_))
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

**Rationale**:
- Type system prevents mixing estimated/exact counts
- Clear semantics: `TokenCount::Estimated(42)` is self-documenting
- Display trait handles formatting automatically (`~42` vs. `42`)

---

### 2. ContentType (Enum)

**Purpose**: Classify input text for adaptive token estimation

**Location**: `src/tokenizers/claude/estimation.rs`

```rust
/// Classification of text content for adaptive token estimation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// Code-heavy content (>15% code indicators)
    /// Examples: source files, JSON, config files
    /// Token ratio: ~3.0 chars/token
    Code,
    
    /// Prose-heavy content (<5% code indicators)
    /// Examples: documentation, natural language, articles
    /// Token ratio: ~4.5 chars/token
    Prose,
    
    /// Mixed content (5-15% code indicators)
    /// Examples: markdown with code blocks, technical docs
    /// Token ratio: ~3.75 chars/token
    Mixed,
}

impl ContentType {
    /// Get the estimated characters per token for this content type
    pub fn chars_per_token(&self) -> f64 {
        match self {
            Self::Code => 3.0,
            Self::Prose => 4.5,
            Self::Mixed => 3.75,
        }
    }
}
```

**Detection Algorithm**:
- Count code indicators: `{`, `}`, `[`, `]`, `(`, `)`, `;`, `//`, `fn`, `def`, `const`, etc.
- Calculate ratio: `indicators / total_chars`
- Classify:
  - `ratio > 0.15` → Code
  - `ratio < 0.05` → Prose
  - Otherwise → Mixed

---

### 3. ClaudeTokenizer (Struct)

**Purpose**: Main tokenizer implementation for Claude models

**Location**: `src/tokenizers/claude/mod.rs`

```rust
/// Tokenizer for Anthropic Claude models
pub struct ClaudeTokenizer {
    /// Model configuration (name, context window, etc.)
    config: ModelConfig,
    
    /// Optional API client (only if ANTHROPIC_API_KEY set)
    api_client: Option<ClaudeApiClient>,
    
    /// Whether to use accurate mode (--accurate flag)
    use_accurate: bool,
}

impl ClaudeTokenizer {
    /// Create a new Claude tokenizer
    pub fn new(config: ModelConfig, use_accurate: bool) -> Result<Self> {
        let api_client = if use_accurate {
            // Check for API key
            match env::var("ANTHROPIC_API_KEY") {
                Ok(key) if !key.is_empty() => Some(ClaudeApiClient::new(key)?),
                _ => return Err(TokenError::MissingApiKey),
            }
        } else {
            None
        };
        
        Ok(Self {
            config,
            api_client,
            use_accurate,
        })
    }
    
    /// Count tokens using estimation or API
    pub fn count_tokens(&self, text: &str) -> Result<TokenCount> {
        if let Some(client) = &self.api_client {
            // Try API, fall back to estimation on error
            match client.count_tokens(&self.config.name, text) {
                Ok(count) => Ok(TokenCount::Exact(count)),
                Err(e) => {
                    eprintln!("Warning: API call failed ({}), falling back to estimation", e);
                    Ok(TokenCount::Estimated(estimate_tokens(text)))
                }
            }
        } else {
            // Estimation mode
            Ok(TokenCount::Estimated(estimate_tokens(text)))
        }
    }
}

impl Tokenizer for ClaudeTokenizer {
    fn count_tokens(&self, text: &str) -> Result<usize> {
        self.count_tokens(text).map(|tc| tc.value())
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

---

### 4. ClaudeApiClient (Struct)

**Purpose**: HTTP client for Anthropic's count_tokens API

**Location**: `src/tokenizers/claude/api_client.rs`

```rust
/// HTTP client for Anthropic API token counting
pub struct ClaudeApiClient {
    client: reqwest::Client,
    api_key: String,
}

impl ClaudeApiClient {
    /// API endpoint
    const API_ENDPOINT: &'static str = "https://api.anthropic.com/v1/messages/count_tokens";
    
    /// API version header
    const API_VERSION: &'static str = "2023-06-01";
    
    /// Request timeout
    const TIMEOUT_SECS: u64 = 30;
    
    /// Max retry attempts
    const MAX_RETRIES: u32 = 3;
    
    /// Create a new API client
    pub fn new(api_key: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(Self::TIMEOUT_SECS))
            .user_agent(format!("token-count/{}", env!("CARGO_PKG_VERSION")))
            .build()?;
        
        Ok(Self { client, api_key })
    }
    
    /// Count tokens via API with retry logic
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
        
        unreachable!("Should have returned or errored in loop")
    }
    
    /// Single API request attempt
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
    
    /// Parse API error response
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

---

### 5. API Request/Response Types

**Purpose**: Serialize/deserialize Anthropic API payloads

**Location**: `src/tokenizers/claude/api_client.rs`

```rust
/// Request to Anthropic count_tokens API
#[derive(Debug, Serialize)]
struct CountTokensRequest {
    model: String,
    messages: Vec<Message>,
}

/// Message in API request
#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

/// Response from Anthropic count_tokens API
#[derive(Debug, Deserialize)]
struct CountTokensResponse {
    input_tokens: usize,
}

/// API error response
#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    error: ApiError,
}

/// Error details from API
#[derive(Debug, Deserialize)]
struct ApiError {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}
```

**Example JSON**:

Request:
```json
{
  "model": "claude-sonnet-4-6",
  "messages": [
    {
      "role": "user",
      "content": "Hello, Claude!"
    }
  ]
}
```

Response (success):
```json
{
  "input_tokens": 3
}
```

Response (error):
```json
{
  "error": {
    "type": "invalid_request_error",
    "message": "Invalid API key"
  }
}
```

---

### 6. ConsentPrompt (Struct)

**Purpose**: Reusable consent mechanism for API calls

**Location**: `src/api/consent.rs`

```rust
/// Configuration for API consent prompt
pub struct ConsentPrompt {
    /// Provider name (e.g., "Anthropic", "OpenAI")
    pub provider: &'static str,
    
    /// API endpoint URL (for transparency)
    pub api_endpoint: &'static str,
}

impl ConsentPrompt {
    /// Ask user for consent (interactive mode)
    pub fn ask(&self) -> Result<bool> {
        // Check if stdin is a TTY (terminal)
        if !std::io::stdin().is_terminal() {
            return Err(TokenError::NonInteractiveWithoutYes);
        }
        
        // Display prompt on stderr (don't pollute stdout)
        eprintln!();
        eprintln!("This will send your input to {}'s API for accurate token counting.", self.provider);
        eprintln!("Your input will be transmitted over HTTPS to: {}", self.api_endpoint);
        eprintln!();
        eprint!("Proceed with API call? (y/N): ");
        
        // Flush stderr to ensure prompt is visible
        std::io::stderr().flush()?;
        
        // Read user response from stdin
        let mut response = String::new();
        std::io::stdin().read_line(&mut response)?;
        
        // Accept: y, Y, yes, YES (case-insensitive)
        // Reject: n, N, no, NO, empty, anything else
        let normalized = response.trim().to_lowercase();
        Ok(normalized == "y" || normalized == "yes")
    }
    
    /// Check if consent is needed (false if --yes flag or non-interactive)
    pub fn is_needed(args: &Cli) -> bool {
        !args.yes && std::io::stdin().is_terminal()
    }
}
```

---

### 7. CLI Arguments Extension

**Purpose**: New flags for accurate mode and consent

**Location**: `src/cli/args.rs`

```rust
#[derive(Parser, Debug)]
#[command(name = "token-count")]
#[command(version, about)]
pub struct Cli {
    // [Existing fields...]
    
    /// Use API for exact token counts (requires ANTHROPIC_API_KEY for Claude models)
    #[arg(long)]
    pub accurate: bool,
    
    /// Skip API consent prompt (for scripting/automation, requires --accurate)
    #[arg(short = 'y', long)]
    pub yes: bool,
}
```

**Validation**:
- `--yes` requires `--accurate` (or is ignored)
- `--accurate` with Claude models requires `ANTHROPIC_API_KEY`
- `--accurate` with OpenAI models is no-op (already exact)

---

### 8. Error Types Extension

**Purpose**: Claude-specific error variants

**Location**: `src/error.rs`

```rust
#[derive(Error, Debug)]
pub enum TokenError {
    // [Existing variants...]
    
    /// API key not found in environment
    #[error("Accurate mode requires ANTHROPIC_API_KEY environment variable.\n\n\
             Get your API key from: https://console.anthropic.com/\n\
             Then set: export ANTHROPIC_API_KEY=\"sk-ant-...\"\n\n\
             For offline estimation (no API key needed), omit --accurate flag:\n  \
             token-count --model {model}")]
    MissingApiKey { model: String },
    
    /// Invalid API key (401 response)
    #[error("Invalid ANTHROPIC_API_KEY. Please check your API key.\n\n\
             Get a valid key from: https://console.anthropic.com/")]
    InvalidApiKey,
    
    /// Rate limit exceeded (429 response)
    #[error("Anthropic API rate limit exceeded. Please try again later.\n\n\
             Rate limits: https://docs.anthropic.com/en/api/rate-limits")]
    RateLimited,
    
    /// API server error (5xx response)
    #[error("Anthropic API server error (HTTP {0}). The service may be temporarily unavailable.\n\n\
             Check status: https://status.anthropic.com/")]
    ApiServerError(u16),
    
    /// Generic API error
    #[error("Anthropic API error: {0}")]
    ApiError(String),
    
    /// Non-interactive mode without --yes flag
    #[error("API call requires consent. Running in non-interactive mode (stdin not a TTY).\n\n\
             Options:\n  \
             1. Add -y/--yes flag to skip prompt:\n     \
                cat file.txt | token-count --model {model} --accurate -y\n  \n  \
             2. Use estimation mode (no API call):\n     \
                cat file.txt | token-count --model {model}")]
    NonInteractiveWithoutYes { model: String },
}
```

**Exit Codes**:
- `MissingApiKey` → 1
- `InvalidApiKey` → 1
- `RateLimited` → 1 (after fallback to estimation)
- `ApiServerError` → 1 (after fallback to estimation)
- `NonInteractiveWithoutYes` → 1

---

### 9. Model Registry Extension

**Purpose**: Claude model definitions

**Location**: `src/tokenizers/registry.rs`

```rust
impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self { models: HashMap::new(), aliases: HashMap::new() };
        
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
        
        registry.add_model(ModelConfig {
            name: "claude-sonnet-4-6".to_string(),
            encoding: "anthropic-claude".to_string(),
            context_window: 1_000_000,
            description: "Claude Sonnet 4.6 (1M context)".to_string(),
            aliases: vec![
                "sonnet-4-6".to_string(),
                "sonnet".to_string(),
                "claude".to_string(), // Default Claude alias
                "anthropic/claude-sonnet-4-6".to_string(),
            ],
        });
        
        registry.add_model(ModelConfig {
            name: "claude-haiku-4-5".to_string(),
            encoding: "anthropic-claude".to_string(),
            context_window: 200_000,
            description: "Claude Haiku 4.5 (200K context)".to_string(),
            aliases: vec![
                "haiku-4-5".to_string(),
                "haiku".to_string(),
                "anthropic/claude-haiku-4-5".to_string(),
            ],
        });
        
        // [Additional Claude models: 4.5, 4.1, 4.0...]
        
        registry
    }
}
```

**Total Models**: 8 Claude models
- 3x Claude 4.6 (Opus, Sonnet, Haiku)
- 3x Claude 4.5 (Opus, Sonnet, Haiku)
- 1x Claude 4.1 (Opus)
- 1x Claude 4.0 (Opus, Sonnet)

---

## Data Flow

### Estimation Mode (Default)

```
User Input (stdin)
    ↓
CLI Parser (args.rs)
    ↓ model="claude-sonnet-4-6", accurate=false
ModelRegistry::get_tokenizer()
    ↓ encoding="anthropic-claude"
ClaudeTokenizer::new(config, use_accurate=false)
    ↓ api_client=None
ClaudeTokenizer::count_tokens(text)
    ↓
estimate_tokens(text)
    ↓
detect_content_type(text) → ContentType::Code/Prose/Mixed
    ↓
char_count / chars_per_token → usize
    ↓
TokenCount::Estimated(count)
    ↓
Output Formatter (simple.rs)
    ↓
stdout: "~42"
```

### API Mode (--accurate)

```
User Input (stdin)
    ↓
CLI Parser (args.rs)
    ↓ model="claude-sonnet-4-6", accurate=true, yes=false
ConsentPrompt::ask()
    ↓ [Interactive TTY check]
User Prompt → User enters 'y'
    ↓
ClaudeTokenizer::new(config, use_accurate=true)
    ↓ ANTHROPIC_API_KEY env var
ClaudeApiClient::new(api_key)
    ↓
ClaudeTokenizer::count_tokens(text)
    ↓
ClaudeApiClient::count_tokens(model, text)
    ↓ [Retry loop: 3 attempts, exponential backoff]
HTTP POST → api.anthropic.com
    ↓
CountTokensResponse { input_tokens: 42 }
    ↓
TokenCount::Exact(42)
    ↓
Output Formatter (simple.rs)
    ↓
stdout: "42"
```

### API Mode with Fallback

```
[Same as above until API call]
    ↓
HTTP POST → api.anthropic.com
    ↓ [Network error / 5xx response]
ClaudeApiClient::count_tokens() → Err(TokenError::ApiError)
    ↓
ClaudeTokenizer::count_tokens() catches error
    ↓
eprintln!("Warning: API call failed, falling back to estimation")
    ↓
estimate_tokens(text) → usize
    ↓
TokenCount::Estimated(count)
    ↓
stdout: "~42"
```

---

## Validation Rules

### Input Validation

1. **UTF-8 Validation**: Text must be valid UTF-8 (checked before tokenization)
2. **Empty Input**: Returns `TokenCount::Estimated(0)` or `TokenCount::Exact(0)`
3. **Large Input**: No artificial limit (streaming not yet implemented, loads to memory)

### API Key Validation

1. **Environment Variable**: `ANTHROPIC_API_KEY` must exist and be non-empty
2. **Format**: Any non-empty string (API validates format, not client)
3. **Error Handling**: Missing key → helpful error, invalid key → API returns 401

### Consent Validation

1. **TTY Detection**: `std::io::stdin().is_terminal()` must be true
2. **User Response**: Accept only `y`, `Y`, `yes`, `YES` (case-insensitive)
3. **Default**: Any other input (including empty/Enter) → decline

---

## State Management

**Principle**: Stateless CLI (no persistent state between invocations)

**Session State**:
- `ClaudeTokenizer` instance created per invocation
- `ClaudeApiClient` instance created per invocation
- No caching (removed from original spec)
- Each run is independent

**Environment Dependencies**:
- `ANTHROPIC_API_KEY` read once at startup (if `--accurate` flag set)
- Stdin TTY status checked once for consent prompt
- No files read/written (no disk state)

---

## Performance Characteristics

### Estimation Mode

- **Time Complexity**: O(n) where n = character count
- **Space Complexity**: O(1) (no allocations beyond input)
- **Latency**: <10ms for 1KB, <100ms for 1MB

### API Mode

- **Time Complexity**: O(n) network + O(m) where m = response parse time
- **Space Complexity**: O(n) for request body + O(1) for response
- **Latency**: 100-500ms (dominated by network RTT)
- **Retry Impact**: 3 attempts × backoff (2s + 4s + 8s) = up to 14s additional on repeated failures

---

## Testing Data Sets

### Unit Test Cases

1. **Empty String**: `""` → `Estimated(0)`
2. **Single Character**: `"a"` → `Estimated(1)`
3. **Pure Code**: `fn main() { println!("test"); }` → ContentType::Code
4. **Pure Prose**: `"The quick brown fox jumps over the lazy dog."` → ContentType::Prose
5. **Mixed**: `"## Title\n\n```rust\nfn test() {}\n```"` → ContentType::Mixed
6. **Unicode**: `"Hello 世界 🌍"` → Correct UTF-8 char counting
7. **Large Input**: 1MB text file → No panic, completes <100ms

### Integration Test Cases

1. **CLI Estimation**: `echo "test" | token-count --model claude` → `~2`
2. **CLI API (No Key)**: `token-count --model claude --accurate` → Error
3. **CLI API (With Key)**: `ANTHROPIC_API_KEY=sk-test token-count --model claude --accurate -y` → Exact count
4. **CLI Non-Interactive**: `cat file | token-count --model claude --accurate` → Error (no -y)
5. **Consent Decline**: User types 'n' → Falls back to estimation

---

## Relationships Diagram

```
┌─────────────────┐
│   Cli (args)    │
│  - model: str   │
│  - accurate:bool│
│  - yes: bool    │
└────────┬────────┘
         │
         ↓
┌─────────────────────┐
│  ModelRegistry      │
│  - resolve_model()  │
│  - get_tokenizer()  │
└────────┬────────────┘
         │
         ↓
┌──────────────────────────┐
│   ClaudeTokenizer        │
│   - config: ModelConfig  │
│   - api_client: Option   │
│   - use_accurate: bool   │
└─────────┬────────────────┘
          │
          ├─────────────────────┐
          │                     │
          ↓                     ↓
┌──────────────────┐   ┌────────────────────┐
│  estimate_tokens │   │ ClaudeApiClient    │
│  (estimation.rs) │   │ - count_tokens()   │
└────────┬─────────┘   └─────────┬──────────┘
         │                       │
         ↓                       ↓
┌──────────────────┐   ┌────────────────────┐
│  ContentType     │   │ CountTokensRequest │
│  - Code/Prose    │   │ CountTokensResponse│
└──────────────────┘   └────────────────────┘
         │                       │
         └───────────┬───────────┘
                     ↓
            ┌────────────────┐
            │  TokenCount    │
            │  Estimated/    │
            │  Exact         │
            └────────┬───────┘
                     │
                     ↓
            ┌────────────────┐
            │  Output        │
            │  Formatter     │
            └────────────────┘
```

---

## Summary

**Total New Types**: 9
1. `TokenCount` (enum)
2. `ContentType` (enum)
3. `ClaudeTokenizer` (struct)
4. `ClaudeApiClient` (struct)
5. `CountTokensRequest` (struct)
6. `CountTokensResponse` (struct)
7. `ConsentPrompt` (struct)
8. `TokenError` extensions (4 new variants)
9. CLI args extension (2 new fields)

**Key Design Principles**:
- Type safety: Enums distinguish states at compile time
- Clear ownership: No shared mutable state
- Graceful degradation: API errors fall back to estimation
- User control: Explicit opt-in for network calls
- Reusability: Consent pattern works for future providers

**Next Steps**:
1. ⏭️ Create API contracts in `contracts/`
2. ⏭️ Create quickstart validation guide
3. ⏭️ Break down into implementation tasks
