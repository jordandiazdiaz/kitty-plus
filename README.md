# Kitty Plus - Enhanced Terminal Emulator

A modern, GPU-accelerated terminal emulator built in Rust with AI-powered features and extensive customization options.

## Features

### Core Improvements Over Kitty
- **Command Palette** - VS Code-style command search with fuzzy matching
- **Activity Indicators** - Visual notifications for background tab activity
- **AI Command Suggestions** - Context-aware command completion and explanations
- **Session Recording** - Built-in asciinema-compatible session recording
- **Auto Font Sizing** - Dynamic font adjustment based on window size
- **Plugin System** - WebAssembly-based extensions for custom functionality

### Performance
- **GPU Acceleration** - Hardware-accelerated rendering with wgpu
- **Async Architecture** - Non-blocking terminal operations with Tokio
- **Efficient Text Rendering** - Optimized glyph caching and batching
- **Memory Management** - Smart buffer management for large scrollback

### User Experience
- **Modern UI** - Clean, responsive interface with smooth animations
- **Customizable Themes** - JSON-based theme system with hot reloading
- **Keyboard Shortcuts** - Extensive keyboard navigation and shortcuts
- **Tab Management** - Enhanced tab system with activity indicators

## Installation

### Quick Install (Recommended)
```bash
# One-liner for Linux/macOS
curl -sSL https://raw.githubusercontent.com/jordandiazdiaz/kitty-plus/main/install.sh | bash

# Or download and run
wget https://raw.githubusercontent.com/jordandiazdiaz/kitty-plus/main/install.sh
chmod +x install.sh
./install.sh
```

### Package Managers

**macOS (Homebrew)**
```bash
brew install kitty-plus
```

**Windows (Chocolatey)**
```powershell
choco install kitty-plus
```

**Linux (Multiple options)**
```bash
# Ubuntu/Debian
sudo apt install kitty-plus

# Arch Linux (AUR)
yay -S kitty-plus

# Snap
sudo snap install kitty-plus

# Flatpak
flatpak install flathub com.github.kitty-plus

# AppImage
wget https://github.com/jordandiazdiaz/kitty-plus/releases/latest/download/kitty-plus-x86_64.AppImage
chmod +x kitty-plus-x86_64.AppImage
./kitty-plus-x86_64.AppImage
```

### From Source
```bash
git clone https://github.com/jordandiazdiaz/kitty-plus
cd kitty-plus
cargo build --release
```

### Binary Releases
Download pre-built binaries from [GitHub Releases](https://github.com/jordandiazdiaz/kitty-plus/releases) for:
- **Linux**: x86_64, ARM64 (glibc and musl)
- **macOS**: Intel and Apple Silicon
- **Windows**: x86_64

## Configuration

Configuration is stored in `~/.config/kitty-plus/config.toml`:

```toml
[font]
family = "JetBrains Mono"
size = 14.0
auto_size = true
ligatures = true

[colors]
background = "#1e1e2e"
foreground = "#cdd6f4"
# ... more color options

[features]
command_palette = true
activity_indicators = true
ai_suggestions = true
session_recording = true
plugins = true

[performance]
gpu_acceleration = true
render_fps = 120
cache_size_mb = 256
```

## Usage

### Command Palette
- Press `Ctrl+Shift+P` to open the command palette
- Type to search commands, settings, and actions
- Use arrow keys to navigate, Enter to execute

### AI Features
- Enable AI suggestions in settings
- Get contextual command suggestions as you type
- Ask for command explanations with `Ctrl+Shift+?`

### Session Recording
- Start recording with `Ctrl+Shift+R`
- Stop recording with `Ctrl+Shift+S`
- Recordings are saved in asciinema format

### Plugins
- Install plugins to `~/.config/kitty-plus/plugins/`
- Enable/disable plugins through the command palette
- Write custom plugins in Rust compiled to WebAssembly

## Development

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Features
- Default: All features enabled
- `ai`: AI-powered command suggestions
- `plugins`: WebAssembly plugin system

### Architecture
- **Core** (`src/core/`): Terminal logic, AI, plugins, session recording
- **GPU** (`src/gpu/`): Hardware-accelerated rendering
- **UI** (`src/ui/`): User interface components
- **Terminal** (`src/terminal.rs`): Terminal emulation and state management

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Based on the excellent Kitty terminal emulator
- Inspired by modern terminal emulators like Alacritty and WezTerm
- Built with the amazing Rust ecosystem