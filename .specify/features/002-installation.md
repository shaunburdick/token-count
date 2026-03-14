# Feature 002: Installation & Distribution

**Version**: 1.1  
**Status**: Specified (Clarifications Complete)  
**Priority**: P1 (Critical - MVP)  
**Last Updated**: 2026-03-13  
**Dependencies**: Feature 001 (Core CLI)

## Problem Statement

Even the best CLI tool is useless if users can't easily install it. Developers expect modern CLI tools to support:
- One-command installation (`curl | bash`, `brew install`, `cargo install`)
- Automatic updates via package managers
- Pre-built binaries (no compilation required)
- Cross-platform support (Linux, macOS, Windows)

**Pain Points**:
- Compiling Rust projects is slow (5-10 minutes on average laptops)
- Not all developers have Rust toolchain installed
- Manual binary downloads require finding the right platform, extracting archives, moving to PATH
- No consistency in how tools are installed across projects

## Solution Overview

Provide multiple installation methods with automated release pipeline:

### Primary Methods (P1 - MVP)
1. **curl | bash installer** - Single command for Linux/macOS
2. **Homebrew tap** - `brew install shaunburdick/tap/token-count`
3. **Cargo** - `cargo install token-count`
4. **GitHub Releases** - Manual binary downloads with checksums

### Release Automation
- GitHub Actions builds binaries on every tagged release
- Automatic Homebrew formula updates
- SHA256 checksums for verification
- Release notes generated from CHANGELOG

## User Stories

### US-010: One-Line Install (curl | bash)
**As a** developer on Linux/macOS  
**I want to** install the tool with a single curl command  
**So that** I can start using it immediately without prerequisites

**Acceptance Criteria**:
- [ ] Installer script hosted at `https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh`
- [ ] Script detects platform (Linux x64, macOS x64/ARM64)
- [ ] Downloads correct binary from latest GitHub Release
- [ ] Verifies SHA256 checksum
- [ ] Installs to `~/.local/bin` or `/usr/local/bin` (with permission check)
- [ ] Updates PATH if necessary (with user consent)
- [ ] Prints success message with usage example

**Example**:
```bash
$ curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
Detecting platform... macOS ARM64
Downloading token-count v0.1.0...
Verifying checksum... ✓
Installing to /usr/local/bin... ✓

token-count installed successfully!

Try it out:
  echo "Hello world" | token-count --model gpt-4

$ token-count --version
token-count 0.1.0
```

**Error Handling**:
- Unsupported platform → Error with message listing supported platforms
- Download failure → Retry up to 3 times, then error
- Checksum mismatch → Error, delete partial download
- Permission denied → Suggest using `sudo` or installing to home directory

---

### US-011: Homebrew Installation
**As a** macOS/Linux developer using Homebrew  
**I want to** install via `brew install`  
**So that** I can manage updates alongside other tools

**Acceptance Criteria**:
- [ ] Homebrew formula in `shaunburdick/homebrew-tap` repository
- [ ] Formula includes SHA256 checksums for all platform bottles
- [ ] `brew install shaunburdick/tap/token-count` installs latest version
- [ ] `brew upgrade token-count` updates to newer versions
- [ ] Formula updated automatically on new releases

**Example**:
```bash
$ brew tap shaunburdick/tap
$ brew install token-count

==> Downloading https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-v0.1.0-x86_64-apple-darwin.tar.gz
==> Pouring token-count-v0.1.0-x86_64-apple-darwin.tar.gz
🍺  /usr/local/Cellar/token-count/0.1.0: 1 file, 22.1MB

$ token-count --version
token-count 0.1.0

$ brew upgrade token-count
==> Upgrading shaunburdick/tap/token-count 0.1.0 -> 0.2.0
```

---

### US-012: Cargo Installation
**As a** Rust developer  
**I want to** install via `cargo install`  
**So that** I can build from source with my local toolchain

