# Command Reference

Complete documentation for all zprof commands.

## Global Options

All commands support these global flags:

- `-h, --help` - Display help information
- `-V, --version` - Display version information

## Commands

### `zprof init`

Initialize the zprof directory structure.

```bash
zprof init
```

**What it does:**
- Creates `~/.zsh-profiles/` directory structure
- Detects existing zsh frameworks (oh-my-zsh, zimfw, etc.)
- Offers to import your current configuration as a profile
- Sets up shared history and customizations
- Creates initial configuration file

**Behavior:**
- If `~/.zsh-profiles/` already exists, skips initialization (safe)
- Non-destructive: never modifies your original `~/.zshrc`
- Backs up `~/.zshenv` before modifying it

---

### `zprof create <NAME>`

Create a new profile.

```bash
zprof create <profile-name>
```

**Examples:**
```bash
# Interactive mode with setup choice (Quick or Custom)
zprof create work

# Create from a preset (skip wizard entirely)
zprof create work --preset minimal
zprof create dev-env --preset developer
zprof create fancy-term --preset fancy

# Custom setup with pre-selected options
zprof create personal --framework zimfw
```

**Setup Modes:**

When run without `--preset`, the wizard first asks you to choose:
- **Quick Setup:** Select from curated presets (Minimal, Performance, Fancy, Developer)
- **Custom Setup:** Full wizard with granular control

**Interactive Wizard (Custom Setup):**
1. Choose framework (oh-my-zsh, zimfw, prezto, zinit, zap)
2. Select prompt mode (standalone engine or framework theme)
3. Pick plugins (multi-select browser)
4. Review and confirm

**What it creates:**
- Profile directory: `~/.zsh-profiles/profiles/<name>/`
- Manifest: `profile.toml`
- Shell configs: `.zshrc`, `.zshenv`
- Framework installation (if selected)

**Options:**
- `--preset <name>` - Create from preset, skipping all wizard steps
  - Available presets: `minimal`, `performance`, `fancy`, `developer`
  - See [Presets Guide](presets.md) for detailed preset information
- `--framework <name>` - Pre-select framework (skips wizard step in Custom mode)
- `--theme <name>` - Pre-select theme (Custom mode only)
- `--plugins <list>` - Comma-separated plugin list (Custom mode only)

**Preset Details:**

| Preset | Framework | Prompt | Plugins | Best For |
|--------|-----------|--------|---------|----------|
| `minimal` | Zap | Pure | 3 | Beginners, speed |
| `performance` | Zinit | Starship | 5 | Fast startup, async features |
| `fancy` | Oh-My-Zsh | Powerlevel10k | 12 | Feature-rich, beautiful |
| `developer` | Zimfw | Starship | 8 | Professional dev work |

---

### `zprof list`

List all available profiles.

```bash
zprof list
```

**Output:**
```
Available profiles:

  default (active)
    Framework: oh-my-zsh
    Theme: robbyrussell
    Plugins: 8

  work
    Framework: zimfw
    Theme: starship
    Plugins: 12

  experimental
    Framework: zap
    Theme: pure
    Plugins: 3
```

---

### `zprof current`

Display information about the active profile.

```bash
zprof current
```

**Output:**
```
Active profile: work

Framework: zimfw
Theme: starship
Plugins: git, zsh-autosuggestions, zsh-syntax-highlighting, fzf, docker, kubectl
Location: /Users/you/.zsh-profiles/profiles/work
Created: 2025-11-01 10:00:00
Modified: 2025-11-15 14:30:00
```

---

### `zprof use <NAME>`

Switch to a different profile.

```bash
zprof use <profile-name>
```

**Examples:**
```bash
zprof use work
zprof use personal
```

**What it does:**
- Updates `~/.zsh-profiles/config.toml` to set active profile
- Modifies `~/.zshenv` to point `ZDOTDIR` to the new profile
- Preserves shared history across all profiles

**After switching:**
```bash
# Start a new shell to activate the profile
exec zsh
```

---

### `zprof delete <NAME>`

Delete a profile.

```bash
zprof delete <profile-name>
```

