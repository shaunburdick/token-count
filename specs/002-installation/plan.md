# Implementation Plan: Installation & Distribution

**Branch**: `002-installation` | **Date**: 2026-03-14 | **Spec**: [002-installation.md](./spec.md)  
**Input**: Feature specification from `.specify/features/002-installation.md` (v1.1)

## Summary

Implement automated release pipeline and multiple installation methods to make token-count easily installable without requiring users to compile from source. This feature adds GitHub Actions workflows for cross-platform binary builds, curl|bash installer script, Homebrew formula, cargo publish to crates.io, and comprehensive documentation. The release automation will build binaries for 4 platforms (Linux x64, macOS x64/ARM64, Windows x64) on every tagged release, with SHA256 checksums for security.

## Technical Context

**Language/Version**: Rust 1.85.0+ (stable channel, MSRV established in constitution)  
**Primary Dependencies**: 
- GitHub Actions (ubuntu-latest, macos-latest, windows-latest runners)
- cargo (build tool, already in project)
- tar/zip (binary packaging)
- shasum/sha256sum (checksum generation)
- gh CLI (optional, for release automation testing)

**Storage**: GitHub Releases (binary artifacts), crates.io (source package)  
**Testing**: 
- Shell script testing (shellcheck for install.sh)
- GitHub Actions workflow validation (test tags)
- Manual testing on Ubuntu 22.04, macOS 12+, macOS 13+ (Apple Silicon)
- Binary smoke tests (echo "test" | token-count)

**Target Platform**: 
- Build: GitHub Actions runners (Linux/macOS/Windows)
- Deployment: GitHub Releases, crates.io, Homebrew tap
- Runtime: Linux (Ubuntu 20.04+), macOS (10.15+), Windows (10+)

**Project Type**: CLI tool distribution infrastructure  
**Performance Goals**: 
- Install script execution: <30 seconds on broadband
- GitHub Actions workflow: <15 minutes total (parallel builds)
- Homebrew install: <1 minute
- Cargo install: <5 minutes (compilation time)

**Constraints**: 
- Binary size: <10MB compressed archives (currently 9.2MB uncompressed binary)
- Security: HTTPS-only downloads, SHA256 verification required
- No sudo required for user-local installs
- Cross-platform compatibility (bash for Unix, PowerShell for Windows)

**Scale/Scope**: 
- 4 platform build targets
- 1 install script (install.sh)
- 1 GitHub Actions workflow (.github/workflows/release.yml)
- 1 Homebrew formula repository (shaunburdick/homebrew-tap)
- 1 crates.io package publication
- Documentation updates (README.md, new INSTALL.md)

## Constitution Check

✅ **PASS** - All principles upheld:

### I. POSIX Simplicity
✅ **Compliant**: Install methods maintain simplicity - one command installs (curl|bash, brew install, cargo install). No complex configuration.

### II. Accuracy Over Speed  
✅ **Not Applicable**: This feature doesn't affect tokenization accuracy.

### III. Zero External Dependencies at Runtime
✅ **Compliant**: Binary remains standalone. Install infrastructure is separate from runtime.

### IV. Cross-Platform First-Class Support
✅ **Compliant**: Building binaries for all platforms (Linux x64, macOS x64/ARM64, Windows x64). Equal treatment.

### V. Fail Fast with Clear Errors
✅ **Compliant**: Install script validates platform, checksums, and provides helpful error messages with suggestions.

### VI. Installation Should Be Trivial ⭐
✅ **DIRECTLY IMPLEMENTS THIS PRINCIPLE**: This entire feature exists to fulfill Principle VI.

### VII. Semantic Versioning Strictly Enforced
✅ **Compliant**: Release workflow generates releases from semantic version tags (v*.*.*). CHANGELOG.md updated as part of release process.

**No constitutional violations.** This feature directly implements Principle VI while maintaining all other principles.

## Project Structure

### Documentation (this feature)

```text
specs/002-installation/
├── plan.md              # This file - Implementation plan
├── research.md          # Research on GitHub Actions, Homebrew, cargo publish
├── quickstart.md        # Validation scenarios for testing installation methods
├── contracts/           # API contracts (GitHub Actions outputs, install.sh interface)
│   ├── github-release.yaml     # GitHub Release asset structure
│   └── install-script.yaml     # Install script interface/options
└── spec.md              # Symlink to .specify/features/002-installation.md
```

