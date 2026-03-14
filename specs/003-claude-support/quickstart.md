# Quickstart: Claude Model Support

**Feature**: 003-claude-support  
**Purpose**: Key validation scenarios to verify implementation correctness  
**For**: Developers implementing and testing this feature

---

## Prerequisites

### Development Environment

```bash
# Rust toolchain
rustc --version  # Should be 1.85.0+
cargo --version

# Clone and build
git checkout 003-claude-support
cargo build --release

# Verify binary
./target/release/token-count --version
```

### Optional: API Key (for accurate mode testing)

```bash
# Get API key from https://console.anthropic.com/
export ANTHROPIC_API_KEY="sk-ant-api03-..."

# Verify key is set
echo $ANTHROPIC_API_KEY
```

---

## Quick Validation Checks

### 1. Basic Estimation Mode (No API Key Required)

**Test**: Count tokens offline using estimation

```bash
echo "Hello, Claude!" | ./target/release/token-count --model claude-sonnet-4-6
```

**Expected Output**:
```
~4
```

**Verify**:
- ✅ Output starts with `~` (tilde indicates estimation)
- ✅ Number is reasonable (2-6 tokens for this input)
- ✅ No network calls made (works offline)
- ✅ Exit code 0

---

### 2. Model Alias Resolution

**Test**: Short aliases work correctly

```bash
# All should produce same result
echo "test" | ./target/release/token-count --model claude-sonnet-4-6
echo "test" | ./target/release/token-count --model sonnet
echo "test" | ./target/release/token-count --model claude
```

**Expected**: All outputs identical (`~2`)

**Verify**:
- ✅ `claude` → `claude-sonnet-4-6` (default)
- ✅ `sonnet` → `claude-sonnet-4-6`
- ✅ Case-insensitive (`CLAUDE`, `Claude`, `claude` all work)

---

### 3. List Claude Models

**Test**: Models appear in `--list-models`

```bash
./target/release/token-count --list-models
```

**Expected Output** (excerpt):
```
Anthropic Claude Models:
  
  claude-opus-4-6
    Tokenization: Estimation (±10%) or API (with --accurate)
    Context window: 1000000 tokens
    Aliases: opus-4-6, opus, anthropic/claude-opus-4-6
  
  claude-sonnet-4-6 (default Claude)
    Tokenization: Estimation (±10%) or API (with --accurate)
    Context window: 1000000 tokens
    Aliases: sonnet-4-6, sonnet, claude, anthropic/claude-sonnet-4-6
```

**Verify**:
- ✅ 8 Claude models listed (Opus/Sonnet/Haiku 4.6/4.5/4.1/4.0)
- ✅ Shows context window size
- ✅ Shows aliases
- ✅ Indicates tokenization method

---

### 4. Verbose Output (Estimation Details)

**Test**: Show estimation method with `-v`

```bash
echo "fn main() { println!(\"test\"); }" | ./target/release/token-count --model claude -v
```

**Expected Output**:
```
Model: claude-sonnet-4-6
Tokens: ~12 (estimated)
Estimation method: Adaptive (detected: Code, 3.0 chars/token)
Accuracy: ±10% target from actual count
Context window: 1000000 tokens (0.001% used)

For exact count, use: --accurate (requires ANTHROPIC_API_KEY)
```

**Verify**:
- ✅ Shows content type detection (Code/Prose/Mixed)
- ✅ Shows chars/token ratio used
- ✅ Mentions `--accurate` option
- ✅ Shows context window usage

---

### 5. API Mode Without Key (Error Handling)

**Test**: Clear error when API key missing

```bash
unset ANTHROPIC_API_KEY
echo "test" | ./target/release/token-count --model claude --accurate
```

**Expected Output** (stderr):
```
Error: Accurate mode requires ANTHROPIC_API_KEY environment variable.

Get your API key from: https://console.anthropic.com/
Then set: export ANTHROPIC_API_KEY="sk-ant-..."

For offline estimation (no API key needed), omit --accurate flag:
  token-count --model claude-sonnet-4-6
```

**Verify**:
- ✅ Clear error message
- ✅ Shows how to get API key
- ✅ Shows how to use estimation instead
- ✅ Exit code 1

---

### 6. Non-Interactive Mode Without `-y` (Error)

**Test**: Errors when piped without `-y` flag

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
cat some-file.txt | ./target/release/token-count --model claude --accurate
```

**Expected Output** (stderr):
```
Error: API call requires consent. Running in non-interactive mode (stdin not a TTY).

