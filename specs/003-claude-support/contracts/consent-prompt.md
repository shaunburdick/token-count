# Consent Prompt Contract

**Feature**: 003-claude-support  
**Purpose**: Reusable consent mechanism for all future API integrations  
**Applies To**: Anthropic, OpenAI (future), Google Gemini (future), any external API calls

---

## Overview

The consent prompt is a user-facing mechanism that requests explicit permission before sending data to external APIs. This pattern ensures transparency and user control over data transmission.

**Design Principles**:
1. **Opt-in by default**: Network calls require explicit user consent
2. **Transparency**: Show API endpoint URL so users know where data is sent
3. **Safe default**: "No" is the default answer (pressing Enter = decline)
4. **Non-interactive support**: Provide `-y` flag for scripts/automation
5. **Fail fast**: Error immediately in non-interactive mode without `-y`

---

## Interface

### ConsentPrompt Struct

```rust
pub struct ConsentPrompt {
    /// Provider name (e.g., "Anthropic", "OpenAI", "Google")
    pub provider: &'static str,
    
    /// API endpoint URL (for transparency)
    pub api_endpoint: &'static str,
}
```

### Methods

```rust
impl ConsentPrompt {
    /// Ask user for consent in interactive mode
    /// Returns Ok(true) if user consents, Ok(false) if declines
    /// Returns Err if in non-interactive mode
    pub fn ask(&self) -> Result<bool, TokenError>;
    
    /// Check if consent prompt is needed
    /// Returns false if --yes flag provided or stdin is not a TTY
    pub fn is_needed(args: &Cli) -> bool;
}
```

---

## Behavior

### Interactive Mode (stdin is TTY)

**Scenario**: User runs command in terminal

```bash
$ echo "test" | token-count --model claude-sonnet-4-6 --accurate
```

**Prompt** (written to stderr):
```
This will send your input to Anthropic's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://api.anthropic.com

Proceed with API call? (y/N): _
```

**User Input → Outcome**:
| Input | Result | Next Action |
|-------|--------|-------------|
| `y` or `Y` | Consent granted | Proceed with API call |
| `yes` or `YES` | Consent granted | Proceed with API call |
| `n` or `N` | Consent declined | Fall back to estimation |
| `no` or `NO` | Consent declined | Fall back to estimation |
| Empty (just press Enter) | Consent declined | Fall back to estimation |
| Any other text | Consent declined | Fall back to estimation |

**After Declining**:
```bash
Falling back to estimation (API call cancelled by user)

~4
```

---

### Interactive Mode with `-y` Flag

**Scenario**: User explicitly skips prompt with `-y/--yes`

```bash
$ echo "test" | token-count --model claude-sonnet-4-6 --accurate -y
```

**Behavior**:
- No prompt shown
- Proceeds directly to API call
- Assumes user has already consented

**Output**:
```
3
```

---

### Non-Interactive Mode (stdin NOT a TTY)

**Scenario**: Piped input or redirect (e.g., CI/CD, scripts)

```bash
$ cat file.txt | token-count --model claude-sonnet-4-6 --accurate
```

