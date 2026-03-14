# Feature 001: Core CLI Token Counting

**Version**: 1.1  
**Status**: Specified (Clarifications Complete)  
**Priority**: P1 (Critical - MVP)  
**Last Updated**: 2026-03-13  
**Dependencies**: None

## Problem Statement

Developers need a fast, reliable way to count tokens for LLM API calls directly from the command line. Currently, they must:
- Copy-paste text into web-based tokenizers (slow, breaks workflow)
- Write custom scripts for each model's tokenizer (maintenance burden)
- Use language-specific libraries that don't integrate with shell pipelines

**Pain Points**:
- Context switching between terminal and browser
- Uncertainty about token counts before API calls (leading to failed requests or wasted context)
- Inconsistent tokenization across different tools
- No quick way to check if text fits in model context windows

## Solution Overview

A single-binary CLI tool that counts tokens for LLM models with POSIX-style interface:
```bash
# Basic usage
echo "Hello world" | token-count --model gpt-4
2

# From file
token-count --model claude-sonnet < document.txt
1842

# From command output
ls -la | token-count --model gpt-3.5-turbo
156
```

**Key Features**:
- Accepts input from stdin (pipes) or files
- Exact tokenization for OpenAI models (MVP), estimation for others (post-MVP)
- Multiple verbosity levels (simple number → detailed breakdown)
- Smart model name resolution (aliases + exact names)
- Cross-platform: works identically on Windows, Linux, macOS

## User Stories

### US-001: Quick Token Count
**As a** developer using GPT-4 API  
**I want to** pipe my prompt into a tool and instantly see token count  
**So that** I can verify it fits within context limits before making expensive API calls

**Acceptance Criteria**:
- [ ] Command accepts stdin input
- [ ] Outputs single number by default
- [ ] Completes in <10ms for small inputs (<10KB)
- [ ] Exit code 0 on success

**Example**:
```bash
$ echo "Translate this: Hello world" | token-count --model gpt-4
6
$ echo $?
0
```

---

### US-002: Model Alias Support
**As a** developer switching between models  
**I want to** use short, memorable model names like "gpt4" instead of "gpt-4-turbo-2024-04-09"  
**So that** I can work faster without memorizing exact version strings

**Acceptance Criteria**:
- [ ] Accepts exact model names (e.g., `gpt-4-turbo-2024-04-09`)
- [ ] Accepts simple aliases (e.g., `gpt4` → latest GPT-4)
- [ ] Accepts provider/model format (e.g., `openai/gpt-4`)
- [ ] Case-insensitive matching
- [ ] Lists supported models with `--list-models`

**Example**:
```bash
$ echo "Test" | token-count --model gpt4
1

$ echo "Test" | token-count --model openai/gpt-4
1

$ echo "Test" | token-count --model GPT-4-TURBO
1

$ token-count --list-models
Supported Models:
  OpenAI:
    gpt-3.5-turbo (aliases: gpt35, gpt3.5)
    gpt-4 (aliases: gpt4)
    gpt-4-turbo (aliases: gpt4-turbo)
    gpt-4o (aliases: gpt4o)
```

---

### US-003: Default Model
**As a** developer primarily using one model  
**I want to** omit the `--model` flag and use a sensible default  
**So that** I can save keystrokes for common operations

**Acceptance Criteria**:
- [ ] Default model is `gpt-3.5-turbo` (most common, cost-effective)
- [ ] Documented in `--help` output
- [ ] Can override default with `--model` flag

**Example**:
```bash
$ echo "Hello" | token-count
1

$ token-count --help | grep default
  --model <MODEL>    Model to use for tokenization [default: gpt-3.5-turbo]
```

---

### US-004: Verbose Output Levels
**As a** developer debugging token counts  
**I want to** see detailed information about tokenization  
**So that** I can understand how my text is being processed

**Acceptance Criteria**:
- [ ] Default (no flag): Number only
- [ ] `-v`: Model name + token count
- [ ] `-vv`: Add context window usage percentage
- [ ] `-vvv`: Add sample token IDs and decoded tokens (first 10 tokens max)

**Examples**:

