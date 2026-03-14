# Quickstart: Installation & Distribution Testing

**Feature**: 002-installation  
**Purpose**: Validation scenarios for testing all installation methods  
**Audience**: Developers implementing or testing this feature

---

## Prerequisites

### Required Access
- GitHub repository: `shaunburdick/token-count` (write access)
- GitHub Packages/Releases (automatic with repo access)
- crates.io account (for cargo publish)
- Homebrew tap repository created: `shaunburdick/homebrew-tap`

### Required Tools
- **Development machine**: Linux or macOS
- **Test machines**: Ubuntu 22.04, macOS (Intel), macOS (Apple Silicon) - can use VMs or GitHub Actions
- **CLI tools**: git, cargo, gh (GitHub CLI), shellcheck, curl, tar

### Repository State
- Feature branch `002-installation` created
- Feature 001 (Core CLI) complete and merged to main
- v0.1.0 binary ready (9.2MB at target/release/token-count)

---

## Phase 1: GitHub Actions Release Workflow

### Setup
```bash
# Create workflow file
mkdir -p .github/workflows
touch .github/workflows/release.yml
```

### Validation Steps

#### 1.1 Workflow Syntax Validation
```bash
# Install GitHub CLI (if not installed)
# brew install gh  # macOS
# apt install gh   # Ubuntu

# Validate workflow syntax
gh workflow view release  # Should show workflow details

# Or use act (local GitHub Actions runner) for testing
brew install act
act -l  # List jobs in workflow
```

**Expected output**:
```
NAME          STATE   ID
Release       active  1234567
```

#### 1.2 Test Workflow with Test Tag
```bash
# Create test release tag
git tag v0.1.0-test -m "Test release workflow"
git push origin v0.1.0-test

# Monitor workflow
gh run watch

# Check release
gh release view v0.1.0-test
```

**Expected output**:
```
title: v0.1.0-test
tag: v0.1.0-test
--
TITLE     TYPE  SIZE    UPLOADED
token-count-v0.1.0-test-x86_64-unknown-linux-gnu.tar.gz  Asset  7.8 MB  3 minutes ago
token-count-v0.1.0-test-x86_64-apple-darwin.tar.gz       Asset  7.9 MB  3 minutes ago
token-count-v0.1.0-test-aarch64-apple-darwin.tar.gz      Asset  7.8 MB  3 minutes ago
token-count-v0.1.0-test-x86_64-pc-windows-msvc.zip       Asset  8.1 MB  3 minutes ago
checksums.txt                                             Asset  0.3 KB  3 minutes ago
```

#### 1.3 Download and Verify Binaries
```bash
# Download a binary
curl -LO https://github.com/shaunburdick/token-count/releases/download/v0.1.0-test/token-count-v0.1.0-test-x86_64-unknown-linux-gnu.tar.gz

# Download checksums
curl -LO https://github.com/shaunburdick/token-count/releases/download/v0.1.0-test/checksums.txt

# Verify checksum
shasum -a 256 -c checksums.txt --ignore-missing
```

**Expected output**:
```
token-count-v0.1.0-test-x86_64-unknown-linux-gnu.tar.gz: OK
```

#### 1.4 Test Binary Functionality
```bash
# Extract binary
tar xzf token-count-v0.1.0-test-x86_64-unknown-linux-gnu.tar.gz

# Test version
./token-count --version

# Test basic functionality
echo "test" | ./token-count --model gpt-4
```

**Expected output**:
```
token-count 0.1.0
1
```

### Success Criteria
- [ ] Workflow triggered on tag push
- [ ] 4 platform builds completed successfully
- [ ] All binaries packaged correctly (tar.gz/zip)
- [ ] checksums.txt generated with correct format
- [ ] All assets uploaded to GitHub Release
- [ ] Binaries are executable and functional
- [ ] Workflow completed in <15 minutes

---

## Phase 2: Install Script (install.sh)

