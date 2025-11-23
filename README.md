# zprof

Manage multiple zsh profiles with ease - via CLI or GUI.

## Overview

`zprof` is a tool that helps you manage multiple zsh shell configurations (profiles) with different frameworks, themes, and plugins. Switch between profiles instantly based on your workflow needs.

**Choose your interface:**

- 💻 **CLI** (Command-Line Interface) - Fast, scriptable, perfect for power users and automation
- 🖥️ **GUI** (Graphical Interface) - Visual, intuitive, with theme previews and guided workflows

Both interfaces are complementary and share the same business logic - use whichever fits your workflow!

## Features

- 🚀 **Multiple Profiles**: Create and manage separate zsh configurations for different contexts (work, personal, experimental)
- 🎨 **Framework Support**: Works with oh-my-zsh, zimfw, prezto, zinit, and zap
- 🔄 **Quick Switching**: Switch between profiles with a single command
- 📝 **TOML Manifests**: Human-readable configuration files with live validation
- 📦 **Export & Import**: Share profiles as portable `.zprof` archives
- 🌐 **GitHub Integration**: Import profiles directly from GitHub repositories
- 🛡️ **Non-Destructive**: All operations create automatic backups
- 🔙 **Easy Rollback**: Restore your original configuration anytime

## Installation

### CLI Installation

Install the command-line interface:

```bash
cargo install zprof
```

Or build from source:

```bash
git clone https://github.com/yourusername/zprof.git
cd zprof
cargo build --release
```

For a CLI-only build without GUI dependencies:

```bash
cargo build --release --no-default-features
```

### GUI Installation

The GUI is included by default. To launch it:

```bash
# From CLI (informational - full GUI integration coming soon)
zprof gui

# Or build and run the GUI directly
cargo tauri dev       # Development mode
cargo tauri build     # Production build
```

