# GitHub Release Structure Contract

## Release Naming
- **Tag format**: `v{major}.{minor}.{patch}` (e.g., `v0.1.0`)
- **Release title**: `v{major}.{minor}.{patch}`
- **Release body**: Extracted from CHANGELOG.md for this version

## Assets Structure

### Binary Archives (4 files)
```
token-count-v{version}-x86_64-unknown-linux-gnu.tar.gz
token-count-v{version}-x86_64-apple-darwin.tar.gz
token-count-v{version}-aarch64-apple-darwin.tar.gz
token-count-v{version}-x86_64-pc-windows-msvc.zip
```

**Naming convention**:
- Pattern: `{name}-v{version}-{target}.{extension}`
- Name: Always `token-count` (hyphenated)
- Version: Matches git tag (e.g., `0.1.0`)
- Target: Rust target triple
- Extension: `.tar.gz` for Unix, `.zip` for Windows

**Archive contents**:
- Unix (tar.gz): `token-count` (executable, +x permission)
- Windows (zip): `token-count.exe` (executable)
- No subdirectories, binary at archive root

**Size constraints**:
- Uncompressed: ~9.2 MB
- Compressed: <10 MB (target: 7-8 MB)

### Checksums File (1 file)
```
checksums.txt
```

**Format** (compatible with `shasum -c`):
```
{sha256_hash} token-count-v{version}-x86_64-unknown-linux-gnu.tar.gz
{sha256_hash} token-count-v{version}-x86_64-apple-darwin.tar.gz
{sha256_hash} token-count-v{version}-aarch64-apple-darwin.tar.gz
{sha256_hash} token-count-v{version}-x86_64-pc-windows-msvc.zip
```

**Requirements**:
- Algorithm: SHA256
- Hash format: Lowercase hexadecimal (64 characters)
- Separator: Single space between hash and filename
- Line endings: LF (Unix style)
- Order: Alphabetical by filename (optional but consistent)

## Example Release (v0.1.0)

### Assets List
```
token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz    (7.8 MB)
token-count-v0.1.0-x86_64-apple-darwin.tar.gz         (7.9 MB)
token-count-v0.1.0-aarch64-apple-darwin.tar.gz        (7.8 MB)
token-count-v0.1.0-x86_64-pc-windows-msvc.zip         (8.1 MB)
checksums.txt                                          (0.3 KB)
```

### checksums.txt Example
```
a1b2c3d4e5f6... token-count-v0.1.0-aarch64-apple-darwin.tar.gz
b2c3d4e5f6a1... token-count-v0.1.0-x86_64-apple-darwin.tar.gz
c3d4e5f6a1b2... token-count-v0.1.0-x86_64-pc-windows-msvc.zip
d4e5f6a1b2c3... token-count-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
```

### Release Body Template
```markdown
# token-count v0.1.0

**Release Date**: 2026-03-XX

## Installation

**curl | bash (Linux/macOS)**:
```bash
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

**Homebrew (macOS/Linux)**:
```bash
brew tap shaunburdick/tap
brew install token-count
```

**Cargo (Rust)**:
```bash
cargo install token-count
```

**Manual Download**:
Download the appropriate binary for your platform from the assets below, extract, and add to PATH.

## Checksum Verification

```bash
curl -LO https://github.com/shaunburdick/token-count/releases/download/v0.1.0/checksums.txt
shasum -a 256 -c checksums.txt --ignore-missing
```

## What's New

[Content extracted from CHANGELOG.md]

---

**Full Changelog**: https://github.com/shaunburdick/token-count/blob/main/CHANGELOG.md
```

## GitHub Actions Outputs

### Job: build
**Inputs**:
- `matrix.target`: Rust target triple
- `matrix.os`: GitHub Actions runner OS
- `matrix.archive`: Archive extension (tar.gz or zip)

**Outputs**:
- `artifact_name`: `token-count-v{version}-{target}.{archive}`
- `artifact_path`: Path to artifact file
- `checksum`: SHA256 hash of artifact

**Upload Artifact**:
- Name: `binary-{target}`
- Path: `token-count-v{version}-{target}.{archive}`
- Retention: 90 days

### Job: release
**Inputs**:
- Downloaded artifacts from build job
- CHANGELOG.md (for release notes)

**Outputs**:
- GitHub Release created
- All artifacts attached
- Release ID (for reference)

**Permissions Required**:
- `contents: write` (create release, upload assets)

## API URLs

### Download Binary
```
https://github.com/shaunburdick/token-count/releases/download/v{version}/token-count-v{version}-{target}.{extension}
```

### Download Checksums
```
https://github.com/shaunburdick/token-count/releases/download/v{version}/checksums.txt
```

### Latest Release API
```
https://api.github.com/repos/shaunburdick/token-count/releases/latest
```

**Response fields** (relevant):
- `tag_name`: Version tag (e.g., "v0.1.0")
- `name`: Release title
- `body`: Release notes
- `assets[]`: Array of asset objects
  - `name`: Filename
  - `browser_download_url`: Direct download URL
  - `size`: File size in bytes

## Validation Checklist

Before marking release as complete:
- [ ] 4 binary archives present (Linux, macOS x64, macOS ARM, Windows)
- [ ] 1 checksums.txt file present
- [ ] All files have correct naming convention
- [ ] checksums.txt format valid (shasum -c passes)
- [ ] Release notes extracted from CHANGELOG.md
- [ ] Tag matches version in Cargo.toml
- [ ] All binaries are executable and pass smoke test
- [ ] Archives extract correctly on target platform
- [ ] File sizes within expected range (<10MB compressed)

## Error Scenarios

### Build Failure
- **Symptom**: One or more platform builds fail
- **Resolution**: Re-run workflow, check build logs, fix code issue
- **Impact**: Release blocked until all builds succeed

### Checksum Mismatch
- **Symptom**: shasum -c reports "FAILED"
- **Resolution**: Regenerate checksums, investigate tampering
- **Impact**: Critical security issue, block release

### Upload Failure
- **Symptom**: Assets not attached to release
- **Resolution**: Re-run release job, check GitHub API rate limits
- **Impact**: Release incomplete, users can't download

### Wrong Version
- **Symptom**: Tag doesn't match Cargo.toml version
- **Resolution**: Delete tag, update Cargo.toml, re-tag
- **Impact**: Confusion, version mismatch
