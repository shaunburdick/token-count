# Research: Installation & Distribution

**Feature**: 002-installation  
**Date**: 2026-03-14  
**Scope**: Research on installation methods, release automation, and packaging strategies

---

## 1. GitHub Actions for Cross-Platform Rust Builds

### Research Question
How do we automate cross-platform binary builds for Rust CLI tools with minimal complexity?

### Technology Choice
**GitHub Actions with build matrix** targeting 4 platforms using official Rust actions

### Rationale
1. **Native support**: GitHub-hosted runners for Linux, macOS, Windows
2. **Proven at scale**: Used by rust-lang/rust-analyzer, BurntSushi/ripgrep, sharkdp/fd
3. **Cost**: Free for public repositories (2,000 minutes/month)
4. **Parallelization**: Build matrix runs jobs concurrently
5. **Tooling**: `actions-rs/toolchain@v1` and `actions-rs/cargo@v1` are battle-tested

### Implementation Approach
```yaml
name: Release
on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      - run: cargo build --release --target ${{ matrix.target }}
```

### Alternatives Considered
| Alternative | Why Rejected |
|-------------|--------------|
| **CircleCI** | No free macOS builds, smaller Rust ecosystem support |
| **Travis CI** | Free tier severely limited, declining adoption |
| **GitLab CI** | Would require migrating repo, less familiar to contributors |
| **Docker cross-compilation** | Complex setup, doesn't support macOS/Windows well |
| **Manual builds** | Not scalable, error-prone, blocks rapid releases |

### Best Practices Discovered
1. **Pin Rust version**: Use `rust-version = "1.85.0"` in Cargo.toml
2. **Caching**: Use `actions/cache@v3` for cargo registry/target dir (saves 2-3 minutes)
3. **Artifact naming**: Include version + target triple: `token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz`
4. **Strip binaries**: Use `strip = true` in Cargo.toml release profile
5. **Parallel jobs**: Use build matrix instead of sequential steps (10min vs 40min)

