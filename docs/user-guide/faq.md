# FAQ

Frequently asked questions about zprof.

## General

### What is zprof?

zprof is a command-line tool for managing multiple zsh configurations (profiles). It lets you switch between different shell setups instantly—perfect for separating work, personal, and experimental configurations.

### How is this different from just having multiple .zshrc files?

zprof provides:
- **Framework management**: Installs and manages oh-my-zsh, zimfw, etc.
- **Instant switching**: One command to change entire environment
- **Shared resources**: Shared history and customizations across profiles
- **Safe experimentation**: Try new setups without breaking your main config
- **Import/Export**: Share profiles as portable archives
- **Non-destructive**: Your original config remains untouched

### Is zprof safe? Will it break my shell?

Yes, zprof is safe:
- **Non-destructive**: Never modifies your original `~/.zshrc`
- **Automatic backups**: Creates backups before any changes
- **Easy rollback**: `zprof rollback` restores original config
- **Validation**: Checks manifests before generating configs
- **ZDOTDIR-based**: Uses zsh's built-in profile mechanism

### What frameworks does zprof support?

Currently: **oh-my-zsh**, **zimfw**, **prezto**, **zinit**, and **zap**.

More frameworks may be added in future versions based on user demand.

### Can I use zprof with my existing oh-my-zsh setup?

Yes! `zprof init` detects your existing framework and offers to import it as a profile. Your original files remain untouched.

---

## Installation & Setup

### What are the system requirements?

- **Shell**: zsh (any recent version)
- **OS**: macOS, Linux, WSL2
- **For installation**: Rust 1.70+ (if installing via cargo)
- **For framework features**: git

### Do I need to uninstall my current framework first?

No. zprof can import your existing setup during initialization.

### Can I use zprof without Rust installed?

