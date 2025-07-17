#!/bin/bash

# Kitty Plus Installer
# Supports Linux, macOS, and Windows (via WSL/Git Bash)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="jordandiazdiaz/kitty-plus"
APP_NAME="kitty-plus"
INSTALL_DIR="$HOME/.local/bin"

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case $os in
        linux*)
            os="linux"
            ;;
        darwin*)
            os="macos"
            ;;
        msys*|mingw*|cygwin*)
            os="windows"
            ;;
        *)
            echo -e "${RED}Error: Unsupported OS: $os${NC}"
            exit 1
            ;;
    esac
    
    case $arch in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        *)
            echo -e "${RED}Error: Unsupported architecture: $arch${NC}"
            exit 1
            ;;
    esac
    
    echo "${os}-${arch}"
}

# Get latest release version
get_latest_version() {
    curl -s "https://api.github.com/repos/$REPO/releases/latest" | \
        grep '"tag_name":' | \
        sed -E 's/.*"([^"]+)".*/\1/'
}

# Download and install
install_kitty_plus() {
    local platform=$(detect_platform)
    local version=$(get_latest_version)
    
    if [ -z "$version" ]; then
        echo -e "${RED}Error: Could not fetch latest version${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}Installing Kitty Plus $version for $platform...${NC}"
    
    local filename=""
    case $platform in
        linux-*)
            filename="kitty-plus-${platform}.tar.gz"
            ;;
        macos-*)
            filename="kitty-plus-${platform}.tar.gz"
            ;;
        windows-*)
            filename="kitty-plus-${platform}.zip"
            ;;
    esac
    
    local download_url="https://github.com/$REPO/releases/download/$version/$filename"
    local temp_dir=$(mktemp -d)
    
    echo -e "${YELLOW}Downloading from: $download_url${NC}"
    
    # Download
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "$temp_dir/$filename" "$download_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$temp_dir/$filename" "$download_url"
    else
        echo -e "${RED}Error: curl or wget required${NC}"
        exit 1
    fi
    
    # Create install directory
    mkdir -p "$INSTALL_DIR"
    
    # Extract
    cd "$temp_dir"
    case $filename in
        *.tar.gz)
            tar -xzf "$filename"
            ;;
        *.zip)
            unzip "$filename"
            ;;
    esac
    
    # Install
    local binary_name="kitty-plus"
    if [ "$platform" = "windows-x86_64" ]; then
        binary_name="kitty-plus.exe"
    fi
    
    cp "$binary_name" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$binary_name"
    
    # Cleanup
    rm -rf "$temp_dir"
    
    echo -e "${GREEN}✓ Kitty Plus installed successfully!${NC}"
    echo -e "${YELLOW}Add $INSTALL_DIR to your PATH if not already present${NC}"
    echo -e "${YELLOW}Run: export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    echo -e "${YELLOW}Or add it to your shell profile (.bashrc, .zshrc, etc.)${NC}"
}

# Check for dependencies
check_dependencies() {
    local missing_deps=()
    
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        missing_deps+=("curl or wget")
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        missing_deps+=("tar")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        echo -e "${RED}Error: Missing dependencies: ${missing_deps[*]}${NC}"
        echo -e "${YELLOW}Please install the missing dependencies and try again${NC}"
        exit 1
    fi
}

# Main installation
main() {
    echo -e "${GREEN}Kitty Plus Installer${NC}"
    echo -e "${YELLOW}This will install Kitty Plus to $INSTALL_DIR${NC}"
    echo ""
    
    check_dependencies
    install_kitty_plus
    
    echo ""
    echo -e "${GREEN}Installation complete!${NC}"
    echo -e "${YELLOW}Run 'kitty-plus' to start the terminal${NC}"
}

# Handle script arguments
case "${1:-}" in
    --help|-h)
        echo "Kitty Plus Installer"
        echo "Usage: curl -sSL https://raw.githubusercontent.com/$REPO/main/install.sh | bash"
        echo "   or: bash install.sh"
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac