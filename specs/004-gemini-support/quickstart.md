# Quickstart: Feature 004 Validation Guide

**Feature**: Google Gemini Token Counting Support  
**Branch**: `004-gemini-support`  
**Date**: 2026-03-14

---

## Setup Instructions

### 1. Checkout Feature Branch

```bash
git checkout 004-gemini-support
```

### 2. Build Release Binary

```bash
cargo build --release
```

**Expected output**:
```
   Compiling gemini-tokenizer v0.2.0
   Compiling token-count v0.3.0
    Finished release [optimized] target(s) in 45.2s
```

### 3. Verify Binary Size

```bash
ls -lh target/release/token-count
```

**Expected**: ~11-12MB (target: <15MB)

### 4. Run All Tests

```bash
cargo test
```

**Expected**: 187+ tests pass (152 existing + 35+ new)

### 5. Run Linter

```bash
cargo clippy -- -D warnings
```

**Expected**: Zero warnings

### 6. Check Formatting

```bash
cargo fmt --check
```

**Expected**: No formatting issues

---

## Key User Flows to Test Manually

### Flow 1: Quick Gemini Token Count (US-015)

**Goal**: Pipe text and get accurate token count

```bash
echo "Hello, Gemini!" | ./target/release/token-count --model gemini
```

**Expected Output**:
```
3
```

**Validation**:
- ✅ Output is just the number (default verbosity level 0)
- ✅ Exit code 0
- ✅ Completes in <10ms

**Verbose Mode**:
```bash
echo "Hello, Gemini!" | ./target/release/token-count --model gemini -v
```

**Expected Output**:
```
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 3
Context window: 1000000 tokens (0.0003% used)
```

---

### Flow 2: Model Aliases (US-016)

**Goal**: Verify all aliases work correctly

```bash
# Short alias
echo "test" | ./target/release/token-count --model gemini
echo "test" | ./target/release/token-count --model gemini-pro
echo "test" | ./target/release/token-count --model gemini-flash
echo "test" | ./target/release/token-count --model gemini-lite

# Provider format
echo "test" | ./target/release/token-count --model google/gemini
echo "test" | ./target/release/token-count --model google/gemini-pro

# Case-insensitive
echo "test" | ./target/release/token-count --model GEMINI
echo "test" | ./target/release/token-count --model Gemini
```

**Expected**: All commands return `1` (same token count)

---

### Flow 3: Multiple Model Versions (US-017)

**Goal**: Verify all 8 Gemini models work

```bash
# Gemini 3.x (Preview)
echo "test" | ./target/release/token-count --model gemini-3.1-pro-preview
echo "test" | ./target/release/token-count --model gemini-3-flash-preview
echo "test" | ./target/release/token-count --model gemini-3.1-flash-lite-preview

# Gemini 2.5 (Deprecated June 2026)
echo "test" | ./target/release/token-count --model gemini-2.5-pro
echo "test" | ./target/release/token-count --model gemini-2.5-flash
echo "test" | ./target/release/token-count --model gemini-2.5-flash-lite

# Gemini 1.5 (Legacy)
echo "test" | ./target/release/token-count --model gemini-1.5-pro
echo "test" | ./target/release/token-count --model gemini-1.5-flash
```

**Expected**: All commands return `1` (same tokenizer for all models)

---

### Flow 4: Context Window Validation (US-018)

**Goal**: Verify context window sizes are correct

```bash
# Gemini 1.5 Pro (2M context)
echo "test" | ./target/release/token-count --model gemini-1.5-pro -v

# All other models (1M context)
echo "test" | ./target/release/token-count --model gemini-3-flash-preview -v
```

**Expected Output (1.5-pro)**:
```
Model: gemini-1.5-pro (gemini-gemma3)
Tokens: 1
Context window: 2000000 tokens (0.00005% used)
```

**Expected Output (others)**:
```
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 1
Context window: 1000000 tokens (0.0001% used)
```

---

### Flow 5: List Gemini Models (US-019)

**Goal**: Verify all Gemini models appear in list

```bash
./target/release/token-count --list-models | grep -A 30 "Google Gemini"
```