**Default**:
```bash
$ echo "Hello world" | token-count --model gpt-4
2
```

**Verbose (-v)**:
```bash
$ echo "Hello world" | token-count --model gpt-4 -v
Model: gpt-4 (cl100k_base encoding)
Tokens: 2
```

**Very Verbose (-vv)**:
```bash
$ echo "Hello world" | token-count --model gpt-4 -vv
Model: gpt-4 (cl100k_base encoding)
Tokens: 2
Context Window: 8,192 tokens
Usage: 0.02%
```

**Debug (-vvv)**:
```bash
$ echo "Hello world" | token-count --model gpt-4 -vvv
Model: gpt-4 (cl100k_base encoding)
Tokens: 2
Token IDs: [15339, 1917]
Decoded Tokens: ["Hello", " world"]
Context Window: 8,192 tokens
Usage: 0.02%
```

---

### US-005: File Input
**As a** developer with large documents  
**I want to** pass files directly to the tool  
**So that** I can avoid shell limitations with large stdin buffers

**Acceptance Criteria**:
- [ ] Accepts file path via stdin redirection (`< file.txt`)
- [ ] Streams large files (>100MB) without loading entirely into memory
- [ ] Processes files with any UTF-8 content

**Example**:
```bash
$ token-count --model gpt-4 < large-document.txt
15847

$ cat file1.txt file2.txt | token-count --model gpt-4
3421
```

---

### US-006: Error Handling - Invalid Model
**As a** developer using the tool  
**I want to** see helpful error messages when I specify an unknown model  
**So that** I can quickly correct my mistake

**Acceptance Criteria**:
- [ ] Unknown model exits with code 2
- [ ] Error message includes suggestions for similar models
- [ ] Suggests using `--list-models` to see all options

**Example**:
```bash
$ echo "Test" | token-count --model gpt5
Error: Unknown model 'gpt5'

Did you mean one of these?
  - gpt-4
  - gpt-4o
  - gpt-3.5-turbo

Use --list-models to see all supported models
$ echo $?
2
```

---

### US-007: Error Handling - Invalid UTF-8
**As a** developer piping binary data accidentally  
**I want to** see a clear error message  
**So that** I know the input is invalid

**Acceptance Criteria**:
- [ ] Binary/invalid UTF-8 input exits with code 1
- [ ] Error message clearly states "Input contains invalid UTF-8"
- [ ] No partial token counts printed

**Example**:
```bash
$ cat binary-file.png | token-count --model gpt-4
Error: Input contains invalid UTF-8

token-count requires valid UTF-8 text input.
Binary files cannot be tokenized.
$ echo $?
1
```

---

### US-008: Error Handling - Empty Input
**As a** developer piping potentially empty output  
**I want to** see `0` tokens without errors  
**So that** my scripts don't break on empty inputs

**Acceptance Criteria**:
- [ ] Empty stdin prints `0`
- [ ] Exit code 0 (success)
- [ ] No error messages

**Example**:
```bash
$ echo "" | token-count --model gpt-4
0
$ echo $?
0

$ cat /dev/null | token-count --model gpt-4
0
```

---

### US-009: Help and Version Info
**As a** developer using the tool  
**I want to** see usage instructions and version information  
**So that** I know what options are available

**Acceptance Criteria**:
- [ ] `--help` or `-h` shows usage, options, examples
- [ ] `--version` or `-V` shows version number
- [ ] Help includes examples of common usage patterns

**Example**:
```bash
$ token-count --help
token-count 0.1.0
Count tokens for LLM models using exact tokenization

USAGE:
    token-count [OPTIONS]

OPTIONS:
    -m, --model <MODEL>      Model to use [default: gpt-3.5-turbo]
    -v, --verbose            Increase output verbosity (can be repeated)
        --list-models        List all supported models
    -h, --help               Print help information
    -V, --version            Print version information

EXAMPLES:
    # Count tokens from stdin
    echo "Hello world" | token-count --model gpt-4

    # Count tokens from file
    token-count --model claude-sonnet < document.txt

    # Verbose output
    cat prompt.txt | token-count -vv

$ token-count --version
token-count 0.1.0
```