**Examples:**
```bash
zprof delete old-profile
```

**Safety features:**
- Cannot delete the currently active profile (switch first)
- Creates automatic backup before deletion
- Prompts for confirmation
- Backup location: `~/.zsh-profiles/cache/backups/`

**Options:**
- `-y, --yes` - Skip confirmation prompt
- `--no-backup` - Skip creating backup (dangerous!)

---

### `zprof edit <NAME>`

Edit a profile's manifest with live validation.

```bash
zprof edit <profile-name>
```

**Examples:**
```bash
zprof edit work
```

**What it does:**
- Opens `profile.toml` in your preferred editor ($VISUAL, $EDITOR, or vim)
- On save: validates TOML syntax and schema
- If valid: regenerates `.zshrc` and `.zshenv`
- If invalid: shows helpful error messages

**Editor precedence:**
1. `$VISUAL` environment variable
2. `$EDITOR` environment variable
3. `vim` (fallback)

---

### `zprof export <NAME>`

Export a profile to a portable archive.

```bash
zprof export <profile-name> [OPTIONS]
```

**Examples:**
```bash
# Export to current directory
zprof export work

# Export to specific location
zprof export work --output ~/backups/work.zprof

# Overwrite existing archive
zprof export work --force
```

**Options:**
- `-o, --output <path>` - Output file path (default: `./<name>.zprof`)
- `-f, --force` - Overwrite existing archive without prompting

**Archive contents:**
- `profile.toml` - Profile manifest
- `metadata.json` - Export info (date, zprof version)
- Shell configs (for reference only)

**Note:** Framework binaries are excluded to keep archives small.

---

### `zprof import <SOURCE>`

Import a profile from an archive or GitHub.

```bash
zprof import <source> [OPTIONS]
```

**From local archive:**
```bash
zprof import work.zprof
zprof import ~/downloads/experimental.zprof
```

**From GitHub:**
```bash
zprof import github:username/repo
zprof import --github username/dotfiles-zsh
```

**Options:**
- `-n, --name <name>` - Import with a different profile name
- `-f, --force` - Overwrite existing profile without prompting
- `-g, --github <repo>` - Import from GitHub repository

**What it does:**
1. Downloads/extracts the source
2. Validates manifest
3. Checks for name conflicts (prompts if exists)
4. Installs framework and plugins
5. Generates shell configurations

---

### `zprof regenerate <NAME>`

Regenerate shell configurations from manifest.

```bash
zprof regenerate <profile-name>
```

**Examples:**
```bash
zprof regenerate work
```

**When to use:**
- After manually editing `profile.toml`
- After updating zprof to a new version
- To fix corrupted shell configs

**What it does:**
- Reads `profile.toml`
- Regenerates `.zshrc` and `.zshenv` from scratch
- Validates generated configs with `zsh -n`

---

### `zprof rollback`

Restore your original pre-zprof shell configuration.

```bash
zprof rollback [OPTIONS]
```

**Options:**
- `-y, --yes` - Skip confirmation prompt
- `-p, --profile <name>` - Rollback from specific profile's backup

**What it does:**
1. Locates `.zshrc.pre-zprof` backup (created during init)
2. Shows preview of what will be restored
3. Prompts for confirmation
4. Creates safety backup (`.zshrc.pre-rollback`)
5. Restores original `.zshrc`
6. Moves framework directories back to `~/`
7. Preserves `~/.zsh-profiles/` for reference

**After rollback:**
```bash
# Activate restored configuration
source ~/.zshrc

# Optionally remove zprof directory
rm -rf ~/.zsh-profiles/
```

---

### `zprof uninstall`

Safely remove zprof with flexible restoration options.

```bash
zprof uninstall [OPTIONS]
```

**What it does:**
- Creates a safety backup of your entire setup before making changes
- Offers three restoration options:
  1. Restore your original pre-zprof configuration
  2. Promote one of your profiles to become your root config
  3. Clean removal without restoration
- Removes all zprof files and directories
- Provides detailed confirmation summary before proceeding

**Restoration Options:**