**Expected Output** (sample):
```
Google Gemini models:

  gemini-3.1-pro-preview
    Encoding: gemini-gemma3
    Context window: 1000000 tokens
    Aliases: gemini-pro, gemini-3-pro, gemini-3.1-pro, google/gemini-pro

  gemini-3-flash-preview (default)
    Encoding: gemini-gemma3
    Context window: 1000000 tokens
    Aliases: gemini, gemini-flash, gemini-3-flash, google/gemini, google/gemini-flash

  gemini-3.1-flash-lite-preview
    Encoding: gemini-gemma3
    Context window: 1000000 tokens
    Aliases: gemini-lite, gemini-3-lite, gemini-3.1-lite, google/gemini-lite

  [... 5 more models ...]
```

**Validation**:
- ✅ All 8 models listed
- ✅ Models sorted by generation (3.x, then 2.5, then 1.5)
- ✅ Default model marked (`gemini-3-flash-preview`)
- ✅ Context windows correct (1M or 2M)
- ✅ Aliases shown for each model

---

### Flow 6: Debug Mode Token Details (US-020)

**Goal**: Verify debug mode shows token IDs

```bash
echo "Hello, world!" | ./target/release/token-count --model gemini -vvv
```

**Expected Output**:
```
Model: gemini-3-flash-preview (gemini-gemma3)
Tokens: 4

Token details (showing first 4):
  [8699] "Hello"
  [235269] ","
  [2134] " world"
  [235341] "!"

Note: Use compute_tokens() API for programmatic access to all tokens.
```

**Validation**:
- ✅ Shows token IDs in brackets
- ✅ Shows decoded token strings in quotes
- ✅ Limits output for large inputs (first 20 tokens)

---

## Edge Case Testing

### Edge Case 1: Empty Input

```bash
echo -n "" | ./target/release/token-count --model gemini
```

**Expected Output**: `0`

---

### Edge Case 2: Invalid UTF-8

```bash
# Create binary file
echo -e '\xFF\xFE' > /tmp/binary.bin
./target/release/token-count --model gemini < /tmp/binary.bin
```

**Expected Output**:
```
Error: Input contains invalid UTF-8 at byte 0
```

**Expected Exit Code**: 1

---

### Edge Case 3: Unknown Model (Typo)

```bash
echo "test" | ./target/release/token-count --model gemini-4
```

**Expected Output**:
```
Error: Unknown model: 'gemini-4'

Did you mean one of these Gemini models?
  - gemini-3-flash-preview
  - gemini-3.1-pro-preview
  - gemini-3.1-flash-lite-preview

See all models: token-count --list-models
```

**Expected Exit Code**: 2

---

### Edge Case 4: Large Input (1MB)

```bash
# Generate 1MB file
dd if=/dev/urandom bs=1024 count=1024 | base64 > /tmp/large.txt

# Time the tokenization
time ./target/release/token-count --model gemini < /tmp/large.txt
```

**Expected**:
- ✅ Completes in <100ms
- ✅ No memory issues
- ✅ Returns valid token count

---

### Edge Case 5: Gemini 1.5 Pro (2M Context Window)

```bash
# Create large file (100K tokens ≈ 400KB text)
cat /tmp/large.txt | ./target/release/token-count --model gemini-1.5-pro -v
```

**Expected Output** (sample):
```
Model: gemini-1.5-pro (gemini-gemma3)
Tokens: 95234
Context window: 2000000 tokens (4.76% used)
```

**Validation**:
- ✅ Context window is 2M (not 1M)
- ✅ Percentage calculated correctly

---

## Performance Validation

### Benchmark 1: Small Input (<10KB)

```bash
# Create 1KB file
echo "Hello Gemini" > /tmp/small.txt

# Benchmark (requires criterion)
cargo bench --bench tokenization -- gemini_small
```

**Expected**: <10ms (target), ~1ms (actual)

---

### Benchmark 2: Medium Input (1MB)

```bash
# Use large.txt from edge case testing

cargo bench --bench tokenization -- gemini_medium
```

**Expected**: <100ms (target), ~50ms (actual)

---

### Benchmark 3: Large Input (100MB)

```bash
# Generate 100MB file
dd if=/dev/urandom bs=1024 count=102400 | base64 > /tmp/huge.txt

# Time tokenization
time ./target/release/token-count --model gemini < /tmp/huge.txt
```

**Expected**: <10 seconds

---

## Cross-Platform Validation

### Linux (x86_64)

```bash
cargo build --release --target x86_64-unknown-linux-gnu
./target/x86_64-unknown-linux-gnu/release/token-count --model gemini --version
```

