# Feature Specification: Claude Model Support

**Feature ID**: 003  
**Version**: 1.2  
**Status**: Specification Phase  
**Priority**: P1 (Critical - Major feature addition)  
**Estimated Effort**: 3-5 days  
**Dependencies**: None  
**Last Updated**: 2026-03-14

---

## Problem Statement

`token-count` currently supports only OpenAI models (GPT-3.5, GPT-4, GPT-4-Turbo, GPT-4o). Anthropic Claude models represent a significant portion of the LLM market, especially after the Claude 4.6 release (February 2026) which introduced major improvements in coding and reasoning capabilities. Developers using Claude need accurate token counting for cost estimation, context window management, and API request planning.

**User Pain Points**:
- Developers using Claude must use separate tools or Anthropic's web interface for token counting
- No offline option exists for Claude token counting
- Switching between multiple tools for OpenAI and Claude is inefficient
- Anthropic's API-only approach doesn't work in air-gapped or offline environments

---

## Solution Overview

Add support for Anthropic Claude models using a **hybrid tokenization strategy**:

1. **Default: Offline Estimation** - Character-based heuristics for instant, offline token counting (±15-20% accuracy)
2. **Optional: Accurate API Mode** - Anthropic's official count_tokens API for exact counts (requires API key and network)

This approach balances the constitutional principles of **Zero Runtime Dependencies** (offline estimation) with **Accuracy Over Speed** (optional API mode).

**Key Features**:
- Support for all active Claude 4.6, 4.5, 4.1, 4.0 models
- Clear UX distinction between estimated (`~42`) and exact (`42`) counts
- Graceful fallback if API is unavailable
- No API key required for basic usage (estimation)
- Optional API integration for users who need exact counts

---

## User Stories

### US-001: Basic Claude Token Counting (Offline)
**As a** developer working offline  
**I want to** count tokens for Claude models without network access  
**So that** I can estimate costs and manage context windows while coding

**Acceptance Criteria**:
- ✅ Can count tokens using `--model claude-sonnet-4-6` with stdin input
- ✅ Default output shows estimated count with `~` prefix (e.g., `~42`)
- ✅ Works completely offline (no network calls)
- ✅ Processes 1KB of text in <10ms
- ✅ No API key required

**Example**:
```bash
echo "Hello, Claude" | token-count --model claude-sonnet-4-6
~4
```

---

### US-002: Accurate Token Counting (Online)
**As a** developer planning API usage  
**I want to** get exact token counts from Anthropic's official API  
**So that** I can accurately predict costs and avoid context window overflows

**Acceptance Criteria**:
- ✅ Can use `--accurate` flag to request exact count via Anthropic API
- ✅ Requires `ANTHROPIC_API_KEY` environment variable
- ✅ Prompts for consent before making API call (unless `-y/--yes` flag provided)
- ✅ Output shows exact count without `~` prefix (e.g., `3`)
- ✅ Falls back to estimation if API unavailable (with warning)

**Example (Interactive Prompt)**:
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "Hello, Claude" | token-count --model claude-sonnet-4-6 --accurate

This will send your input to Anthropic's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://api.anthropic.com

Proceed with API call? (y/N): y
3
```

**Example (Non-Interactive with -y)**:
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "Hello, Claude" | token-count --model claude-sonnet-4-6 --accurate -y
3
```

---

### US-003: Model Information Display
**As a** developer exploring Claude models  
**I want to** see available Claude models with their details  
**So that** I can choose the right model for my use case

**Acceptance Criteria**:
- ✅ `--list-models` includes Claude models
- ✅ Shows model encoding type, context window, pricing tier
- ✅ Indicates estimation method (heuristic vs. API)
- ✅ Groups models by family (Opus, Sonnet, Haiku)

**Example**:
```bash
token-count --list-models

Anthropic Claude Models:
  
  claude-opus-4-6
    Tokenization: Estimation (±15-20%) or API (with --accurate)
    Context window: 1000000 tokens
    Pricing: $5/$25 per MTok (input/output)
    Aliases: claude-opus-4.6, opus-4-6, anthropic/claude-opus-4-6
  
  claude-sonnet-4-6 (recommended)
    Tokenization: Estimation (±15-20%) or API (with --accurate)
    Context window: 1000000 tokens  
    Pricing: $3/$15 per MTok (input/output)
    Aliases: claude-sonnet-4.6, sonnet-4-6, claude, anthropic/claude-sonnet-4-6
  
  claude-haiku-4-5
    Tokenization: Estimation (±15-20%) or API (with --accurate)
    Context window: 200000 tokens
    Pricing: $1/$5 per MTok (input/output)
    Aliases: claude-haiku-4.5, haiku-4-5, anthropic/claude-haiku-4-5
```

