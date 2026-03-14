# Quickstart: Core CLI Token Counting

**Date**: 2026-03-13 | **Feature**: 001-core-cli | **Plan**: [plan.md](./plan.md)

## Purpose

This quickstart guide provides key validation scenarios for testing the token-count CLI tool during development. Use these scenarios to verify that each phase is working correctly before moving to the next.

---

## Prerequisites

### Build the Binary
```bash
cargo build --release
export PATH="./target/release:$PATH"  # Add to PATH for easy testing
```

### Verify Binary Exists
```bash
which token-count
# Expected: /path/to/token-count/target/release/token-count

token-count --version
# Expected: token-count 0.1.0
```

---

## Phase 0: Project Setup

### Test 1: Cargo Build Succeeds
```bash
cargo build
# Expected: Compiles without errors, generates debug binary

cargo build --release
# Expected: Compiles without errors, generates optimized binary
```

### Test 2: Linting Passes
```bash
cargo fmt --check
# Expected: No formatting issues

cargo clippy -- -D warnings
# Expected: No warnings or errors
```

### Test 3: Tests Run Successfully
```bash
cargo test
# Expected: All tests pass (initially just placeholder tests)
```

---

## Phase 1: Core Tokenization Logic

### Test 4: Simple Token Count (US-001)
```bash
echo -n "Hello world" | token-count --model gpt-4
# Expected: 2

echo -n "Translate this: Hello world" | token-count --model gpt-4
# Expected: 6
```

### Test 5: Model Aliases (US-002)
```bash
echo -n "Test" | token-count --model gpt4
# Expected: 1

echo -n "Test" | token-count --model GPT-4
# Expected: 1 (case-insensitive)

echo -n "Test" | token-count --model openai/gpt-4
# Expected: 1 (provider format)
```

### Test 6: Default Model (US-003)
```bash
echo -n "Hello" | token-count
# Expected: 1 (uses gpt-3.5-turbo by default)
```

### Test 7: List Models
```bash
token-count --list-models
# Expected:
# Supported Models:
#   OpenAI:
#     gpt-3.5-turbo (aliases: gpt35, gpt3.5, openai/gpt-3.5-turbo)
#     gpt-4 (aliases: gpt4, openai/gpt-4)
#     gpt-4-turbo (aliases: gpt4-turbo, openai/gpt-4-turbo)
#     gpt-4o (aliases: gpt4o, openai/gpt-4o)
```

### Test 8: Tokenization Accuracy
Test against hardcoded reference values (pre-generated using Python tiktoken):

```bash
# Reference values generated once using:
# python3 -c "import tiktoken; enc = tiktoken.encoding_for_model('gpt-4'); print(len(enc.encode('Hello world')))"

echo -n "Hello world" | token-count --model gpt-4
# Expected: 2 (verified against tiktoken 0.5.2)

echo -n "Hello 世界 🌍" | token-count --model gpt-4
# Expected: 8 (verified against tiktoken 0.5.2)
```

