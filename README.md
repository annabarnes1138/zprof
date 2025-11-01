# zprof

Manage multiple zsh profiles with ease.

## Overview

`zprof` is a command-line tool that helps you manage multiple zsh shell configurations (profiles) with different frameworks, themes, and plugins. Switch between profiles instantly based on your workflow needs.

## Features

- 🚀 **Multiple Profiles**: Create and manage separate zsh configurations for different contexts (work, personal, experimental)
- 🎨 **Framework Support**: Works with oh-my-zsh, zimfw, prezto, zinit, and zap
- 🔄 **Quick Switching**: Switch between profiles with a single command
- 🛡️ **Non-Destructive**: All operations create automatic backups
- 🔙 **Easy Rollback**: Restore your original configuration anytime

## Installation

```bash
cargo install zprof
```

Or build from source:

```bash
git clone https://github.com/yourusername/zprof.git
cd zprof
cargo build --release
```

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

## Directory Structure

```
~/.zsh-profiles/
├── profiles/                 # Individual profile directories
│   ├── work/
│   │   ├── .zshrc           # Profile-specific shell config
│   │   ├── profile.toml     # Profile metadata
│   │   ├── .zshrc.pre-zprof # Backup of original .zshrc (created during init)
│   │   └── .oh-my-zsh/      # Framework installation (if applicable)
│   └── personal/
├── shared/
│   └── .zsh_history         # Shared command history
├── cache/
│   ├── backups/             # Safety backups from deletions and rollbacks
│   └── downloads/           # Downloaded frameworks and themes
└── config.toml              # Global configuration
```

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
| `zprof create <name>` | Create a new profile |
| `zprof list` | List all available profiles |
| `zprof current` | Display currently active profile |
| `zprof use <name>` | Switch to a different profile |
| `zprof delete <name>` | Delete a profile (with backup) |
| `zprof rollback` | Restore pre-zprof configuration |

## Safety and Backups

zprof prioritizes data safety:

- **Non-destructive initialization**: Original configurations are backed up automatically
- **Profile deletion backups**: Deleted profiles are backed up to `~/.zsh-profiles/cache/backups/`
- **Rollback safety**: Creates safety backup before restoring original configuration
- **Framework migration**: Uses copy instead of move to preserve originals
- **Error recovery**: Failed operations are rolled back automatically

## Examples

### Scenario: Create Work and Personal Profiles

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

### Scenario: Rollback and Uninstall

```bash
# Restore original configuration
zprof rollback

# Restart shell
source ~/.zshrc

# Clean up (optional)
rm -rf ~/.zsh-profiles/
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Credits

Built with Rust 🦀