### Source Code (repository root)

**Existing structure** (no changes to src/):
```text
src/
├── lib.rs              # Library API (no changes)
├── main.rs             # Binary entry point (no changes)
├── cli/                # CLI argument parsing (no changes)
├── tokenizers/         # Tokenization engine (no changes)
├── output/             # Output formatters (no changes)
└── error.rs            # Error types (no changes)

tests/                  # Existing tests (no changes)
benches/                # Existing benchmarks (no changes)
```

**New distribution infrastructure**:
```text
.github/
└── workflows/
    └── release.yml     # NEW: Release automation workflow

install.sh              # NEW: curl|bash installer script
INSTALL.md              # NEW: Detailed installation guide

# Updated files:
Cargo.toml              # Update: Add crates.io metadata (homepage, keywords, etc.)
README.md               # Update: Installation section with all methods
CHANGELOG.md            # Update: Prepare v0.1.0 release notes
```

**External repository** (to be created):
```text
shaunburdick/homebrew-tap/  # NEW repository
└── Formula/
    └── token-count.rb      # Homebrew formula
```

**Structure Decision**: 
- **Minimal changes to existing code**: Zero changes to src/ or tests/ - this is purely infrastructure
- **Root-level installer**: install.sh lives at repo root for easy curl access
- **GitHub Actions standard location**: .github/workflows/ follows convention
- **Separate Homebrew repo**: Follows Homebrew tap convention (requires separate repo)
- **Documentation co-located**: INSTALL.md at root for easy discovery

## Complexity Tracking

**No violations to justify.** This feature has zero constitutional violations.

---

## Architecture & Implementation Phases

### Phase 1: GitHub Actions Release Workflow (Foundation)
**Priority**: P0 (Blocks everything else)  
**Duration**: 2-3 hours  
**Why first**: Homebrew formula needs checksums from real releases, install.sh needs binaries to download

**Components**:
1. `.github/workflows/release.yml` - Main release automation
2. Build matrix for 4 platforms
3. Binary packaging (tar.gz for Unix, zip for Windows)
4. Checksum generation
5. GitHub Release creation

**Dependencies**: None (can start immediately)

**Deliverables**:
- Working GitHub Actions workflow
- Test release with v0.1.0-test tag
- Verified binaries for all platforms
- checksums.txt file

**Validation**:
```bash
# Push test tag
git tag v0.1.0-test
git push origin v0.1.0-test

# Wait for workflow
gh run watch

# Verify release
gh release view v0.1.0-test
# Should show 4 binary archives + checksums.txt
```

---

### Phase 2: Install Script (Quick User Win)
**Priority**: P1 (High user value)  
**Duration**: 2-3 hours  
**Why second**: Can be tested immediately after Phase 1 completes

**Components**:
1. `install.sh` - Platform detection, download, verification, installation
2. Shellcheck validation (linting for bash scripts)
3. Error handling with helpful messages
4. Installation path logic (~/.local/bin vs /usr/local/bin)

**Dependencies**: Phase 1 (needs GitHub Releases with binaries)

**Deliverables**:
- install.sh script at repo root
- Tested on Ubuntu 22.04 + macOS
- README.md updated with curl|bash command

**Validation**:
```bash
# Test fresh install (Ubuntu)
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/002-installation/install.sh | bash

# Test fresh install (macOS)
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/002-installation/install.sh | bash

# Verify installation
token-count --version
echo "test" | token-count --model gpt-4
```

---

### Phase 3: Homebrew Formula (macOS/Linux Package Manager)
**Priority**: P1 (Critical for macOS users)  
**Duration**: 2-3 hours  
**Why third**: Requires checksums from real release (Phase 1), includes auto-update setup

**Components**:
1. Create `shaunburdick/homebrew-tap` repository
2. `Formula/token-count.rb` - Homebrew formula
3. Multi-platform bottle URLs (macOS Intel, macOS ARM, Linux)
4. SHA256 checksums (copy from Phase 1 release)
5. Test installation block
6. **Auto-update job** in release.yml using `mislav/bump-homebrew-formula-action@v3`