---

### US-004: Verbose Output with Estimation Transparency
**As a** developer debugging token counts  
**I want to** understand how token counts were calculated  
**So that** I can trust the results and know their accuracy

**Acceptance Criteria**:
- ✅ `-v` flag shows estimation method for Claude models
- ✅ Displays accuracy range (±15-20% for estimation)
- ✅ Shows whether count is estimated or exact
- ✅ Provides guidance on using `--accurate` for exact counts

**Example (Estimation)**:
```bash
echo "Build a CLI tool" | token-count --model claude-sonnet-4-6 -v
Model: claude-sonnet-4-6
Tokens: ~5 (estimated)
Estimation method: Character-based heuristic (4 chars/token avg)
Accuracy: ±15-20% from actual count
Context window: 1000000 tokens (0.0005% used)

For exact count, use: --accurate (requires ANTHROPIC_API_KEY)
```

**Example (Accurate)**:
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "Build a CLI tool" | token-count --model claude-sonnet-4-6 --accurate -v
Model: claude-sonnet-4-6
Tokens: 4 (exact via Anthropic API)
Context window: 1000000 tokens (0.0004% used)
API latency: 127ms
```

---

### US-005: Model Alias Support
**As a** developer typing commands quickly  
**I want to** use short aliases for Claude models  
**So that** I can work efficiently without memorizing long model names

**Acceptance Criteria**:
- ✅ Support canonical names (e.g., `claude-sonnet-4-6`)
- ✅ Support short aliases (e.g., `sonnet`, `opus`, `haiku`)
- ✅ Support version aliases (e.g., `claude-4.6`, `opus-4.6`)
- ✅ Support provider prefix (e.g., `anthropic/claude-sonnet-4-6`)
- ✅ Case-insensitive matching (e.g., `CLAUDE-SONNET-4-6`)
- ✅ Fuzzy suggestions on typos (e.g., `claude-sonet` → "Did you mean: claude-sonnet-4-6?")

**Example**:
```bash
# All equivalent
token-count --model claude-sonnet-4-6 < input.txt
token-count --model sonnet < input.txt
token-count --model claude < input.txt  # "claude" = sonnet-4-6 (current default)
token-count --model anthropic/claude-sonnet-4-6 < input.txt
```

---

### US-006: API Consent Prompt
**As a** privacy-conscious developer  
**I want to** be prompted before my input is sent to external APIs  
**So that** I can control when my data is transmitted and avoid accidental API calls

**Acceptance Criteria**:
- ✅ Prompts user for consent when `--accurate` flag triggers API call
- ✅ Prompt displays API endpoint URL (transparency)
- ✅ Defaults to "No" (safe default) - user must type 'y' or 'Y' to proceed
- ✅ `-y/--yes` flag skips prompt (for scripting/automation)
- ✅ Pressing Enter without input → defaults to "No", returns estimation
- ✅ Non-interactive mode (stdin not a TTY) → requires `-y` flag or errors

**Example (Interactive - User Accepts)**:
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "Sensitive data" | token-count --model claude-sonnet-4-6 --accurate

This will send your input to Anthropic's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://api.anthropic.com

Proceed with API call? (y/N): y
3
```

**Example (Interactive - User Declines)**:
```bash
echo "Sensitive data" | token-count --model claude-sonnet-4-6 --accurate

This will send your input to Anthropic's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://api.anthropic.com

Proceed with API call? (y/N): n
Falling back to estimation (API call cancelled by user)

~4
```

**Example (Non-Interactive with -y)**:
```bash
# In scripts - no prompt
cat file.txt | token-count --model claude-sonnet-4-6 --accurate -y
42
```