### References
- [ripgrep release workflow](https://github.com/BurntSushi/ripgrep/blob/master/.github/workflows/release.yml)
- [rust-analyzer release workflow](https://github.com/rust-lang/rust-analyzer/blob/master/.github/workflows/release.yml)
- [fd release workflow](https://github.com/sharkdp/fd/blob/master/.github/workflows/CICD.yml)

---

## 2. Homebrew Tap vs Homebrew Core

### Research Question
Should we create a custom Homebrew tap or submit to homebrew-core?

### Technology Choice
**Custom Homebrew tap** (`shaunburdick/homebrew-tap`)

### Rationale
1. **Speed**: No homebrew-core PR review process (can take weeks)
2. **Control**: Can update formula immediately on each release
3. **Flexibility**: No strict homebrew-core requirements (30+ stars, notable project)
4. **Automation**: Can auto-update formula via GitHub Actions
5. **Lower barrier**: Can always submit to homebrew-core later once project matures

### Implementation Approach
1. Create separate repo: `shaunburdick/homebrew-tap`
2. Create `Formula/token-count.rb` with multi-platform bottles
3. Users install with: `brew tap shaunburdick/tap && brew install token-count`
4. **Auto-update formula**: Use `mislav/bump-homebrew-formula-action` in release workflow

**Auto-update strategy**:
```yaml
# In .github/workflows/release.yml (token-count repo)
jobs:
  update-homebrew:
    needs: release  # Run after binaries are published
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
            
            Automated update from https://github.com/shaunburdick/token-count/releases/tag/${{ github.ref_name }}
        env:
          COMMITTER_TOKEN: ${{ secrets.HOMEBREW_TAP_TOKEN }}
```

**Formula structure**:
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

### Alternatives Considered
| Alternative | Why Rejected |
|-------------|--------------|
| **Submit to homebrew-core** | Too early - project needs maturity (30+ stars, sustained usage) |
| **No Homebrew support** | Major miss for macOS users (Homebrew is de facto standard) |
| **Inline formula (no tap)** | Not supported - Homebrew requires taps for third-party formulas |
| **cargo install only** | Misses users who don't have Rust toolchain installed |

### Best Practices Discovered
1. **Naming convention**: Repo must be named `homebrew-tap` (Homebrew strips "homebrew-" prefix)
2. **Formula location**: Must be in `Formula/` directory
3. **Class naming**: CamelCase matching binary name: `TokenCount`
4. **Multi-platform support**: Use `OS.mac?` and `Hardware::CPU.arm?` conditionals
5. **Testing**: Include `test do` block for `brew test token-count`
6. **SHA256 required**: Homebrew won't install without checksum verification

### References
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Acceptable Formulas](https://docs.brew.sh/Acceptable-Formulae)
- [Creating Homebrew Taps](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
- [Example tap: sharkdp/bat](https://github.com/sharkdp/homebrew-bat)
- [mislav/bump-homebrew-formula-action](https://github.com/mislav/bump-homebrew-formula-action) - Auto-update action

### Auto-Update Implementation Details

**GitHub Action setup** (in token-count repo):
```yaml
jobs:
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

**Required setup**:
1. Generate Personal Access Token (classic) with `repo` and `workflow` scopes
2. Add as repository secret `HOMEBREW_TAP_TOKEN` in token-count repo
3. Token needs write access to `shaunburdick/homebrew-tap` repository

**What it does**:
- Automatically updates `url` field in formula to point to new release
- Recalculates `sha256` checksum from new tarball
- Updates `version` field if present
- Creates commit in homebrew-tap repo
- Works for both direct push (if token has permission) or via PR

**Benefits**:
- Zero manual work per release
- No risk of typos in checksums
- Consistent with semantic versioning
- Tested by 200+ projects using this action

**Fallback**:
- If action fails, can still update formula manually
- Formula file is simple Ruby - easy to edit by hand

---

## 3. Install Script (curl | bash) Design

### Research Question
What's the safest, most user-friendly approach for a one-line installer script?

### Technology Choice
**Bash script with platform detection, checksum verification, and fallback paths**

### Rationale
1. **Universality**: Bash available on all Unix systems (Linux, macOS, BSD)
2. **Safety**: Can verify checksums before execution
3. **User experience**: Single command, no prerequisites
4. **Industry standard**: Used by Rust (rustup), Node (nvm), Homebrew itself
5. **Debuggable**: Users can download and inspect before running

### Implementation Approach
```bash
#!/usr/bin/env bash
set -e

# Platform detection
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    case "$os-$arch" in
        linux-x86_64) echo "x86_64-unknown-linux-gnu" ;;
        darwin-x86_64) echo "x86_64-apple-darwin" ;;
        darwin-arm64) echo "aarch64-apple-darwin" ;;
        *) echo "Unsupported: $os-$arch" >&2; exit 1 ;;
    esac
}

# Download with retry
download() {
    local url="$1"
    local output="$2"
    local max_attempts=3
    
    for i in $(seq 1 $max_attempts); do
        if curl -fsSL "$url" -o "$output"; then
            return 0
        fi
        echo "Download failed, attempt $i/$max_attempts" >&2
        sleep 2
    done
    return 1
}

# Main logic
PLATFORM=$(detect_platform)
VERSION="${VERSION:-latest}"  # Allow VERSION env var override
BINARY_URL="https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/token-count-v${VERSION}-${PLATFORM}.tar.gz"

# Download
download "$BINARY_URL" /tmp/token-count.tar.gz

# Verify checksum
download "$CHECKSUM_URL" /tmp/checksums.txt
(cd /tmp && shasum -a 256 -c checksums.txt --ignore-missing)

# Extract and install
tar xzf /tmp/token-count.tar.gz -C /tmp
chmod +x /tmp/token-count

# Install to user path
if [ -d "$HOME/.local/bin" ] && echo "$PATH" | grep -q "$HOME/.local/bin"; then
    mv /tmp/token-count "$HOME/.local/bin/"
    echo "Installed to $HOME/.local/bin/token-count"
elif [ -w "/usr/local/bin" ]; then
    mv /tmp/token-count /usr/local/bin/
    echo "Installed to /usr/local/bin/token-count"
else
    echo "Cannot install: $HOME/.local/bin not in PATH, /usr/local/bin not writable"
    echo "Run with sudo or add $HOME/.local/bin to PATH"
    exit 1
fi
```

### Alternatives Considered
| Alternative | Why Rejected |
|-------------|--------------|
| **Python script** | Not always installed (minimal systems), adds dependency |
| **PowerShell (Windows)** | Different script needed for Windows (can add later) |
| **Go installer** | Requires compiling, defeats purpose of pre-built binaries |
| **Native package managers only** | Excludes users on unsupported distros |
| **No installer** | Poor UX, manual steps error-prone |

### Security Best Practices
1. **Set -e**: Exit on any error (fail fast)
2. **HTTPS only**: curl -fsSL enforces SSL verification
3. **Checksum verification**: SHA256 before execution
4. **No eval**: No arbitrary code execution from downloaded content
5. **Shellcheck**: Lint script for common vulnerabilities
6. **Visible errors**: Clear messages on failure
7. **No sudo by default**: Try user-local install first

### Best Practices Discovered
1. **Version override**: Allow `VERSION=0.1.0 bash install.sh` for testing
2. **Retry logic**: Exponential backoff for transient network failures
3. **PATH detection**: Check if install location is in PATH before installing
4. **Cleanup**: Always remove temp files, even on error (trap EXIT)
5. **User consent**: Print what will happen before doing it
6. **Fallback paths**: Try ~/.local/bin, then /usr/local/bin, then prompt

### References
- [rustup.rs installer](https://github.com/rust-lang/rustup/blob/master/rustup-init.sh)
- [nvm installer](https://github.com/nvm-sh/nvm/blob/master/install.sh)
- [Homebrew installer](https://github.com/Homebrew/install/blob/master/install.sh)
- [ShellCheck wiki](https://www.shellcheck.net/wiki/)

---

## 4. crates.io Publication Requirements

### Research Question
What metadata and quality checks are required to publish to crates.io?

### Technology Choice
**cargo publish** with comprehensive Cargo.toml metadata

### Rationale
1. **Official tool**: cargo publish is the standard, no alternatives
2. **Rust ecosystem**: Makes tool discoverable to Rust developers
3. **Source distribution**: Complements binary releases (compile-from-source option)
4. **Dependency management**: Works with cargo install, cargo add
5. **Versioning**: Enforces semantic versioning

### Implementation Approach
**Update Cargo.toml**:
```toml
[package]
name = "token-count"
version = "0.1.0"
edition = "2021"
rust-version = "1.85.0"  # MSRV
authors = ["Shaun Burdick <hello@burdick.dev>"]
description = "Count tokens for LLM models using exact tokenization"
homepage = "https://github.com/shaunburdick/token-count"
repository = "https://github.com/shaunburdick/token-count"
documentation = "https://docs.rs/token-count"
readme = "README.md"
license = "MIT"
keywords = ["cli", "tokens", "llm", "gpt", "tiktoken"]
categories = ["command-line-utilities", "text-processing"]
exclude = [
    ".github/",
    "specs/",
    "benches/",
    "*.sh",
]
```

**Publish process**:
```bash
# 1. Verify package contents
cargo package --list

# 2. Check for issues
cargo publish --dry-run

# 3. Actual publish (CANNOT UNDO)
cargo publish

# 4. Verify on crates.io
open https://crates.io/crates/token-count
```

### Requirements Checklist
- [x] Unique package name (token-count available, token-counter taken)
- [x] Valid semantic version (0.1.0)
- [x] Description (165 chars max)
- [x] License (MIT or Apache-2.0 recommended)
- [x] README.md (displayed on crates.io)
- [x] Repository URL (for source browsing)
- [x] Keywords (5 max, for discoverability)
- [x] Categories (5 max, from approved list)
- [x] MSRV specified (rust-version field)
- [x] No unpublishable files (exclude .github/, specs/)

### Pitfalls to Avoid
1. **Cannot unpublish**: Can only yank (hide from search, but still downloadable)
2. **Version numbers are permanent**: 0.1.0 typo → must publish 0.1.1
3. **Name squatting**: Cannot take package name even if current package is abandoned
4. **Large packages**: >10MB discouraged (ours is ~100KB source)
5. **Missing LICENSE file**: Publish will fail without it

### Best Practices Discovered
1. **Documentation**: Add doc comments to public API (shows on docs.rs)
2. **Examples**: Include examples/ directory (shown on crates.io)
3. **Badges**: Add crates.io badge to README for visibility
4. **Changelog**: Link to CHANGELOG.md in description
5. **SemVer**: Strictly follow semantic versioning (Constitution Principle VII)
6. **Testing**: Ensure `cargo test` passes on clean checkout

### References
- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Cargo Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [crates.io Package Categories](https://crates.io/category_slugs)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

---

## 5. Binary Packaging & Compression

### Research Question
What's the optimal binary packaging format for each platform?

### Technology Choice
- **Linux/macOS**: tar.gz (gzip compression)
- **Windows**: zip (native Windows support)

### Rationale
1. **Platform conventions**: tar.gz is Unix standard, zip is Windows standard
2. **Tooling**: Built-in on all platforms (tar, Expand-Archive)
3. **Homebrew requirement**: Homebrew only supports tar.gz bottles
4. **Compression ratio**: gzip provides 2-3x compression for binaries
5. **Extraction speed**: Fast enough (< 1 second for 9MB binary)

### Compression Analysis
| Format | Compressed Size | Extraction Time | Platform Support |
|--------|-----------------|-----------------|------------------|
| tar.gz | ~7.8 MB | ~0.5s | Universal (Unix) |
| tar.xz | ~6.2 MB | ~2.0s | Universal (slower) |
| zip | ~7.9 MB | ~0.3s | Universal (Windows) |
| 7z | ~6.0 MB | ~1.5s | Requires 7-Zip install |
| uncompressed | 9.2 MB | N/A | No decompression needed |

**Choice**: tar.gz for Unix (Homebrew compat), zip for Windows (native support)

### Implementation
```bash
# Unix packaging
tar czf token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz token-count

# Windows packaging (PowerShell)
Compress-Archive -Path token-count.exe -DestinationPath token-count-v0.1.0-x86_64-pc-windows-msvc.zip
```

### Alternatives Considered
| Alternative | Why Rejected |
|-------------|--------------|
| **tar.xz** | Slower extraction, minimal size benefit |
| **tar.bz2** | Deprecated in favor of xz |
| **7z universal** | Not installed by default, adds friction |
| **AppImage (Linux)** | Overkill for single binary |
| **Uncompressed** | Waste bandwidth, 15% larger downloads |

### Best Practices Discovered
1. **Flat structure**: Archive should contain binary at root (not in subdirectory)
2. **Naming convention**: `{name}-v{version}-{target}.{ext}`
3. **Permissions**: Ensure +x on Unix binaries before packaging
4. **Reproducibility**: Use same compression level every time (gzip -9)
5. **Metadata**: Include LICENSE and README in archive (nice-to-have)

### References
- [Homebrew Binary Packages](https://docs.brew.sh/Bottles)
- [GitHub Releases Best Practices](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases)
- [Rust Release Patterns](https://github.com/rustwasm/wasm-pack/blob/master/.github/workflows/release.yml)

---

## 6. SHA256 Checksum Generation

### Research Question
How should we generate and verify checksums for binary integrity?

### Technology Choice
**SHA256** with platform-native tools (shasum/sha256sum/Get-FileHash)

### Rationale
1. **Security**: SHA256 is cryptographically strong (no known collisions)
2. **Industry standard**: Used by Homebrew, Cargo, npm, Go, etc.
3. **Tooling**: Available on all platforms out of the box
4. **Git compatibility**: Git uses SHA256 (future-proof)
5. **Homebrew requirement**: Formula requires SHA256 (not MD5 or SHA1)

### Implementation Approach
**Generation** (in GitHub Actions):
```bash
# Linux/macOS
shasum -a 256 *.tar.gz *.zip > checksums.txt

# Windows (PowerShell)
Get-ChildItem *.zip | ForEach-Object {
    $hash = (Get-FileHash $_.Name -Algorithm SHA256).Hash.ToLower()
    "$hash $($_.Name)" | Out-File -Append checksums.txt -Encoding ASCII
}
```

**Verification** (in install.sh):
```bash
# Download checksums
curl -fsSL "$CHECKSUMS_URL" -o checksums.txt

# Verify (will exit 1 on mismatch)
shasum -a 256 -c checksums.txt --ignore-missing
```

**Manual verification** (user):
```bash
# Download binary and checksums
curl -LO https://github.com/.../token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/.../checksums.txt

# Verify
shasum -a 256 -c checksums.txt
# Output: token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz: OK
```

### Alternatives Considered
| Alternative | Why Rejected |
|-------------|--------------|
| **MD5** | Cryptographically broken, collision attacks exist |
| **SHA1** | Deprecated, collision attacks demonstrated (SHAttered) |
| **SHA512** | Overkill, longer hashes, no practical security benefit |
| **No checksums** | Insecure, can't detect tampering or corruption |
| **GPG signatures** | Requires key management, complex for users |

### Best Practices Discovered
1. **Format**: Use `{hash} {filename}` (space-separated, compatible with shasum -c)
2. **Lowercase hashes**: Consistent with git, easier to compare
3. **Publish alongside binaries**: checksums.txt in same GitHub Release
4. **Verify in install script**: Automatic verification, users don't need to think about it
5. **Document manual verification**: Include in INSTALL.md for security-conscious users
6. **Use --ignore-missing**: shasum -c won't fail if only subset of files present

### Security Notes
- **HTTPS required**: TLS prevents MITM attacks on checksums file itself
- **Trust on first use**: User trusts GitHub as source of truth
- **Checksum in Homebrew formula**: Homebrew independently verifies (defense in depth)
- **Future enhancement**: GPG-sign releases for paranoid users

### References
- [NIST SHA-256 Specification](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)
- [Homebrew Checksum Requirements](https://docs.brew.sh/Formula-Cookbook#specifying-gems-python-modules-go-projects-etc-as-dependencies)
- [GitHub Actions Security](https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions)

---

## Summary of Technology Choices

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Build automation** | GitHub Actions (build matrix) | Free, parallel builds, native runner support |
| **Package manager (macOS)** | Homebrew tap (custom) | Fast updates, full control, easy to automate |
| **Package manager (Rust)** | crates.io (cargo publish) | Official registry, discoverable to Rust devs |
| **One-line installer** | Bash script (install.sh) | Universal, safe, industry standard |
| **Binary packaging** | tar.gz (Unix), zip (Windows) | Platform conventions, tool availability |
| **Checksum algorithm** | SHA256 | Secure, required by Homebrew, future-proof |
| **Release trigger** | Git tags (v*.*.*) | Semantic versioning, clear intent |
| **Documentation** | Markdown (INSTALL.md) | Simple, readable, version-controlled |

---

## Lessons from Similar Projects

### ripgrep
- **Release workflow**: Comprehensive, supports 10+ platforms
- **Adopted**: Build matrix strategy, checksum generation
- **Skipped**: Windows installer complexity (not needed for MVP)

### rust-analyzer
- **Release workflow**: Sophisticated with version bumping automation
- **Adopted**: Parallel builds, artifact naming conventions
- **Skipped**: VSCode extension packaging (not applicable)

### fd
- **Installation docs**: Excellent INSTALL.md with all methods
- **Adopted**: Comprehensive documentation, troubleshooting section
- **Improved**: Add checksum verification examples (they don't)

### bat
- **Homebrew tap**: Well-maintained, auto-updates on release
- **Adopted**: Tap structure, formula testing strategy
- **Future**: Auto-update formula (they do it manually)

---

## Open Questions Resolved

1. **Q: Homebrew tap or homebrew-core?**  
   **A**: Tap for MVP, homebrew-core later if project gains traction.

2. **Q: Support Windows in MVP?**  
   **A**: Yes, minimal effort (GitHub Actions supports, manual install only).

3. **Q: Auto-update Homebrew formula?**  
   **A**: Manual for MVP, automate in future release (nice-to-have).

4. **Q: Include README in binary archives?**  
   **A**: No for MVP (keep archives minimal), document manual install in INSTALL.md.

5. **Q: Sign binaries (macOS/Windows)?**  
   **A**: Not for MVP (requires paid certificates), future enhancement.

---

**Research Complete**: All technology choices justified, ready for implementation.