**Behavior**:
- Detects stdin is not a TTY
- Checks for `-y/--yes` flag
- If `-y` absent → Error immediately (don't hang)

**Error Output** (stderr):
```
Error: API call requires consent. Running in non-interactive mode (stdin not a TTY).

Options:
  1. Add -y/--yes flag to skip prompt:
       cat file.txt | token-count --model claude-sonnet-4-6 --accurate -y
  
  2. Use estimation mode (no API call):
       cat file.txt | token-count --model claude-sonnet-4-6
```

**Exit Code**: 1

---

### Non-Interactive Mode with `-y` Flag

**Scenario**: User provides `-y` in script/pipeline

```bash
$ cat file.txt | token-count --model claude-sonnet-4-6 --accurate -y
```

**Behavior**:
- Detects stdin is not a TTY
- `-y` flag present → Skip prompt, proceed with API
- No user interaction required

**Output**:
```
42
```

---

## TTY Detection

### Implementation

```rust
use std::io::IsTerminal;

pub fn is_interactive() -> bool {
    std::io::stdin().is_terminal()
}
```

### Platform Behavior

| Platform | stdin is TTY | stdin NOT TTY |
|----------|--------------|---------------|
| Linux terminal | `isatty(0) == true` | Piped/redirected |
| macOS terminal | `isatty(0) == true` | Piped/redirected |
| Windows cmd/PowerShell | `_isatty(_fileno(stdin))` | Piped/redirected |
| CI/CD (GitHub Actions, GitLab) | `false` (no TTY) | Always false |
| Docker container | `false` (unless `-t` flag) | Always false |

---

## Prompt Text Template

### Format

```
This will send your input to {PROVIDER}'s API for accurate token counting.
Your input will be transmitted over HTTPS to: {API_ENDPOINT}

Proceed with API call? (y/N): 
```

### Variables

- `{PROVIDER}`: Provider name (e.g., "Anthropic", "OpenAI")
- `{API_ENDPOINT}`: Full URL (e.g., "https://api.anthropic.com")

### Examples

**Anthropic**:
```
This will send your input to Anthropic's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://api.anthropic.com

Proceed with API call? (y/N): 
```

**OpenAI (future)**:
```
This will send your input to OpenAI's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://api.openai.com

Proceed with API call? (y/N): 
```

**Google Gemini (future)**:
```
This will send your input to Google's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://generativelanguage.googleapis.com

Proceed with API call? (y/N): 
```

---

## Error Messages

### Non-Interactive Without `-y`

```
Error: API call requires consent. Running in non-interactive mode (stdin not a TTY).

Options:
  1. Add -y/--yes flag to skip prompt:
       cat file.txt | token-count --model {model} --accurate -y
  
  2. Use estimation mode (no API call):
       cat file.txt | token-count --model {model}
```

**Variables**:
- `{model}`: Actual model name used (e.g., "claude-sonnet-4-6")

---

## Decision Flow

```
┌─────────────────────┐
│ --accurate flag?    │
└──────┬──────────────┘
       │ No
       ↓
┌─────────────────────┐
│ Use estimation      │
│ (no prompt needed)  │
└─────────────────────┘

       │ Yes
       ↓
┌─────────────────────┐
│ -y/--yes flag?      │
└──────┬──────────────┘
       │ Yes
       ↓
┌─────────────────────┐
│ Skip prompt         │
│ Proceed with API    │
└─────────────────────┘

       │ No
       ↓
┌─────────────────────┐
│ stdin is TTY?       │
└──────┬──────────────┘
       │ No
       ↓
┌─────────────────────┐
│ ERROR               │
│ (non-interactive)   │
└─────────────────────┘

       │ Yes
       ↓
┌─────────────────────┐
│ Show consent prompt │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│ User responds       │
└──────┬──────────────┘
       │
       ├─── y/yes ───→ Proceed with API
       │
       └─── else ────→ Fall back to estimation
```

---

## Implementation Example

### Rust Code

```rust
// src/api/consent.rs

use std::io::{self, IsTerminal, Write};
use crate::cli::Cli;
use crate::error::TokenError;

pub struct ConsentPrompt {
    pub provider: &'static str,
    pub api_endpoint: &'static str,
}

impl ConsentPrompt {
    /// Ask user for consent (interactive mode only)
    pub fn ask(&self) -> Result<bool, TokenError> {
        // Check if stdin is a TTY
        if !io::stdin().is_terminal() {
            return Err(TokenError::NonInteractiveWithoutYes {
                model: "claude".to_string(), // Pass from caller
            });
        }
        
        // Display prompt on stderr (don't pollute stdout)
        eprintln!();
        eprintln!(
            "This will send your input to {}'s API for accurate token counting.",
            self.provider
        );
        eprintln!("Your input will be transmitted over HTTPS to: {}", self.api_endpoint);
        eprintln!();
        eprint!("Proceed with API call? (y/N): ");
        
        // Flush stderr to ensure prompt is visible
        io::stderr().flush()?;
        
        // Read user response from stdin
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        // Normalize and check response
        let normalized = response.trim().to_lowercase();
        Ok(normalized == "y" || normalized == "yes")
    }
    
    /// Check if consent is needed
    pub fn is_needed(args: &Cli) -> bool {
        // Consent needed if:
        // 1. --accurate flag is set
        // 2. --yes flag is NOT set
        // 3. stdin is a TTY (interactive)
        args.accurate && !args.yes && io::stdin().is_terminal()
    }
}
```

### Usage in main.rs

```rust
// main.rs

fn main() -> Result<()> {
    let args = Cli::parse_args();
    
    // [... model resolution ...]
    
    // Check if API call requires consent
    if args.accurate && is_claude_model(&model) {
        if !args.yes && !io::stdin().is_terminal() {
            // Non-interactive without -y → Error
            return Err(TokenError::NonInteractiveWithoutYes { model });
        }
        
        if ConsentPrompt::is_needed(&args) {
            let consent = ConsentPrompt {
                provider: "Anthropic",
                api_endpoint: "https://api.anthropic.com",
            };
            
            if !consent.ask()? {
                eprintln!("Falling back to estimation (API call cancelled by user)");
                eprintln!();
                // Proceed with estimation mode
            }
        }
    }
    
    // [... proceed with tokenization ...]
}
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consent_needed_accurate_no_yes() {
        let args = Cli {
            accurate: true,
            yes: false,
            ..Default::default()
        };
        
        // In CI (no TTY), consent not needed (will error instead)
        // In terminal (TTY), consent needed
        // This test just checks the is_needed logic
        assert_eq!(ConsentPrompt::is_needed(&args), io::stdin().is_terminal());
    }
    
    #[test]
    fn test_consent_not_needed_yes_flag() {
        let args = Cli {
            accurate: true,
            yes: true,
            ..Default::default()
        };
        
        // -y flag skips consent
        assert!(!ConsentPrompt::is_needed(&args));
    }
    
    #[test]
    fn test_consent_not_needed_no_accurate() {
        let args = Cli {
            accurate: false,
            yes: false,
            ..Default::default()
        };
        
        // No --accurate → no API call → no consent
        assert!(!ConsentPrompt::is_needed(&args));
    }
}
```

### Integration Tests

```rust
// tests/consent_prompt.rs

#[test]
fn test_cli_accurate_interactive_yes() {
    // Simulate user typing 'y'
    let output = run_cli_with_input(
        &["--model", "claude", "--accurate"],
        "test input",
        "y\n" // User response
    );
    
    // Should show prompt and proceed with API
    assert!(output.contains("Proceed with API call?"));
}

#[test]
fn test_cli_accurate_interactive_no() {
    // Simulate user typing 'n'
    let output = run_cli_with_input(
        &["--model", "claude", "--accurate"],
        "test input",
        "n\n" // User response
    );
    
    // Should show prompt, fall back to estimation
    assert!(output.contains("Falling back to estimation"));
    assert!(output.contains("~")); // Estimated count
}

#[test]
fn test_cli_accurate_non_interactive_without_yes() {
    // Pipe input (non-interactive)
    let result = pipe_stdin_expect_error(
        &["--model", "claude", "--accurate"],
        "test input"
    );
    
    // Should error immediately
    assert!(result.contains("Non-interactive mode"));
    assert!(result.contains("-y/--yes"));
}

#[test]
fn test_cli_accurate_non_interactive_with_yes() {
    // Pipe input with -y flag
    let output = pipe_stdin(
        &["--model", "claude", "--accurate", "-y"],
        "test input"
    );
    
    // Should proceed without prompt (mocked API)
    assert!(!output.contains("Proceed with API call?"));
}
```

---

## Future Enhancements

### Persistent Consent (Not in MVP)

**Idea**: Remember consent per provider for 24 hours
```bash
$ token-count --model claude --accurate --remember-consent
```

Stores in `~/.cache/token-count/consent.json`:
```json
{
  "anthropic": {
    "consented_at": "2026-03-14T12:00:00Z",
    "expires_at": "2026-03-15T12:00:00Z"
  }
}
```

**Not implemented**: Adds complexity, file I/O, expiry logic

---

### Per-Provider Consent Flags (Not in MVP)

**Idea**: Separate consent for each provider
```bash
$ token-count --anthropic-consent
$ token-count --openai-consent
```

**Not implemented**: `-y` flag works for all providers (simpler)

---

## Related Documents

- [Data Model](../data-model.md) - ConsentPrompt struct definition
- [Plan](../plan.md) - Architecture decision for consent mechanism
- [Anthropic API](./anthropic-api.md) - API that requires consent
