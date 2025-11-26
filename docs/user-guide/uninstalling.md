# Uninstalling zprof

This guide walks you through safely removing zprof from your system with different restoration options.

## Overview

The `zprof uninstall` command provides a complete, safe removal process with three flexible options for handling your shell configuration:

1. **Restore Original** - Go back to your pre-zprof setup
2. **Promote Profile** - Keep one of your zprof profiles as your permanent config
3. **Clean Removal** - Remove everything and start fresh

All options include a safety backup of your entire setup before making any changes.

---

## Quick Start

For most users, the interactive mode is the easiest:

```bash
zprof uninstall
```

This launches an interactive guide that walks you through the process step-by-step.

---

## Restoration Options

### Option 1: Restore Original (Pre-zprof Backup)

**Best for:** Users who want to return to exactly how their shell was configured before installing zprof.

**What happens:**
- Your original shell configuration is restored from the pre-zprof backup created during `zprof init`
- All files return to their original locations: `.zshrc`, `.zshenv`, `.zsh_history`, etc.
- Your command history is restored to its pre-zprof state
- All zprof files and profiles are removed

**Requirements:**
- Must have run `zprof init` (which creates the pre-zprof backup automatically)
- Backup directory exists at `~/.zsh-profiles/backups/pre-zprof/`

**Example:**

```bash
# Interactive
zprof uninstall
# Then select "Restore original (pre-zprof backup)"

# Non-interactive
zprof uninstall --restore original --yes
```

**What you'll see:**

```
📦 Creating safety backup...
✓ Safety backup created: final-snapshot-20251124-143000.tar.gz (2.15 MB)
  Location: /Users/you/.zsh-profiles/backups/final-snapshot-20251124-143000.tar.gz

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Uninstall Summary

Restoration:
  • Restore pre-zprof backup (November 20, 2025)
  • 4 files will be restored to HOME
  • History: 15,234 entries

Cleanup:
  • Remove 3 profiles (work, personal, minimal)
  • Remove ~/.zsh-profiles/ (2.3 MB)
  • Remove zprof shell integration

Safety:
  • Final backup: final-snapshot-20251124-143000.tar.gz (2.15 MB)

Continue? [y/N]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔄 Restoring pre-zprof configuration...
  ✓ Restored .zshrc
  ✓ Restored .zshenv
  ✓ Restored .zprofile
  ✓ Restored .zsh_history

🗑️  Removing zprof files...
  ✓ Removed ~/.zsh-profiles/profiles/
  ✓ Removed ~/.zsh-profiles/shared/
  ✓ Removed ~/.zsh-profiles/config.toml
  ✓ Removed zprof-generated ~/.zshenv

✅ zprof uninstalled successfully

  Your shell configuration has been restored to its pre-zprof state.

  Safety backup available (2.15 MB):
  /Users/you/.zsh-profiles/backups/final-snapshot-20251124-143000.tar.gz
  You can extract this backup if you need to recover any data.

  Restart your shell to complete the uninstall:
  exec zsh
```

---

### Option 2: Promote Profile

**Best for:** Users who found a profile they love and want to make it their permanent shell configuration.

**What happens:**
- The selected profile's configuration files are copied to your HOME directory
- The profile becomes your new root shell configuration
- Profile history is moved to `~/.zsh_history`
- All zprof files and other profiles are removed
- You keep the promoted profile's setup permanently

**Requirements:**
- At least one profile exists in `~/.zsh-profiles/profiles/`

**Example:**

```bash
# Interactive - you'll choose the profile from a menu
zprof uninstall
# Then select "Promote profile to root"
# Then choose which profile to promote

# Semi-interactive - specify promote, then choose profile from menu
zprof uninstall --restore promote
```

**What you'll see:**

