# Shared Customizations Feature

## Overview

User environment customizations (like cargo, nvm, PATH modifications, etc.) are now extracted **once** during `zprof init` or `zprof create` and saved to a shared file that all profiles source.

## Architecture

### Shared File Location
```
~/.zsh-profiles/shared/custom.zsh
```

### How It Works

1. **During `zprof init`**:
   - Extracts user customizations from `~/.zshrc`
   - Saves to `~/.zsh-profiles/shared/custom.zsh`
   - File is created with helpful header comments

2. **During `zprof create <profile>`**:
   - Ensures `shared/custom.zsh` exists (creates if missing)
   - Generated `.zshrc` includes a source statement:
     ```bash
     # Source shared customizations (edit ~/.zsh-profiles/shared/custom.zsh)
     [ -f "$HOME/.zsh-profiles/shared/custom.zsh" ] && source "$HOME/.zsh-profiles/shared/custom.zsh"
     ```

3. **When shells load**:
   - Framework initializes first
   - Then `shared/custom.zsh` is sourced
   - Your environment is ready to go

## What Gets Extracted

The extraction intelligently identifies:
- Rust cargo environment (`source $HOME/.cargo/env`)
- Node Version Manager (NVM)
- Go Version Manager (GVM)
- Google Cloud SDK
- Docker/Rancher Desktop configurations
- Custom PATH modifications
- Sourced alias files
- Other `source` or `.` commands

Specifically **excludes** framework-specific lines:
- `export ZSH=`
- `ZSH_THEME=`
- `plugins=(`
- oh-my-zsh, zimfw, prezto, zinit initialization

## File Structure

```bash
# Shared Custom Configuration
# =============================
#
# This file is sourced by ALL profiles.
# Use this for environment setup that should be available everywhere:
#   - PATH modifications
#   - Language version managers (nvm, cargo, gvm, etc.)
#   - Tool configurations (docker, kubectl, etc.)
#   - Shared aliases
#
# For profile-specific configuration, edit the profile's .zshrc directly.
#

# Extracted from your original ~/.zshrc

. "$HOME/.local/bin/env"

source $HOME/.cargo/env

export NVM_DIR="/Users/username/.nvm"
[ -s "/opt/homebrew/opt/nvm/nvm.sh" ] && \. "/opt/homebrew/opt/nvm/nvm.sh"

# ... more customizations ...
```

## User Workflows

### Global Changes (Affect All Profiles)

Edit the shared file:
```bash
vim ~/.zsh-profiles/shared/custom.zsh
```

Or use your editor of choice. Changes apply to all profiles immediately on next shell load.

### Profile-Specific Changes

Edit the individual profile's `.zshrc`:
```bash
vim ~/.zsh-profiles/profiles/<profile-name>/.zshrc
```

Add your customizations at the end of the file. They will only affect that specific profile.

## Benefits

1. **Single Source of Truth**: Edit environment setup in one place
2. **No Duplication**: Customizations not copied into every profile
3. **Easy Updates**: Change once, affects all profiles
4. **Profile Isolation**: Can still override or add profile-specific config
5. **Maintainable**: Clear separation between framework config and user config

## Example Use Cases

### Adding a New Tool for All Profiles

```bash
echo 'export PATH="$HOME/.my-tool/bin:$PATH"' >> ~/.zsh-profiles/shared/custom.zsh
```

Next time you load any profile, the tool is available.

### Profile-Specific Alias

For a work profile only:
```bash
echo 'alias deploy="kubectl apply -f"' >> ~/.zsh-profiles/profiles/work/.zshrc
```

This alias only exists in the "work" profile.

## Migration from Old Approach

If you have old profiles created before this feature:

1. **Regenerate profiles**:
   ```bash
   zprof regenerate <profile-name>
   ```

2. Or **manually add** to existing `.zshrc`:
   ```bash
   echo '
   # Source shared customizations (edit ~/.zsh-profiles/shared/custom.zsh)
   [ -f "$HOME/.zsh-profiles/shared/custom.zsh" ] && source "$HOME/.zsh-profiles/shared/custom.zsh"
   ' >> ~/.zsh-profiles/profiles/<profile-name>/.zshrc
   ```

## Technical Details

### Idempotent Creation

`create_shared_customizations()` is safe to call multiple times:
- Only creates file if it doesn't exist
- Never overwrites existing `custom.zsh`
- Called during both `init` and `create` operations

### File Permissions

Default permissions: `0644` (readable by all, writable by user)

### Loading Order

1. `~/.zshenv` (sets ZDOTDIR)
2. `$ZDOTDIR/.zshenv` (sets HISTFILE)
3. `$ZDOTDIR/.zshrc`:
   - Framework initialization
   - **Shared customizations** (`custom.zsh`)
   - Profile-specific additions (if any)

This ensures customizations load after the framework is initialized, so they can override framework defaults if needed.
