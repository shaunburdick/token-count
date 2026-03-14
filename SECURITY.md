# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of `token-count` seriously. If you discover a security vulnerability, please report it responsibly.

### How to Report

**Email**: hello@burdick.dev

**Please include**:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

**Please DO NOT**:
- Open a public GitHub issue for security vulnerabilities
- Share the vulnerability publicly before we've had a chance to address it

### Response Timeline

- **Initial response**: Within 48 hours
- **Status update**: Within 7 days
- **Fix timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-14 days
  - Medium: 14-30 days
  - Low: Next release cycle

### Disclosure Policy

- We will acknowledge your report within 48 hours
- We will provide a detailed response within 7 days
- We will work with you to understand and resolve the issue
- We will credit you in the release notes (unless you prefer to remain anonymous)
- We will publicly disclose the vulnerability after a fix is released

## Security Best Practices

### For Users

#### Resource Limits

`token-count` processes input text and can consume memory proportional to input size.

**Recommended limits**:
```bash
# Limit virtual memory to 500MB
ulimit -v $((500 * 1024))

# Limit CPU time to 30 seconds
ulimit -t 30

# Then run token-count
echo "text" | token-count --model gpt-4
```

#### Untrusted Input

When processing untrusted input, use timeout to prevent potential hangs:

```bash
timeout 30s token-count --model gpt-4 < untrusted-input.txt
```

#### CI/CD Pipelines

Limit concurrent processes to avoid resource exhaustion:

```bash
ulimit -n 1024                    # Limit file descriptors
ulimit -v $((500 * 1024))        # Limit virtual memory
echo "text" | token-count --model gpt-4
```

### Known Limitations

#### Stack Overflow with Pathological Inputs

The underlying tiktoken-rs library can experience stack overflow when processing highly repetitive single-character inputs (e.g., 1MB+ of the same character). This is due to regex backtracking in the tokenization engine.

**Impact**: Minimal - real-world documents rarely exhibit this pattern  
**Workaround**: Break extremely large repetitive inputs into smaller chunks  
**Status**: Tracked upstream in tiktoken-rs

**Not considered a security vulnerability** as it requires intentionally crafted input that doesn't represent legitimate use cases.

### Supply Chain Security

#### Binary Verification

All pre-built binaries include SHA256 checksums for verification:

```bash
# Download checksums
curl -LO "https://github.com/shaunburdick/token-count/releases/download/v0.1.0/checksums.txt"

# Verify downloaded binary
grep "token-count-0.1.0-x86_64-unknown-linux-gnu.tar.gz" checksums.txt | shasum -a 256 -c -
```

The install script automatically verifies checksums before installation.

#### Dependency Auditing

We regularly audit dependencies for known vulnerabilities:

```bash
# Check for vulnerabilities (done in CI)
cargo audit

# View dependency tree
cargo tree
```

**Current status** (as of 2026-03-14):
- 0 critical vulnerabilities
- 0 high vulnerabilities
- 0 medium vulnerabilities
- 5 direct dependencies (all audited)

### Build Security

#### Release Process

1. **Automated builds**: GitHub Actions builds all binaries in isolated runners
2. **Checksum generation**: SHA256 hashes computed for all artifacts
3. **Reproducible builds**: Pinned Rust version (1.85.0) and locked dependencies
4. **No manual steps**: Reduces risk of human error or tampering

#### Code Review

- All code changes reviewed before merging
- Automated testing (100 tests) on every commit
- Strict linting with zero warnings tolerated
- No disabled security checks or suppressions

### Runtime Security

#### Memory Safety

Rust's memory safety guarantees prevent common vulnerabilities:
- No buffer overflows
- No use-after-free
- No null pointer dereferences
- No data races (when using threading)

#### Input Validation

- **UTF-8 validation**: All input validated before processing
- **Error handling**: Clear error messages, no panics in normal operation
- **Resource limits**: Documented maximum input size (100MB)

#### No Network Access

`token-count` is a fully offline tool:
- No network requests during operation
- No telemetry or analytics
- No automatic updates
- All tokenizers embedded in binary

## Security Audit History

| Date | Auditor | Findings | Status |
|------|---------|----------|--------|
| 2026-03-13 | Internal | 0 vulnerabilities | All clear ✅ |

## Security Updates

Security updates are released as patch versions (e.g., 0.1.1) and documented in the [CHANGELOG](CHANGELOG.md).

To update:
```bash
# Install script
curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash

# Homebrew
brew upgrade token-count

# Cargo
cargo install token-count --force
```

## Contact

- **Security issues**: hello@burdick.dev (private)
- **General issues**: [GitHub Issues](https://github.com/shaunburdick/token-count/issues) (public)
- **Security advisories**: [GitHub Security Advisories](https://github.com/shaunburdick/token-count/security/advisories)

## Acknowledgments

We appreciate responsible disclosure and will publicly acknowledge security researchers who report vulnerabilities (with their permission).

---

**Last updated**: 2026-03-14  
**Policy version**: 1.0