```
┌─────────────────────────────────────────┐
│ Which profile would you like to promote?│
│                                         │
│  > work                                 │
│    personal                             │
│    minimal                              │
└─────────────────────────────────────────┘

📦 Creating safety backup...
✓ Safety backup created: final-snapshot-20251124-143000.tar.gz (2.15 MB)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Uninstall Summary

Restoration:
  • Promote profile 'work' to root configuration
  • 5 files will be copied to HOME
  • History: 8,420 entries

Cleanup:
  • Remove 3 profiles (work, personal, minimal)
  • Remove ~/.zsh-profiles/ (2.3 MB)

Safety:
  • Final backup: final-snapshot-20251124-143000.tar.gz (2.15 MB)

Continue? [y/N]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔄 Promoting profile 'work' to root configuration...
  ✓ Copied .zshrc
  ✓ Copied .zshenv
  ✓ Copied .zprofile
  ✓ Copied .zsh_history

🗑️  Removing zprof files...
  ✓ Removed ~/.zsh-profiles/

✅ zprof uninstalled successfully

  Profile 'work' has been promoted to your root shell configuration.

  Restart your shell to complete the uninstall:
  exec zsh
```

**Important Notes:**
- The promoted profile's framework (oh-my-zsh, zimfw, etc.) remains installed in the profile directory, which is then removed
- If you want to keep the framework, you'll need to reinstall it manually or extract it from the safety backup
- The generated `.zshrc` from the profile is copied to your HOME - this includes all the framework/plugin configurations

---

### Option 3: Clean Removal

**Best for:** Users who want to completely remove zprof and configure their shell manually or use a different tool.

**What happens:**
- All zprof files and directories are removed
- No configuration is restored to HOME
- Your HOME directory shell configs are left empty (or in whatever state they were)
- You'll need to manually configure your shell after uninstall

**Example:**

```bash
# Interactive
zprof uninstall
# Then select "Clean removal (no restoration)"

# Non-interactive
zprof uninstall --restore clean --yes
```

**What you'll see:**

```
📦 Creating safety backup...
✓ Safety backup created: final-snapshot-20251124-143000.tar.gz (2.15 MB)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Uninstall Summary

Restoration:
  • No restoration (clean removal)

Cleanup:
  • Remove 3 profiles (work, personal, minimal)
  • Remove ~/.zsh-profiles/ (2.3 MB)
  • Remove zprof shell integration

Safety:
  • Final backup: final-snapshot-20251124-143000.tar.gz (2.15 MB)

Continue? [y/N]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🗑️  Removing zprof files...
  ✓ Removed ~/.zsh-profiles/

✅ zprof uninstalled successfully

  All zprof files have been removed.
  You can now configure your shell manually or install a different tool.

  Safety backup available (2.15 MB):
  /Users/you/.zsh-profiles/backups/final-snapshot-20251124-143000.tar.gz

  Restart your shell to complete the uninstall:
  exec zsh
```

**After clean removal, you'll need to:**
1. Create a new `.zshrc` file in your HOME directory
2. Configure your shell from scratch or install a framework manually
3. Set up your prompt, plugins, aliases, etc.

---

## Command-Line Options

### Skip Confirmation (`--yes`)

Use `-y` or `--yes` to skip all confirmation prompts:

```bash
zprof uninstall --restore original --yes
```

**Useful for:**
- Automation scripts
- CI/CD pipelines
- When you're absolutely sure of what you want

### Skip Safety Backup (`--no-backup`)

⚠️  **Not recommended!** Skips creating the safety backup:

```bash
zprof uninstall --restore original --no-backup
```

**Only use this if:**
- You've already backed up your data externally
- Disk space is critically low
- You're certain you won't need to recover anything

### Keep Backups Directory (`--keep-backups`)

Preserves the `~/.zsh-profiles/backups/` directory when removing zprof:

```bash
zprof uninstall --keep-backups
```

**What gets kept:**
- Pre-zprof backup
- Final safety snapshot
- Any other backups in the directory

**What still gets removed:**
- Profiles
- Shared files
- Config files
- Everything else in `~/.zsh-profiles/` except `backups/`

---

## Safety Backup

### What is the Safety Backup?

Before making any changes, `zprof uninstall` creates a complete archive of your entire `~/.zsh-profiles/` directory as a `.tar.gz` file.

**Location:** `~/.zsh-profiles/backups/final-snapshot-<timestamp>.tar.gz`

