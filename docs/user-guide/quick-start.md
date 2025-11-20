# Quick Start

Get up and running with zprof in 5 minutes.

## Step 1: Initialize zprof

```bash
zprof init
```

This creates `~/.zsh-profiles/` and detects your existing zsh setup. If you already have oh-my-zsh or another framework, zprof will offer to import it:

```
Existing oh-my-zsh detected with 8 plugins and 'robbyrussell' theme.
Import as a profile? [Y/n]: y
Profile name [default]:
```

Press Enter to accept the defaults. Your existing setup is now a zprof profile!

## Step 2: Create a New Profile

Let's create a second profile for experimentation:

```bash
zprof create experimental
```

This launches an interactive wizard:

1. **Choose a framework**: oh-my-zsh, zimfw, prezto, zinit, or zap
2. **Select plugins**: Browse and select from hundreds of available plugins
3. **Pick a theme**: Choose a prompt theme or use a standalone engine like Starship
4. **Confirm**: Review your selections

The wizard looks like this:

```
┌─────────────────────────────────────────────────┐
│ Which framework would you like to use?          │
├─────────────────────────────────────────────────┤
│ > oh-my-zsh     Extensive plugin ecosystem      │
│   zimfw         Fast and minimal                │
│   prezto        Feature-rich configuration      │
│   zinit         Advanced plugin management      │
│   zap           Minimal and beginner-friendly   │
└─────────────────────────────────────────────────┘
```

## Step 3: List Your Profiles

```bash
zprof list
```

Output:
```
Available profiles:

  default (active)
    Framework: oh-my-zsh
    Theme: robbyrussell
    Plugins: 8

  experimental
    Framework: zimfw
    Theme: starship
    Plugins: 5
```

## Step 4: Switch Between Profiles

```bash
zprof use experimental
```

Output:
```
✓ Switched to profile: experimental
→ Start a new shell session: exec zsh
```

Now start a new shell to see your new profile:

```bash
exec zsh
```

## Step 5: View Current Profile

```bash
zprof current
```

Output:
```
Active profile: experimental

Framework: zimfw
Theme: starship
Plugins: git, zsh-autosuggestions, zsh-syntax-highlighting, fzf, docker
Location: /Users/you/.zsh-profiles/profiles/experimental
```

## Common Workflows

### Try a Different Theme

Edit your profile's configuration:

```bash
zprof edit experimental
```

This opens `profile.toml` in your editor. Change the theme:

```toml
[profile]
name = "experimental"
framework = "zimfw"
theme = "pure"  # Changed from "starship"
```

Save and close. zprof automatically regenerates your `.zshrc`.

### Add More Plugins

Edit the same file and add to the plugins array:

```toml
[plugins]
enabled = ["git", "zsh-autosuggestions", "zsh-syntax-highlighting", "fzf", "docker", "kubectl"]
```

### Export and Share

Share your profile as a portable archive:

```bash
zprof export experimental
```

Creates `experimental.zprof` that others can import:

```bash
zprof import experimental.zprof
```

### Import from GitHub

Import a profile from a GitHub repository:

```bash
zprof import --github username/dotfiles
```

## Next Steps

- [Command Reference](commands.md) - Learn all available commands
- [Understanding Profiles](profiles.md) - Deep dive into how profiles work
- [Supported Frameworks](frameworks.md) - Details on each framework
- [Troubleshooting](troubleshooting.md) - Common issues and solutions

## Tips

- **Shared History**: All profiles share the same command history by default
- **Shared Customizations**: Add custom aliases/functions to `~/.zsh-profiles/shared/custom.zsh`
- **Backups**: zprof automatically backs up files before making changes
- **Non-Destructive**: Your original `~/.zshrc` is never modified (zprof uses `ZDOTDIR`)