**Acceptance Criteria**:
- [ ] Published to crates.io as `token-count`
- [ ] `cargo install token-count` builds and installs to `~/.cargo/bin`
- [ ] Works with Rust 1.75.0+ (MSRV documented)
- [ ] Compilation completes in <5 minutes on modern hardware

**Example**:
```bash
$ cargo install token-count
    Updating crates.io index
  Downloaded token-count v0.1.0
  Downloaded 1 crate (45.2 KB) in 0.83s
   Compiling token-count v0.1.0
    Finished release [optimized] target(s) in 3m 42s
   Installing ~/.cargo/bin/token-count
    Installed package `token-count v0.1.0`

$ token-count --version
token-count 0.1.0
```

---

### US-013: GitHub Releases Manual Download
**As a** developer on Windows or air-gapped system  
**I want to** manually download pre-built binaries  
**So that** I can install without running scripts or package managers

**Acceptance Criteria**:
- [ ] Each release has binaries for all platforms:
  - `token-count-v{version}-x86_64-unknown-linux-gnu.tar.gz`
  - `token-count-v{version}-x86_64-apple-darwin.tar.gz`
  - `token-count-v{version}-aarch64-apple-darwin.tar.gz`
  - `token-count-v{version}-x86_64-pc-windows-msvc.zip`
- [ ] SHA256 checksums in `checksums.txt`
- [ ] Release notes include installation instructions
- [ ] Archives contain single binary (no subdirectories)

**Example**:
```bash
# Linux/macOS
$ curl -LO https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
$ tar xzf token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
$ sudo mv token-count /usr/local/bin/
$ token-count --version
token-count 0.1.0

# Windows (PowerShell)
PS> Invoke-WebRequest -Uri https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-v0.1.0-x86_64-pc-windows-msvc.zip -OutFile token-count.zip
PS> Expand-Archive token-count.zip
PS> Move-Item token-count\token-count.exe C:\Windows\System32\
PS> token-count --version
token-count 0.1.0
```

---

### US-014: Checksum Verification
**As a** security-conscious developer  
**I want to** verify binary integrity with checksums  
**So that** I can ensure downloads haven't been tampered with

**Acceptance Criteria**:
- [ ] `checksums.txt` file in each GitHub Release
- [ ] Contains SHA256 hashes for all platform binaries
- [ ] Format: `{hash} {filename}` (compatible with `shasum -c`)
- [ ] Install script verifies checksums automatically
- [ ] Manual verification instructions in README

**Example checksums.txt**:
```
a1b2c3d4e5f6... token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
f6e5d4c3b2a1... token-count-v0.1.0-x86_64-apple-darwin.tar.gz
1a2b3c4d5e6f... token-count-v0.1.0-aarch64-apple-darwin.tar.gz
6f5e4d3c2b1a... token-count-v0.1.0-x86_64-pc-windows-msvc.zip
```

**Verification**:
```bash
$ curl -LO https://github.com/shaunburdick/token-count/releases/download/v0.1.0/checksums.txt
$ shasum -a 256 -c checksums.txt
token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz: OK
```

---

## Functional Requirements

### FR-010: GitHub Actions Release Pipeline
The repository **must** have a GitHub Actions workflow that:

**Trigger**: On git tag push matching `v*.*.*` (semantic versioning)

**Jobs**:
1. **Build Matrix**:
   - Platform: Linux x64, macOS x64, macOS ARM64, Windows x64
   - Runs `cargo build --release`
   - Strips debug symbols
   - Packages binary into `.tar.gz` (Unix) or `.zip` (Windows)

2. **Generate Checksums**:
   - SHA256 for each binary archive
   - Outputs `checksums.txt`

3. **Create GitHub Release**:
   - Release title: `v{version}`
   - Release body: Content from `CHANGELOG.md` for this version
   - Attach all binary archives + checksums.txt

4. **Update Homebrew Formula** (post-MVP can be manual):
   - Clone `homebrew-tap` repository
   - Update `Formula/token-count.rb` with new version + checksums
   - Create PR or direct commit