**Example (Non-Interactive without -y - Error)**:
```bash
cat file.txt | token-count --model claude-sonnet-4-6 --accurate
Error: API call requires consent. Running in non-interactive mode (stdin not a TTY).

Options:
  1. Add -y/--yes flag to skip prompt:
       cat file.txt | token-count --model claude-sonnet-4-6 --accurate -y
  
  2. Use estimation mode (no API call):
       cat file.txt | token-count --model claude-sonnet-4-6

Exit code: 1
```

---

### US-007: Error Handling for API Issues
**As a** developer using accurate mode  
**I want to** receive clear error messages if API calls fail  
**So that** I can understand what went wrong and how to fix it

**Acceptance Criteria**:
- ✅ Clear error if `--accurate` used without `ANTHROPIC_API_KEY`
- ✅ Automatic fallback to estimation if API unreachable (with warning)
- ✅ Helpful error on invalid API key
- ✅ Rate limit errors include retry suggestions
- ✅ Network errors include troubleshooting hints

**Example (Missing API Key)**:
```bash
echo "test" | token-count --model claude-sonnet-4-6 --accurate
Error: Accurate mode requires ANTHROPIC_API_KEY environment variable

Get your API key from: https://console.anthropic.com/
Then set: export ANTHROPIC_API_KEY="sk-ant-..."

For offline estimation (no API key needed), omit --accurate flag:
  echo "test" | token-count --model claude-sonnet-4-6
```

**Example (Network Failure with Fallback)**:
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "test" | token-count --model claude-sonnet-4-6 --accurate
Warning: Anthropic API unreachable (network error), falling back to estimation

Model: claude-sonnet-4-6
Tokens: ~2 (estimated, API unavailable)

The API may be temporarily down. Check status: https://status.anthropic.com/
```

---

## Functional Requirements

### FR-001: Model Registry
**Description**: Extend model registry to include Claude models with metadata

**Requirements**:
- Support Claude 4.6 models (Opus, Sonnet, Haiku)
- Support legacy models (4.5, 4.1, 4.0)
- Store context window size, pricing tier, tokenization method
- Support multiple aliases per model
- Mark deprecated models (e.g., Haiku 3 deprecated April 2026)

**Models to Support**:

| Model | API ID | Aliases | Context | Status |
|-------|--------|---------|---------|--------|
| Claude Opus 4.6 | `claude-opus-4-6` | `opus-4-6`, `opus`, `anthropic/claude-opus-4-6` | 1M tokens | Active |
| Claude Sonnet 4.6 | `claude-sonnet-4-6` | `sonnet-4-6`, `sonnet`, `claude`, `anthropic/claude-sonnet-4-6` | 1M tokens | Active (default) |
| Claude Haiku 4.5 | `claude-haiku-4-5-20251001` | `haiku-4-5`, `haiku`, `anthropic/claude-haiku-4-5` | 200K tokens | Active |
| Claude Sonnet 4.5 | `claude-sonnet-4-5-20250929` | `sonnet-4-5` | 200K-1M tokens | Active |
| Claude Opus 4.5 | `claude-opus-4-5-20251101` | `opus-4-5` | 200K tokens | Active |
| Claude Opus 4.1 | `claude-opus-4-1-20250805` | `opus-4-1` | 200K tokens | Active |
| Claude Sonnet 4.0 | `claude-sonnet-4-20250514` | `sonnet-4-0` | 200K-1M tokens | Active |
| Claude Opus 4.0 | `claude-opus-4-20250514` | `opus-4-0` | 200K tokens | Active |

**Notes**:
- `claude` (no version) → defaults to `claude-sonnet-4-6` (most popular)
- Haiku 3 (`claude-3-haiku-20240307`) deprecated April 19, 2026 - EXCLUDED from MVP

---

### FR-002: Adaptive Token Estimation
**Description**: Implement content-aware heuristic estimation for offline token counting

**Algorithm (Adaptive)**:
```rust
// Detect content type
let content_type = detect_content_type(text);