Options:
  1. Add -y/--yes flag to skip prompt:
       cat some-file.txt | token-count --model claude-sonnet-4-6 --accurate -y
  
  2. Use estimation mode (no API call):
       cat some-file.txt | token-count --model claude-sonnet-4-6
```

**Verify**:
- ✅ Detects non-interactive mode (stdin not TTY)
- ✅ Requires `-y` flag explicitly
- ✅ Shows both options (with -y or without --accurate)
- ✅ Exit code 1

---

### 7. API Mode with `-y` Flag (Skips Prompt)

**Test**: Non-interactive with `-y` works

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "Hello, world!" | ./target/release/token-count --model claude --accurate -y
```

**Expected Output**:
```
3
```

**Verify**:
- ✅ No prompt shown (skipped due to `-y`)
- ✅ Exact count (no `~` prefix)
- ✅ Uses Anthropic API
- ✅ Exit code 0

---

### 8. Interactive Consent Prompt (Accept)

**Test**: User accepts consent in terminal

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "test" | ./target/release/token-count --model claude --accurate
# [User types 'y' and presses Enter]
```

**Expected Interaction**:
```
This will send your input to Anthropic's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://api.anthropic.com

Proceed with API call? (y/N): y

2
```

**Verify**:
- ✅ Prompt shown on stderr (not stdout)
- ✅ Shows provider name (Anthropic)
- ✅ Shows API endpoint URL
- ✅ Default is "No" (capital N)
- ✅ After typing 'y', proceeds with API call
- ✅ Output is exact count (no `~`)

---

### 9. Interactive Consent Prompt (Decline)

**Test**: User declines consent

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
echo "test" | ./target/release/token-count --model claude --accurate
# [User types 'n' or just presses Enter]
```

**Expected Interaction**:
```
This will send your input to Anthropic's API for accurate token counting.
Your input will be transmitted over HTTPS to: https://api.anthropic.com

Proceed with API call? (y/N): n

Falling back to estimation (API call cancelled by user)

~2
```

**Verify**:
- ✅ Shows same prompt
- ✅ After declining, falls back to estimation
- ✅ Shows message explaining fallback
- ✅ Output is estimated count (with `~`)
- ✅ Exit code 0 (not an error)

---

### 10. Content Type Detection (Code vs. Prose)

**Test**: Adaptive estimation uses different ratios

```bash
# Test 1: Pure code (should use ~3.0 chars/token)
echo 'fn main() { let x = 42; }' | ./target/release/token-count --model claude -v

# Test 2: Pure prose (should use ~4.5 chars/token)
echo 'The quick brown fox jumps over the lazy dog.' | ./target/release/token-count --model claude -v
```

**Expected**:
- Code: Shows "detected: Code, 3.0 chars/token"
- Prose: Shows "detected: Prose, 4.5 chars/token"

**Verify**:
- ✅ Code input → higher token count (more symbols)
- ✅ Prose input → lower token count (longer words)
- ✅ Verbose mode shows detection result

---

### 11. API Fallback on Network Error

**Test**: Falls back to estimation if API unreachable

```bash
export ANTHROPIC_API_KEY="invalid-key"
echo "test" | ./target/release/token-count --model claude --accurate -y
```

**Expected Output**:
```
Warning: API call failed (invalid API key), falling back to estimation

~2
```

**Verify**:
- ✅ Shows warning message
- ✅ Falls back to estimation automatically
- ✅ Output is estimated count (with `~`)
- ✅ Exit code 0 (graceful degradation)

---

### 12. Large Input Handling

**Test**: Handle large files (1MB+)

```bash
# Create 1MB file
dd if=/dev/urandom bs=1024 count=1024 | base64 > large-file.txt

# Test estimation (should be fast)
time cat large-file.txt | ./target/release/token-count --model claude
```

**Expected**:
- Completes in <100ms
- Reasonable token count (350K-450K tokens for 1MB)
- No memory issues

**Verify**:
- ✅ No panic or crash
- ✅ Completes quickly (<100ms)
- ✅ Memory usage reasonable (<100MB)

---

### 13. Edge Case: Empty Input

**Test**: Handle empty string

```bash
echo "" | ./target/release/token-count --model claude
```

**Expected Output**:
```
~0
```

**Verify**:
- ✅ Returns 0 tokens
- ✅ No error
- ✅ Exit code 0

---

### 14. Edge Case: Unicode and Emoji

**Test**: Handle UTF-8 correctly

```bash
echo "Hello 世界 🌍!" | ./target/release/token-count --model claude -v
```