**Workflow File**: `.github/workflows/release.yml`

---

### FR-011: Install Script Requirements
The `install.sh` script **must**:

**Platform Detection**:
- Detect OS: Linux, macOS, or unsupported
- Detect architecture: x86_64, aarch64, or unsupported
- Exit with error on unsupported platforms

**Download Logic**:
```bash
PLATFORM=$(detect_platform)  # e.g., x86_64-unknown-linux-gnu
VERSION=$(get_latest_version)  # Fetch from GitHub API or hardcoded "latest"
URL="https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/token-count-v${VERSION}-${PLATFORM}.tar.gz"
```

**Installation Steps**:
1. Download binary archive to `/tmp`
2. Download checksums.txt
3. Verify SHA256 checksum
4. Extract binary
5. Determine install directory:
   - If `~/.local/bin` exists and is in PATH → install there
   - Else if `/usr/local/bin` is writable → install there
   - Else prompt user to choose directory or run with sudo
6. Move binary to install directory
7. Chmod +x (ensure executable)
8. Clean up temp files
9. Print success message

**Error Handling**:
- Check for `curl` or `wget` (exit if neither available)
- Retry downloads up to 3 times with exponential backoff
- Checksum mismatch → delete file, exit with error
- Permission denied → provide helpful message with sudo example

**Script Location**: `install.sh` in repository root

---

### FR-012: Homebrew Formula Specification
The Homebrew formula **must** follow this structure:

**Repository**: `https://github.com/shaunburdick/homebrew-tap`

**Formula File**: `Formula/token-count.rb`

**Contents**:
```ruby
class TokenCount < Formula
  desc "Count tokens for LLM models with exact tokenization"
  homepage "https://github.com/shaunburdick/token-count"
  version "0.1.0"

  if OS.mac? && Hardware::CPU.intel?
    url "https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-v0.1.0-x86_64-apple-darwin.tar.gz"
    sha256 "abc123..."
  elsif OS.mac? && Hardware::CPU.arm?
    url "https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-v0.1.0-aarch64-apple-darwin.tar.gz"
    sha256 "def456..."
  elsif OS.linux?
    url "https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "ghi789..."
  end

  def install
    bin.install "token-count"
  end

  test do
    output = shell_output("echo 'test' | #{bin}/token-count --model gpt-4")
    assert_match /^\d+$/, output
  end
end
```

**Update Process**:
- Automated: GitHub Actions updates formula on each release
- Manual fallback: Maintainer can update via PR

---

### FR-013: Cargo Publish Requirements
The crate **must**:
- Be published to crates.io as `token-count`
- Include comprehensive README.md
- Specify MSRV in `Cargo.toml`: `rust-version = "1.75.0"`
- Include LICENSE file (MIT or Apache-2.0)
- Have keywords: `["cli", "tokens", "llm", "gpt", "tiktoken"]`
- Have categories: `["command-line-utilities", "text-processing"]`

**Cargo.toml metadata**:
```toml
[package]
name = "token-count"
version = "0.1.0"
edition = "2021"
rust-version = "1.75.0"
authors = ["Shaun Burdick <shaun.burdick@gmail.com>"]
description = "Count tokens for LLM models with exact tokenization"
homepage = "https://github.com/shaunburdick/token-count"
repository = "https://github.com/shaunburdick/token-count"
license = "MIT"
keywords = ["cli", "tokens", "llm", "gpt", "tiktoken"]
categories = ["command-line-utilities", "text-processing"]

[dependencies]
# ... dependencies here
```

**Publish Process**:
```bash
cargo publish --dry-run  # Verify first
cargo publish
```

---

### FR-014: Release Checklist
Each release **must** follow this process:

