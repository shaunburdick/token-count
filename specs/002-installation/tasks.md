# Feature 002: Installation & Distribution - Task Breakdown

**Status**: In Progress  
**Branch**: `002-installation`  
**Spec Version**: 1.1  
**Created**: 2026-03-14

## Task Legend
- `[P]` = Can be executed in parallel with other `[P]` tasks
- `[ ]` = Not started
- `[x]` = Completed
- `[~]` = In progress
- `[!]` = Blocked (dependencies not met)

---

## Phase 0: Planning & Preparation
**Status**: Complete  
**Estimated Time**: 3-4 hours  
**Actual Time**: ~4 hours

- [x] **TASK-000**: Research GitHub Actions cross-platform builds
- [x] **TASK-001**: Research Homebrew tap vs homebrew-core
- [x] **TASK-002**: Research Homebrew auto-update solutions
- [x] **TASK-003**: Research install script security best practices
- [x] **TASK-004**: Research crates.io publication requirements
- [x] **TASK-005**: Create `plan.md` with technical context and phases
- [x] **TASK-006**: Create `research.md` documenting technology choices
- [x] **TASK-007**: Create `quickstart.md` with validation scenarios
- [x] **TASK-008**: Create `contracts/github-release.md`
- [x] **TASK-009**: Create `contracts/install-script.md`
- [x] **TASK-010**: Create `HOMEBREW-AUTO-UPDATE.md` guide
- [x] **TASK-011**: Update email addresses in `Cargo.toml` and `README.md`
- [x] **TASK-012**: Verify constitution compliance (zero violations)
- [x] **TASK-013**: Create this `tasks.md` file

---

## Phase 1: GitHub Actions Release Workflow
**Status**: Not Started  
**Estimated Time**: 2-3 hours  
**Blocks**: All other phases (need binaries for testing)

### TASK-100: Create `.github/workflows/` directory structure
**Effort**: 5 minutes  
**Dependencies**: None

- [ ] Create `.github/workflows/` directory if it doesn't exist
- [ ] Verify directory permissions

### TASK-101: Create `release.yml` workflow skeleton
**Effort**: 15 minutes  
**Dependencies**: TASK-100

- [ ] Create `.github/workflows/release.yml`
- [ ] Add workflow metadata (name, description)
- [ ] Define trigger: `on: push: tags: ['v*']`
- [ ] Add workflow_dispatch for manual testing

### TASK-102: Implement build matrix for 4 platforms
**Effort**: 45 minutes  
**Dependencies**: TASK-101

- [ ] Define matrix strategy with 4 platforms:
  - `x86_64-unknown-linux-gnu`
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`
  - `x86_64-pc-windows-msvc`
- [ ] Map each platform to GitHub runner OS
- [ ] Configure Rust toolchain installation (stable)
- [ ] Add cross-compilation setup for macOS ARM64

### TASK-103: Implement build steps
**Effort**: 30 minutes  
**Dependencies**: TASK-102

- [ ] Checkout code with `actions/checkout@v4`
- [ ] Install Rust with `dtolnay/rust-toolchain@stable`
- [ ] Add target with `rustup target add ${{ matrix.target }}`
- [ ] Run `cargo build --release --target ${{ matrix.target }}`
- [ ] Verify binary exists in `target/${{ matrix.target }}/release/`

### TASK-104: Implement binary packaging
**Effort**: 45 minutes  
**Dependencies**: TASK-103

- [ ] Strip binaries with `strip` command (Linux/macOS only)
- [ ] Create archive names: `token-count-$VERSION-$TARGET.tar.gz` (Unix) / `.zip` (Windows)
- [ ] Package binaries with appropriate format:
  - Linux/macOS: `tar czf` with `token-count/` directory
  - Windows: `7z a` or PowerShell `Compress-Archive`
- [ ] Generate SHA256 checksums for each archive
- [ ] Create `checksums.txt` file with all hashes

### TASK-105: Implement GitHub Release creation
**Effort**: 30 minutes  
**Dependencies**: TASK-104

- [ ] Use `softprops/action-gh-release@v1`
- [ ] Extract version from `$GITHUB_REF` (strip `refs/tags/v`)
- [ ] Upload all 4 platform archives
- [ ] Upload `checksums.txt`
- [ ] Set release name to `token-count v$VERSION`
- [ ] Mark as draft initially for manual review

### TASK-106: Add Homebrew auto-update job
**Effort**: 30 minutes  
**Dependencies**: TASK-105, Phase 3 complete (Homebrew tap exists)

- [ ] Add separate job `update-homebrew` that depends on `build` job
- [ ] Runs only on: `if: startsWith(github.ref, 'refs/tags/v')`
- [ ] Uses `mislav/bump-homebrew-formula-action@v3`
- [ ] Configure with:
  - `formula-name: token-count`
  - `homebrew-tap: shaunburdick/homebrew-tap`
  - `base-branch: main`
  - `download-url: https://github.com/shaunburdick/token-count/archive/refs/tags/$VERSION.tar.gz`
  - `commit-message: 'Brew formula update for token-count version $VERSION'`