### Setup
```bash
# Create install script
touch install.sh
chmod +x install.sh

# Commit to feature branch
git add install.sh
git commit -m "Add install script"
git push origin 002-installation
```

### Validation Steps

#### 2.1 Shellcheck Validation
```bash
# Install shellcheck (if not installed)
# brew install shellcheck  # macOS
# apt install shellcheck   # Ubuntu

# Run shellcheck
shellcheck install.sh
```

**Expected output**:
```
(no output = no issues)
```

#### 2.2 Test on Ubuntu 22.04
```bash
# Option 1: Local Ubuntu (VM or Docker)
docker run -it ubuntu:22.04 bash
apt update && apt install -y curl

# Option 2: GitHub Codespace or EC2

# Run install script
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/002-installation/install.sh | bash
```

**Expected output**:
```
Detecting platform... Linux x86_64 (x86_64-unknown-linux-gnu)
Fetching latest version... v0.1.0-test
Downloading token-count v0.1.0-test...
Verifying checksum... ✓
Extracting archive...
Installing to /home/ubuntu/.local/bin... ✓

token-count v0.1.0-test installed successfully!

Try it out:
  echo "Hello world" | token-count --model gpt-4
```

#### 2.3 Test on macOS (Intel)
```bash
# On macOS Intel machine or VM
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/002-installation/install.sh | bash

# Verify installation
token-count --version
echo "test" | token-count --model gpt-4
```

**Expected output**:
```
Detecting platform... macOS x86_64 (x86_64-apple-darwin)
...
token-count v0.1.0-test installed successfully!
```

#### 2.4 Test on macOS (Apple Silicon)
```bash
# On macOS ARM machine (M1/M2/M3)
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/002-installation/install.sh | bash

# Verify native ARM binary
file $(which token-count)
```

**Expected output**:
```
Detecting platform... macOS ARM64 (aarch64-apple-darwin)
...
/usr/local/bin/token-count: Mach-O 64-bit executable arm64
```

#### 2.5 Test Error Scenarios
```bash
# Test unsupported platform
export FORCE_PLATFORM=linux-armv7
bash install.sh

# Expected: Error message with suggestions

# Test checksum mismatch (simulate)
# (Modify checksums.txt locally to test)

# Test network failure
# (Disconnect network or use invalid URL)
```

### Success Criteria
- [ ] Shellcheck passes with zero warnings
- [ ] Ubuntu 22.04: Installs to ~/.local/bin or /usr/local/bin
- [ ] macOS Intel: Installs correctly with x86_64 binary
- [ ] macOS ARM: Installs correctly with ARM64 binary
- [ ] Checksum verification works
- [ ] Error messages are helpful and actionable
- [ ] Install completes in <30 seconds on broadband

---

## Phase 3: Homebrew Formula

### Setup
```bash
# Create Homebrew tap repository
gh repo create shaunburdick/homebrew-tap --public --description "Homebrew tap for token-count"

# Clone locally
git clone https://github.com/shaunburdick/homebrew-tap
cd homebrew-tap
mkdir Formula
touch Formula/token-count.rb
```

### Validation Steps

#### 3.1 Formula Syntax Validation
```bash
# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Audit formula
brew audit --strict Formula/token-count.rb
```

**Expected output**:
```
(no output = no issues)
```

#### 3.2 Test Installation on macOS
```bash
# Add tap
brew tap shaunburdick/tap

# Verify tap added
brew tap

# Install token-count
brew install token-count

# Verify installation
brew list token-count
which token-count
token-count --version
```

**Expected output**:
```
==> Tapping shaunburdick/tap
Cloning into '/usr/local/Homebrew/Library/Taps/shaunburdick/homebrew-tap'...

==> Downloading https://github.com/shaunburdick/token-count/releases/download/v0.1.0-test/token-count-v0.1.0-test-aarch64-apple-darwin.tar.gz
==> Pouring token-count-v0.1.0-test-aarch64-apple-darwin.tar.gz
🍺  /opt/homebrew/Cellar/token-count/0.1.0-test: 1 file, 9.2MB

/opt/homebrew/bin/token-count
token-count 0.1.0
```