Yes, download pre-built binaries from [GitHub Releases](https://github.com/annabarnes1138/zprof/releases).

---

## Using Profiles

### How many profiles can I create?

No limit. Create as many as you need.

### Do all profiles share the same history?

By default, yes. This is usually desirable—commands from one profile are available in others.

To use separate histories, edit `profile.toml`:
```toml
[env]
HISTFILE = "$ZDOTDIR/.zsh_history"
```

### Can I have the same framework in multiple profiles?

Yes. Each profile has its own framework installation:
```
profiles/work/.oh-my-zsh/      # Independent installation
profiles/personal/.oh-my-zsh/  # Independent installation
```

### How do I add custom aliases that work in all profiles?

Add them to shared customizations:
```bash
vim ~/.zsh-profiles/shared/custom.zsh
```

Example:
```bash
alias ll='ls -lah'
alias gs='git status'
```

### Can I version control my profiles?

Yes! Each profile's `profile.toml` is perfect for git:

```bash
cd ~/.zsh-profiles/profiles/work
git init
git add profile.toml
git commit -m "Initial work profile"
git remote add origin https://github.com/yourname/work-profile.git
git push
```

Others can then import via:
```bash
zprof import github:yourname/work-profile
```

---

## Switching & Performance

### Do I need to restart my terminal after switching profiles?

You need to start a new shell (`exec zsh` or open a new terminal tab). You don't need to restart the entire terminal application.

### How fast is profile switching?

Profile switching itself is instant (< 100ms). Shell startup time depends on your framework and plugins:
- **zimfw/zinit**: ~100-200ms
- **oh-my-zsh**: ~500ms-1s

### Does zprof slow down my shell?

No. zprof only runs when you execute `zprof` commands. Normal shell usage is unaffected.

### Can I switch profiles automatically based on directory?

Not directly, but you can create a shell hook:

```bash
# Add to ~/.zsh-profiles/shared/custom.zsh
autoload -U add-zsh-hook

switch_profile_by_dir() {
    if [[ $PWD == ~/work/* ]]; then
        [[ $(zprof current --quiet) != "work" ]] && zprof use work
    fi
}

add-zsh-hook chpwd switch_profile_by_dir
```

---

## Frameworks & Plugins

### Can I use Starship/Powerlevel10k with any framework?

Yes! Prompt engines work with all frameworks. zprof automatically:
- Disables the framework's theme system
- Installs the prompt engine
- Initializes it correctly

### How do I add a plugin that's not in the registry?

For zap (which uses GitHub URLs):
```toml
[plugins]
enabled = ["username/plugin-repo"]
```

For other frameworks, if a plugin isn't recognized, open an issue or add it manually to your `.zshrc` in `custom.zsh`.

### Can I mix plugins from different sources?

Yes. For example, oh-my-zsh plugins + external zsh-users plugins work together.

### Do I need Nerd Fonts?

Only if you use themes that require them (Powerlevel10k, some Starship configs). zprof will warn you if needed.

---

## Import/Export

### What's included in a `.zprof` archive?

- `profile.toml` manifest
- Metadata (export date, zprof version)
- Shell configs (for reference)

**Not included**: Framework binaries (too large). The manifest describes what to install.

### Can I import profiles from oh-my-zsh users who don't use zprof?

Not directly. They would need to:
1. Install zprof
2. Run `zprof init` (imports their setup)
3. Export the created profile
4. Share the `.zprof` file

Alternatively, you can manually create a `profile.toml` based on their `.zshrc`.

### Can I share profiles publicly?

Yes! Host on GitHub and others can import:
```bash
zprof import github:yourname/my-zsh-profile
```

Just make sure `profile.toml` is in the repository root.

---

## Troubleshooting

### Profile isn't activating after `zprof use`

Did you start a new shell?
```bash
exec zsh
```

### I see `ZDOTDIR` errors

Check if something else is also managing `ZDOTDIR`. zprof needs exclusive control of this variable.

### Shell startup is slow

Check startup time:
```bash
time zsh -i -c exit
```

Solutions:
- Reduce plugins
- Switch to faster framework (zimfw, zinit)
- Use async prompt (Starship, Pure)

See [Troubleshooting Guide](troubleshooting.md) for more.

### How do I completely uninstall zprof?

Use the `zprof uninstall` command for safe, complete removal:

```bash
# Interactive mode (recommended) - walks you through options
zprof uninstall

# Restore your original pre-zprof config automatically
zprof uninstall --restore original --yes

# Promote one of your profiles to become your permanent setup
zprof uninstall --restore promote

# Clean removal (removes everything, no restoration)
zprof uninstall --restore clean --yes
```

See the [Uninstalling Guide](uninstalling.md) for detailed instructions and all options.

### Can I remove zprof and go back to my old setup?

Yes! Use `zprof uninstall` with the "Restore original" option:

```bash
zprof uninstall --restore original
```

This restores your shell configuration from the pre-zprof backup that was created during `zprof init`. Your command history, configs, and everything else will be restored to exactly how they were before you installed zprof.

**Safety:** A complete backup of your zprof setup is created before making any changes, so you can recover if needed.

### What happens to my command history when I uninstall?

It depends on which restoration option you choose:

| Option | What Happens to History |
|--------|--------------------------|
| **Restore Original** | Your pre-zprof command history is restored from backup |
| **Promote Profile** | The selected profile's history becomes your new `.zsh_history` |
| **Clean Removal** | No history file is restored (but it's saved in the safety backup) |

**Example:** If you have 15,000 command history entries from before zprof, and you choose "Restore Original," those 15,000 entries will be restored to `~/.zsh_history`.

### Where are my backups stored?

Backups are stored in `~/.zsh-profiles/backups/`:

```
~/.zsh-profiles/backups/
├── pre-zprof/              # Your original config from before zprof init
│   ├── backup-manifest.toml
│   ├── .zshrc
│   ├── .zshenv
│   └── .zsh_history
└── final-snapshot-*.tar.gz  # Safety backup created during uninstall
```

- **Pre-zprof backup**: Created automatically during `zprof init`, contains your original shell config
- **Final snapshot**: Created automatically during `zprof uninstall`, contains your entire zprof setup

### What if something goes wrong during uninstall?

zprof creates a safety backup before making any changes. If something fails:

1. The safety backup is at `~/.zsh-profiles/backups/final-snapshot-*.tar.gz`
2. Extract it to recover your data:
   ```bash
   mkdir ~/recovery
   tar -xzf ~/.zsh-profiles/backups/final-snapshot-*.tar.gz -C ~/recovery
   ```
3. Copy files you need:
   ```bash
   cp ~/recovery/.zsh-profiles/backups/pre-zprof/.zshrc ~/
   ```

See the [Uninstalling Guide - Troubleshooting](uninstalling.md#troubleshooting) for detailed recovery instructions.

### Can I keep some profiles after uninstall?

The `zprof uninstall` command removes all profiles, but you have options:

**Option 1 - Export before uninstalling:**
```bash
# Export profiles you want to keep
zprof export work --output ~/work-profile.zprof
zprof export personal --output ~/personal-profile.zprof

# Uninstall
zprof uninstall

# Later, if you reinstall zprof:
zprof init
zprof import ~/work-profile.zprof
```

**Option 2 - Promote one profile:**
```bash
# Make one profile your permanent root config
zprof uninstall --restore promote
# Then select the profile you want to keep
```

### Will uninstalling break my active shell sessions?

Active shell sessions will continue working temporarily, but they may show errors when trying to access removed zprof files.

**Solution:** Restart all shell sessions after uninstalling:
```bash
exec zsh
```

Or just close and reopen your terminal.

**Tip:** The uninstall confirmation screen warns you about this.

---

## Advanced Usage

### Can I edit .zshrc directly instead of profile.toml?

Not recommended. Generated `.zshrc` files have a warning header. If you edit them, your changes will be overwritten on the next `zprof regenerate`.

**Instead**: Add customizations to `~/.zsh-profiles/shared/custom.zsh`.

### Can I use zprof with bash or fish?

No, zprof is zsh-specific. It relies on zsh's `ZDOTDIR` mechanism.

### Can I have different environment variables per profile?

Yes! Use the `[env]` section in `profile.toml`:

```toml
[env]
NODE_ENV = "production"
AWS_PROFILE = "work-account"
EDITOR = "vim"
```

### Can profiles have different PATH variables?

Yes, via the `[env]` section:
```toml
[env]
PATH = "/custom/path:$PATH"
```

Or add to `custom.zsh` with conditionals:
```bash
if [[ $ZDOTDIR == *"work"* ]]; then
    export PATH="/work/bin:$PATH"
fi
```

---

## Contributing & Development

### Can I add support for a new framework?

Yes! See the [Developer Guide](../developer/adding-frameworks.md) for a step-by-step guide.

### How can I contribute?

See [Contributing Guidelines](../developer/contributing.md).

### Where do I report bugs?

[GitHub Issues](https://github.com/annabarnes1138/zprof/issues)

### Where can I request features?

[GitHub Discussions](https://github.com/annabarnes1138/zprof/discussions) or open a feature request issue.

---

## More Help

- [Quick Start Guide](quick-start.md) - Get started in 5 minutes
- [Command Reference](commands.md) - Complete CLI documentation
- [Troubleshooting](troubleshooting.md) - Common issues and solutions
- [Understanding Profiles](profiles.md) - Deep dive into how profiles work

**Still have questions?** Ask in [GitHub Discussions](https://github.com/annabarnes1138/zprof/discussions)!