- [ ] Use `secrets.HOMEBREW_TAP_TOKEN` for authentication

### TASK-107: Test workflow with test tag
**Effort**: 30 minutes  
**Dependencies**: TASK-106

- [ ] Commit workflow file
- [ ] Create test tag: `git tag v0.1.0-test && git push origin v0.1.0-test`
- [ ] Verify all 4 builds succeed in GitHub Actions
- [ ] Verify binaries are uploaded to draft release
- [ ] Verify checksums.txt is correct
- [ ] Test download and extract on local machine
- [ ] Delete test tag and draft release
- [ ] Fix any issues found

**Phase 1 Deliverable**: Working `.github/workflows/release.yml` that produces 4 platform binaries + checksums

---

## Phase 2: Install Script
**Status**: Not Started  
**Estimated Time**: 2-3 hours  
**Dependencies**: Phase 1 complete (need release binaries to test against)

### TASK-200: Create `install.sh` skeleton
**Effort**: 15 minutes  
**Dependencies**: None

- [ ] Create `install.sh` in repository root
- [ ] Add shebang: `#!/usr/bin/env bash`
- [ ] Add copyright header and MIT license comment
- [ ] Add script description and usage instructions
- [ ] Set `set -euo pipefail` for safety

### TASK-201: Implement platform detection
**Effort**: 30 minutes  
**Dependencies**: TASK-200

- [ ] Detect OS with `uname -s`: Linux, Darwin, Windows (MINGW/MSYS)
- [ ] Detect architecture with `uname -m`: x86_64, arm64/aarch64
- [ ] Map to target triple format (e.g., `x86_64-unknown-linux-gnu`)
- [ ] Error out on unsupported platforms with clear message
- [ ] Add debug output (if `$DEBUG` env var set)

### TASK-202: Implement version resolution
**Effort**: 20 minutes  
**Dependencies**: TASK-201

- [ ] Accept optional version argument: `./install.sh [VERSION]`
- [ ] Default to latest release if no version specified
- [ ] Fetch latest version from GitHub API:
  - `curl -sSfL https://api.github.com/repos/shaunburdick/token-count/releases/latest`
  - Parse JSON with `jq` or `grep`/`sed` (prefer no dependencies)
- [ ] Validate version format (v*.*.*)
- [ ] Set `VERSION` variable

### TASK-203: Implement download with verification
**Effort**: 45 minutes  
**Dependencies**: TASK-202

- [ ] Construct download URLs:
  - Binary: `https://github.com/shaunburdick/token-count/releases/download/v$VERSION/token-count-$VERSION-$TARGET.tar.gz`
  - Checksums: `https://github.com/shaunburdick/token-count/releases/download/v$VERSION/checksums.txt`