See the [GUI Development](#gui-development) section for more details.

## Quick Start

### Initialize zprof

```bash
zprof init
```

This creates the `~/.zsh-profiles/` directory structure and automatically migrates your existing zsh configuration if you have one.

### Create a Profile

```bash
zprof create work
```

Follow the interactive wizard to select a framework, theme, and plugins.

### List Profiles

```bash
zprof list
```

### Switch Profiles

```bash
zprof use work
```

### View Current Profile

```bash
zprof current
```

### Delete a Profile

```bash
zprof delete old-profile
```

Profiles are safely backed up before deletion to `~/.zsh-profiles/cache/backups/`.

## Advanced Features

### Edit Profile Manifest

Edit your profile's TOML configuration with live validation:

```bash
zprof edit work
```

This opens your profile's `profile.toml` in your preferred editor (respects `$VISUAL`, `$EDITOR`, or falls back to `vim`). After saving, zprof automatically:
- Validates the TOML syntax
- Checks for required fields and valid values
- Regenerates shell configuration files if valid
- Shows helpful error messages if invalid

### Export Profile to Archive

Share your profile by exporting it to a portable `.zprof` archive:

```bash
zprof export work
```

This creates a `work.zprof` file (tar.gz archive) containing:
- `profile.toml` manifest
- Generated shell configurations (for reference)
- Custom configuration files
- Export metadata (date, zprof version, exported by)

**Note:** Framework binaries are excluded to keep archives small. The manifest describes what should be installed.

**Export Options:**
```bash
# Export to custom location
zprof export work --output ~/backups/work.zprof

# Overwrite existing archive
zprof export work --force
```

### Import Profile from Archive

Import a profile from a `.zprof` archive:

```bash
zprof import work.zprof
```

This will:
1. Extract and validate the archive
2. Check for profile name conflicts (prompts for resolution)
3. Install the framework and plugins per manifest
4. Create the profile in `~/.zsh-profiles/profiles/`
5. Regenerate shell configurations

**Import Options:**
```bash
# Import with a different name
zprof import work.zprof --name work-backup

# Skip conflict prompts (overwrite)
zprof import work.zprof --force
```

### Import Profile from GitHub

Import profiles directly from GitHub repositories:

```bash
zprof import github:username/repo
```

This clones the repository, searches for `profile.toml`, and imports it as a profile. Perfect for:
- Sharing team standardized profiles
- Using community profiles
- Publishing your own profiles

**Examples:**
```bash
# Import from public repository
zprof import github:myteam/zsh-work-profile

# Import from repository with hyphens
zprof import github:my-org/zsh-config
```

**Requirements:**
- Repository must contain `profile.toml` in the root
- For private repos, git credentials must be configured

### Regenerate Shell Configurations

Regenerate `.zshrc` and `.zshenv` from your profile's TOML manifest:

```bash
zprof regenerate work
```

Useful after manually editing the `profile.toml` file or when updating to a new zprof version.

## Rollback to Pre-zprof State

If you want to uninstall zprof and restore your original shell configuration:

```bash
zprof rollback
```

### What Rollback Does

The rollback command:

1. **Finds your backup**: Locates the `.zshrc.pre-zprof` file created during initialization
2. **Shows a preview**: Displays exactly what will be restored and what will be moved
3. **Requires confirmation**: Prompts you to confirm before making any changes
4. **Restores safely**:
   - Creates a safety backup of your current `.zshrc` (`.zshrc.pre-rollback`)
   - Restores your original `.zshrc` from backup
   - Moves framework directories back to your home directory (if applicable)
   - Preserves `~/.zsh-profiles/` for reference
5. **Provides instructions**: Tells you how to activate the restored configuration

### Rollback Examples

**Basic rollback** (with confirmation):
```bash
zprof rollback
```

**Skip confirmation** (for scripts):
```bash
zprof rollback --yes
```

**Rollback from specific profile**:
```bash
zprof rollback --profile my-profile
```

### After Rollback

Once rollback is complete:

1. **Restart your shell** or run:
   ```bash
   source ~/.zshrc
   ```

2. **Verify your configuration** is working correctly

3. **Optional cleanup**: You can delete the `~/.zsh-profiles/` directory if you no longer need it:
   ```bash
   rm -rf ~/.zsh-profiles/
   ```

### Rollback Safety Features

- **Automatic safety backup**: Your current `.zshrc` is backed up before restoration
- **Non-destructive**: Original files are never deleted, only copied
- **Preserved backups**: All backups remain in `~/.zsh-profiles/cache/backups/`
- **Error handling**: If rollback fails, your configuration remains unchanged
- **Framework relocation**: Framework directories are moved back to their original locations

### Troubleshooting Rollback

**"No backup file found"**
- The backup is created during `zprof init` when migrating an existing configuration
- If you initialized with a clean system, there may be no backup to restore
- Check `~/.zsh-profiles/cache/backups/` for manual backup files

**"Backup file corrupted"**
- The backup file may have been manually modified or damaged
- You can manually restore from backups in `~/.zsh-profiles/cache/backups/`

**Framework not moved back**
- If the framework directory already exists in your home directory, rollback skips the move
- Manually remove or rename the existing directory and run rollback again

## Documentation

**Full documentation is available in the [`docs/`](docs/) directory:**

- **[User Guide](docs/user-guide/)** - Installation, usage, troubleshooting
- **[Developer Guide](docs/developer/)** - Contributing, architecture, adding frameworks
- **[Roadmap](docs/planning/roadmap.md)** - Future plans and releases

## Directory Structure

```
~/.zsh-profiles/
├── profiles/                 # Individual profile directories
│   ├── work/
│   │   ├── profile.toml     # Profile manifest (editable configuration)
│   │   ├── .zshrc           # Generated shell config (auto-regenerated)
│   │   ├── .zshenv          # Generated environment config
│   │   └── .oh-my-zsh/      # Framework installation (if applicable)
│   └── personal/
│       ├── profile.toml
│       ├── .zshrc
│       └── .zimfw/
├── shared/
│   └── .zsh_history         # Shared command history
├── cache/
│   ├── backups/             # Safety backups from deletions and rollbacks
│   │   ├── .zshrc.pre-zprof
│   │   └── deleted-profiles/
│   └── downloads/           # Downloaded frameworks and themes
└── config.toml              # Global configuration
```

### Profile TOML Manifest

Each profile contains a `profile.toml` file that serves as the source of truth for configuration:

```toml
[profile]
name = "work"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-11-01T12:00:00Z"
modified = "2025-11-01T12:00:00Z"

[plugins]
enabled = ["git", "docker", "kubectl", "aws"]

[env]
NODE_ENV = "development"
EDITOR = "vim"
```

Edit with `zprof edit <name>` for automatic validation and regeneration.

## Supported Frameworks

- **oh-my-zsh**: Popular zsh framework with extensive plugins
- **zimfw**: Fast, blazing fast zsh framework
- **prezto**: Configuration framework for zsh
- **zinit**: Flexible and fast plugin manager
- **zap**: Minimal zsh plugin manager

## Commands Reference

| Command | Description |
|---------|-------------|
| `zprof init` | Initialize zprof directory structure |
| `zprof create <name>` | Create a new profile with interactive wizard |
| `zprof list` | List all available profiles |
| `zprof current` | Display currently active profile |
| `zprof use <name>` | Switch to a different profile |
| `zprof delete <name>` | Delete a profile (with backup) |
| `zprof edit <name>` | Edit profile's TOML manifest with live validation |
| `zprof export <name>` | Export profile to portable `.zprof` archive |
| `zprof import <file.zprof>` | Import profile from local archive |
| `zprof import github:<user>/<repo>` | Import profile from GitHub repository |
| `zprof regenerate <name>` | Regenerate shell configs from TOML manifest |
| `zprof rollback` | Restore pre-zprof configuration |
| `zprof gui` | Launch the graphical user interface |

## Safety and Backups

zprof prioritizes data safety:

- **Non-destructive initialization**: Original configurations are backed up automatically
- **Profile deletion backups**: Deleted profiles are backed up to `~/.zsh-profiles/cache/backups/`
- **Rollback safety**: Creates safety backup before restoring original configuration
- **Framework migration**: Uses copy instead of move to preserve originals
- **Error recovery**: Failed operations are rolled back automatically

## Examples

### Scenario 1: Create Work and Personal Profiles

```bash
# Initialize zprof
zprof init

# Create work profile with oh-my-zsh
zprof create work
# Select oh-my-zsh, robbyrussell theme, git and docker plugins

# Create personal profile with zimfw
zprof create personal
# Select zimfw, pure theme, minimal plugins

# Switch to work
zprof use work

# Later, switch to personal
zprof use personal
```

### Scenario 2: Customize Profile with TOML Editing

```bash
# Create a profile
zprof create dev

# Edit the profile manifest
zprof edit dev

# In your editor, modify profile.toml:
# - Change theme to "powerlevel10k"
# - Add plugins: ["git", "docker", "kubectl", "terraform"]
# - Add environment variables

# Save and close - zprof automatically validates and regenerates configs

# Activate the updated profile
zprof use dev
```

### Scenario 3: Share Profile with Team

```bash
# Export your team's standardized work profile
zprof export team-work

# Share the work.zprof file with teammates
# They can import it:
zprof import team-work.zprof

# Or publish to GitHub and they can import directly:
zprof import github:mycompany/zsh-work-profile
```

### Scenario 4: Backup and Restore Profiles

```bash
# Export all your profiles for backup
zprof export work --output ~/backups/
zprof export personal --output ~/backups/
zprof export dev --output ~/backups/

# Later, restore on a new machine
zprof init
zprof import ~/backups/work.zprof
zprof import ~/backups/personal.zprof
zprof import ~/backups/dev.zprof
```

### Scenario 5: Import Community Profile from GitHub

```bash
# Try a popular community profile
zprof import github:username/awesome-zsh-config

# Review what was imported
zprof list
zprof current

# Customize it for your needs
zprof edit awesome-zsh-config

# Export your customized version
zprof export awesome-zsh-config
```

### Scenario 6: Rollback and Uninstall

```bash
# Restore original configuration
zprof rollback

# Restart shell
source ~/.zshrc

# Clean up (optional)
rm -rf ~/.zsh-profiles/
```

## Testing

zprof has comprehensive test coverage to ensure reliability:

```bash
# Run all tests
cargo test --all-features

# Run specific test suite
cargo test --test export_test

# Check code quality
cargo clippy --all-targets --all-features

# Build release binary
cargo build --release
```

**Test Coverage:**
- 204 automated tests (100% passing)
- Unit tests for all core modules
- Integration tests for workflows
- Framework detection tests for all 5 frameworks
- Error handling and edge case tests
- Performance validation tests

## Version

**Current Version:** v0.1.0

**Release Notes:**
- ✅ Core profile management (create, list, use, delete, rollback)
- ✅ Interactive TUI wizard for profile creation
- ✅ Support for 5 zsh frameworks (oh-my-zsh, zimfw, prezto, zinit, zap)
- ✅ TOML manifest configuration with live validation
- ✅ Profile export/import as portable archives
- ✅ GitHub repository import support
- ✅ Shell configuration regeneration
- ✅ Comprehensive safety and backup features

**Tested on:**
- macOS (Darwin 25.0.0)
- Zsh 5.9+

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### Development

#### CLI Development

```bash
# Clone repository
git clone https://github.com/yourusername/zprof.git
cd zprof

# Build CLI only
cargo build

# Run tests
cargo test

# Run specific command
cargo run -- list
```

#### GUI Development

The GUI application uses Tauri with a Svelte frontend.

**Prerequisites:**

- Rust 1.70+
- Node.js 18+
- Tauri prerequisites for your platform:
  - macOS: Xcode Command Line Tools
  - Linux: See [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

**Build and Run:**

```bash
# Install Tauri CLI
cargo install tauri-cli

# Install frontend dependencies
cd src-ui
npm install
cd ..

# Run in development mode (hot reload)
cargo tauri dev

# Build production bundle
cargo tauri build
```

The development server will launch the GUI application with hot reload enabled. The production build creates a platform-specific bundle:

- macOS: `.dmg` file in `src-tauri/target/release/bundle/dmg/`
- Linux: `.deb` and `.AppImage` in `src-tauri/target/release/bundle/`

**GUI Structure:**

- `src-tauri/`: Tauri Rust backend and IPC layer
- `src-ui/`: Svelte frontend application
- `src/`: Shared core business logic (used by both CLI and GUI)

## Troubleshooting

### Profile not switching properly

Make sure to restart your shell or run `source ~/.zshrc` after using `zprof use`.

### Editor not opening for `zprof edit`

Set your preferred editor:
```bash
export EDITOR=vim  # or nano, emacs, code, etc.
```

### Import from GitHub fails

- Verify repository exists and is accessible
- For private repos, configure git credentials
- Check that `profile.toml` exists in repository root

### Archive validation errors

Archives must contain:
- `metadata.json` with valid structure
- `profile.toml` with valid TOML syntax
- Supported framework specification

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Credits

Built with Rust 🦀

Developed using the BMad Modern Methodology (BMM) workflow system.