#### 3.3 Test Homebrew Test Block
```bash
# Run formula test
brew test token-count
```

**Expected output**:
```
Testing token-count
==> /opt/homebrew/bin/token-count --version
token-count 0.1.0
```

#### 3.4 Test on Homebrew on Linux (Optional)
```bash
# Install Homebrew on Linux
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Tap and install
brew tap shaunburdick/tap
brew install token-count

# Verify
token-count --version
```

#### 3.5 Test Upgrade Path
```bash
# Simulate new version release (v0.1.1-test)
# Update formula with new checksums

# Upgrade
brew upgrade token-count

# Verify new version
token-count --version
```

### Success Criteria
- [ ] Formula passes `brew audit --strict`
- [ ] Installation succeeds on macOS Intel
- [ ] Installation succeeds on macOS ARM
- [ ] `brew test token-count` passes
- [ ] Binary installed to correct location
- [ ] Upgrade path works correctly
- [ ] Formula works on Homebrew on Linux (bonus)

---

## Phase 4: Cargo Publish

### Setup
```bash
# Login to crates.io (one-time)
cargo login
# Paste API token from https://crates.io/settings/tokens
```

### Validation Steps

#### 4.1 Package Content Verification
```bash
# Check what will be published
cargo package --list

# Expected: Only source files, no .github/, specs/, etc.
```

**Expected output** (excerpt):
```
Cargo.toml
LICENSE
README.md
src/lib.rs
src/main.rs
src/cli/args.rs
src/cli/input.rs
...
```

#### 4.2 Dry Run Publish
```bash
# Test publish without actually publishing
cargo publish --dry-run

# Check for errors
```

**Expected output**:
```
    Updating crates.io index
   Packaging token-count v0.1.0 (/path/to/token-count)
   Verifying token-count v0.1.0 (/path/to/token-count)
   Compiling token-count v0.1.0 (/path/to/token-count/target/package/token-count-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 3m 42s
   Uploading token-count v0.1.0 (/path/to/token-count)
     warning: aborting upload due to dry run
```

#### 4.3 Actual Publish (Point of No Return)
```bash
# IMPORTANT: Cannot undo! Double-check everything first.

# Final checks
cargo test --all-features
cargo clippy -- -D warnings
cargo fmt -- --check

# Publish
cargo publish

# Wait for crates.io index update (~1 minute)
```

**Expected output**:
```
    Updating crates.io index
   Packaging token-count v0.1.0 (/path/to/token-count)
   Uploading token-count v0.1.0 (/path/to/token-count)
```

#### 4.4 Verify on crates.io
```bash
# Open in browser
open https://crates.io/crates/token-count

# Check that docs are generated (takes 5-10 minutes)
open https://docs.rs/token-count
```

**Expected**:
- Package page shows correct description, keywords, categories
- README displayed correctly
- Documentation link works (after docs build)
- License displayed (MIT)

#### 4.5 Test cargo install
```bash
# Fresh machine or new environment
# Wait 1-2 minutes for crates.io index propagation

# Install from crates.io
cargo install token-count

# Verify
token-count --version
echo "test" | token-count --model gpt-4
```

**Expected output**:
```
    Updating crates.io index
  Downloaded token-count v0.1.0
  Downloaded 1 crate (45.2 KB) in 0.83s
   Compiling token-count v0.1.0
    Finished release [optimized] target(s) in 3m 42s
  Installing ~/.cargo/bin/token-count
   Installed package `token-count v0.1.0`

token-count 0.1.0
1
```

### Success Criteria
- [ ] `cargo publish --dry-run` succeeds
- [ ] Package excludes unnecessary files (< 100 KB source)
- [ ] Actual publish succeeds
- [ ] Package appears on crates.io within 1 minute
- [ ] `cargo install token-count` works
- [ ] Documentation builds on docs.rs within 10 minutes
- [ ] All metadata correct (description, keywords, license)