**Dependencies**: Phase 1 (needs SHA256 checksums from real release)

**Deliverables**:
- New GitHub repo: shaunburdick/homebrew-tap
- Formula file with all platform bottles
- README in tap repo with usage instructions
- Tested brew install command
- Auto-update job in release workflow
- Personal Access Token (PAT) with repo + workflow scopes added as `HOMEBREW_TAP_TOKEN` secret

**Validation**:
```bash
# Add tap
brew tap shaunburdick/tap

# Install
brew install token-count

# Test
brew test token-count

# Verify
token-count --version
echo "test" | token-count --model gpt-4

# Test auto-update (after next release)
# Push v0.1.1 tag, verify formula automatically updates
```

**Auto-Update Setup**:
1. Generate PAT at https://github.com/settings/tokens (classic token)
   - Scopes: `repo`, `workflow`
2. Add secret to token-count repo: `HOMEBREW_TAP_TOKEN`
3. Add job to `.github/workflows/release.yml`:
```yaml
update-homebrew:
  needs: release
  runs-on: ubuntu-latest
  steps:
    - uses: mislav/bump-homebrew-formula-action@v3
      with:
        formula-name: token-count
        formula-path: Formula/token-count.rb
        homebrew-tap: shaunburdick/homebrew-tap
        download-url: https://github.com/shaunburdick/token-count/releases/download/${{ github.ref_name }}/token-count-${{ github.ref_name }}-x86_64-unknown-linux-gnu.tar.gz
        commit-message: |
          token-count ${{ github.ref_name }}
          
          Automated update from release
      env:
        COMMITTER_TOKEN: ${{ secrets.HOMEBREW_TAP_TOKEN }}
```

---

### Phase 4: Cargo Publish (Rust Ecosystem)
**Priority**: P2 (Nice to have, lower priority)  
**Duration**: 1 hour  
**Why fourth**: Independent of other phases, appeals to Rust developers

**Components**:
1. Update `Cargo.toml` with crates.io metadata
2. Verify all required fields (description, homepage, repository, keywords, categories)
3. Run `cargo publish --dry-run` to validate
4. Publish to crates.io

**Dependencies**: None (independent of other phases)

**Deliverables**:
- Updated Cargo.toml with full metadata
- Published package on crates.io
- README.md updated with cargo install command

**Validation**:
```bash
# Dry run first
cargo publish --dry-run

# Actual publish (ONE-WAY, cannot undo)
cargo publish

# Test installation (wait ~1 min for crates.io index update)
cargo install token-count

# Verify
token-count --version
```

---

### Phase 5: Documentation (User-Facing)
**Priority**: P1 (Critical for discoverability)  
**Duration**: 1-2 hours  
**Why fifth**: Can be done after installation methods are working

**Components**:
1. `INSTALL.md` - Comprehensive installation guide
2. Update `README.md` installation section
3. Update `CHANGELOG.md` for v0.1.0 release
4. Add manual installation instructions (GitHub Releases)
5. Add checksum verification examples

**Dependencies**: Phases 1-4 (document what exists)

**Deliverables**:
- INSTALL.md with all methods
- README.md with updated installation section
- CHANGELOG.md ready for release
- Release checklist document

**Validation**:
- All installation methods documented
- Examples tested and working
- Links point to correct URLs
- Checksum instructions verified

---

### Phase 6: Testing & Release (Final Validation)
**Priority**: P0 (Must complete before declaring feature done)  
**Duration**: 2-3 hours  
**Why last**: Validates everything works end-to-end

**Components**:
1. Manual testing matrix (all OS + all install methods)
2. Create release checklist
3. Prepare v0.1.0 release
4. Push release tag
5. Verify all installation methods work with official release
6. Update Homebrew formula with official release checksums

**Dependencies**: Phases 1-5 (everything must be ready)

**Deliverables**:
- Tested all installation methods on all platforms
- Official v0.1.0 release published
- Homebrew formula points to official release
- All documentation updated