---

## Functional Requirements

### FR-001: CLI Argument Parsing
The tool **must** parse command-line arguments using `clap` with the following options:

| Flag | Short | Type | Default | Description |
|------|-------|------|---------|-------------|
| `--model` | `-m` | String | `gpt-3.5-turbo` | Model name or alias |
| `--verbose` | `-v` | Counter | 0 | Verbosity level (repeatable) |
| `--list-models` | - | Flag | - | Show supported models |
| `--help` | `-h` | Flag | - | Show help |
| `--version` | `-V` | Flag | - | Show version |

**Validation**:
- Model name must be non-empty
- Verbosity level capped at 3 (anything >3 treated as 3)

---

### FR-002: Input Processing
The tool **must**:
- Read from stdin until EOF
- Support UTF-8 text only
- Detect invalid UTF-8 and exit with error code 1
- Stream large inputs (>100MB) without loading entire content into memory
- Handle empty input (0 bytes) by outputting `0`

**Memory Budget**: Maximum 500MB for any input size

---

### FR-003: Model Resolution (MVP Phase)
The tool **must** support OpenAI models in MVP with exact tokenization:

| Model Family | Exact Names | Aliases | Tokenizer |
|--------------|-------------|---------|-----------|
| GPT-3.5 | `gpt-3.5-turbo` | `gpt35`, `gpt3.5` | cl100k_base |
| GPT-4 Base | `gpt-4`, `gpt-4-0613` | `gpt4` | cl100k_base |
| GPT-4 Turbo | `gpt-4-turbo`, `gpt-4-turbo-2024-04-09` | `gpt4-turbo` | cl100k_base |
| GPT-4o | `gpt-4o` | `gpt4o` | o200k_base |

**Resolution Logic**:
1. Normalize input: lowercase, trim whitespace
2. Check exact match first
3. Check aliases
4. Check provider/model format (split on `/`, match second part)
5. If no match: exit with error code 2 + suggestions

**Suggestions Algorithm**:
- Fuzzy match on edit distance (Levenshtein distance ≤ 2)
- Show up to 3 suggestions
- Always include link to `--list-models`

---

### FR-003a: Model Support Post-MVP (Phase 2)
After MVP (v0.2.0), add support for:

| Provider | Models | Method | Priority |
|----------|--------|--------|----------|
| Anthropic | Claude 3/3.5/4 (Haiku/Sonnet/Opus) | Smart estimation (char-based + 20% margin) | P1 |
| Google | Gemini 1.5/2.0 | llm-tokenizer or estimation | P1 |
| Meta | Llama 2/3/4 | llm-tokenizer (SentencePiece) | P2 |
| Mistral | Mistral, Codestral, Pixtral | llm-tokenizer | P2 |

**Estimation Method for Claude/Gemini** (if exact tokenization unavailable):
- Character count ÷ 3.5 (average chars per token)
- Add 20% safety margin
- Show warning in verbose output: "⚠ Estimated count (±10%)"

**Not Supported**: DeepSeek, Qwen (insufficient Rust library support)

---

### FR-004: Tokenization Logic
The tool **must**:
- Use `tiktoken-rs` for OpenAI models (exact tokenization)
- Load tokenizer once per execution (cache in memory)
- Handle special tokens according to model specifications
- Return accurate token count matching official OpenAI tokenizers

**Verification**: Token counts must match OpenAI's official tiktoken Python library for identical inputs.

---

### FR-005: Output Formatting
The tool **must** format output based on verbosity:

**Verbosity 0 (default)**:
- Single integer on stdout
- No trailing newline after number
- Example: `142`

**Verbosity 1 (-v)**:
- Model name + encoding + token count
- Format: `Model: {model} ({encoding})\nTokens: {count}`

**Verbosity 2 (-vv)**:
- Add context window info
- Format: `Model: {model} ({encoding})\nTokens: {count}\nContext Window: {window} tokens\nUsage: {percentage}%`

**Verbosity 3 (-vvv)**:
- Add token IDs + decoded tokens (first 10 only)
- Format: Include `Token IDs: [{ids}]\nDecoded Tokens: [{tokens}]`

