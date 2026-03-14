# Research Findings: Homebrew & Cargo Naming

**Date**: 2026-03-13  
**Researcher**: Spec-Driven Planner Agent

---

## Issue 1: Homebrew Tap Naming ❌

### Problem
The specification incorrectly stated:
```bash
brew install shaunburdick/tap/token-counter
```

### Correct Homebrew Tap Naming Convention

According to [Homebrew Documentation](https://docs.brew.sh/Taps):

**Repository naming**:
- GitHub repository MUST be named: `homebrew-<tapname>`
- The prefix "homebrew-" is NOT optional

**Tap reference format**:
- Format: `<user>/<tapname>` (NOT `<user>/tap/<formulaname>`)
- The "homebrew-" prefix is automatically inferred

**Example**:
```bash
# Repository name: shaunburdick/homebrew-tap
# Tap command: brew tap shaunburdick/tap
# Install command: brew install shaunburdick/tap/token-count
```

### Corrected Installation Flow

```bash
# 1. Add the tap (one-time setup)
brew tap shaunburdick/tap
# This clones: https://github.com/shaunburdick/homebrew-tap

# 2. Install the formula
brew install token-count
# OR with full reference:
brew install shaunburdick/tap/token-count
```

### Repository Structure
```
shaunburdick/homebrew-tap/
├── Formula/
│   └── token-count.rb      # NOT token-counter.rb
├── .github/
│   └── workflows/
│       └── tests.yml
└── README.md
```

**Key Points**:
- Repository: `homebrew-tap` (not `homebrew-token-counter`)
- Formula file: `token-count.rb` (matches binary name)
- Tap reference: `shaunburdick/tap` (homebrew- prefix auto-stripped)

---

## Issue 2: Cargo Package Name Conflict ❌

### Problem
The specification assumed `token-counter` was available on crates.io.

### Finding: Name Already Taken
```bash
$ cargo info token-counter
token-counter #cli #tokenizer #nlp
`wc` for tokens: count tokens in files with HF Tokenizers
version: 0.1.0
repository: https://github.com/EndlessReform/token-counter
```

**Conflict Details**:
- Package: `token-counter` v0.1.0
- Created: Recently (CLI for HuggingFace tokenizers)
- Binary name: `tc` (token count)
- Purpose: Similar to our project (token counting CLI)
- Active: 12 stars, 2 forks on GitHub

### Other Taken Names
- `tokencount` v0.1.1 - "Parallel CLI that counts GPT-style tokens"
- `llm-tokenizer` v1.3.0 - Library (not CLI)

### Available Alternative Names

After research, these names are AVAILABLE on crates.io:

| Name | Available? | Binary Name | Pros | Cons |
|------|-----------|-------------|------|------|
| `token-count` ✅ | YES | `token-count` | Simple, descriptive | Hyphen in binary name |
| `tokcount` ✅ | YES | `tokcount` | Short, memorable | Loses "token" clarity |
| `tcount` ✅ | YES | `tcount` | Very short (like `wc`) | Too generic |
| `llm-token-count` ✅ | YES | `llm-token-count` | Descriptive, SEO-friendly | Longer |
| `tiktoken-cli` ✅ | YES | `tiktoken` | Matches library name | Ties to OpenAI only |

---

## Recommendation: Use `token-count`

### Rationale

**Pros**:
1. ✅ Available on crates.io
2. ✅ Descriptive and clear purpose
3. ✅ Consistent with project name
4. ✅ Good SEO (people search "token count")
5. ✅ Matches Homebrew formula naming convention

**Cons**:
1. ⚠️ Hyphen in binary name (requires quoting in some contexts)
2. ⚠️ Slightly longer than `tc` or `tokcount`

**Mitigation**:
- Hyphens in binary names are common in Rust CLI tools (e.g., `cargo-watch`, `git-cliff`)
- Users can create aliases: `alias tc=token-count`
- Package managers handle hyphens naturally

### Alternative: `tokcount`

If you prefer NO hyphen:

**Pros**:
1. ✅ Short, easy to type
2. ✅ Available on crates.io
3. ✅ No hyphen (works in all contexts)

**Cons**:
1. ❌ Less clear (what is "tok"?)
2. ❌ Harder to discover via search
3. ❌ Slightly less professional

---

## Updated Naming Scheme

### Recommended: `token-count`

| Context | Name | Example |
|---------|------|---------|
| **Crates.io package** | `token-count` | `cargo install token-count` |
| **Binary name** | `token-count` | `echo "hi" \| token-count` |
| **GitHub repo** | `token-count` | `github.com/shaunburdick/token-count` |
| **Homebrew repo** | `homebrew-tap` | `github.com/shaunburdick/homebrew-tap` |
| **Homebrew formula** | `token-count.rb` | `Formula/token-count.rb` |
| **Homebrew install** | `shaunburdick/tap/token-count` | `brew install shaunburdick/tap/token-count` |

### File Renames Needed

```bash
# Repository rename (if created)
github.com/shaunburdick/token-counter → token-count

# Binary name in code
"token-counter" → "token-count"

# Homebrew formula
Formula/token-counter.rb → Formula/token-count.rb

# Cargo.toml
name = "token-counter" → name = "token-count"
```

---

## Corrected Homebrew Installation Instructions

### Setup (First Time)

```bash
# Create Homebrew tap repository
$ brew tap-new shaunburdick/homebrew-tap
Initialized empty Git repository in /opt/homebrew/Library/Taps/shaunburdick/homebrew-tap/.git/

# Push to GitHub
$ gh repo create shaunburdick/homebrew-tap --public --push \
    --source "$(brew --repository shaunburdick/tap)"
✓ Created repository shaunburdick/homebrew-tap
```

### Formula Creation

```bash
# Create formula (after first release)
$ brew create https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-v0.1.0-x86_64-apple-darwin.tar.gz \
    --tap shaunburdick/tap \
    --set-name token-count

# Editing /opt/homebrew/Library/Taps/shaunburdick/homebrew-tap/Formula/token-count.rb
```

### User Installation

```bash
# Method 1: Short form (after tapping)
$ brew tap shaunburdick/tap
$ brew install token-count

# Method 2: Full form (no tapping needed)
$ brew install shaunburdick/tap/token-count

# Verify
$ token-count --version
token-count 0.1.0
```

---

## Impact on Specifications

### Files Requiring Updates

1. **Constitution** (`.specify/memory/constitution.md`)
   - Update package name in technical stack table
   - Update Cargo.toml example

2. **Feature 001** (`.specify/features/001-core-cli.md`)
   - Update all command examples
   - Update FR-013 (Cargo publish section)

3. **Feature 002** (`.specify/features/002-installation.md`)
   - Update US-011 (Homebrew installation)
   - Update US-012 (Cargo installation)
   - Update FR-012 (Homebrew formula specification)
   - Update FR-013 (Cargo.toml metadata)

4. **Project Summary** (`.specify/PROJECT-SUMMARY.md`)
   - Update all references to package name

5. **README** (`README.md`)
   - Update installation instructions
   - Update repository references

### Global Find/Replace

```bash
# Package names
token-counter → token-count

# Homebrew instructions
brew install shaunburdick/tap/token-counter → brew install shaunburdick/tap/token-count
brew install token-counter → brew install token-count

# Repository references
token-counter/ → token-count/
token-counter.rb → token-count.rb

# Binary execution
token-counter → token-count (in command examples)

# Cargo commands
cargo install token-counter → cargo install token-count
```

---

## Action Items

### For User (Immediate)

**Decision Required**: Choose final package name

**Option A: `token-count` (RECOMMENDED)**
- Descriptive, clear, good SEO
- Hyphen in name (minor inconvenience)
- Consistent across all contexts

**Option B: `tokcount`**
- No hyphen, shorter
- Less discoverable, less clear

**Option C: Other (suggest alternative)**

### For Spec Updates (After Decision)

1. [ ] Update constitution with correct package name
2. [ ] Update Feature 001 with correct command examples
3. [ ] Update Feature 002 with correct Homebrew instructions
4. [ ] Update Project Summary with correct references
5. [ ] Update README with correct installation commands
6. [ ] Verify all documentation consistency

---

## References

- [Homebrew Taps Documentation](https://docs.brew.sh/Taps)
- [How to Create and Maintain a Tap](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
- [Homebrew Tap Naming Discussion](https://github.com/orgs/Homebrew/discussions/6656)
- [Crates.io Search Results](https://crates.io)
- [Existing token-counter Package](https://github.com/EndlessReform/token-counter)

---

**Status**: Awaiting user decision on package name  
**Next Step**: Update all specifications once name is finalized
