# ✅ Specifications Updated: `token-count` Naming Applied

**Date**: 2026-03-13  
**Update Version**: Spec 1.1.0  
**Status**: Complete

---

## 🎯 Changes Summary

All specifications have been updated from `token-counter` to `token-count` following your approval of Option A.

### What Was Changed

#### 1. **Constitution** (v1.0.0 → v1.1.0)
- Updated architecture diagram: `token-counter` → `token-count`
- Added Amendment 1.1.0 documenting the naming change
- Rationale and impact documented

**File**: `.specify/memory/constitution.md`

#### 2. **Feature 001: Core CLI** (v1.0 → v1.1)
- Updated all 28 command examples
- Updated binary name references
- Added clarification round 2 documenting the change
- No functional requirement changes (naming only)

**File**: `.specify/features/001-core-cli.md`

#### 3. **Feature 002: Installation** (v1.0 → v1.1)
- Corrected Homebrew tap naming:
  - Repository: `homebrew-tap` (not `homebrew-token-count`)
  - Install: `brew install shaunburdick/tap/token-count`
  - Formula file: `Formula/token-count.rb`
  - Formula class: `TokenCount` (not `TokenCounter`)
- Updated Cargo package name throughout
- Updated all installation examples (curl, Homebrew, Cargo, GitHub Releases)
- Updated FR-012 (Homebrew formula specification)
- Updated FR-013 (Cargo.toml metadata)
- Added clarification round 2 with full Homebrew corrections

**File**: `.specify/features/002-installation.md`

#### 4. **Project Summary**
- Updated all references to `token-count`
- Updated repository URLs
- Updated installation commands
- Added "Specification Updates" section documenting this change

**File**: `.specify/PROJECT-SUMMARY.md`

#### 5. **README**
- Updated project title
- Updated all command examples
- Updated installation instructions (Homebrew, Cargo, curl)
- Updated repository references

**File**: `README.md`

#### 6. **New Documents Created**

**Research Findings**:
- Detailed analysis of Homebrew naming conventions
- Cargo package availability research
- Alternative name evaluation
- Rationale for `token-count` choice

**File**: `.specify/RESEARCH-HOMEBREW-CARGO.md`

**Specification Changelog**:
- Tracks all specification changes
- Documents version history
- Explains impact of each change

**File**: `.specify/CHANGELOG.md`

---

## 📊 Updated Naming Scheme

| Context                 | Name                                      |
| ----------------------- | ----------------------------------------- |
| **Crates.io package**   | `token-count`                               |
| **Binary name**         | `token-count`                               |
| **GitHub repository**   | `shaunburdick/token-count`                  |
| **Homebrew repository** | `shaunburdick/homebrew-tap`                 |
| **Homebrew tap**        | `shaunburdick/tap`                          |
| **Homebrew formula**    | `Formula/token-count.rb`                    |
| **Formula class**       | `TokenCount`                                |
| **Install (Homebrew)**  | `brew install shaunburdick/tap/token-count` |
| **Install (Cargo)**     | `cargo install token-count`                 |
| **Command usage**       | `echo "test" \| token-count --model gpt-4`  |

---

## 🔍 Key Corrections Made

### Homebrew Corrections (Critical)

**Before** (INCORRECT):
```bash
# Wrong repository name
shaunburdick/homebrew-token-counter

# Wrong install command
brew install shaunburdick/tap/token-counter

# Wrong formula class
class TokenCounter < Formula
```

**After** (CORRECT):
```bash
# Correct repository name
shaunburdick/homebrew-tap

# Correct install command
brew install shaunburdick/tap/token-count
# OR after tapping:
brew tap shaunburdick/tap
brew install token-count

# Correct formula class
class TokenCount < Formula
```

### Cargo Corrections

**Before** (CONFLICT):
```toml
# This name was already taken!
name = "token-counter"
```

**After** (AVAILABLE):
```toml
# This name is available
name = "token-count"
```

---

## 📋 Files Modified

```
.specify/
├── memory/
│   └── constitution.md              ✏️ Updated (v1.1.0)
├── features/
│   ├── 001-core-cli.md             ✏️ Updated (v1.1)
│   └── 002-installation.md         ✏️ Updated (v1.1)
├── PROJECT-SUMMARY.md               ✏️ Updated
├── RESEARCH-HOMEBREW-CARGO.md       ✨ New
└── CHANGELOG.md                     ✨ New

README.md                             ✏️ Updated
```

**Total Changes**:
- 5 existing files updated
- 2 new documents created
- 28+ command examples corrected
- 0 functional requirements changed

---

## ✅ Verification Checklist

Verified that all occurrences were updated:

- [x] Constitution architecture diagram
- [x] Constitution amendment added
- [x] Feature 001: All command examples (28 instances)
- [x] Feature 001: Clarification section updated
- [x] Feature 002: Homebrew installation (US-011)
- [x] Feature 002: Cargo installation (US-012)
- [x] Feature 002: GitHub Releases (US-013)
- [x] Feature 002: Homebrew formula specification (FR-012)
- [x] Feature 002: Cargo.toml metadata (FR-013)
- [x] Feature 002: Clarification section updated
- [x] Project Summary: All references
- [x] Project Summary: Repository URLs
- [x] README: Title and description
- [x] README: All installation commands
- [x] README: All example commands
- [x] Research document created
- [x] Changelog created
- [x] Version numbers bumped (1.0 → 1.1)

---

## 🎓 What This Means Going Forward

### For Implementation
When development begins:

1. **Create GitHub Repository**: `shaunburdick/token-count` (not token-counter)
2. **Cargo.toml**: Use `name = "token-count"`
3. **Binary Name**: `token-count` (in `[[bin]]` section)
4. **Homebrew Tap Repository**: Create `shaunburdick/homebrew-tap`
5. **Formula File**: Create `Formula/token-count.rb` with class `TokenCount`

### For Users
Installation commands:

```bash
# Homebrew (after tap created)
brew tap shaunburdick/tap
brew install token-count

# Cargo (after published)
cargo install token-count

# Usage
echo "Hello world" | token-count --model gpt-4
```

### For Documentation
All future documentation should use:
- Package name: `token-count`
- Binary name: `token-count`
- No references to `token-counter`

---

## 🚀 Next Steps

### Immediate (Complete)
- [x] All specifications updated
- [x] Version numbers bumped
- [x] Clarifications documented
- [x] Research findings captured
- [x] Changelog created

### When Implementation Starts
1. [ ] Create GitHub repository: `shaunburdick/token-count`
2. [ ] Initialize Cargo project with correct name
3. [ ] Create Homebrew tap repository: `homebrew-tap`
4. [ ] Verify binary name in build output
5. [ ] Test installation commands match specifications

### For User (Now)
**Action**: Review the updated specifications

The naming corrections are complete and documented. You can now:
1. Review the updated specs (if desired)
2. Approve and hand off to `modern-architect-engineer` for planning phase

All specifications remain **unambiguous and implementable**—only names changed, no functional changes.

---

## 📚 Reference Documents

- **Research**: `.specify/RESEARCH-HOMEBREW-CARGO.md` - Why we chose `token-count`
- **Changelog**: `.specify/CHANGELOG.md` - Full version history
- **Constitution**: `.specify/memory/constitution.md` - v1.1.0 with Amendment
- **Feature 001**: `.specify/features/001-core-cli.md` - v1.1 with updated examples
- **Feature 002**: `.specify/features/002-installation.md` - v1.1 with Homebrew corrections

---

**Status**: All Specifications Updated ✅  
**Ready For**: Planning Phase 🏗️  
**Blocker**: None