**Expected**: Binary runs without errors

---

### macOS (Intel)

```bash
cargo build --release --target x86_64-apple-darwin
./target/x86_64-apple-darwin/release/token-count --model gemini --version
```

**Expected**: Binary runs without errors

---

### macOS (Apple Silicon)

```bash
cargo build --release --target aarch64-apple-darwin
./target/aarch64-apple-darwin/release/token-count --model gemini --version
```

**Expected**: Binary runs without errors

---

### Windows (x86_64)

```bash
cargo build --release --target x86_64-pc-windows-gnu
# Test on Windows machine
```

**Expected**: Binary runs without errors

---

## Accuracy Validation (One-Time)

### Compare with Google's Python SDK

**Goal**: Verify 100% token count accuracy

```bash
# Python script (requires google-generativeai)
python3 << 'EOF'
import google.generativeai as genai

test_cases = [
    "Hello, Gemini!",
    "The quick brown fox jumps over the lazy dog.",
    "# Code comment\ndef hello():\n    print('world')",
    "Unicode: 你好世界 🚀",
    "A" * 1000,  # Repeated characters
]

model = genai.GenerativeModel("gemini-3-flash-preview")

for text in test_cases:
    count = model.count_tokens(text).total_tokens
    print(f"{count}\t{text[:50]}")
EOF
```

**Then compare with Rust**:

```bash
echo "Hello, Gemini!" | ./target/release/token-count --model gemini
# Should match Python output
```

**Expected**: 100% match across all test cases

---

## Documentation Validation

### Verify README Updates

```bash
grep -A 10 "Gemini" README.md
```

**Expected**: Gemini models mentioned in supported models section

---

### Verify CHANGELOG Entry

```bash
grep -A 20 "0.3.0" CHANGELOG.md
```

**Expected**: Comprehensive v0.3.0 entry with Gemini support details

---

## Cleanup

```bash
# Remove test files
rm /tmp/binary.bin /tmp/large.txt /tmp/small.txt /tmp/huge.txt
```

---

## Pre-Commit Checklist

Before committing code, verify:

- [ ] All 187+ tests pass (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt --check`)
- [ ] Release build succeeds (`cargo build --release`)
- [ ] Binary size <15MB
- [ ] All 6 user flows work as expected
- [ ] All 5 edge cases handled correctly
- [ ] Performance benchmarks meet targets
- [ ] Documentation updated (README, CHANGELOG)
- [ ] Manual validation complete (this guide)

---

## Post-Merge Validation

After merging to main:

- [ ] CI passes on all platforms (Linux, macOS Intel/ARM, Windows)
- [ ] Release binary builds successfully
- [ ] Binary size within budget
- [ ] No regressions in existing features (OpenAI, Claude)
- [ ] GitHub release created with binaries
- [ ] Homebrew formula updated (if applicable)

---

## Troubleshooting

### Issue: "Failed to initialize Gemini tokenizer"

**Cause**: gemini-tokenizer initialization failure

**Solution**:
1. Check gemini-tokenizer version: `cargo tree | grep gemini-tokenizer`
2. Verify sentencepiece is installed correctly
3. Check for binary corruption (re-build from scratch)

---

### Issue: Token counts don't match Google's SDK

**Cause**: Version mismatch or tokenizer issue

**Solution**:
1. Verify gemini-tokenizer version (should be 0.2.0+)
2. Check if Google updated their tokenizer (breaking change)
3. Run comparison tests with multiple inputs
4. File issue with exact reproduction steps

---

### Issue: Binary size >15MB

**Cause**: Debug symbols or unoptimized build

**Solution**:
1. Verify release profile settings in Cargo.toml (LTO, strip, opt-level=3)
2. Check for duplicate dependencies: `cargo tree --duplicates`
3. Ensure gemini-tokenizer doesn't embed duplicate models

---

## Success Criteria

✅ Feature is ready for release if:

- All tests pass (187+ tests, zero failures)
- All user flows work correctly
- All edge cases handled gracefully
- Performance meets targets (<10ms small, <100ms medium)
- Binary size <15MB
- Documentation complete
- No regressions in existing features
- Cross-platform compatibility verified

---

**Status**: Validation guide complete  
**Next**: Move to Phase 5 (Tasking) - Create `tasks.md` with ordered task list