**Test Matrix**:
| OS | Install Method | Status |
|----|---------------|--------|
| Ubuntu 22.04 | curl\|bash | ☐ |
| Ubuntu 22.04 | GitHub Release (manual) | ☐ |
| Ubuntu 22.04 | Homebrew | ☐ |
| Ubuntu 22.04 | cargo install | ☐ |
| macOS 12 Intel | curl\|bash | ☐ |
| macOS 12 Intel | GitHub Release (manual) | ☐ |
| macOS 12 Intel | Homebrew | ☐ |
| macOS 13 ARM | curl\|bash | ☐ |
| macOS 13 ARM | GitHub Release (manual) | ☐ |
| macOS 13 ARM | Homebrew | ☐ |
| Windows 10 | GitHub Release (manual) | ☐ |

**Validation**:
```bash
# Release process
./scripts/release.sh v0.1.0  # If we create helper script

# Or manual:
git checkout main
git pull
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# Wait for GitHub Actions (monitor)
gh run watch

# Verify release
gh release view v0.1.0

# Test all installation methods
# (See test matrix above)
```

---

## Key Technical Decisions

### 1. GitHub Actions Build Matrix Strategy
**Decision**: Use build matrix with 4 platform targets, build in parallel

**Rationale**:
- Faster: Parallel builds complete in ~10 minutes vs 40 minutes sequential
- Cleaner: Single workflow file vs 4 separate workflows
- Easier to maintain: Change once, applies to all platforms

**Implementation**:
```yaml
strategy:
  matrix:
    include:
      - target: x86_64-unknown-linux-gnu
        os: ubuntu-latest
        archive: tar.gz
      - target: x86_64-apple-darwin
        os: macos-latest
        archive: tar.gz
      - target: aarch64-apple-darwin
        os: macos-latest
        archive: tar.gz
      - target: x86_64-pc-windows-msvc
        os: windows-latest
        archive: zip
```

**Alternatives Considered**:
- ❌ **Separate workflows per platform**: Harder to maintain, no parallelization benefit
- ❌ **cargo-binstall + pre-built binaries**: Adds dependency, less control
- ❌ **Docker-based cross-compilation**: Unnecessary complexity for GitHub Actions

---

### 2. Install Script Platform Detection
**Decision**: Use `uname -s` and `uname -m` for platform detection

**Rationale**:
- Standard POSIX commands available everywhere
- Reliable platform identification
- Simple to test and debug

**Implementation**:
```bash
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$os" in
        linux)
            case "$arch" in
                x86_64|amd64) echo "x86_64-unknown-linux-gnu" ;;
                *) error "Unsupported architecture: $arch" ;;
            esac
            ;;
        darwin)
            case "$arch" in
                x86_64) echo "x86_64-apple-darwin" ;;
                arm64) echo "aarch64-apple-darwin" ;;
                *) error "Unsupported architecture: $arch" ;;
            esac
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac
}
```

**Alternatives Considered**:
- ❌ **rustup target detection**: Not available if Rust not installed
- ❌ **lsb_release**: Linux-only, not POSIX
- ❌ **Manual user selection**: Poor UX, error-prone

---

### 3. Homebrew Tap vs Homebrew Core
**Decision**: Create custom tap (`shaunburdick/homebrew-tap`) instead of submitting to homebrew-core

**Rationale**:
- Faster: No homebrew-core PR review process
- Flexible: Can update formula immediately on each release
- Control: Own the release process
- Lower barrier: homebrew-core has strict requirements (30+ stars, notable project)

**Implementation**:
- Separate repo: `shaunburdick/homebrew-tap`
- Users run: `brew tap shaunburdick/tap && brew install token-count`
- Auto-update formula via GitHub Actions (future enhancement)

**Alternatives Considered**:
- ❌ **Submit to homebrew-core**: Too early, requires project maturity
- ❌ **No Homebrew support**: Major miss for macOS users (primary dev platform)
- ❌ **Inline formula (no tap)**: Not supported by Homebrew

---

### 4. Binary Packaging Format
**Decision**: tar.gz for Unix, zip for Windows

**Rationale**:
- Standard: Expected format for each platform
- Tooling: Built-in tools (`tar`, `Expand-Archive`)
- Compression: Good balance of size vs extraction speed
- Homebrew compatible: tar.gz required for Homebrew bottles