**Expected**:
- Counts Unicode characters correctly
- No UTF-8 errors
- Reasonable token count (4-8 tokens)

**Verify**:
- ✅ No UTF-8 errors
- ✅ Handles CJK characters
- ✅ Handles emoji

---

### 15. Integration Test: Full Pipeline

**Test**: End-to-end workflow

```bash
# 1. List models
./target/release/token-count --list-models | grep claude

# 2. Estimate tokens
echo "Build a CLI tool" | ./target/release/token-count --model claude

# 3. Get exact count (with consent)
export ANTHROPIC_API_KEY="sk-ant-..."
echo "Build a CLI tool" | ./target/release/token-count --model claude --accurate -y

# 4. Compare results
```

**Verify**:
- ✅ All commands succeed
- ✅ Estimation and API count are close (within ±10%)
- ✅ No errors or warnings (except expected consent prompts)

---

## Automated Test Commands

### Run All Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# Clippy (no warnings allowed)
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check
```

### Test Coverage

```bash
# Install tarpaulin (if not installed)
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage/

# Open coverage report
open coverage/index.html
```

**Target**: ≥80% coverage for new code

---

## Validation Script

### Accuracy Validation (Requires API Key)

```bash
# Run validation script
export ANTHROPIC_API_KEY="sk-ant-..."
./scripts/validate-claude-accuracy.sh
```

**Expected Output**:
```
Testing Claude estimation accuracy...

Tested: 127 inputs
Mean error: -8.2% (estimation tends to over-count)
Median error: -6.1%
95th percentile: 14.7%
Max error: 18.3% (technical code with many symbols)

✅ Target accuracy (±10% average) achieved
```

**Verify**:
- ✅ Mean error within ±10%
- ✅ 95th percentile within ±20%
- ✅ No catastrophic failures (>50% error)

---

## Performance Benchmarks

### Benchmark Commands

```bash
# Run criterion benchmarks
cargo bench

# Check benchmark results
open target/criterion/report/index.html
```

**Expected Performance**:
- Estimation (1KB input): <10ms
- Estimation (1MB input): <100ms
- API call (mock): <500ms

---

## Common Issues & Solutions

### Issue 1: "Unknown model: claude"

**Cause**: Model not in registry

**Solution**: Check `registry.rs` has Claude models added

```bash
grep "claude-sonnet-4-6" src/tokenizers/registry.rs
```

---

### Issue 2: Prompt doesn't show

**Cause**: Running in non-interactive mode

**Solution**: Run in terminal (not piped), or use `-y` flag

```bash
# Wrong (piped, no TTY)
cat file.txt | token-count --model claude --accurate

# Right (terminal input)
token-count --model claude --accurate < file.txt
# Or: Use -y flag
cat file.txt | token-count --model claude --accurate -y
```

---

### Issue 3: API call always fails

**Cause**: Invalid API key or network issue

**Solution**: Verify API key and network

```bash
# Test API key with curl
curl -X POST https://api.anthropic.com/v1/messages/count_tokens \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"model": "claude-sonnet-4-6", "messages": [{"role": "user", "content": "test"}]}'
```

---

## Success Criteria Checklist

### Must Have (MVP)

- [ ] All 15 quick validation checks pass
- [ ] `cargo test` passes (all tests green)
- [ ] `cargo clippy` passes (zero warnings)
- [ ] `cargo fmt --check` passes
- [ ] Integration tests pass (estimation + API modes)
- [ ] Non-interactive mode works with `-y`
- [ ] Consent prompt works in terminal
- [ ] Graceful fallback on API errors
- [ ] Accurate mode uses Anthropic API (verified with API key)
- [ ] Test coverage ≥80%

### Should Have

- [ ] Validation script shows ±10% average accuracy
- [ ] Performance benchmarks meet targets
- [ ] All edge cases handled (empty, unicode, large)
- [ ] Error messages are clear and actionable

### Documentation

- [ ] README updated with Claude examples
- [ ] CHANGELOG.md updated
- [ ] This quickstart validates all user stories

---

## Next Steps After Validation

1. ✅ All checks pass → Ready for code review
2. ❌ Some checks fail → Fix issues, re-run validation
3. 📝 Document any accuracy variance in README
4. 🚀 Create PR with test results

---

## Related Documents

- [Implementation Plan](./plan.md)
- [Data Model](./data-model.md)
- [API Contracts](./contracts/)
- [Feature Specification](../../.specify/features/003-claude-support.md)