**Note**: Use `echo -n` to avoid counting the newline character. All test fixtures are pre-generated and stored in `tests/fixtures/tokenization_reference.json`. No runtime Python dependency required. See [Fixture Generation Guide](#fixture-generation) for regeneration instructions.

---

## Phase 2: CLI Argument Parsing

### Test 9: Help Output (US-009)
```bash
token-count --help
# Expected:
# token-count 0.1.0
# Count tokens for LLM models using exact tokenization
# 
# USAGE:
#     token-count [OPTIONS]
# 
# OPTIONS:
#     -m, --model <MODEL>      Model to use [default: gpt-3.5-turbo]
#     -v, --verbose            Increase output verbosity (can be repeated)
#         --list-models        List all supported models
#     -h, --help               Print help information
#     -V, --version            Print version information
# 
# EXAMPLES:
#     # Count tokens from stdin
#     echo "Hello world" | token-count --model gpt-4
#     ...
```

### Test 10: Version Output (US-009)
```bash
token-count --version
# Expected: token-count 0.1.0
```

### Test 11: Invalid Arguments
```bash
token-count --invalid-flag
# Expected: Error message from clap, exit code 2
```

---

## Phase 3: Input Processing

### Test 12: File Input (US-005)
```bash
echo "Hello world from file" > /tmp/test.txt
token-count --model gpt-4 < /tmp/test.txt
# Expected: 4

cat /tmp/test.txt | token-count --model gpt-4
# Expected: 4
```

### Test 13: Empty Input (US-008)
```bash
echo "" | token-count --model gpt-4
# Expected: 0

cat /dev/null | token-count --model gpt-4
# Expected: 0
```

### Test 14: Large Input (Streaming)
```bash
# Generate 1MB of text
python3 -c "print('Hello world ' * 100000)" > /tmp/large.txt
ls -lh /tmp/large.txt
# Expected: ~1.2MB file

time token-count --model gpt-4 < /tmp/large.txt
# Expected: Completes in <1 second, outputs token count
# Memory usage should be <500MB (check with /usr/bin/time -v on Linux)
```

### Test 15: Unicode Input
```bash
echo "Hello 世界 🌍 مرحبا" | token-count --model gpt-4
# Expected: Token count (should handle Unicode correctly)

echo "Emoji test: 😀 🎉 🚀 ⭐ 💯" | token-count --model gpt-4
# Expected: Token count (emoji should tokenize correctly)
```

---

## Phase 4: Output Formatting

### Test 16: Verbosity 0 - Simple Output (US-004)
```bash
echo "Hello world" | token-count --model gpt-4
# Expected: 2

echo "Hello world" | token-count --model gpt-4 -v 0
# Expected: 2 (explicit verbosity 0)
```

### Test 17: Verbosity 1 - Model Info (US-004)
```bash
echo "Hello world" | token-count --model gpt-4 -v
# Expected:
# Model: gpt-4 (cl100k_base encoding)
# Tokens: 2
```

### Test 18: Verbosity 2 - Context Window (US-004)
```bash
echo "Hello world" | token-count --model gpt-4 -vv
# Expected:
# Model: gpt-4 (cl100k_base encoding)
# Tokens: 2
# Context Window: 8,192 tokens
# Usage: 0.02%
```

### Test 19: Verbosity 3 - Debug Output (US-004)
```bash
echo "Hello world" | token-count --model gpt-4 -vvv
# Expected:
# Model: gpt-4 (cl100k_base encoding)
# Tokens: 2
# Token IDs: [15339, 1917]
# Decoded Tokens: ["Hello", " world"]
# Context Window: 8,192 tokens
# Usage: 0.02%
```

### Test 20: Verbosity 3 - Long Input (Truncation)
```bash
echo "One two three four five six seven eight nine ten eleven twelve" | token-count --model gpt-4 -vvv
# Expected:
# Model: gpt-4 (cl100k_base encoding)
# Tokens: 12
# Token IDs: [3198, 1403, 2380, 3116, 4330, 4848, 8254, 8223, 11888, 5899, ...]
# Decoded Tokens: ["One", " two", " three", " four", " five", " six", " seven", " eight", " nine", " ten", ...]
# Context Window: 8,192 tokens
# Usage: 0.15%
```

---

## Phase 5: Error Handling

### Test 21: Invalid UTF-8 (US-007)
```bash
# Create binary file
echo -n -e '\xff\xfe' > /tmp/binary.bin
cat /tmp/binary.bin | token-count --model gpt-4
# Expected:
# Error: Input contains invalid UTF-8
# 
# token-count requires valid UTF-8 text input.
# Binary files cannot be tokenized.
# Exit code: 1

echo $?
# Expected: 1
```

### Test 22: Unknown Model (US-006)
```bash
echo "Test" | token-count --model gpt5
# Expected:
# Error: Unknown model 'gpt5'
# 
# Did you mean one of these?
#   - gpt-4
#   - gpt-4o
#   - gpt-3.5-turbo
# 
# Use --list-models to see all supported models
# Exit code: 2

echo $?
# Expected: 2
```

### Test 23: Model Suggestions (Fuzzy Matching)
```bash
echo "Test" | token-count --model gpt4-turb
# Expected:
# Error: Unknown model 'gpt4-turb'
# 
# Did you mean one of these?
#   - gpt-4-turbo
#   - gpt-4
# 
# Use --list-models to see all supported models
```

### Test 24: Exit Codes
```bash
echo "Test" | token-count --model gpt-4
echo $?
# Expected: 0 (success)

echo "Test" | token-count --model invalid-model
echo $?
# Expected: 2 (user error)

echo -e '\xff\xfe' | token-count --model gpt-4
echo $?
# Expected: 1 (runtime error)
```

---

## Phase 6: Integration & Testing

### Test 25: Cross-Platform Line Endings
On Windows, create file with CRLF:
```bash
# Windows PowerShell
"Hello`r`nworld" | Out-File -Encoding ASCII test.txt
Get-Content test.txt | token-count --model gpt-4
# Expected: 2 (should handle CRLF correctly)
```

On Linux/macOS:
```bash
printf "Hello\r\nworld" | token-count --model gpt-4
# Expected: 2 (should handle CRLF correctly)
```

### Test 26: Binary Size Check (Informational)
```bash
ls -lh target/release/token-count
# Expected: 40-60MB (embedded tokenizers, acceptable per Amendment 1.3.0)

# macOS
du -h target/release/token-count
# Expected: 40-60MB

# Linux
du -h target/release/token-count
# Expected: 40-60MB

# Windows
dir target\release\token-count.exe
# Expected: 40-60MB

# Note: Binary size no longer has hard limit. Accuracy takes precedence.
```

### Test 27: Performance Benchmarks
```bash
# Install criterion (if not already in dev-dependencies)
cargo bench

# Expected output:
# tokenization/small (100 bytes)  time: ~5ms
# tokenization/medium (1MB)        time: ~50ms
# tokenization/large (100MB)       time: ~3s
```

### Test 28: Memory Usage (Large File)
```bash
# Linux (requires time package)
/usr/bin/time -v sh -c 'cat /tmp/large.txt | token-count --model gpt-4'
# Expected: Maximum resident set size: <500MB

# macOS
/usr/bin/time -l sh -c 'cat /tmp/large.txt | token-count --model gpt-4'
# Expected: maximum resident set size < 500MB
```

---

## Phase 7: Documentation & Polish

### Test 29: README Examples Work
Copy-paste examples from README and verify they work as documented.

```bash
# Example 1: Quick token count
echo "Hello world" | token-count --model gpt-4
# Expected: 2 (as shown in README)

# Example 2: From file
token-count --model claude-sonnet < document.txt
# Expected: Error (Claude not supported in MVP) or token count if implemented
```

### Test 30: Help Text Quality
```bash
token-count --help | wc -l
# Expected: ≤24 lines (should fit in standard terminal)

token-count --help
# Verify:
# - Clear usage instructions
# - Examples included
# - Default values documented
# - Flags explained
```

---

## Automated Test Suite

### Run All Tests
```bash
cargo test --all
# Expected: All tests pass

cargo test --all --release
# Expected: All tests pass (release mode)
```

### Run Integration Tests Only
```bash
cargo test --test '*'
# Expected: All integration tests pass
```

### Run Unit Tests Only
```bash
cargo test --lib
# Expected: All unit tests pass
```

### Run Benchmarks
```bash
cargo bench
# Expected: Benchmarks complete, performance targets met
```

### Code Coverage
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html
# Expected: ≥80% coverage

# Open coverage report
open tarpaulin-report.html  # macOS
xdg-open tarpaulin-report.html  # Linux
```

---

## Continuous Integration

### Local CI Simulation
```bash
# Run all CI checks locally
./scripts/ci-check.sh

# Or manually:
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all
cargo build --release
```

### GitHub Actions
After pushing to feature branch:
```bash
git push origin 001-core-cli
# Check GitHub Actions: https://github.com/shaunburdick/token-count/actions
# Expected: All checks pass (build, test, lint)
```

---

## Release Validation (Final Check)

### Test Installation Methods

**1. Cargo Install (from local)**
```bash
cargo install --path .
which token-count
# Expected: ~/.cargo/bin/token-count

token-count --version
# Expected: token-count 0.1.0
```

**2. Binary Release Simulation**
```bash
# Build release binary
cargo build --release

# Copy to system location
sudo cp target/release/token-count /usr/local/bin/
which token-count
# Expected: /usr/local/bin/token-count

token-count --version
# Expected: token-count 0.1.0
```

**3. Cross-Platform Builds**
```bash
# Build for all platforms (requires cross-compilation setup)
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu

# Verify binary sizes (informational)
ls -lh target/*/release/token-count*
# Expected: 40-60MB (acceptable per Amendment 1.3.0)
```

---

## Troubleshooting

### Binary Size Optimization (Optional)
```bash
# Check what's taking up space
cargo install cargo-bloat
cargo bloat --release

# Optional size optimizations in Cargo.toml (if needed):
# opt-level = "z"     # Optimize for size
# strip = true        # Strip debug symbols
# lto = "fat"         # Link-time optimization

# Note: Size optimization is best effort. Accuracy takes precedence.
```

### Performance Issues
```bash
# Profile with flamegraph
cargo install flamegraph
cargo flamegraph -- token-count --model gpt-4 < /tmp/large.txt

# View flamegraph.svg to identify bottlenecks
```

### Memory Leaks
```bash
# Linux: valgrind
valgrind --leak-check=full token-count --model gpt-4 < /tmp/test.txt

# macOS: instruments
instruments -t Leaks target/release/token-count
```

---

## Success Criteria Checklist

Before marking feature complete, verify:

- [ ] All 30 quickstart tests pass
- [ ] Token counts match reference fixtures (Test 8)
- [ ] Binary size tracked (Test 26, informational only)
- [ ] Performance benchmarks meet targets (Test 27)
- [ ] Memory usage <500MB for large files (Test 28)
- [ ] All unit tests pass (≥80% coverage)
- [ ] All integration tests pass
- [ ] Cross-platform tests pass (Linux, macOS, Windows)
- [ ] `cargo clippy` zero warnings
- [ ] `cargo fmt --check` passes
- [ ] Documentation is complete and accurate
- [ ] Help text fits in 24 lines (Test 30)
- [ ] All error messages are helpful (Tests 21-24)
- [ ] CI pipeline is green

---

## Fixture Generation

Test fixtures in `tests/fixtures/tokenization_reference.json` are pre-generated using Python tiktoken. **This is a one-time setup step, not required for running tests.**

### Initial Fixture Generation

```bash
# Create fixture generation script
cat > scripts/generate_fixtures.py <<'EOF'
#!/usr/bin/env python3
"""Generate tokenization reference fixtures for testing."""
import json
import tiktoken

test_cases = [
    # Basic ASCII
    "Hello world",
    "The quick brown fox jumps over the lazy dog",
    "",
    " ",
    "\n",
    
    # Unicode
    "Hello 世界 🌍",
    "Emoji test: 🎉🎊🎈",
    "Japanese: こんにちは世界",
    "Arabic: مرحبا بالعالم",
    
    # Edge cases
    "a" * 1000,  # Repeated chars
    "Word " * 100,  # Repeated words
    "Mixed 123 !@# symbols",
]

encodings = {
    "gpt2": "gpt2",
    "p50k_base": "text-davinci-003",
    "cl100k_base": "gpt-4",
    "o200k_base": "gpt-4o",
}

fixtures = {}
for enc_name, model in encodings.items():
    enc = tiktoken.encoding_for_model(model)
    fixtures[enc_name] = []
    for text in test_cases:
        token_count = len(enc.encode(text))
        fixtures[enc_name].append({
            "input": text,
            "expected_tokens": token_count
        })

with open("tests/fixtures/tokenization_reference.json", "w") as f:
    json.dump(fixtures, f, indent=2, ensure_ascii=False)

print(f"✅ Generated {sum(len(v) for v in fixtures.values())} test fixtures")
EOF

chmod +x scripts/generate_fixtures.py
```

### Regenerate Fixtures (if tiktoken-rs updates)

```bash
# Install Python tiktoken (same version as tiktoken-rs)
pip install tiktoken==0.5.2

# Generate fixtures
mkdir -p tests/fixtures
python3 scripts/generate_fixtures.py

# Verify fixtures
cat tests/fixtures/tokenization_reference.json | jq '.cl100k_base[0]'
# Expected: {"input": "Hello world", "expected_tokens": 2}
```

**When to Regenerate**:
- After updating tiktoken-rs dependency
- When adding new test cases to `generate_fixtures.py`
- If OpenAI changes encoding implementations (rare)

---

**Quickstart Version**: 1.1 | **Last Updated**: 2026-03-13
