# Installation Guide

This guide covers all methods for installing `token-count` on your system.

## Quick Install (Recommended)

### Linux / macOS

```bash
curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

This script will:
- Detect your platform automatically
- Download the latest version
- Verify checksums for security
- Install to an appropriate directory
- Provide PATH configuration help if needed

**Custom installation:**
```bash
# Install specific version
VERSION=0.1.0 curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash

# Install to custom directory
INSTALL_DIR=$HOME/bin curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash

# Debug mode
DEBUG=1 bash <(curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh)
```

## Installation Methods

### Homebrew (macOS / Linux)

```bash
brew install shaunburdick/tap/token-count
```

**Update to latest version:**
```bash
brew update
brew upgrade token-count
```

**Uninstall:**
```bash
brew uninstall token-count
brew untap shaunburdick/tap
```

### Cargo (All Platforms)

If you have Rust installed:

```bash
cargo install token-count
```

**Requirements:**
- Rust 1.85.0 or later
- Cargo (comes with Rust)

**Update:**
```bash
cargo install token-count --force
```

**Uninstall:**
```bash
cargo uninstall token-count
```

**Install Rust:** Visit https://rustup.rs/

### Manual Download (All Platforms)

Download pre-built binaries from [GitHub Releases](https://github.com/shaunburdick/token-count/releases).

#### Linux (x86_64)

```bash
VERSION=0.1.0
curl -LO "https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/token-count-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
curl -LO "https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/checksums.txt"

# Verify checksum
grep "token-count-${VERSION}-x86_64-unknown-linux-gnu.tar.gz" checksums.txt | shasum -a 256 -c -

# Extract and install
tar xzf "token-count-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
sudo install -m 755 token-count/token-count /usr/local/bin/token-count
```

#### macOS (Intel)

```bash
VERSION=0.1.0
curl -LO "https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/token-count-${VERSION}-x86_64-apple-darwin.tar.gz"
curl -LO "https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/checksums.txt"

# Verify checksum
grep "token-count-${VERSION}-x86_64-apple-darwin.tar.gz" checksums.txt | shasum -a 256 -c -

# Extract and install
tar xzf "token-count-${VERSION}-x86_64-apple-darwin.tar.gz"
sudo install -m 755 token-count/token-count /usr/local/bin/token-count
```

#### macOS (Apple Silicon)

```bash
VERSION=0.1.0
curl -LO "https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/token-count-${VERSION}-aarch64-apple-darwin.tar.gz"
curl -LO "https://github.com/shaunburdick/token-count/releases/download/v${VERSION}/checksums.txt"

# Verify checksum
grep "token-count-${VERSION}-aarch64-apple-darwin.tar.gz" checksums.txt | shasum -a 256 -c -

# Extract and install
tar xzf "token-count-${VERSION}-aarch64-apple-darwin.tar.gz"
sudo install -m 755 token-count/token-count /usr/local/bin/token-count
```

#### Windows (x86_64)

1. Download: [token-count-0.1.0-x86_64-pc-windows-msvc.zip](https://github.com/shaunburdick/token-count/releases/download/v0.1.0/token-count-0.1.0-x86_64-pc-windows-msvc.zip)
2. Download: [checksums.txt](https://github.com/shaunburdick/token-count/releases/download/v0.1.0/checksums.txt)
3. Verify checksum (PowerShell):
   ```powershell
   $hash = (Get-FileHash -Algorithm SHA256 token-count-0.1.0-x86_64-pc-windows-msvc.zip).Hash
   Select-String -Path checksums.txt -Pattern "token-count-0.1.0-x86_64-pc-windows-msvc.zip"
   # Compare the hashes manually
   ```
4. Extract the ZIP file
5. Add the `token-count` directory to your PATH or move `token-count.exe` to a directory already in your PATH

### Build from Source

```bash
# Clone repository
git clone https://github.com/shaunburdick/token-count.git
cd token-count

# Build release binary
cargo build --release