// Apply appropriate estimation rate
let estimated_tokens = match content_type {
    ContentType::Code => ceil(utf8_char_count / 3.0),      // Code: more tokens (symbols, operators)
    ContentType::Prose => ceil(utf8_char_count / 4.5),     // Prose: fewer tokens (longer words)
    ContentType::Mixed => ceil(utf8_char_count / 3.75),    // Mixed: balanced rate
};
```

**Content Detection Heuristics**:
```rust
fn detect_content_type(text: &str) -> ContentType {
    let code_indicators = count_code_indicators(text);
    let total_chars = text.chars().count();
    
    // Code indicators: {}, [], (), ;, //, #, def, fn, const, etc.
    let code_ratio = code_indicators as f64 / total_chars as f64;
    
    if code_ratio > 0.15 {
        ContentType::Code
    } else if code_ratio > 0.05 {
        ContentType::Mixed
    } else {
        ContentType::Prose
    }
}
```

**Rationale**:
- Code has more symbols and operators → tokenizes into more tokens per character
- Natural language has longer words → tokenizes into fewer tokens per character
- Adaptive approach improves accuracy from ±20% (simple) to ±10% (adaptive)
- Research shows Claude tokenization is similar to GPT models

**Performance Requirements**:
- Content detection: O(n) single pass through text
- Should add <5ms overhead for 1KB inputs
- Total time: <15ms for 1KB inputs (including detection + estimation)

**Testing**:
- Validate against 100+ diverse inputs from Anthropic API
- Test categories: pure code, pure prose, mixed content
- Document actual accuracy range per category
- Include edge cases: empty string, single character, 1MB input, emoji-heavy text, CJK text

**Example Accuracy Targets**:
| Content Type | Target Accuracy | Baseline (Simple) |
|--------------|-----------------|-------------------|
| Code         | ±10%            | ±25% (simple)     |
| Prose        | ±8%             | ±15% (simple)     |
| Mixed        | ±12%            | ±20% (simple)     |
| Overall      | ±10%            | ±20% (simple)     |

---

### FR-003: Anthropic API Client
**Description**: HTTP client for Anthropic's token counting API

**Endpoint**: `POST https://api.anthropic.com/v1/messages/count_tokens`

**Request Format**:
```json
{
  "model": "claude-sonnet-4-6",
  "messages": [
    {"role": "user", "content": "Hello, Claude"}
  ]
}
```

**Response Format**:
```json
{
  "input_tokens": 3
}
```

**Requirements**:
- Use `reqwest` crate with `rustls-tls` (no OpenSSL dependency)
- Set timeout: 30 seconds
- Retry logic: 3 attempts with exponential backoff (2s, 4s, 8s)
- User-Agent: `token-count/{version} (Rust)`
- API version header: `anthropic-version: 2023-06-01`

**Error Handling**:
- 401 Unauthorized → "Invalid API key"
- 429 Rate Limited → "Rate limit exceeded, try again in {retry_after}s"
- 5xx Server Error → "Anthropic API error, check status.anthropic.com"
- Network timeout → "Network timeout, check internet connection"
- Connection refused → Fall back to estimation with warning

---

### FR-004: Output Formatting
**Description**: Clear visual distinction between estimated and exact counts

**Simple Mode** (default):
```bash
# Estimation
echo "test" | token-count --model claude-sonnet-4-6
~2

# Exact
echo "test" | token-count --model claude-sonnet-4-6 --accurate
1
```

**Verbose Mode** (`-v`):
```bash
# Estimation
echo "test" | token-count --model claude-sonnet-4-6 -v
Model: claude-sonnet-4-6
Tokens: ~2 (estimated)
Estimation method: Character-based heuristic (4 chars/token avg)
Accuracy: ±15-20% from actual count
Context window: 1000000 tokens (0.0002% used)

For exact count, use: --accurate (requires ANTHROPIC_API_KEY)

# Exact
export ANTHROPIC_API_KEY="sk-ant-..."
echo "test" | token-count --model claude-sonnet-4-6 --accurate -v
Model: claude-sonnet-4-6
Tokens: 1 (exact via Anthropic API)
Context window: 1000000 tokens (0.0001% used)
API latency: 127ms
```

---

### FR-005: CLI Argument Handling
**Description**: New CLI flags for accurate mode and consent

**New Flags**:
```
--accurate
    Use Anthropic API for exact token counts (requires ANTHROPIC_API_KEY)
    Prompts for consent before making API call (unless -y specified)
    Falls back to estimation if API unavailable or user declines

-y, --yes
    Skip API consent prompt (for scripting/automation)
    Only takes effect when --accurate is used
    Must be combined with --accurate (e.g., --accurate -y)
```

