#!/usr/bin/env bash
#
# token-count installer
# 
# Quick install: curl -sSfL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash
#
# Environment variables:
#   VERSION      - Version to install (default: latest)
#   INSTALL_DIR  - Installation directory (default: auto-detect)
#   DEBUG        - Enable debug output (default: 0)
#
# Copyright (c) 2026 Shaun Burdick <hello@burdick.dev>
# Licensed under the MIT License

set -euo pipefail

# Configuration
GITHUB_REPO="shaunburdick/token-count"
BINARY_NAME="token-count"
INSTALL_DIR="${INSTALL_DIR:-}"
VERSION="${VERSION:-}"
DEBUG="${DEBUG:-0}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print functions
info() {
    echo -e "${BLUE}==>${NC} $*"
}

success() {
    echo -e "${GREEN}✓${NC} $*"
}

warn() {
    echo -e "${YELLOW}Warning:${NC} $*" >&2
}

error() {
    echo -e "${RED}Error:${NC} $*" >&2
    exit 1
}

debug() {
    if [ "$DEBUG" = "1" ]; then
        echo -e "${YELLOW}[DEBUG]${NC} $*" >&2
    fi
}

# Cleanup on exit
TEMP_DIR=""
cleanup() {
    if [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
        debug "Cleaning up temporary directory: $TEMP_DIR"
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

# Check for required commands
check_dependencies() {
    local missing_deps=()
    
    # Check for download tool
    if ! command -v curl >/dev/null 2>&1; then
        if ! command -v wget >/dev/null 2>&1; then
            missing_deps+=("curl or wget")
        fi
    fi
    
    # Check for tar
    if ! command -v tar >/dev/null 2>&1; then
        missing_deps+=("tar")
    fi
    
    # Check for checksum tool
    if ! command -v shasum >/dev/null 2>&1; then
        if ! command -v sha256sum >/dev/null 2>&1; then
            missing_deps+=("shasum or sha256sum")
        fi
    fi
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        error "Missing required dependencies: ${missing_deps[*]}\nPlease install them and try again."
    fi
}

# Detect platform
detect_platform() {
    local os arch target
    
    os="$(uname -s)"
    arch="$(uname -m)"
    
    debug "Detected OS: $os, Architecture: $arch"
    
    case "$os" in
        Linux)
            case "$arch" in
                x86_64|amd64)
                    target="x86_64-unknown-linux-gnu"
                    ;;
                aarch64|arm64)
                    error "Linux ARM64 is not yet supported.\n\nSupported platforms:\n  - Linux x86_64\n  - macOS x86_64 (Intel)\n  - macOS aarch64 (Apple Silicon)\n\nFor other platforms, try:\n  cargo install $BINARY_NAME\n\nIssue tracker: https://github.com/$GITHUB_REPO/issues"
                    ;;
                *)
                    error "Unsupported architecture: $arch\n\nSupported platforms:\n  - Linux x86_64\n  - macOS x86_64 (Intel)\n  - macOS aarch64 (Apple Silicon)\n\nFor other platforms, try:\n  cargo install $BINARY_NAME\n\nIssue tracker: https://github.com/$GITHUB_REPO/issues"
                    ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)
                    target="x86_64-apple-darwin"
                    ;;
                arm64|aarch64)
                    target="aarch64-apple-darwin"
                    ;;
                *)
                    error "Unsupported macOS architecture: $arch"
                    ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            error "Windows is not supported by this install script.\n\nFor Windows, please:\n  1. Download the .zip file from: https://github.com/$GITHUB_REPO/releases\n  2. Or install via cargo: cargo install $BINARY_NAME\n\nSee installation guide: https://github.com/$GITHUB_REPO/blob/main/INSTALL.md"
            ;;
        *)
            error "Unsupported operating system: $os"
            ;;
    esac
    
    echo "$target"
}