---

## Phase 5: Documentation

### Files to Create/Update
1. **INSTALL.md** (new)
2. **README.md** (update installation section)
3. **CHANGELOG.md** (prepare v0.1.0 release notes)

### Validation Steps

#### 5.1 INSTALL.md Completeness Check
```bash
# Read INSTALL.md and verify all methods documented:
cat INSTALL.md | grep -E "^##" 

# Expected sections:
# ## curl | bash
# ## Homebrew
# ## Cargo
# ## Manual Download (GitHub Releases)
# ## Checksum Verification
# ## Troubleshooting
```

#### 5.2 README.md Installation Section
```bash
# Verify README has updated installation section
grep -A 20 "## Installation" README.md

# Should include:
# - curl | bash (with command)
# - Homebrew (with brew tap + install)
# - cargo install
# - Link to INSTALL.md for details
```

#### 5.3 Link Validation
```bash
# Install markdown-link-check
npm install -g markdown-link-check

# Check all links in documentation
markdown-link-check README.md
markdown-link-check INSTALL.md
markdown-link-check CHANGELOG.md
```

**Expected output**:
```
✓ All links valid
```

#### 5.4 Example Command Testing
```bash
# Extract all bash code blocks from INSTALL.md
# Test each command manually (sample)

# Example: Check curl | bash command is correct
grep "curl -fsSL" INSTALL.md
# Should match actual install script URL
```

### Success Criteria
- [ ] INSTALL.md created with all installation methods
- [ ] README.md installation section updated
- [ ] CHANGELOG.md prepared for v0.1.0 release
- [ ] All documentation links work (no 404s)
- [ ] Code examples in docs are tested and work
- [ ] Checksum verification instructions clear
- [ ] Troubleshooting section addresses common issues

---

## Phase 6: End-to-End Testing

### Test Matrix

| Platform | curl\|bash | Homebrew | cargo install | Manual Download |
|----------|-----------|----------|---------------|-----------------|
| Ubuntu 22.04 | ☐ | ☐ | ☐ | ☐ |
| macOS 12 (Intel) | ☐ | ☐ | ☐ | ☐ |
| macOS 13 (ARM) | ☐ | ☐ | ☐ | ☐ |
| Windows 10 | N/A | N/A | ☐ | ☐ |

### Validation Steps

#### 6.1 Full Installation Test (Ubuntu 22.04)
```bash
# Fresh Ubuntu 22.04 environment (Docker or VM)
docker run -it --rm ubuntu:22.04 bash

# Install curl
apt update && apt install -y curl

# Test 1: curl | bash
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
token-count --version
echo "test" | token-count --model gpt-4

# Clean up
rm $(which token-count)

# Test 2: Manual download
curl -LO https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/shaunburdick/token-count/releases/download/v0.1.0/checksums.txt
shasum -a 256 -c checksums.txt --ignore-missing
tar xzf token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
./token-count --version

# Test 3: cargo install (requires Rust)
apt install -y cargo
cargo install token-count
~/.cargo/bin/token-count --version
```

#### 6.2 Full Installation Test (macOS)
```bash
# Test 1: Homebrew
brew tap shaunburdick/tap
brew install token-count
token-count --version
brew uninstall token-count

# Test 2: curl | bash
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
token-count --version

# Test 3: cargo install
cargo install token-count
~/.cargo/bin/token-count --version
```

#### 6.3 Upgrade Path Testing
```bash
# Install v0.1.0
brew install token-count
token-count --version  # Should show 0.1.0

# Release v0.1.1 (simulate by updating formula)
# Update formula with v0.1.1 URLs and checksums

# Upgrade
brew upgrade token-count
token-count --version  # Should show 0.1.1
```

