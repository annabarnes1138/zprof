# Installation

This guide covers installing zprof on your system.

## Prerequisites

- **Zsh**: zprof requires zsh to be installed
- **Rust** (for building from source): Rust 1.70 or later
- **Git**: Required for framework installation and GitHub imports

### Check if you have zsh

```bash
zsh --version
```

If not installed:
- **macOS**: `brew install zsh`
- **Ubuntu/Debian**: `sudo apt install zsh`
- **Fedora**: `sudo dnf install zsh`
- **Arch**: `sudo pacman -S zsh`

## Installation Methods

### Option 1: From Crates.io (Recommended)

```bash
cargo install zprof
```

### Option 2: Build from Source

```bash
git clone https://github.com/annabarnes1138/zprof.git
cd zprof
cargo build --release

# The binary will be at target/release/zprof
# Optionally, copy to your PATH:
sudo cp target/release/zprof /usr/local/bin/
```

### Option 3: Pre-built Binaries

Download the latest release from [GitHub Releases](https://github.com/annabarnes1138/zprof/releases):

```bash
# macOS (Apple Silicon)
wget https://github.com/annabarnes1138/zprof/releases/download/v0.1.1/zprof-macos-arm64

# macOS (Intel)
wget https://github.com/annabarnes1138/zprof/releases/download/v0.1.1/zprof-macos-x64

# Linux (x86_64)
wget https://github.com/annabarnes1138/zprof/releases/download/v0.1.1/zprof-linux-x64

# Make executable and move to PATH
chmod +x zprof-*
sudo mv zprof-* /usr/local/bin/zprof
```

## Verify Installation

```bash
zprof --version
```

You should see something like:
```
zprof 0.1.1
```

## Next Steps

Once installed, initialize zprof:

```bash
zprof init
```

This will:
- Create the `~/.zsh-profiles/` directory structure
- Detect and optionally import your existing zsh configuration
- Set up shared history and customizations

Continue to the [Quick Start](quick-start.md) guide to create your first profile.

## Troubleshooting

### Permission denied when installing

If you get permission errors with `cargo install`, try:

```bash
cargo install --force zprof
```

### Command not found after installation

Make sure `~/.cargo/bin` is in your PATH:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Already have ~/.zsh-profiles/?

If you already have a `~/.zsh-profiles/` directory from a previous installation, `zprof init` will skip initialization to preserve your data. This is safe and expected.

For more help, see the [Troubleshooting Guide](troubleshooting.md).