# Fetch latest version from GitHub API
fetch_latest_version() {
    local api_url="https://api.github.com/repos/$GITHUB_REPO/releases/latest"
    local version
    
    debug "Fetching latest version from: $api_url"
    
    if command -v curl >/dev/null 2>&1; then
        version=$(curl -sSfL "$api_url" | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/' | head -n 1)
    elif command -v wget >/dev/null 2>&1; then
        version=$(wget -qO- "$api_url" | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/' | head -n 1)
    else
        error "Neither curl nor wget found"
    fi
    
    if [ -z "$version" ]; then
        error "Failed to fetch latest version from GitHub API"
    fi
    
    debug "Latest version: $version"
    echo "$version"
}

# Download file with retry
download_file() {
    local url="$1"
    local output="$2"
    local max_attempts=3
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        debug "Download attempt $attempt/$max_attempts: $url"
        
        if command -v curl >/dev/null 2>&1; then
            if curl -fsSL --connect-timeout 30 --max-time 300 -o "$output" "$url"; then
                return 0
            fi
        elif command -v wget >/dev/null 2>&1; then
            if wget -q --timeout=30 -O "$output" "$url"; then
                return 0
            fi
        fi
        
        if [ $attempt -lt $max_attempts ]; then
            warn "Download failed, retrying in 2 seconds..."
            sleep 2
        fi
        
        attempt=$((attempt + 1))
    done
    
    return 1
}

# Verify checksum
verify_checksum() {
    local file="$1"
    local checksums_file="$2"
    local filename
    filename="$(basename "$file")"
    
    debug "Verifying checksum for: $filename"
    
    # Extract expected hash for this file
    local expected_hash
    expected_hash=$(grep "$filename" "$checksums_file" | awk '{print $1}')
    
    if [ -z "$expected_hash" ]; then
        error "Checksum not found in checksums.txt for: $filename"
    fi
    
    # Compute actual hash
    local actual_hash
    if command -v shasum >/dev/null 2>&1; then
        actual_hash=$(shasum -a 256 "$file" | awk '{print $1}')
    elif command -v sha256sum >/dev/null 2>&1; then
        actual_hash=$(sha256sum "$file" | awk '{print $1}')
    else
        error "No checksum tool found (shasum or sha256sum)"
    fi
    
    debug "Expected hash: $expected_hash"
    debug "Actual hash:   $actual_hash"
    
    # Compare (case-insensitive)
    if [ "${expected_hash,,}" != "${actual_hash,,}" ]; then
        rm -f "$file"
        error "Checksum verification failed!\n\nExpected: $expected_hash\nActual:   $actual_hash\n\nThis could indicate:\n  - Corrupted download\n  - Tampered binary\n\nFor security, the downloaded file has been deleted.\n\nPlease try again. If the problem persists, report it:\n  https://github.com/$GITHUB_REPO/issues"
    fi
    
    success "Checksum verified"
}

# Determine installation directory
determine_install_dir() {
    # If user specified INSTALL_DIR, use it
    if [ -n "$INSTALL_DIR" ]; then
        if [ ! -d "$INSTALL_DIR" ]; then
            mkdir -p "$INSTALL_DIR" || error "Failed to create directory: $INSTALL_DIR"
        fi
        if [ ! -w "$INSTALL_DIR" ]; then
            error "Directory not writable: $INSTALL_DIR"
        fi
        echo "$INSTALL_DIR"
        return
    fi
    
    # Try $HOME/.local/bin (most common user-local bin dir)
    if [ -d "$HOME/.local/bin" ] && [ -w "$HOME/.local/bin" ]; then
        echo "$HOME/.local/bin"
        return
    fi
    
    # Try /usr/local/bin if writable without sudo
    if [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
        echo "/usr/local/bin"
        return
    fi
    
    # Fall back to $HOME/.local/bin and create it
    local fallback_dir="$HOME/.local/bin"
    mkdir -p "$fallback_dir" || error "Failed to create directory: $fallback_dir"
    echo "$fallback_dir"
}

# Check if directory is in PATH
check_path() {
    local dir="$1"
    
    if [[ ":$PATH:" == *":$dir:"* ]]; then
        return 0
    else
        return 1
    fi
}

# Print PATH instructions
print_path_instructions() {
    local install_dir="$1"
    
    echo ""
    warn "The installation directory is not in your PATH!"
    echo ""
    echo "To use $BINARY_NAME, add this to your shell configuration:"
    echo ""
    
    # Detect shell and provide specific instructions
    if [ -n "${BASH_VERSION:-}" ] || [ -n "${ZSH_VERSION:-}" ]; then
        local rc_file
        if [ -n "${ZSH_VERSION:-}" ]; then
            rc_file="~/.zshrc"
        else
            rc_file="~/.bashrc"
        fi
        echo "  echo 'export PATH=\"$install_dir:\$PATH\"' >> $rc_file"
        echo "  source $rc_file"
    elif [ -n "${FISH_VERSION:-}" ]; then
        echo "  fish_add_path $install_dir"
    else
        echo "  export PATH=\"$install_dir:\$PATH\""
        echo ""
        echo "  Add this line to your shell's configuration file (e.g., ~/.bashrc, ~/.zshrc)"
    fi
    echo ""
}

# Main installation function
main() {
    echo ""
    info "Installing $BINARY_NAME..."
    echo ""
    
    # Check dependencies
    check_dependencies
    
    # Detect platform
    info "Detecting platform..."
    local target
    target=$(detect_platform)
    local platform_name
    case "$target" in
        x86_64-unknown-linux-gnu) platform_name="Linux x86_64" ;;
        x86_64-apple-darwin) platform_name="macOS Intel" ;;
        aarch64-apple-darwin) platform_name="macOS Apple Silicon" ;;
        *) platform_name="$target" ;;
    esac
    success "Platform: $platform_name ($target)"
    
    # Determine version
    if [ -z "$VERSION" ]; then
        info "Fetching latest version..."
        VERSION=$(fetch_latest_version)
    fi
    success "Version: $VERSION"
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    debug "Created temporary directory: $TEMP_DIR"
    
    # Construct URLs
    local archive_name="${BINARY_NAME}-${VERSION}-${target}.tar.gz"
    local archive_url="https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${archive_name}"
    local checksums_url="https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/checksums.txt"
    
    debug "Archive URL: $archive_url"
    debug "Checksums URL: $checksums_url"
    
    # Download archive
    info "Downloading $BINARY_NAME v$VERSION..."
    local archive_path="$TEMP_DIR/$archive_name"
    if ! download_file "$archive_url" "$archive_path"; then
        error "Failed to download $BINARY_NAME after 3 attempts\n\nThis could be due to:\n  - Network connectivity issues\n  - GitHub downtime\n  - Invalid version (v$VERSION)\n\nTry again later or download manually:\n  https://github.com/$GITHUB_REPO/releases"
    fi
    success "Downloaded"
    
    # Download checksums
    info "Downloading checksums..."
    local checksums_path="$TEMP_DIR/checksums.txt"
    if ! download_file "$checksums_url" "$checksums_path"; then
        error "Failed to download checksums file"
    fi
    success "Downloaded"
    
    # Verify checksum
    info "Verifying checksum..."
    verify_checksum "$archive_path" "$checksums_path"
    
    # Extract archive
    info "Extracting archive..."
    tar xzf "$archive_path" -C "$TEMP_DIR"
    local binary_path="$TEMP_DIR/$BINARY_NAME/$BINARY_NAME"
    
    if [ ! -f "$binary_path" ]; then
        error "Binary not found after extraction: $binary_path"
    fi
    success "Extracted"
    
    # Test binary
    debug "Testing binary..."
    if ! "$binary_path" --version >/dev/null 2>&1; then
        error "Binary smoke test failed (--version returned non-zero)"
    fi
    
    # Determine install directory
    info "Determining installation directory..."
    local install_dir
    install_dir=$(determine_install_dir)
    success "Install directory: $install_dir"
    
    # Install binary
    info "Installing binary..."
    cp "$binary_path" "$install_dir/$BINARY_NAME"
    chmod +x "$install_dir/$BINARY_NAME"
    success "Installed"
    
    # Verify installation
    if [ ! -f "$install_dir/$BINARY_NAME" ]; then
        error "Installation verification failed: binary not found at $install_dir/$BINARY_NAME"
    fi
    
    # Success message
    echo ""
    success "$BINARY_NAME v$VERSION installed successfully!"
    echo ""
    
    # Check PATH
    if ! check_path "$install_dir"; then
        print_path_instructions "$install_dir"
    fi
    
    # Print usage instructions
    echo "Try it out:"
    echo "  echo 'Hello world' | $BINARY_NAME"
    echo ""
    echo "Documentation: https://github.com/$GITHUB_REPO"
    echo ""
}

# Run main function
main "$@"
