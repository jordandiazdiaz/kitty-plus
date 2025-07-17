#!/bin/bash

# Package manager distribution script
# Creates packages for Homebrew, Chocolatey, and package managers

set -e

VERSION=${1:-$(grep "version" Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')}
REPO_URL="https://github.com/jordandiazdiaz/kitty-plus"

echo "Creating packages for version $VERSION"

# Create Homebrew formula
create_homebrew_formula() {
    local formula_dir="packaging/homebrew"
    mkdir -p "$formula_dir"
    
    cat > "$formula_dir/kitty-plus.rb" << EOF
class KittyPlus < Formula
  desc "Modern, GPU-accelerated terminal emulator with AI features"
  homepage "$REPO_URL"
  version "$VERSION"
  
  on_macos do
    if Hardware::CPU.arm?
      url "$REPO_URL/releases/download/v$VERSION/kitty-plus-macos-aarch64.tar.gz"
      sha256 "SHA256_ARM64_PLACEHOLDER"
    else
      url "$REPO_URL/releases/download/v$VERSION/kitty-plus-macos-x86_64.tar.gz"
      sha256 "SHA256_X86_64_PLACEHOLDER"
    end
  end
  
  on_linux do
    url "$REPO_URL/releases/download/v$VERSION/kitty-plus-linux-x86_64.tar.gz"
    sha256 "SHA256_LINUX_PLACEHOLDER"
  end
  
  depends_on "openssl"
  
  def install
    bin.install "kitty-plus"
    
    # Create shell completions directory
    bash_completion.install "completions/kitty-plus.bash" if File.exist?("completions/kitty-plus.bash")
    zsh_completion.install "completions/_kitty-plus" if File.exist?("completions/_kitty-plus")
    fish_completion.install "completions/kitty-plus.fish" if File.exist?("completions/kitty-plus.fish")
  end
  
  test do
    system "#{bin}/kitty-plus", "--version"
  end
end
EOF
    
    echo "Created Homebrew formula: $formula_dir/kitty-plus.rb"
}

# Create Chocolatey package
create_chocolatey_package() {
    local choco_dir="packaging/chocolatey"
    mkdir -p "$choco_dir/tools"
    
    cat > "$choco_dir/kitty-plus.nuspec" << EOF
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>kitty-plus</id>
    <version>$VERSION</version>
    <packageSourceUrl>$REPO_URL</packageSourceUrl>
    <owners>Kitty Plus Team</owners>
    <title>Kitty Plus</title>
    <authors>Kitty Plus Team</authors>
    <projectUrl>$REPO_URL</projectUrl>
    <iconUrl>$REPO_URL/raw/main/assets/icon.png</iconUrl>
    <copyright>2024 Kitty Plus Team</copyright>
    <licenseUrl>$REPO_URL/blob/main/LICENSE</licenseUrl>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
    <projectSourceUrl>$REPO_URL</projectSourceUrl>
    <bugTrackerUrl>$REPO_URL/issues</bugTrackerUrl>
    <tags>terminal emulator gpu ai cross-platform</tags>
    <summary>Modern, GPU-accelerated terminal emulator with AI features</summary>
    <description>
Kitty Plus is a modern, GPU-accelerated terminal emulator built in Rust with AI-powered features and extensive customization options.

Features:
- Command Palette with fuzzy search
- Activity indicators on tabs
- AI-powered command suggestions
- Session recording and playback
- GPU acceleration for smooth rendering
- Cross-platform support
- Plugin system with WebAssembly
    </description>
  </metadata>
  <files>
    <file src="tools\\**" target="tools" />
  </files>
</package>
EOF
    
    cat > "$choco_dir/tools/chocolateyinstall.ps1" << EOF
\$ErrorActionPreference = 'Stop';
\$toolsDir = "\$(Split-Path -parent \$MyInvocation.MyCommand.Definition)"
\$url64 = '$REPO_URL/releases/download/v$VERSION/kitty-plus-windows-x86_64.zip'

\$packageArgs = @{
  packageName   = 'kitty-plus'
  unzipLocation = \$toolsDir
  url64bit      = \$url64
  softwareName  = 'Kitty Plus*'
  checksum64    = 'SHA256_WINDOWS_PLACEHOLDER'
  checksumType64= 'sha256'
}

Install-ChocolateyZipPackage @packageArgs
EOF
    
    echo "Created Chocolatey package: $choco_dir/"
}

# Create Debian package structure
create_debian_package() {
    local debian_dir="packaging/debian"
    mkdir -p "$debian_dir/DEBIAN"
    mkdir -p "$debian_dir/usr/bin"
    mkdir -p "$debian_dir/usr/share/applications"
    mkdir -p "$debian_dir/usr/share/pixmaps"
    
    cat > "$debian_dir/DEBIAN/control" << EOF
Package: kitty-plus
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Depends: libc6, libssl3
Maintainer: Kitty Plus Team <team@kitty-plus.com>
Description: Modern, GPU-accelerated terminal emulator with AI features
 Kitty Plus is a modern, GPU-accelerated terminal emulator built in Rust
 with AI-powered features and extensive customization options.
 .
 Features include command palette, activity indicators, AI suggestions,
 session recording, and cross-platform support.
EOF
    
    cat > "$debian_dir/usr/share/applications/kitty-plus.desktop" << EOF
[Desktop Entry]
Name=Kitty Plus
Comment=Modern GPU-accelerated terminal emulator
Exec=kitty-plus
Icon=kitty-plus
Type=Application
Categories=System;TerminalEmulator;
StartupNotify=true
StartupWMClass=kitty-plus
EOF
    
    echo "Created Debian package structure: $debian_dir/"
}

# Create AppImage
create_appimage() {
    local appimage_dir="packaging/appimage"
    mkdir -p "$appimage_dir"
    
    cat > "$appimage_dir/kitty-plus.desktop" << EOF
[Desktop Entry]
Name=Kitty Plus
Comment=Modern GPU-accelerated terminal emulator
Exec=kitty-plus
Icon=kitty-plus
Type=Application
Categories=System;TerminalEmulator;
EOF
    
    cat > "$appimage_dir/AppRun" << '#!/bin/bash
HERE="$(dirname "$(readlink -f "${0}")")"
export PATH="${HERE}/usr/bin:${PATH}"
exec "${HERE}/usr/bin/kitty-plus" "$@"
EOF'
    
    chmod +x "$appimage_dir/AppRun"
    
    echo "Created AppImage structure: $appimage_dir/"
}

# Create Snap package
create_snap_package() {
    local snap_dir="packaging/snap"
    mkdir -p "$snap_dir"
    
    cat > "$snap_dir/snapcraft.yaml" << EOF
name: kitty-plus
version: '$VERSION'
summary: Modern GPU-accelerated terminal emulator
description: |
  Kitty Plus is a modern, GPU-accelerated terminal emulator built in Rust
  with AI-powered features and extensive customization options.
  
  Features include command palette, activity indicators, AI suggestions,
  session recording, and cross-platform support.

grade: stable
confinement: strict
base: core22

apps:
  kitty-plus:
    command: bin/kitty-plus
    plugs:
      - home
      - network
      - x11
      - wayland
      - opengl
      - audio-playback

parts:
  kitty-plus:
    plugin: nil
    source: .
    override-build: |
      # Download and install binary
      curl -L -o kitty-plus.tar.gz "$REPO_URL/releases/download/v$VERSION/kitty-plus-linux-x86_64.tar.gz"
      tar -xzf kitty-plus.tar.gz
      install -m 755 kitty-plus \$SNAPCRAFT_PART_INSTALL/bin/kitty-plus
EOF
    
    echo "Created Snap package: $snap_dir/"
}

# Main execution
main() {
    echo "Creating package distributions for Kitty Plus v$VERSION"
    
    create_homebrew_formula
    create_chocolatey_package
    create_debian_package
    create_appimage
    create_snap_package
    
    echo ""
    echo "Package distributions created in packaging/ directory"
    echo "Next steps:"
    echo "1. Update SHA256 checksums in package files"
    echo "2. Submit to respective package managers"
    echo "3. Update installation documentation"
}

main "$@"