**Behavior**:
- If `--accurate` + OpenAI model → ignored (exact counts already provided, no API call)
- If `--accurate` + Claude model + no API key → error with helpful message
- If `--accurate` + Claude model + API key + interactive mode → prompt for consent
- If `--accurate` + Claude model + API key + non-interactive mode (stdin not TTY) → require `-y` or error
- If `--accurate -y` + Claude model + API key → skip prompt, proceed directly
- If no `--accurate` + Claude model → always use estimation (no prompt)

**Mutually Exclusive**:
- `--accurate` is independent of `-v/--verbose` (can combine)
- `-y` requires `--accurate` (using `-y` alone has no effect)
- `--accurate` is per-invocation (no persistent config)

**TTY Detection**:
```rust
// Detect if stdin is interactive
use std::io::IsTerminal;

if !std::io::stdin().is_terminal() {
    // Non-interactive mode (piped/redirected)
    // Require -y flag or error
}
```

---

### FR-006: API Consent Mechanism
**Description**: Interactive prompt before making external API calls (pattern for all future API integrations)

**Prompt Format**:
```
This will send your input to {Provider}'s API for accurate token counting.
Your input will be transmitted over HTTPS to: {API_ENDPOINT}

Proceed with API call? (y/N): 
```

**Variables**:
- `{Provider}`: "Anthropic" (extensible for future providers: "OpenAI", "Google", etc.)
- `{API_ENDPOINT}`: Full API URL (e.g., "https://api.anthropic.com")

