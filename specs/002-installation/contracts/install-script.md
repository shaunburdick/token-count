# Install Script Interface Contract

## Script Location
**URL**: `https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh`

## Usage

### Basic Usage
```bash
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

### With Version Override
```bash
VERSION=0.1.0 curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

### With Custom Install Directory
```bash
INSTALL_DIR=$HOME/bin curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

### Download and Inspect First (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh -o install.sh
less install.sh  # Review before running
bash install.sh
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VERSION` | `latest` | Version to install (e.g., `0.1.0`) |
| `INSTALL_DIR` | (auto-detect) | Installation directory override |
| `SKIP_CHECKSUM` | `0` | Skip checksum verification (NOT RECOMMENDED) |

## Supported Platforms

### Linux
- **x86_64 (amd64)** - Ubuntu 20.04+, Debian 10+, Fedora 35+
- **aarch64 (arm64)** - Not in MVP (future)

### macOS
- **x86_64 (Intel)** - macOS 10.15+ (Catalina)
- **aarch64 (Apple Silicon)** - macOS 11.0+ (Big Sur)

### Unsupported (exits with error)
- Windows (use PowerShell install script or manual download)
- 32-bit systems (i686, armv7)
- BSD variants (future consideration)

## Installation Paths

**Priority order** (first writable location wins):

1. **$INSTALL_DIR** - If environment variable set
2. **$HOME/.local/bin** - If exists AND in $PATH
3. **/usr/local/bin** - If writable without sudo
4. **Prompt user** - If none of above work

**PATH check**: Script verifies install location is in $PATH, warns if not.

## Exit Codes

| Code | Meaning | Example |
|------|---------|---------|
| `0` | Success | Installation completed |
| `1` | I/O Error | Download failed, file permissions issue |
| `2` | Checksum Mismatch | Downloaded file corrupted or tampered |
| `3` | Unsupported Platform | Running on Windows or armv7 |
| `4` | Dependencies Missing | curl or tar not found |
| `5` | Installation Failed | No writable install directory |

## Required Dependencies

| Tool | Purpose | Fallback |
|------|---------|----------|
| `curl` | Download files | `wget` (auto-detected) |
| `tar` | Extract archive | None (required) |
| `shasum` | Verify checksum | `sha256sum` (auto-detected) |
| `uname` | Platform detection | None (required) |

**Dependency check**: Script checks for required tools and exits with helpful error if missing.

## Script Behavior

### 1. Platform Detection
```bash
$ uname -s  # OS
$ uname -m  # Architecture
```

**Examples**:
- Linux + x86_64 → `x86_64-unknown-linux-gnu`
- Darwin + x86_64 → `x86_64-apple-darwin`
- Darwin + arm64 → `aarch64-apple-darwin`

### 2. Version Resolution
```bash
# If VERSION not set, fetch latest from GitHub API
VERSION=${VERSION:-$(fetch_latest_version)}
```

**Latest version detection**:
```bash
curl -fsSL https://api.github.com/repos/shaunburdick/token-count/releases/latest | grep '"tag_name"' | sed -E 's/.*"v([^"]+)".*/\1/'
```

### 3. Download with Retry
**URL pattern**:
```
https://github.com/shaunburdick/token-count/releases/download/v{version}/token-count-v{version}-{platform}.tar.gz
```

**Retry logic**:
- Max attempts: 3
- Backoff: 2 seconds between attempts
- Timeout: 30 seconds per attempt

### 4. Checksum Verification
**Download checksums**:
```bash
curl -fsSL https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/checksums.txt
```

**Verify** (stops on mismatch):
```bash
shasum -a 256 -c checksums.txt --ignore-missing
```

**Behavior on mismatch**:
- Print error message with actual vs expected hash
- Delete downloaded file (security precaution)
- Exit with code 2
- Suggest manual download and verification

### 5. Extraction
```bash
tar xzf token-count.tar.gz -C /tmp/
```

**Validation**:
- Check binary exists after extraction
- Verify binary is executable (not just +x bit)
- Smoke test: `./token-count --version` (confirms it runs)

### 6. Installation
```bash
# Determine install directory
INSTALL_DIR=$(determine_install_dir)

# Move binary
mv /tmp/token-count "$INSTALL_DIR/"

# Ensure executable
chmod +x "$INSTALL_DIR/token-count"
```

**Post-install validation**:
- Verify binary exists at install path
- Check PATH includes install directory
- Run `token-count --version` from install path

### 7. Cleanup
```bash
# Always runs (trap EXIT)
rm -f /tmp/token-count*
```