- [ ] Create temp directory: `mktemp -d`
- [ ] Download archive with `curl -sSfL -o`
- [ ] Download checksums file
- [ ] Verify SHA256 checksum:
  - Extract expected hash for current platform from checksums.txt
  - Compute actual hash with `shasum -a 256` or `sha256sum`
  - Compare hashes (case-insensitive)
  - Error out if mismatch with clear message
- [ ] Clean up temp directory on exit (trap)

### TASK-204: Implement extraction and installation
**Effort**: 30 minutes  
**Dependencies**: TASK-203

- [ ] Extract archive: `tar xzf` in temp directory
- [ ] Locate binary: `token-count/token-count` (or `.exe` on Windows)
- [ ] Determine install directory:
  - Default: `$HOME/.local/bin` (create if doesn't exist)
  - Allow override with `$INSTALL_DIR` env var
  - Require `sudo` if installing to system paths
- [ ] Copy binary to install directory with `install -m 755`
- [ ] Verify binary is executable
- [ ] Test binary: `$INSTALL_DIR/token-count --version`

### TASK-205: Implement user feedback and PATH guidance
**Effort**: 20 minutes  
**Dependencies**: TASK-204

- [ ] Print success message with installed version and location
- [ ] Check if install directory is in `$PATH`
- [ ] If not in PATH, print instructions to add it:
  - Bash: `export PATH="$INSTALL_DIR:$PATH"` in `~/.bashrc`
  - Zsh: Add to `~/.zshrc`
  - Fish: `fish_add_path $INSTALL_DIR`
- [ ] Print next steps: "Run `token-count --help` to get started"

### TASK-206: Add error handling and logging
**Effort**: 30 minutes  
**Dependencies**: TASK-205

- [ ] Add `error()` function that prints to stderr and exits
- [ ] Add `info()` and `debug()` functions for user feedback
- [ ] Wrap all external commands with error checks
- [ ] Add meaningful error messages for common failures:
  - Network errors (curl failed)
  - Checksum mismatch (possible corruption or MITM)
  - Unsupported platform
  - Permission denied
- [ ] Add cleanup on error (remove partial installs)

### TASK-207: Test install script on 3 platforms
**Effort**: 45 minutes  
**Dependencies**: TASK-206

- [ ] Test on Ubuntu 22.04 (Docker or VM):
  - Fresh install
  - Version override: `./install.sh v0.1.0-test`
  - Custom install dir: `INSTALL_DIR=/tmp/test ./install.sh`
- [ ] Test on macOS Intel (if available)
- [ ] Test on macOS ARM64 (if available)
- [ ] Test error cases:
  - Invalid version
  - Corrupted checksum file
  - No internet connection
  - Permission denied
- [ ] Document any platform-specific quirks

**Phase 2 Deliverable**: Working `install.sh` script with platform detection, checksum verification, and clear error messages

---

## Phase 3: Homebrew Formula
**Status**: Not Started  
**Estimated Time**: 2-3 hours  
**Dependencies**: Phase 1 complete (need real release for checksums)

### TASK-300: Create Homebrew tap repository
**Effort**: 20 minutes  
**Dependencies**: None

- [ ] Create new GitHub repository: `shaunburdick/homebrew-tap`
- [ ] Initialize with README explaining purpose
- [ ] Create `Formula/` directory
- [ ] Add MIT LICENSE file
- [ ] Set repository description: "Homebrew tap for shaunburdick's tools"

### TASK-301: Generate initial formula skeleton
**Effort**: 15 minutes  
**Dependencies**: TASK-300

- [ ] Create `Formula/token-count.rb`
- [ ] Add formula class: `class TokenCount < Formula`
- [ ] Add metadata:
  - `desc "Count tokens for OpenAI API requests"`
  - `homepage "https://github.com/shaunburdick/token-count"`
  - `license "MIT"`

### TASK-302: Add download URL and SHA256
**Effort**: 30 minutes  
**Dependencies**: TASK-301, Phase 1 complete

- [ ] Get SHA256 hash for macOS ARM64 archive from latest release
- [ ] Add `url` pointing to GitHub release tarball:
  - `url "https://github.com/shaunburdick/token-count/archive/refs/tags/v0.1.0.tar.gz"`
- [ ] Add `sha256` hash
- [ ] Add `version "0.1.0"` (will be auto-updated)

### TASK-303: Implement installation logic
**Effort**: 45 minutes  
**Dependencies**: TASK-302

- [ ] Add `depends_on "rust" => :build` (compile from source approach)
- [ ] Implement `install` method:
  - `system "cargo", "install", "--locked", "--root", prefix, "--path", "."`
- [ ] Alternative approach (pre-built binary):
  - Detect platform and download appropriate binary
  - Use `on_macos` and `on_linux` blocks
  - Install binary directly: `bin.install "token-count"`
- [ ] Choose approach based on Homebrew best practices research
- [ ] Add post-install message (optional):
  - `caveats` block with getting started instructions

### TASK-304: Add test block
**Effort**: 20 minutes  
**Dependencies**: TASK-303

- [ ] Implement `test` method:
  - `assert_match "token-count #{version}", shell_output("#{bin}/token-count --version")`
  - Test basic functionality: `echo "test" | #{bin}/token-count`
  - Verify exit code is 0

### TASK-305: Test formula locally
**Effort**: 45 minutes  
**Dependencies**: TASK-304

- [ ] Install formula from local path:
  - `brew install --build-from-source ./Formula/token-count.rb`
- [ ] Verify installation: `which token-count`
- [ ] Test binary: `token-count --version`
- [ ] Test formula test: `brew test token-count`
- [ ] Uninstall: `brew uninstall token-count`
- [ ] Test from tap: `brew tap shaunburdick/tap && brew install token-count`
- [ ] Fix any issues found

### TASK-306: Set up Personal Access Token for auto-updates
**Effort**: 15 minutes  
**Dependencies**: None (can be done in parallel)

- [ ] Go to https://github.com/settings/tokens/new
- [ ] Create token with name: "Homebrew Tap Auto-Update"
- [ ] Select scopes: `repo` and `workflow`
- [ ] Generate token and copy to clipboard
- [ ] Add to token-count repository secrets:
  - Name: `HOMEBREW_TAP_TOKEN`
  - Value: <generated token>
- [ ] Document token purpose in repository (without exposing value)

### TASK-307: Test Homebrew auto-update workflow
**Effort**: 30 minutes  
**Dependencies**: TASK-106 (auto-update job in release.yml), TASK-306

- [ ] Create test release: `v0.1.0-test2`
- [ ] Verify auto-update job runs successfully
- [ ] Check homebrew-tap repository for automatic commit
- [ ] Verify formula was updated:
  - Version number changed
  - URL changed
  - SHA256 hash updated
- [ ] Test installing updated formula
- [ ] Document any issues for troubleshooting guide

**Phase 3 Deliverable**: Working Homebrew tap with auto-updating formula

---

## Phase 4: Cargo Publish Preparation
**Status**: Not Started  
**Estimated Time**: 1 hour  
**Dependencies**: None (can be done in parallel with other phases)

### TASK-400: Verify `Cargo.toml` metadata completeness
**Effort**: 20 minutes  
**Dependencies**: None

- [ ] Verify all required fields present:
  - [x] `name = "token-count"`
  - [x] `version = "0.1.0"`
  - [x] `authors = ["Shaun Burdick <hello@burdick.dev>"]`
  - [x] `edition = "2021"`
  - [x] `description` (descriptive, <140 chars)
  - [x] `license = "MIT"`
  - [x] `repository = "https://github.com/shaunburdick/token-count"`
  - [ ] `readme = "README.md"`
  - [ ] `keywords` (max 5, relevant to discovery)
  - [ ] `categories` (from crates.io categories list)
  - [ ] `homepage` (optional, same as repository)
  - [ ] `documentation` (optional, auto-generated from docs.rs)
- [ ] Verify fields follow crates.io guidelines
- [ ] Update if needed

### TASK-401: Add crate-level documentation
**Effort**: 20 minutes  
**Dependencies**: TASK-400

- [ ] Add `//!` doc comments to `src/main.rs` or `src/lib.rs`
- [ ] Include crate overview and purpose
- [ ] Add usage examples
- [ ] Document main features
- [ ] Test documentation: `cargo doc --open`

### TASK-402: Verify README and LICENSE
**Effort**: 10 minutes  
**Dependencies**: None

- [ ] Verify README.md exists and is comprehensive
- [ ] Verify LICENSE file exists (MIT)
- [ ] Check that license identifier in Cargo.toml matches LICENSE file
- [ ] Ensure no broken links in README

### TASK-403: Create crates.io account and get API token
**Effort**: 10 minutes  
**Dependencies**: None

- [ ] Go to https://crates.io/ and log in with GitHub
- [ ] Go to Account Settings → API Tokens
- [ ] Create new token: "token-count releases"
- [ ] Copy token (one-time display)
- [ ] Store securely in GitHub Actions secrets:
  - Name: `CARGO_REGISTRY_TOKEN`
  - Value: <generated token>

### TASK-404: Add cargo publish to release workflow
**Effort**: 20 minutes  
**Dependencies**: TASK-403

- [ ] Add new job `publish-crate` to `.github/workflows/release.yml`
- [ ] Runs only on: `if: startsWith(github.ref, 'refs/tags/v') && !contains(github.ref, '-')`
  - Excludes test tags like `v0.1.0-test`
- [ ] Steps:
  - Checkout code
  - Install Rust
  - Run `cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}`
- [ ] Handle idempotency (publishing same version twice fails)

### TASK-405: Dry-run cargo publish
**Effort**: 15 minutes  
**Dependencies**: TASK-401, TASK-402

- [ ] Run `cargo publish --dry-run`
- [ ] Review all files that would be published
- [ ] Verify no sensitive files included (check `.gitignore` and `Cargo.toml` `exclude`)
- [ ] Fix any warnings or errors
- [ ] Verify package size is reasonable (<10 MB)

**Phase 4 Deliverable**: `Cargo.toml` ready for crates.io publication, cargo publish job in workflow

---

## Phase 5: Documentation Updates
**Status**: Not Started  
**Estimated Time**: 1-2 hours  
**Dependencies**: Phases 1-4 complete (need to document actual working methods)

### TASK-500: Create `INSTALL.md`
**Effort**: 45 minutes  
**Dependencies**: None

- [ ] Create `INSTALL.md` in repository root
- [ ] Add sections:
  - **Overview**: Brief intro to installation options
  - **Quick Install (Recommended)**: curl|bash one-liner
  - **Homebrew (macOS/Linux)**: `brew install shaunburdick/tap/token-count`
  - **Cargo (Any Platform)**: `cargo install token-count`
  - **Manual Download**: Link to GitHub Releases with instructions
  - **Verification**: How to verify checksums manually
  - **Building from Source**: `cargo build --release` instructions
  - **Troubleshooting**: Common issues and solutions
  - **Uninstallation**: How to remove for each method
- [ ] Include platform-specific notes
- [ ] Add examples and expected output

### TASK-501: Update `README.md` installation section
**Effort**: 20 minutes  
**Dependencies**: TASK-500

- [ ] Update "Installation" section in README.md
- [ ] Add quick install one-liner prominently
- [ ] Link to full `INSTALL.md` for details
- [ ] Keep README focused on essentials (defer details to INSTALL.md)
- [ ] Add badges (optional):
  - Crates.io version
  - GitHub release version
  - License
  - Build status

### TASK-502: Update `CHANGELOG.md` for v0.1.0
**Effort**: 20 minutes  
**Dependencies**: None

- [ ] Add entry for v0.1.0 release
- [ ] List features from Feature 001 and 002:
  - **Added**: CLI with 4 OpenAI models
  - **Added**: Fuzzy model name matching
  - **Added**: Installation via Homebrew, cargo, install.sh
  - **Added**: GitHub Releases with pre-built binaries
  - **Added**: Comprehensive documentation
- [ ] Follow Keep a Changelog format
- [ ] Include date of release

### TASK-503: Update `CONTRIBUTING.md` with release process
**Effort**: 15 minutes  
**Dependencies**: None

- [ ] Add "Release Process" section
- [ ] Document steps for maintainers:
  1. Update version in `Cargo.toml`
  2. Update `CHANGELOG.md`
  3. Commit changes: `git commit -m "chore: bump version to X.Y.Z"`
  4. Create tag: `git tag vX.Y.Z`
  5. Push: `git push origin main --tags`
  6. Monitor GitHub Actions for release workflow
  7. Verify all installation methods work
  8. Announce release
- [ ] Document manual fallback if automation fails

### TASK-504: [P] Add security policy
**Effort**: 15 minutes  
**Dependencies**: None

- [ ] Create `SECURITY.md` in repository root
- [ ] Document supported versions
- [ ] Add security contact: hello@burdick.dev
- [ ] Request private disclosure for vulnerabilities
- [ ] Link to GitHub Security Advisories
- [ ] Mention supply chain security (checksums, signed releases)

### TASK-505: [P] Update repository description and topics
**Effort**: 10 minutes  
**Dependencies**: None

- [ ] Update GitHub repository description
- [ ] Add topics/tags:
  - `tokenizer`
  - `openai`
  - `cli`
  - `rust`
  - `tiktoken`
  - `token-counting`
- [ ] Ensure repository has clear About section

**Phase 5 Deliverable**: Complete documentation suite with `INSTALL.md`, updated README, CHANGELOG, CONTRIBUTING, and SECURITY

---

## Phase 6: Testing & Release
**Status**: Not Started  
**Estimated Time**: 2-3 hours  
**Dependencies**: Phases 1-5 complete

### TASK-600: End-to-end test matrix
**Effort**: 90 minutes  
**Dependencies**: All previous phases

Test all combinations:

#### Linux (x86_64-unknown-linux-gnu)
- [ ] **Install Script**: `curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash`
  - Verify binary installed to `~/.local/bin/token-count`
  - Test: `token-count --version`
  - Test: `echo "hello world" | token-count`
- [ ] **Manual Download**: Download `.tar.gz`, verify checksum, extract, run
- [ ] **Cargo**: `cargo install token-count` (after publishing)

#### macOS Intel (x86_64-apple-darwin)
- [ ] **Install Script**: Same as Linux
- [ ] **Homebrew**: `brew install shaunburdick/tap/token-count`
  - Verify: `which token-count` → `/opt/homebrew/bin/token-count` (ARM) or `/usr/local/bin/token-count` (Intel)
  - Test: `token-count --help`
- [ ] **Manual Download**: Download `.tar.gz`, verify checksum, extract, run
- [ ] **Cargo**: `cargo install token-count`

#### macOS ARM64 (aarch64-apple-darwin)
- [ ] All tests same as macOS Intel
- [ ] Verify binary is native ARM64 (not x86_64 running under Rosetta)

#### Windows (x86_64-pc-windows-msvc)
- [ ] **Manual Download**: Download `.zip`, verify checksum, extract, run
- [ ] **Cargo**: `cargo install token-count`
- [ ] Note: Install script not supported on Windows (document in INSTALL.md)

### TASK-601: Verify automation workflows
**Effort**: 30 minutes  
**Dependencies**: TASK-600

- [ ] Create release candidate tag: `v0.1.0-rc1`
- [ ] Verify GitHub Actions workflow succeeds:
  - All 4 platform builds complete
  - Binaries uploaded to draft release
  - Checksums.txt present and correct
  - Homebrew formula auto-updated (if not test tag)
  - Cargo publish skipped (contains `-` in tag)
- [ ] Download and test one binary manually
- [ ] Delete RC tag and release

### TASK-602: Security review
**Effort**: 30 minutes  
**Dependencies**: None

- [ ] Review install script for security issues:
  - No arbitrary code execution
  - HTTPS-only downloads
  - Checksum verification mandatory
  - Safe temp directory handling
  - Proper error handling
- [ ] Review GitHub Actions workflow:
  - No exposure of secrets in logs
  - Minimal permissions
  - Trusted actions only (official or well-known)
- [ ] Check for any hardcoded credentials or tokens
- [ ] Verify LICENSE and copyright notices

### TASK-603: Final documentation review
**Effort**: 20 minutes  
**Dependencies**: Phase 5 complete

- [ ] Proofread all documentation files
- [ ] Test all commands in documentation
- [ ] Verify all links are working
- [ ] Check for typos and formatting issues
- [ ] Ensure consistent terminology throughout

### TASK-604: Create official v0.1.0 release
**Effort**: 30 minutes  
**Dependencies**: TASK-600, TASK-601, TASK-602, TASK-603

- [ ] Update version in `Cargo.toml` to `0.1.0` (already set)
- [ ] Commit version bump if needed: `git commit -m "chore: release v0.1.0"`
- [ ] Create tag: `git tag v0.1.0`
- [ ] Push: `git push origin 002-installation --tags`
- [ ] Monitor GitHub Actions workflow
- [ ] Verify draft release is created successfully
- [ ] Review release notes
- [ ] Publish release (remove draft status)

### TASK-605: Verify all installation methods post-release
**Effort**: 30 minutes  
**Dependencies**: TASK-604

- [ ] Test install script: `curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash`
- [ ] Test Homebrew (wait ~5 min for auto-update):
  - `brew tap shaunburdick/tap`
  - `brew install token-count`
  - Verify version: `token-count --version` → `0.1.0`
- [ ] Test cargo install:
  - `cargo install token-count`
  - Verify version
- [ ] Test manual download from GitHub Releases
- [ ] Fix any issues immediately (hotfix if critical)

### TASK-606: Merge feature branch to main
**Effort**: 15 minutes  
**Dependencies**: TASK-605

- [ ] Create PR from `002-installation` to `main`
- [ ] Add PR description summarizing feature
- [ ] Self-review changes
- [ ] Merge PR (squash or merge commit, per project convention)
- [ ] Delete feature branch: `git branch -d 002-installation`
- [ ] Verify `main` branch is up to date

### TASK-607: Announce release
**Effort**: 20 minutes  
**Dependencies**: TASK-606

- [ ] Post announcement on GitHub Discussions (if enabled)
- [ ] Update personal website/blog (if applicable)
- [ ] Share on social media (optional):
  - Twitter/X with `#rustlang` hashtag
  - LinkedIn
  - Reddit r/rust (Show and Tell thread)
- [ ] Consider Hacker News Show HN post
- [ ] Update documentation sites (if any)

**Phase 6 Deliverable**: Official v0.1.0 release with all installation methods verified and working

---

## Summary

**Total Estimated Time**: 11-15 hours across 6 phases  
**Total Tasks**: 71 tasks (13 complete, 58 remaining)

**Critical Path**:
1. Phase 1 (GitHub Actions) → Blocks all testing
2. Phase 2 (Install Script) → Can start after Phase 1
3. Phase 3 (Homebrew) → Can start after Phase 1
4. Phase 4 (Cargo) → Can be done in parallel
5. Phase 5 (Documentation) → Can be done in parallel
6. Phase 6 (Testing & Release) → Requires all phases complete

**Parallel Work Opportunities**:
- Phase 4 (Cargo prep) can be done anytime
- Phase 5 (Documentation) can be done anytime
- TASK-306 (PAT setup) can be done early
- TASK-504 (Security policy) can be done anytime
- TASK-505 (Repo metadata) can be done anytime

**Next Immediate Task**: TASK-100 (Start Phase 1 - GitHub Actions)