**Requirements**:
- Default answer: "No" (capital N) - safe default
- Accept: 'y', 'Y', "yes", "YES" (case-insensitive)
- Reject: 'n', 'N', "no", "NO", or Enter/empty input
- Read from stderr, write to stderr (don't pollute stdout with prompts)
- Timeout: None (wait indefinitely for user input)
- If user declines: Return estimation with message "Falling back to estimation (API call cancelled by user)"

**Non-Interactive Mode Detection**:
```rust
// Check if stdin is a TTY (terminal)
if std::io::stdin().is_terminal() {
    // Interactive - show prompt
    show_consent_prompt();
} else {
    // Non-interactive (pipe, redirect, CI/CD)
    if !args.yes {
        return Err("API call requires consent. Running in non-interactive mode. Use -y flag.");
    }
}
```

**Future Extensibility**:
- This mechanism applies to ANY future API integration (OpenAI, Google Gemini, etc.)
- Centralized consent handler in `src/api/consent.rs`
- Configurable per-provider (different prompts for different APIs)

**Error Messages**:
```rust
// Non-interactive without -y
Error: API call requires consent. Running in non-interactive mode (stdin not a TTY).

Options:
  1. Add -y/--yes flag to skip prompt:
       cat file.txt | token-count --model {model} --accurate -y
  
  2. Use estimation mode (no API call):
       cat file.txt | token-count --model {model}

Exit code: 1
```

---

## Non-Functional Requirements

### NFR-001: Performance
- **Estimation mode**: <10ms for 1KB input, <100ms for 1MB input
- **API mode**: <500ms for typical requests (includes network latency)
- **Memory**: <50MB total for 10MB input
- **Binary size**: <15MB increase (reqwest + rustls dependencies)

**Mitigation for Binary Size**:
- Make API client optional via feature flag: `claude-api`
- Default build includes both estimation and API client
- Users can build with `--no-default-features --features claude-estimation` for minimal binary

---

### NFR-002: Accuracy
**Estimation Mode**:
- Target: ±20% from actual count
- Validation: Test against 100+ diverse inputs from Anthropic API
- Document actual measured accuracy in README

**API Mode**:
- Accuracy: Exact (official Anthropic API)
- No deviation from API response

**Testing**:
```bash
# Validation script (requires API key)
./scripts/validate-claude-accuracy.sh

# Outputs:
# Tested 127 inputs
# Mean error: -12.3% (estimation tends to over-count)
# Median error: -8.1%
# 95th percentile error: 24.7%
# Max error: 31.2% (highly technical code with many symbols)
```

---

### NFR-003: Reliability
**Estimation Mode**:
- No network dependencies → 100% reliability
- No failure modes (deterministic algorithm)

**API Mode**:
- Graceful degradation: Falls back to estimation on API failure
- Retry logic: 3 attempts with backoff
- Clear error messages for all failure modes
- No silent failures (always indicate estimation vs. exact)

---

### NFR-004: Security
- **API Key Handling**:
  - Never log API key (sanitize logs)
  - Read from environment variable only (no CLI arg to avoid shell history)
  - No API key validation (let Anthropic API return 401)
  
- **HTTPS Enforcement**:
  - All API calls use HTTPS (rustls)
  - Certificate validation enabled (no `danger_accept_invalid_certs`)
  
- **Input Validation**:
  - UTF-8 validation before sending to API (fail fast on invalid input)
  - No arbitrary code execution from API responses
  - JSON parsing with strict schema validation

---

### NFR-005: Maintainability
- **Model Updates**:
  - New models added via config (no code changes)
  - Deprecation warnings for old models (soft deprecation, no breaking changes)
  
- **API Version**:
  - API version header configurable (currently `2023-06-01`)
  - Future-proof for API changes
  
- **Testing**:
  - Unit tests for estimation algorithm
  - Integration tests with mocked API responses
  - Optional real API tests (gated behind `ANTHROPIC_API_KEY`)

---

## Edge Cases & Error Conditions

### EC-001: Empty Input
**Scenario**: User pipes empty string
```bash
echo "" | token-count --model claude-sonnet-4-6
```

**Expected Behavior**:
- Estimation: `0`
- API: `0`
- No error, exit code 0

---

### EC-002: Very Large Input (>1MB)
**Scenario**: User counts tokens in 10MB file
```bash
cat large-file.txt | token-count --model claude-sonnet-4-6 --accurate
```

**Expected Behavior**:
- Estimation: Stream processing, <500MB memory
- API: Anthropic API supports large inputs (within context window)
- If input exceeds Claude's context window (1M tokens), API returns error → show error + estimated count as fallback

---

### EC-003: Invalid UTF-8
**Scenario**: Binary data piped to stdin
```bash
cat image.png | token-count --model claude-sonnet-4-6
```

**Expected Behavior**:
- Fail fast with clear error
- Exit code 1
- Message: `Error: Input contains invalid UTF-8 at byte offset {offset}`

---

### EC-004: API Rate Limiting
**Scenario**: User exceeds Anthropic rate limits (100 RPM for free tier)

**Expected Behavior**:
- API returns 429 with `retry-after` header
- Tool waits `retry-after` seconds (up to 60s max)
- If still rate limited, fall back to estimation with warning
- Exit code 0 (successful estimation fallback)

---

### EC-005: Network Proxy
**Scenario**: User behind corporate proxy requiring `HTTP_PROXY` env var

**Expected Behavior**:
- `reqwest` automatically honors `HTTP_PROXY`, `HTTPS_PROXY` environment variables
- No additional configuration needed
- If proxy blocks API, fall back to estimation

---

### EC-006: Model Name Typo
**Scenario**: `--model claude-sonet-4-6` (typo: "sonet" not "sonnet")

**Expected Behavior**:
- Fuzzy match using Levenshtein distance (≤3 edits)
- Suggest: `Did you mean: claude-sonnet-4-6, claude-sonnet-4-5?`
- Exit code 2 (unknown model)

---

### EC-007: API Key With Insufficient Permissions
**Scenario**: API key valid but lacks `messages.count_tokens` permission

**Expected Behavior**:
- API returns 403 Forbidden
- Error: `API key lacks permission for token counting`
- Suggest checking API key permissions in console
- Exit code 1 (API error)

---

## Implementation Notes

### Architecture Changes

**New Modules**:
```
src/tokenizers/
├── claude/
│   ├── mod.rs           # Public API
│   ├── estimation.rs    # Offline character-based estimation
│   ├── api_client.rs    # Anthropic API HTTP client
│   └── models.rs        # Claude model definitions
```

**Modified Modules**:
```
src/tokenizers/
├── registry.rs          # Add Claude models to registry
└── mod.rs               # Export Claude tokenizer

src/cli/
└── args.rs              # Add --accurate flag

src/output/
├── simple.rs            # Handle ~ prefix for estimates
└── verbose.rs           # Show estimation method
```

---

### Dependency Changes

**Cargo.toml**:
```toml
[dependencies]
# Existing (no changes)
anyhow = "1.0.102+"
thiserror = "1.0+"
clap = { version = "4.6.0+", features = ["derive"] }
tiktoken-rs = "0.9.1+"
strsim = "0.11+"

# New for Claude support
reqwest = { version = "0.12+", features = ["json", "rustls-tls"], optional = true }
serde = { version = "1.0.149+", features = ["derive"] }
serde_json = "1.0.149+"

[features]
default = ["claude-estimation", "claude-api"]
claude-estimation = []  # Offline estimation (always included)
claude-api = ["reqwest"]  # API client (optional for minimal builds)
```

**Binary Size Impact**:
- With API client: +2-3MB (reqwest + rustls)
- Without API client: +10KB (estimation only)

---

### Testing Strategy

**Unit Tests**:
```rust
// src/tokenizers/claude/estimation.rs
#[test]
fn test_empty_input() {
    assert_eq!(estimate_tokens(""), 0);
}

#[test]
fn test_short_text() {
    let tokens = estimate_tokens("Hello, world!");
    assert!(tokens >= 2 && tokens <= 4);
}

#[test]
fn test_emoji() {
    let tokens = estimate_tokens("👋 Hello 🌍");
    assert!(tokens >= 3 && tokens <= 5);
}

// src/tokenizers/claude/api_client.rs
#[test]
fn test_parse_api_response() {
    let json = r#"{"input_tokens": 42}"#;
    let count = parse_response(json).unwrap();
    assert_eq!(count, 42);
}

#[test]
fn test_api_error_handling() {
    let error = r#"{"error": {"message": "Invalid API key"}}"#;
    let result = parse_response(error);
    assert!(result.is_err());
}
```

**Integration Tests**:
```rust
// tests/claude_estimation.rs
#[test]
fn test_cli_estimation_mode() {
    let output = run_cli(&["--model", "claude-sonnet-4-6"], "Hello");
    assert!(output.starts_with("~"));
}

#[test]
fn test_cli_accurate_mode_no_key() {
    let output = run_cli(&["--model", "claude-sonnet-4-6", "--accurate"], "Hello");
    assert!(output.contains("Error: Accurate mode requires ANTHROPIC_API_KEY"));
}
```

**Accuracy Validation** (requires API key):
```bash
# ./scripts/validate-claude-accuracy.sh
#!/bin/bash
# Compares estimation vs. API for 100+ test inputs
# Outputs accuracy metrics (mean error, median error, 95th percentile)
```

---

### Migration Path

**Phase 1: Add Estimation (Offline)**
- Implement character-based estimation
- Update model registry
- Add CLI tests
- Document in README

**Phase 2: Add API Client (Accurate Mode)**
- Implement HTTP client with retry logic
- Add caching layer
- Add `--accurate` flag
- Integration tests with mocked API

**Phase 3: Documentation & Polish**
- Update README with Claude examples
- Add comparison benchmarks (estimation vs. API accuracy)
- Update CHANGELOG
- Release notes

---

## Success Criteria

### Must Have (MVP)
- ✅ Support Claude Opus 4.6, Sonnet 4.6, Haiku 4.5
- ✅ Offline estimation mode works without API key
- ✅ `--accurate` mode uses Anthropic API
- ✅ Clear visual distinction between estimates (`~`) and exact counts
- ✅ Graceful fallback if API unavailable
- ✅ Zero clippy warnings
- ✅ 80%+ test coverage for new code
- ✅ Documentation in README with examples

### Should Have (Post-MVP)
- ✅ Support for legacy models (Claude 4.5, 4.1, 4.0)
- ✅ Fuzzy model name matching with suggestions
- ✅ Validation script comparing estimation accuracy

### Could Have (Future)
- [ ] JSON output mode for scripting
- [ ] Batch mode (count tokens in multiple files)
- [ ] Configuration file for default model
- [ ] Persistent cache (disk-based)

---

## Open Questions & Decisions Needed

### Q1: Default Claude Model Alias
**Question**: What should `--model claude` (no version) resolve to?

**Options**:
- A) `claude-sonnet-4-6` (current most popular, balanced performance)
- B) `claude-opus-4-6` (highest capability, more expensive)
- C) `claude-haiku-4-5` (fastest, cheapest)