## Output Format

### Success Output
```
Detecting platform... macOS ARM64 (aarch64-apple-darwin)
Fetching latest version... v0.1.0
Downloading token-count v0.1.0...
Verifying checksum... ✓
Extracting archive...
Installing to /usr/local/bin... ✓

token-count v0.1.0 installed successfully!

Try it out:
  echo "Hello world" | token-count --model gpt-4

Documentation: https://github.com/shaunburdick/token-count
```

### Error Output Examples

**Unsupported platform**:
```
Error: Unsupported platform: linux-armv7

token-count currently supports:
  - Linux x86_64
  - macOS x86_64 (Intel)
  - macOS aarch64 (Apple Silicon)

For other platforms, try:
  cargo install token-count

Issue tracker: https://github.com/shaunburdick/token-count/issues
```

**Download failure**:
```
Error: Failed to download token-count after 3 attempts

This could be due to:
  - Network connectivity issues
  - GitHub downtime
  - Invalid version (v0.1.0)

Try again later or download manually:
  https://github.com/shaunburdick/token-count/releases
```

**Checksum mismatch**:
```
Error: Checksum verification failed

Expected: a1b2c3d4e5f6...
Actual:   b2c3d4e5f6a1...

This could indicate:
  - Corrupted download
  - Tampered binary

For security, the downloaded file has been deleted.

Please try again. If the problem persists, report it:
  https://github.com/shaunburdick/token-count/issues
```

**No writable install directory**:
```
Error: Cannot install token-count

Tried:
  - $HOME/.local/bin (not in PATH)
  - /usr/local/bin (permission denied)

Options:
  1. Add $HOME/.local/bin to PATH:
       echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
       source ~/.bashrc
       
  2. Run with sudo (installs to /usr/local/bin):
       curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | sudo bash
       
  3. Install to custom directory:
       INSTALL_DIR=$HOME/bin bash install.sh
```

## Security Features

### 1. HTTPS Enforcement
- All downloads use HTTPS
- `curl -fsSL` enforces SSL certificate verification
- No fallback to HTTP

### 2. Checksum Verification
- SHA256 verification enabled by default
- Cannot be disabled without explicit environment variable
- Checksums downloaded from GitHub (same trust boundary)

### 3. No Arbitrary Code Execution
- No `eval` statements
- No sourcing of downloaded scripts
- No dynamic command construction from user input

### 4. Fail Fast
- `set -e`: Exit on any error
- All errors are loud (printed to stderr)
- No silent failures or partial installations

### 5. Cleanup on Error
- `trap` ensures temp files deleted even on error
- No leftover binaries on checksum failure

### 6. Minimal Privileges
- Tries user-local install first (~/.local/bin)
- Only suggests sudo if necessary
- Never runs commands as root without user consent

## Testing Interface

### Unit Tests (shellcheck)
```bash
shellcheck install.sh
```

### Integration Tests
```bash
# Test matrix
./tests/test-install.sh ubuntu-22.04
./tests/test-install.sh macos-12-intel
./tests/test-install.sh macos-13-arm

# Scenarios
TEST_PLATFORM=linux-x86_64 bash install.sh
TEST_VERSION=0.1.0 bash install.sh
TEST_CHECKSUM_FAIL=1 bash install.sh  # Should exit 2
TEST_UNSUPPORTED_PLATFORM=linux-armv7 bash install.sh  # Should exit 3
```

## Maintenance

### Version Updates
**No changes needed** - Script auto-detects latest version from GitHub API.

### Platform Support Changes
**Update** `detect_platform()` function:
```bash
detect_platform() {
    # Add new platforms here
    case "$os-$arch" in
        linux-aarch64) echo "aarch64-unknown-linux-gnu" ;;  # NEW
        # ... existing platforms
    esac
}
```

### URL Changes
**Update** `RELEASE_URL` variable at top of script:
```bash
RELEASE_URL="https://github.com/shaunburdick/token-count/releases/download"
```

## Future Enhancements

- [ ] PowerShell version for Windows (install.ps1)
- [ ] Progress bar for large downloads
- [ ] Auto-update mechanism (check for new version)
- [ ] Uninstall script (uninstall.sh)
- [ ] System-wide vs user-local install option
- [ ] Proxy support (honor HTTP_PROXY env var)
- [ ] GPG signature verification (in addition to checksum)

## Related Documents
- [GitHub Release Structure](./github-release.yaml)
- [INSTALL.md](../../INSTALL.md) - User-facing documentation
- [install.sh](../../install.sh) - Implementation