**Implementation**:
```bash
# Unix (Linux, macOS)
tar czf token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz token-count

# Windows
Compress-Archive -Path token-count.exe -DestinationPath token-count-v0.1.0-x86_64-pc-windows-msvc.zip
```

**Alternatives Considered**:
- ❌ **7z for all platforms**: Not universally available
- ❌ **xz compression**: Slower extraction, marginal size benefit
- ❌ **Uncompressed**: Large download, waste bandwidth

---

### 5. Checksum Algorithm
**Decision**: SHA256 for all binary verification

**Rationale**:
- Security: Strong cryptographic hash, industry standard
- Tooling: Available on all platforms (shasum, sha256sum, Get-FileHash)
- Homebrew requirement: SHA256 required in formula
- Git compatibility: Git uses SHA256 (future-proof)

**Implementation**:
```bash
# Generate (Unix)
shasum -a 256 *.tar.gz *.zip > checksums.txt

# Verify (Unix)
shasum -a 256 -c checksums.txt

# Verify (Windows PowerShell)
$hash = Get-FileHash token-count.exe -Algorithm SHA256
```

**Alternatives Considered**:
- ❌ **MD5**: Cryptographically broken, not secure
- ❌ **SHA1**: Deprecated, collision attacks exist
- ❌ **SHA512**: Overkill, longer hashes, no practical benefit

---

## Risk Mitigation

### Risk 1: GitHub Actions Workflow Fails on First Run
**Likelihood**: High (complex workflow, untested)  
**Impact**: High (blocks all installation methods)  

**Mitigation**:
1. Test workflow on feature branch with test tags first
2. Use `workflow_dispatch` trigger for manual testing
3. Add extensive error handling and debug output
4. Reference working examples (rust-lang/rust-analyzer releases)

**Contingency**:
- If workflow fails, debug locally with `act` (GitHub Actions local runner)
- Simplify: Start with Linux-only, add platforms incrementally
- Manual release process fallback (build locally, upload manually)

---

### Risk 2: Homebrew Formula Syntax Errors
**Likelihood**: Medium (Ruby DSL, easy to get wrong)  
**Impact**: Medium (breaks Homebrew install, but other methods still work)

**Mitigation**:
1. Use `brew install --build-from-source` to test formula locally
2. Reference official Homebrew formula examples (ripgrep, bat, fd)
3. Use `brew audit` to validate formula syntax
4. Test on both Intel and ARM Macs

**Contingency**:
- If formula fails, document manual install method prominently
- Get help from Homebrew community (GitHub Discussions)
- Use simpler formula without bottles (source build fallback)

---