**Recommendation**: Option A (`claude-sonnet-4-6`) - Matches Anthropic's default on claude.ai

**Decision**: [NEEDS USER INPUT]

---

### Q2: Legacy Model Support Scope
**Question**: Should we support all legacy models or just latest generation?

**Options**:
- A) Latest only (Opus 4.6, Sonnet 4.6, Haiku 4.5)
- B) All active models (4.6, 4.5, 4.1, 4.0)
- C) Include deprecated models with warnings (3.x series)

**Recommendation**: Option B (all active models) - Users may have pinned versions, backward compatibility matters

**Decision**: ✅ **APPROVED** - Support all active Claude 4.6, 4.5, 4.1, 4.0 models

---

### Q3: API Client as Optional Feature
**Question**: Should API client be optional (feature flag) or always included?

**Options**:
- A) Always included (simpler, larger binary +2-3MB)
- B) Optional feature flag (smaller default binary, more build complexity)

**Recommendation**: Option A (always included) - Most users will want both modes, complexity not worth size savings

**Decision**: ✅ **APPROVED** - Always include API client (no optional feature flag)

---

### Q4: Estimation Algorithm Complexity
**Question**: Should we use simple character count or more sophisticated heuristics?

**Options**:
- A) Simple: `chars / 4` (fast, easy to understand, ±20% accuracy)
- B) Complex: Different rates for prose vs. code, punctuation handling (slower, ±10% accuracy)