**Contains:**
- All profiles with their configurations and frameworks
- Shared history and customizations
- All backups (including pre-zprof backup)
- Global configuration
- Everything in `~/.zsh-profiles/`

### Extracting the Safety Backup

If something goes wrong or you need to recover data:

```bash
# 1. Find your backup
ls -lh ~/.zsh-profiles/backups/final-snapshot-*.tar.gz

# 2. Create a recovery directory
mkdir ~/zprof-recovery

# 3. Extract the backup
tar -xzf ~/.zsh-profiles/backups/final-snapshot-20251124-143000.tar.gz -C ~/zprof-recovery

# 4. Browse the extracted files
ls -la ~/zprof-recovery/.zsh-profiles/

# 5. Copy files you need
cp ~/zprof-recovery/.zsh-profiles/profiles/work/.zshrc ~/
cp ~/zprof-recovery/.zsh-profiles/backups/pre-zprof/.zsh_history ~/
```

### Manual Recovery Examples

**Recover a specific profile's config:**
```bash
tar -xzf ~/.zsh-profiles/backups/final-snapshot-20251124-143000.tar.gz \
  --strip-components=3 \
  -C ~/ \
  .zsh-profiles/profiles/work/.zshrc
```

**Recover your history file:**
```bash
tar -xzf ~/.zsh-profiles/backups/final-snapshot-20251124-143000.tar.gz \
  --strip-components=2 \
  -C ~/ \
  .zsh-profiles/shared/.zsh_history
```

**Recover pre-zprof backup:**
```bash
mkdir ~/recovered-backup
tar -xzf ~/.zsh-profiles/backups/final-snapshot-20251124-143000.tar.gz \
  -C ~/recovered-backup \
  .zsh-profiles/backups/pre-zprof/
```

---

## What Happens to My Data?

### Command History

| Restoration Option | What Happens to History |
|--------------------|-------------------------|
| **Restore Original** | Your pre-zprof history is restored from backup |
| **Promote Profile** | The selected profile's history becomes your new `.zsh_history` |
| **Clean Removal** | No history file in HOME (but preserved in safety backup) |

### Profiles

All profiles are removed in all scenarios. The safety backup contains complete copies of all profiles.

### Backups

| Directory | Default Behavior | With `--keep-backups` |
|-----------|------------------|------------------------|
| `backups/pre-zprof/` | Removed | Kept |
| `backups/final-snapshot-*.tar.gz` | Removed | Kept |
| All other backups | Removed | Kept |

**Note:** The final safety backup created during uninstall is never automatically deleted, even without `--keep-backups`, because it's created before the cleanup phase.

### Frameworks (oh-my-zsh, zimfw, etc.)

Framework installations inside profiles are removed with the profiles. If you promoted a profile, you'll need to manually reinstall the framework if you want to continue using it.

---

## Troubleshooting

### Error: "Pre-zprof backup not found"

**Cause:** The backup wasn't created during `zprof init`, or the backup directory was manually deleted.

**Solution:** Choose a different restoration option:
- Use "Promote Profile" to keep one of your zprof profiles
- Use "Clean Removal" and manually configure your shell

**Recovery:** If you need your original config, check if you have any other backups (`.zshrc.bak`, version control, etc.)

### Error: "Cannot proceed with uninstall due to validation failures"

**Cause:** Precondition checks failed (permissions, missing directories, etc.)

**Solution:** Read the error messages carefully and resolve the issues:
- Check file permissions on `~/.zsh-profiles/`
- Ensure you have write access to your HOME directory
- Verify zprof is actually installed

### Uninstall Fails Midway

**Cause:** Unexpected error during restoration or cleanup.