---

### FR-006: Error Handling
The tool **must** handle errors as follows:

| Error Type | Exit Code | Message Format | Example |
|------------|-----------|----------------|---------|
| Invalid UTF-8 | 1 | `Error: Input contains invalid UTF-8\n\n{hint}` | "Binary files cannot be tokenized" |
| Unknown model | 2 | `Error: Unknown model '{name}'\n\nDid you mean:\n  - {suggestions}\n\n{hint}` | See US-006 |
| Unsupported model | 2 | `Error: Model '{name}' not supported\n\n{hint}` | "Use --list-models to see supported models" |
| I/O error | 1 | `Error: {io_error_message}` | "Failed to read stdin" |

All errors **must** go to stderr. Exit immediately on any error (no partial output).

---

### FR-007: Performance Requirements
The tool **must** meet these performance targets:

| Input Size | Max Latency | Max Memory |
|------------|-------------|------------|
| <10KB | 10ms | 50MB |
| 1MB | 100ms | 100MB |
| 100MB | 5s | 500MB |
| 1GB+ | Streaming | 500MB |

**Measurement**: 95th percentile on standard developer laptop (8GB RAM, 4 cores, 2.5GHz)

---

### FR-008: Cross-Platform Compatibility
The tool **must**:
- Work identically on Linux (x64), macOS (x64 + ARM64), Windows (x64)
- Use platform-agnostic stdin reading (no Unixy assumptions)
- Handle line endings correctly (LF, CRLF, CR)
- Support Unicode across all platforms

**Testing**: CI must run full test suite on all target platforms

---

### FR-009: Binary Size
The compiled binary **must**:
- Be ≤30MB uncompressed (single binary)
- Embed all tokenizer data (no external files)
- Strip debug symbols in release builds
- Use optimal compression settings

**Current Baseline** (estimate based on tiktoken-rs):
- tiktoken-rs tokenizer data: ~3-5MB per model
- OpenAI models (MVP): ~15-20MB total
- Additional code + dependencies: ~5-10MB
- **Total estimate**: 20-30MB

---

## Non-Functional Requirements

### NFR-001: Reliability
- Zero crashes on valid UTF-8 input
- Deterministic output (same input → same output)
- No data loss on large inputs
- Graceful degradation if memory limited

**Target**: 99.9% reliability (no crashes per 1,000 executions)

---

### NFR-002: Maintainability
- Code coverage ≥80%
- All public functions have doc comments
- Integration tests cover each user story
- Clippy warnings treated as errors

---

### NFR-003: Usability
- `--help` output fits in 24 lines (standard terminal)
- Error messages explain problem + solution
- Common use cases require ≤20 characters of input
- Example: `tc -m gpt4 < file.txt` (assume `tc` alias)

---

### NFR-004: Performance - Streaming
For inputs >100MB:
- Process in 64KB chunks
- Update token count incrementally
- Release memory after each chunk
- Show progress on stderr if `--verbose` (post-MVP)

---

### NFR-005: Security
- No arbitrary code execution
- No network calls (offline-safe)
- No sensitive data in error messages
- Memory-safe (Rust guarantees)

---

## Out of Scope (Explicitly Not Included)

### Not in MVP
- ❌ **Cost estimation** - Pricing changes frequently, out of scope
- ❌ **Model comparison mode** - Users can call tool multiple times
- ❌ **Interactive REPL** - Use shell history instead
- ❌ **Configuration files** - Unnecessary complexity
- ❌ **JSON output format** - Wait for user demand
- ❌ **Batch file processing** - Use shell loops instead
- ❌ **Token-level highlighting** - Too complex for CLI

### Never Planned
- ❌ **API calls to model providers** - Violates offline principle
- ❌ **GUI or TUI interface** - CLI-only tool
- ❌ **Plugin system** - Adds complexity, binary size
- ❌ **Auto-update mechanism** - Use package managers

---

## Dependencies & Prerequisites

### Build Dependencies
- Rust toolchain 1.75.0+ (stable channel)
- Cargo
- Standard development tools (git, make)

### Runtime Dependencies
- None (single static binary)

