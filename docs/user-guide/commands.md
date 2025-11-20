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
zprof create work
zprof create personal
zprof create experimental
```

**Interactive Wizard:**
1. Choose framework (oh-my-zsh, zimfw, prezto, zinit, zap)
2. Select plugins (multi-select browser)
3. Pick theme or prompt engine
4. Review and confirm

**What it creates:**
- Profile directory: `~/.zsh-profiles/profiles/<name>/`
- Manifest: `profile.toml`
- Shell configs: `.zshrc`, `.zshenv`
- Framework installation (if selected)

**Options:**
- `--framework <name>` - Pre-select framework (skips wizard step)
- `--theme <name>` - Pre-select theme
- `--plugins <list>` - Comma-separated plugin list

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