**What to do:**
1. **Don't panic** - your data is safe in the safety backup
2. Check the error message for specific issues
3. Try running the uninstall again (it's designed to be safe to retry)
4. If retry fails, manually extract the safety backup and recover your files

**Manual cleanup if needed:**
```bash
# Extract safety backup
mkdir ~/recovery
tar -xzf ~/.zsh-profiles/backups/final-snapshot-*.tar.gz -C ~/recovery

# Manually restore what you need
cp ~/recovery/.zsh-profiles/backups/pre-zprof/.zshrc ~/

# Manually remove zprof directory
rm -rf ~/.zsh-profiles/
```

### Active Shell Sessions Show Errors After Uninstall

**Cause:** Existing shell sessions still reference the removed `~/.zsh-profiles/` directory.

**Solution:** Restart all shell sessions:
```bash
# In each open terminal/tab
exec zsh
```

Or just close and reopen your terminal application.

### Want to Reinstall zprof Later

No problem! Your pre-zprof backup and safety snapshots remain unless you manually delete them.

To reinstall:
```bash
zprof init
```

This will create a fresh installation. You can then import profiles from your old safety backup if needed.

---

## FAQ

### Can I undo an uninstall?

Not automatically, but you can manually restore from the safety backup. The safety backup contains everything, so you can recreate any configuration.

### What if I want to keep some profiles but not others?

The uninstall command removes all profiles. If you want to keep some:

1. Before uninstalling, export the profiles you want to keep:
   ```bash
   zprof export work --output ~/work-profile.zprof
   zprof export personal --output ~/personal-profile.zprof
   ```

2. Uninstall zprof

3. If you want to use zprof again later, reinstall and import:
   ```bash
   zprof init
   zprof import ~/work-profile.zprof
   ```

### Will uninstalling break my active shell sessions?

Active shells will continue working until you close them, but they may show errors when trying to access removed zprof files. Restart your shell sessions with `exec zsh` after uninstalling.

### Where can I find my safety backup later?

Look in `~/.zsh-profiles/backups/final-snapshot-*.tar.gz`. The file includes a timestamp in its name.

If you used `--keep-backups`, this directory persists. Otherwise, the final safety backup file will be there until you manually delete it.

### Can I delete the safety backup?

Yes, once you're confident you don't need anything from it:

```bash
rm ~/.zsh-profiles/backups/final-snapshot-*.tar.gz
```

**Recommended:** Wait at least a few days after uninstalling to make sure everything is working as expected.

### How much disk space does the safety backup use?

Typically 2-10 MB, depending on:
- Number of profiles
- Size of history files
- Framework installations (oh-my-zsh is ~20MB, zimfw is ~2MB)
- Number of backups

---

## Complete Examples

### Scenario 1: Trying zprof, Didn't Like It

You installed zprof, created a couple profiles, but decide it's not for you.

```bash
# Uninstall and restore your original setup
zprof uninstall --restore original --yes

# Restart shell
exec zsh

# Optionally, remove the safety backup later
rm ~/.zsh-profiles/backups/final-snapshot-*.tar.gz
```

---

### Scenario 2: Found Your Perfect Profile

You experimented with several profiles and found "work" is perfect. You want to make it permanent.

```bash
# Promote the work profile
zprof uninstall --restore promote
# Select "work" from the menu

# Restart shell
exec zsh

# Your "work" profile's config is now your root config
# You can manually reinstall the framework if needed
```

---

### Scenario 3: Switching to a Different Tool

You're switching from zprof to a different shell manager.

```bash
# Export any profiles you might want to reference later
zprof export work --output ~/backups/work-profile.zprof
zprof export personal --output ~/backups/personal-profile.zprof

# Clean removal
zprof uninstall --restore clean --yes

# Install and configure your new tool
# ...
```

---

### Scenario 4: Automation/Scripting

Scripting an uninstall for automation purposes.

```bash
#!/bin/bash
# Unattended uninstall script

# Restore original config without prompts
zprof uninstall --restore original --yes --keep-backups

# Verify uninstall
if [ ! -d ~/.zsh-profiles/profiles ]; then
    echo "Uninstall successful"
else
    echo "Uninstall failed"
    exit 1
fi
```

---

## See Also

- [Command Reference](commands.md) - Complete `zprof uninstall` documentation
- [FAQ](faq.md) - Common questions about zprof
- [Installation](installation.md) - Reinstalling zprof