#### 6.4 Cross-Platform Smoke Tests
```bash
# For each platform/method combination:
# 1. Install
# 2. Check version
# 3. Run basic command
# 4. Verify output

# Example template:
<install_command>
token-count --version | grep "^token-count 0.1.0$"
echo "hello world" | token-count --model gpt-4 | grep "^2$"
```

### Success Criteria
- [ ] All installation methods work on all supported platforms
- [ ] Binaries are correct architecture (file command check)
- [ ] token-count --version returns correct version
- [ ] Basic tokenization works (smoke test passes)
- [ ] Upgrade path works (old → new version)
- [ ] Uninstallation works cleanly (Homebrew)
- [ ] No broken links in documentation
- [ ] Installation completes in expected time (<30s for curl|bash, <60s for Homebrew)

---

## Final Release Checklist

### Pre-Release
- [ ] All tests passing in CI (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Version updated in Cargo.toml (0.1.0)
- [ ] CHANGELOG.md updated with all changes
- [ ] Documentation reviewed and updated
- [ ] Test tag (v0.1.0-test) successfully created release
- [ ] All installation methods tested manually

### Release Process
```bash
# 1. Merge feature branch to main
git checkout main
git pull
git merge 002-installation --squash
git commit -m "feat: add installation and distribution infrastructure"

# 2. Create release tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# 3. Wait for GitHub Actions (~10-15 minutes)
gh run watch

# 4. Verify release
gh release view v0.1.0

# 5. Update Homebrew formula (if not automated)
cd ../homebrew-tap
# Update Formula/token-count.rb with v0.1.0 URLs and checksums
git commit -am "Update token-count to v0.1.0"
git push

# 6. Publish to crates.io
cd ../token-count
cargo publish
```

### Post-Release Verification
- [ ] GitHub Release published with all assets (5 files)
- [ ] Checksums match actual binaries
- [ ] curl | bash works with main branch URL
- [ ] Homebrew formula updated with official v0.1.0
- [ ] cargo install token-count works from crates.io
- [ ] Manual downloads work for all platforms
- [ ] README installation instructions accurate
- [ ] CHANGELOG published with release notes

### Announcement
- [ ] Tweet/social media post (optional)
- [ ] Reddit post on r/rust (optional)
- [ ] Update project website (if exists)
- [ ] Monitor GitHub Issues for installation problems

---

## Troubleshooting Common Issues

### Issue: GitHub Actions workflow doesn't trigger
**Solution**: Check that tag matches pattern `v*.*.*` and workflow file is on main branch

### Issue: Checksum mismatch when testing install script
**Solution**: Verify using test release (v0.1.0-test), checksums generated correctly

### Issue: Homebrew formula fails with 404
**Solution**: Check release URL matches actual GitHub release, verify public access

### Issue: cargo publish fails with "already uploaded"
**Solution**: Version already exists on crates.io, must bump to 0.1.1

### Issue: Binary not executable on Linux
**Solution**: Ensure `chmod +x` before packaging, verify tar preserves permissions

### Issue: Install script fails on macOS ARM
**Solution**: Check `uname -m` returns `arm64`, verify aarch64-apple-darwin binary exists

---

## Quick Commands Reference

```bash
# Check current version
token-count --version

# Test tokenization
echo "test" | token-count --model gpt-4

# View releases
gh release list

# Check workflow status
gh run list --workflow=release.yml

# Download specific asset
gh release download v0.1.0 -p 'token-count-*-x86_64-unknown-linux-gnu.tar.gz'

# Verify checksum locally
shasum -a 256 token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz

# Test install script locally (dry run mode - if implemented)
DRY_RUN=1 bash install.sh

# Check Homebrew formula
brew info token-count

# Force Homebrew reinstall
brew reinstall token-count

# Check cargo package
cargo search token-count

# Verify published on crates.io
curl https://crates.io/api/v1/crates/token-count | jq .
```

---

**Quickstart Complete**: Follow phases sequentially, check off success criteria, proceed to official v0.1.0 release.