| Option | Description | When to Use |
|--------|-------------|-------------|
| **Restore Original** | Restore pre-zprof backup | Want to go back to exactly how things were before zprof |
| **Promote Profile** | Make a profile your new root config | Want to keep one of your zprof profiles as your permanent setup |
| **Clean Removal** | Remove everything, no restoration | Starting fresh or switching to a different tool |

**Options:**
- `-y, --yes` - Skip confirmation prompts (for automation/scripts)
- `--restore <option>` - Specify restoration choice directly (values: `original`, `promote`, `clean`)
- `--no-backup` - Skip creating safety backup (not recommended!)
- `--keep-backups` - Preserve the backups directory when removing zprof

**Interactive Mode (default):**

When run without arguments, `zprof uninstall` guides you through an interactive process:

1. **Select Restoration Option** - Choose how to handle your shell config
2. **Choose Profile** (if promoting) - Select which profile to promote
3. **Review Summary** - See detailed preview of what will happen
4. **Confirm** - Approve or cancel the operation

**Examples:**

```bash
# Interactive mode with full TUI guidance
zprof uninstall

# Restore original config automatically (non-interactive)
zprof uninstall --restore original --yes

# Promote a specific profile (still asks which profile interactively)
zprof uninstall --restore promote

# Clean removal without restoration or confirmation
zprof uninstall --restore clean --yes

# Keep the backups directory for reference
zprof uninstall --keep-backups
```

**Safety Features:**
- **Safety Backup**: Creates a timestamped tarball (`.tar.gz`) of your entire `~/.zsh-profiles/` directory before making any changes
- **Validation**: Checks for potential issues before starting
- **Rollback**: If restoration fails, you can manually recover from the safety backup
- **Confirmation Summary**: Shows exactly what will be restored/removed before proceeding

**What Gets Removed:**
- `~/.zsh-profiles/` directory (all profiles, configs, shared files)
- Zprof-generated `~/.zshenv` file
- ZDOTDIR references

**What's Preserved:**
- Safety backup tarball (unless you later manually delete it)
- Backups directory (if `--keep-backups` is used)
- Your restored configuration (original or promoted profile)

**After Uninstall:**

You must restart your shell for changes to take effect:
```bash
exec zsh
```

**Recovery:**

If something goes wrong, you can manually extract the safety backup:

```bash
# Find your safety backup
ls -lh ~/.zsh-profiles/backups/final-snapshot-*.tar.gz

# Extract to a temporary location
mkdir ~/zprof-recovery
tar -xzf ~/.zsh-profiles/backups/final-snapshot-20251124-143000.tar.gz -C ~/zprof-recovery

# Manually copy files as needed
cp ~/zprof-recovery/.zsh-profiles/profiles/work/.zshrc ~/
```

**See Also:**
- [Uninstalling Guide](uninstalling.md) - Step-by-step uninstall scenarios
- [FAQ](faq.md) - Common questions about uninstalling

---

## Environment Variables

zprof respects these environment variables:

- `$VISUAL` - Preferred editor for `zprof edit` (priority 1)
- `$EDITOR` - Fallback editor for `zprof edit` (priority 2)
- `$ZDOTDIR` - Managed by zprof (don't modify manually!)
- `$HISTFILE` - Set to shared history location

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Invalid arguments
- `3` - File/directory not found
- `4` - Validation error
- `5` - Operation cancelled by user

## Examples

### Complete workflow: Create, customize, export

```bash
# Create profile
zprof create work

# Switch to it
zprof use work
exec zsh

# Customize by editing manifest
zprof edit work

# Export to share
zprof export work --output ~/work-profile.zprof
```

### Import and test a new framework

```bash
# Create experimental profile
zprof create test-zinit --framework zinit

# Try it out
zprof use test-zinit
exec zsh

# If you like it, keep it. Otherwise delete
zprof delete test-zinit
```

### Backup and restore

```bash
# Export current profile as backup
zprof export personal --output ~/backups/personal-$(date +%Y%m%d).zprof

# Later, if needed...
zprof import ~/backups/personal-20251120.zprof
```