**Recommendation**: Option B (complex/adaptive) - Better accuracy justifies small complexity increase

**Decision**: ✅ **APPROVED** - Use adaptive algorithm with code vs. prose detection (target ±10% accuracy)

---

### Q5: REMOVED - Cache Persistence Decision

**Previous Question**: Should API result cache persist across invocations?

**User Decision**: ✅ **NO CACHE** - Short-lived CLI tool doesn't benefit from caching (each run starts fresh, minimal reuse within single invocation)

**Impact**: Removed FR-004 (Result Caching), simplified architecture, reduced memory requirements, eliminated cache-related complexity

---

## Related Documents

- [Research: Claude Tokenization Options](../RESEARCH-CLAUDE-TOKENIZATION.md)
- [Constitution](../memory/constitution.md) - Core principles guiding this feature
- [Feature 001: Core CLI](./001-core-cli.md) - Existing OpenAI tokenization architecture
- [Anthropic Token Counting API Docs](https://docs.anthropic.com/en/docs/build-with-claude/token-counting)

---

## Changelog

### Version 1.2 (2026-03-14)
- **REMOVED**: All cache-related requirements (FR-004, US-002 cache criteria, EC-006 concurrent cache)
- **RATIONALE**: Short-lived CLI tool doesn't benefit from in-memory cache
- **IMPACT**: Simplified architecture, removed `lru` dependency, reduced memory footprint
- Updated NFR-001 memory requirement: 100MB → 50MB (no cache overhead)
- Simplified consent prompt flow (removed "cache hit bypass" logic)

### Version 1.1 (2026-03-14)
- Added US-006: API Consent Prompt with interactive/non-interactive modes
- Added FR-006: CLI arguments (`--accurate`, `-y/--yes`)
- Added FR-007: API consent mechanism (reusable pattern)
- Clarified TTY detection requirements for non-interactive environments
- All open questions (Q1-Q5) answered by user

### Version 1.0 (2026-03-14)
- Initial specification
- Hybrid approach (estimation + API) decided based on research
- All user stories, functional requirements, NFRs defined
- 5 open questions requiring user input before proceeding to planning phase

---

**Next Steps**:
1. ✅ Specification complete (all clarifications resolved)
2. ✅ All open questions answered by user (Q1-Q4 approved, Q5 cache removed)
3. ⏭️ Hand off to **modern-architect-engineer** for planning phase
4. ⏭️ Create implementation plan with data models and API contracts
5. ⏭️ Break down into tasks
6. ⏭️ Implement with TDD approach

---

**Status**: ✅ Specification Phase Complete - Ready for planning phase  
**Assigned To**: modern-architect-engineer (for planning phase)  
**Estimated Release**: v0.2.0

---

### Q2: Legacy Model Support Scope