### Risk 3: Cross-Platform Binary Compatibility Issues
**Likelihood**: Low (Rust compiles to static binaries)  
**Impact**: High (users can't run binary on their platform)

**Mitigation**:
1. Test on minimum OS versions (Ubuntu 20.04, macOS 10.15, Windows 10)
2. Use GitHub Actions default runners (widely compatible)
3. Strip debug symbols, use LTO (already configured in Cargo.toml)
4. Static linking (Rust default for most dependencies)

**Contingency**:
- Document minimum OS versions in README
- Provide `cargo install` fallback (compile from source)
- Add troubleshooting section for compatibility issues

---

### Risk 4: crates.io Publish Fails (Cannot Undo)
**Likelihood**: Low (dry-run catches most issues)  
**Impact**: Medium (can't use version number again, must bump to 0.1.1)

**Mitigation**:
1. ALWAYS run `cargo publish --dry-run` first
2. Verify all metadata fields in Cargo.toml
3. Check package contents: `cargo package --list`
4. Review included files: ensure no secrets, correct LICENSE
5. Wait until official v0.1.0 release is finalized

**Contingency**:
- If publish fails partway through: Bump version to 0.1.1, republish
- If wrong files included: Yank version, publish 0.1.1 with fix
- Document mistake in CHANGELOG

---

### Risk 5: Checksums Don't Match (Security Issue)
**Likelihood**: Low (automated generation)  
**Impact**: Critical (blocks installation, security concern)

**Mitigation**:
1. Generate checksums in GitHub Actions (same environment as builds)
2. Verify checksums immediately after generation
3. Use consistent line endings (LF)
4. Test checksum verification in install.sh

**Contingency**:
- If mismatch reported: Investigate immediately (compromised runner?)
- Rebuild release from clean checkout
- Add debug output to show actual vs expected hash
- Document checksum verification in INSTALL.md

---

## Performance Targets

### GitHub Actions Workflow
- **Total runtime**: <15 minutes (parallel builds)
- **Per-platform build**: <5 minutes
- **Success rate**: >98% (accounting for transient failures)

### Install Script Performance
- **Execution time**: <30 seconds on broadband (10 Mbps+)
- **Download size**: ~7-8MB (compressed binary)
- **Installation**: <5 seconds (copy + chmod)

### User Experience Targets
- **Time to first token count** (from discovery):
  - curl|bash: <60 seconds
  - Homebrew: <90 seconds  
  - cargo install: <6 minutes
  - Manual download: <2 minutes

---

## Security Considerations

### 1. Install Script Security
**Threats**:
- Man-in-the-middle (MITM) attacks on downloads
- Checksum bypass
- Arbitrary code execution

**Protections**:
- HTTPS-only downloads (curl -fsSL enforces SSL verification)
- SHA256 checksum verification before execution
- No `eval` or arbitrary command execution
- Clear error messages, no silent failures
- Shellcheck validation (catches common vulnerabilities)

**Best Practice for Users**:
```bash
# Review script before running
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | less

# Then run if satisfied
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

### 2. GitHub Actions Security
**Threats**:
- Compromised dependencies in workflow
- Secrets exposure in logs
- Supply chain attacks

**Protections**:
- Pin actions to specific SHA (not `@main` or `@v1`)
- No secrets required (public release workflow)
- Minimal permissions (contents: write for releases)
- Audit logs enabled on repo

### 3. Binary Integrity
**Threats**:
- Tampered binaries in release
- Compromised build environment

**Protections**:
- GitHub-hosted runners (vetted, clean each time)
- Reproducible builds (same commit → same binary)
- SHA256 checksums published alongside binaries
- CHANGELOG documents what's in each release

**Future Enhancement**: Code signing for macOS/Windows binaries (requires paid certificates)

---

## Success Criteria

This feature is **complete** when:

### Functional Requirements
- [x] GitHub Actions workflow builds binaries for 4 platforms on tag push
- [x] Binaries packaged as tar.gz (Unix) and zip (Windows)
- [x] SHA256 checksums generated and published
- [x] GitHub Release created automatically with all assets
- [x] install.sh script downloads, verifies, and installs binary
- [x] Homebrew formula installs token-count from tap
- [x] `cargo install token-count` works from crates.io
- [x] Manual download instructions work for all platforms

### Documentation
- [x] INSTALL.md created with comprehensive guide
- [x] README.md updated with installation section
- [x] CHANGELOG.md prepared for v0.1.0 release
- [x] All installation methods documented with examples
- [x] Checksum verification instructions provided

### Testing
- [x] Install script tested on Ubuntu 22.04, macOS Intel, macOS ARM
- [x] Homebrew installation tested on macOS
- [x] cargo install tested
- [x] Manual download tested on all platforms
- [x] All binaries execute and pass smoke test (echo "test" | token-count)

### Validation Checklist
- [x] Pushed v0.1.0 tag, workflow completed successfully
- [x] Downloaded each binary, verified checksum
- [x] Tested each installation method
- [x] All links in documentation work
- [x] Release notes pulled from CHANGELOG

---

## Next Steps After This Feature

1. **Monitor adoption**: Track download counts from GitHub Releases
2. **Collect feedback**: Watch for installation issues in GitHub Issues
3. **Automate Homebrew updates**: Add workflow to auto-update formula on release
4. **Add Windows installer**: Future: Create PowerShell install script (install.ps1)
5. **Consider cargo-binstall**: Future: Add cargo-binstall metadata for faster installs
6. **Package for Linux distros**: Future: Create .deb and .rpm packages (post-v1.0)

**Feature 002 enables Feature 003+**: Once installation is trivial, more users will adopt the tool, increasing feedback and driving future features (more models, more output formats, etc.).