**Pre-Release**:
1. [ ] Update version in `Cargo.toml`
2. [ ] Update version in `install.sh` (if hardcoded)
3. [ ] Update `CHANGELOG.md` with all changes since last release
4. [ ] Run full test suite: `cargo test --all-features`
5. [ ] Run clippy: `cargo clippy -- -D warnings`
6. [ ] Test install script locally on Linux + macOS
7. [ ] Commit changes: `git commit -m "chore: prepare v0.1.0 release"`

**Release**:
8. [ ] Tag release: `git tag -a v0.1.0 -m "Release v0.1.0"`
9. [ ] Push tag: `git push origin v0.1.0`
10. [ ] Wait for GitHub Actions to complete (5-10 minutes)
11. [ ] Verify binaries uploaded to GitHub Release
12. [ ] Test install script: `curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/v0.1.0/install.sh | bash`
13. [ ] Test Homebrew: `brew install shaunburdick/tap/token-count`

**Post-Release**:
14. [ ] Publish to crates.io: `cargo publish`
15. [ ] Announce on social media / Reddit / HN (if significant release)
16. [ ] Monitor GitHub Issues for installation problems

---

### FR-015: Documentation Requirements
The repository **must** include:

**README.md**:
- Project description
- Installation instructions for all methods
- Quick start examples
- Link to full documentation
- Contribution guidelines

**INSTALL.md**:
- Detailed installation guide for each platform
- Manual installation instructions
- Checksum verification steps
- Troubleshooting section

**CHANGELOG.md**:
- Semantic versioning format
- All changes categorized: Added, Changed, Deprecated, Removed, Fixed, Security
- Example:
  ```markdown
  # Changelog
  
  ## [0.1.0] - 2026-03-20
  ### Added
  - Initial release with OpenAI model support
  - Support for gpt-3.5-turbo, gpt-4, gpt-4-turbo, gpt-4o
  - Verbosity levels: default, -v, -vv, -vvv
  - Install script for Linux/macOS
  - Homebrew formula
  ```

---

## Non-Functional Requirements

### NFR-006: Installation Speed
- curl | bash install: <30 seconds on broadband connection
- Homebrew install: <1 minute
- Cargo install: <5 minutes (compilation)

### NFR-007: Binary Size
- Compressed archives: <10MB (tar.gz/zip)
- Uncompressed binaries: <30MB
- Homebrew bottles optimized for size

### NFR-008: Reliability
- Install script success rate: >95% on supported platforms
- Checksum verification: 100% of installs
- GitHub Actions workflow: >98% success rate (accounting for transient failures)

### NFR-009: Security
- HTTPS-only downloads
- SHA256 checksum verification
- No arbitrary code execution in install script
- No sudo required (prefer user-local install)

### NFR-010: Compatibility
- Install script works on: Ubuntu 20.04+, macOS 10.15+, Debian 10+, Fedora 35+
- Homebrew formula works on: macOS 10.15+, Ubuntu 22.04+ (Homebrew on Linux)
- Windows: PowerShell 5.1+ for manual install

---

## Out of Scope

### Not in MVP
- ❌ **APT/RPM packages** - Requires maintaining separate packaging for each distro
- ❌ **NPM wrapper package** - Unnecessary complexity
- ❌ **Docker image** - CLI tool doesn't need containerization
- ❌ **cargo-binstall support** - Nice-to-have, not critical
- ❌ **Auto-update mechanism** - Use package managers instead
- ❌ **Installer GUI** - CLI tool, CLI install

### Never Planned
- ❌ **Snap/Flatpak/AppImage** - Overkill for single binary
- ❌ **Chocolatey (Windows package manager)** - Low adoption
- ❌ **Scoop (Windows package manager)** - Post-MVP if demand exists

---

## Testing Strategy

### Install Script Testing
**Test matrix**:
- Ubuntu 22.04 (x64)
- macOS 12 (Intel)
- macOS 13 (Apple Silicon)
- Debian 11 (x64)