# Install
sudo install -m 755 target/release/token-count /usr/local/bin/token-count
```

**Requirements:**
- Rust 1.85.0 or later
- Git

## Platform Support

| Platform | Architecture | Support | Installation Method |
|----------|--------------|---------|---------------------|
| Linux | x86_64 | ✅ Full | All methods |
| Linux | aarch64 (ARM64) | ⏳ Planned | Build from source |
| macOS | x86_64 (Intel) | ✅ Full | All methods |
| macOS | aarch64 (Apple Silicon) | ✅ Full | All methods |
| Windows | x86_64 | ✅ Full | Manual download, Cargo |
| Windows | aarch64 | ⏳ Planned | Build from source |

## Verification

After installation, verify it works:

```bash
# Check version
token-count --version

# Test basic functionality
echo "Hello, world!" | token-count

# Test with specific model
echo "The quick brown fox jumps over the lazy dog" | token-count --model gpt-4
```

**Expected output:**
```
9
```

## Security

All pre-built binaries include SHA256 checksums for verification. The install script verifies checksums automatically.

**Manual verification:**
```bash
# Download checksums file
curl -LO "https://github.com/shaunburdick/token-count/releases/download/v0.1.0/checksums.txt"

# Verify your downloaded file
shasum -a 256 -c checksums.txt --ignore-missing
```

**Security contacts:** hello@burdick.dev

## Troubleshooting

### Install script fails with "command not found"

The installation directory may not be in your PATH.

**Solution:**
```bash
# For bash/zsh users
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For fish users
fish_add_path $HOME/.local/bin
```

### Homebrew installation fails

**Issue:** `Error: shaunburdick/tap/token-count not found`

**Solution:**
```bash
brew update
brew tap shaunburdick/tap
brew install token-count
```

### Cargo installation fails

**Issue:** `error: failed to compile token-count`

**Solutions:**
1. Update Rust: `rustup update`
2. Check minimum version: `rustc --version` (need 1.85.0+)
3. Install Rust if missing: https://rustup.rs/

### Permission denied on Linux/macOS

**Issue:** `Permission denied` when copying to `/usr/local/bin`

**Solutions:**
1. Use `sudo` for system-wide install
2. Or install to user directory:
   ```bash
   INSTALL_DIR=$HOME/.local/bin bash install.sh
   ```

### Binary won't run on macOS

**Issue:** "token-count cannot be opened because the developer cannot be verified"

**Solution:**
```bash
xattr -d com.apple.quarantine /usr/local/bin/token-count
```

Or go to System Preferences → Security & Privacy → General → "Allow Anyway"

### Windows SmartScreen warning

**Issue:** Windows Defender SmartScreen blocks the binary

**Solution:**
1. Click "More info"
2. Click "Run anyway"

This happens because the binary is not code-signed. You can verify the checksum to ensure it's authentic.

## Updating

### Install Script / Manual Download
```bash
# Run install script again to get latest version
curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

### Homebrew
```bash
brew update
brew upgrade token-count
```

### Cargo
```bash
cargo install token-count --force
```

## Uninstallation

### Install Script / Manual Download
```bash
# Find where it's installed
which token-count

# Remove it
sudo rm /usr/local/bin/token-count
# Or
rm $HOME/.local/bin/token-count
```

### Homebrew
```bash
brew uninstall token-count
brew untap shaunburdick/tap  # Optional: remove tap
```

### Cargo
```bash
cargo uninstall token-count
```

## Advanced Configuration

### Install to Custom Directory

```bash
INSTALL_DIR=/opt/bin curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
```

### Offline Installation

1. Download the archive and checksums file on a connected machine
2. Transfer to offline machine
3. Verify checksum: `shasum -a 256 -c checksums.txt --ignore-missing`
4. Extract and install: `tar xzf token-count-*.tar.gz && sudo install -m 755 token-count/token-count /usr/local/bin/`

### System-Wide Installation (Linux)

For multi-user systems:

```bash
# Install to /usr/local/bin (accessible to all users)
sudo curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | sudo bash

# Or manually
sudo install -m 755 token-count /usr/local/bin/token-count
```

## Getting Help

- **Documentation:** https://github.com/shaunburdick/token-count
- **Issues:** https://github.com/shaunburdick/token-count/issues
- **Security:** hello@burdick.dev

## Next Steps

After installation, check out:
- [README](README.md) - Usage examples and features
- [CHANGELOG](CHANGELOG.md) - Version history
- [CONTRIBUTING](CONTRIBUTING.md) - How to contribute