### Platform Requirements
- Linux: glibc 2.27+ or musl
- macOS: 10.15+ (Catalina)
- Windows: Windows 10+

---

## Testing Strategy

### Unit Tests
Test coverage for:
- Model name resolution (exact, aliases, provider format)
- Tokenization accuracy (match reference outputs)
- Input validation (UTF-8, empty, large)
- Output formatting (all verbosity levels)
- Error messages (all error types)

**Target**: ≥80% code coverage

---

### Integration Tests
End-to-end tests for each user story:
- Pipe echo output to tool
- Redirect file input
- Test all verbosity flags
- Verify exit codes
- Check error message content

**Test Data**: Include files with:
- ASCII text
- Unicode (emoji, Chinese, Arabic)
- Empty files
- Large files (>10MB)
- Invalid UTF-8 sequences

---

### Performance Tests
Benchmark on:
- Small inputs (100 bytes, 1KB, 10KB)
- Medium inputs (100KB, 1MB)
- Large inputs (10MB, 100MB, 1GB)

**CI Performance Gate**: Fail if regression >20% on any benchmark

---

### Cross-Platform Tests
Run full test suite on:
- Ubuntu 22.04 (x64)
- macOS 12+ (x64)
- macOS 12+ (ARM64)
- Windows Server 2022 (x64)

---

## Acceptance Criteria (Feature Complete)

- [ ] All user stories implemented and tested
- [ ] `cargo test` passes on all platforms
- [ ] `cargo clippy` shows zero warnings
- [ ] Binary size ≤30MB on all platforms
- [ ] Performance benchmarks meet targets (FR-007)
- [ ] Documentation complete (`--help`, README with examples)
- [ ] CI pipeline green (build + test + release)

---

## Open Questions
*None - all clarifications resolved*

---

## Clarifications Applied

### Clarification Round 1 (2026-03-13)
**Questions Asked**: 10 questions covering model support, accuracy vs speed, output formats, model aliases, cost estimation, comparison mode, REPL mode, installation methods, config files, error handling.

**Answers Documented**:
- **Model Priority (FR-003, FR-003a)**: P1 OpenAI/Anthropic/Google, P2 Llama/Mistral, P3 DeepSeek/Qwen
- **Accuracy Strategy (FR-004)**: Smart hybrid - exact for OpenAI (MVP), estimation for others (post-MVP)
- **Output Levels (FR-005)**: Confirmed 4-level verbosity from examples
- **Model Naming (FR-003)**: Support exact + aliases + provider format, default to `gpt-3.5-turbo`
- **Cost Estimation**: Explicitly out of scope (not included)
- **Comparison Mode**: Post-MVP consideration (not in v0.1.0)
- **REPL Mode**: Not included
- **Installation Priority (FR-008)**: P1 = curl|bash + Homebrew + Cargo + GitHub Releases
- **Config Files**: Not included (unnecessary complexity)
- **Error Handling (FR-006)**: Detailed behavior for each edge case

### Updates Made
- Added FR-003a for post-MVP model support
- Clarified binary size includes only MVP models (~20-30MB)
- Added explicit "Out of Scope" section
- Documented estimation method for Claude/Gemini when exact tokenization unavailable
- Specified default model as `gpt-3.5-turbo`
- Removed cost estimation from all user stories and requirements

### Clarification Round 2 (2026-03-13): Package Naming Correction
**Issue**: Package name `token-counter` already taken on crates.io

**Research Findings**:
- Existing package: `token-counter` v0.1.0 (HuggingFace-based CLI, binary name `tc`)
- Alternative package: `tokencount` v0.1.1 also taken

**Decision**: Rename to `token-count` (User approved Option A)

**Updates Applied** (v1.0 → v1.1):
- All command examples updated: `token-counter` → `token-count`
- Binary name: `token-count`
- Cargo package: `token-count`
- No functional requirement changes (naming only)

See `.specify/RESEARCH-HOMEBREW-CARGO.md` for full research details.

---

**Next Steps**: Hand off to `modern-architect-engineer` agent for planning phase (create implementation plan, data models, API contracts).