**Test scenarios**:
- [ ] Fresh install to `~/.local/bin`
- [ ] Fresh install to `/usr/local/bin` (with sudo)
- [ ] Upgrade existing installation
- [ ] Network failure (mock with invalid URL)
- [ ] Checksum mismatch (tampered file)
- [ ] Unsupported platform (e.g., armv7)

---

### Homebrew Testing
**Manual testing** (CI doesn't easily support Homebrew):
- [ ] `brew tap shaunburdick/tap`
- [ ] `brew install token-count`
- [ ] `brew test token-count`
- [ ] `brew upgrade token-count` (after new release)
- [ ] `brew uninstall token-count`

---

### GitHub Actions Testing
**CI checks**:
- [ ] Workflow runs on tag push
- [ ] All platform builds succeed
- [ ] Binaries are executable
- [ ] Checksums generated correctly
- [ ] Release created with all assets
- [ ] Release notes pulled from CHANGELOG

**Test strategy**:
- Push test tags to separate branch
- Verify workflow without creating public release

---

### Cross-Platform Binary Testing
**Automated tests** (in CI):
```yaml
test-binaries:
  strategy:
    matrix:
      os: [ubuntu-22.04, macos-12, macos-13, windows-2022]
  steps:
    - name: Download binary
      run: # Download from release
    - name: Test basic functionality
      run: echo "test" | ./token-count --model gpt-4
    - name: Verify output
      run: # Check output is "1"
```

---

## Acceptance Criteria (Feature Complete)

- [ ] Install script works on Linux (Ubuntu, Debian, Fedora)
- [ ] Install script works on macOS (Intel + Apple Silicon)
- [ ] Homebrew formula available in tap
- [ ] Cargo publish to crates.io successful
- [ ] GitHub Actions release workflow complete
- [ ] All platform binaries tested and functional
- [ ] Documentation complete (README, INSTALL, CHANGELOG)
- [ ] Manual install instructions tested on all platforms
- [ ] Checksum verification works correctly
- [ ] At least 1 successful release published

---

## Dependencies

### Required Before This Feature
- Feature 001: Core CLI (must have working binary to distribute)

### Blocks Other Features
- None (installation is parallel track to core features)

---

## Open Questions
*None - all clarifications resolved*

---

## Clarifications Applied

### Clarification Round 1 (2026-03-13)
**Question 8 Answer**: P1 installation methods are curl|bash, Homebrew, Cargo, and GitHub Releases

**Updates Made**:
- Focused feature on 4 P1 methods only
- Removed NPM, APT/RPM, cargo-binstall from MVP scope
- Added explicit "Out of Scope" section
- Prioritized Homebrew tap over formula submission to homebrew-core
- Documented release automation requirements
- Added checksum verification as security requirement

### Clarification Round 2 (2026-03-13): Homebrew & Cargo Naming Corrections
**Issues Found**:
1. Homebrew tap naming was incorrect (showed wrong install command)
2. Cargo package name `token-counter` already taken on crates.io

**Research Conducted**: See `.specify/RESEARCH-HOMEBREW-CARGO.md`

**Corrections Applied** (v1.0 → v1.1):

**Homebrew Changes**:
- Repository name: Must be `homebrew-tap` (not `homebrew-token-count`)
- Tap reference: `shaunburdick/tap` (Homebrew auto-strips "homebrew-" prefix)
- Install command: `brew install shaunburdick/tap/token-count`
- Formula file: `Formula/token-count.rb`
- Formula class: `TokenCount` (not `TokenCounter`)

**Cargo Changes**:
- Package name: `token-count` (was `token-counter`, now available)
- Binary name: `token-count`
- Install command: `cargo install token-count`

**Impact**:
- Updated US-011 (Homebrew installation examples)
- Updated US-012 (Cargo installation examples)
- Updated FR-011 (install script download URLs)
- Updated FR-012 (Homebrew formula specification)
- Updated FR-013 (Cargo.toml metadata)
- All command examples throughout feature updated

---

**Next Steps**: Hand off to `modern-architect-engineer` agent for planning phase.
