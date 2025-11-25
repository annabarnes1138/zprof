# Presets Guide

Quick setup presets provide curated, battle-tested zsh configurations that work great out of the box. Choose a preset based on your needs and expertise level.

## What Are Presets?

Presets are pre-configured combinations of:
- **Framework** - The plugin manager (oh-my-zsh, zimfw, zinit, zap)
- **Prompt** - Your command-line appearance (Pure, Starship, Powerlevel10k)
- **Plugins** - Essential tools and enhancements
- **Shell Options** - Quality-of-life improvements

Instead of making dozens of decisions, pick one preset and start using zsh immediately.

## Available Presets

### ✨ Minimal

**Best for:** Beginners who want simplicity

A lightweight, fast setup with just the essentials. Perfect for learning zsh or if you prefer a clean, minimal environment.

**Configuration:**
- **Framework:** Zap
- **Prompt:** Pure
- **Plugins (3):**
  - `zsh-autosuggestions` - Command suggestions from history
  - `zsh-syntax-highlighting` - Color-coded command validation
  - `git` - Git status and shortcuts

**Characteristics:**
- **Startup time:** < 50ms (blazing fast)
- **Memory footprint:** Very light
- **Complexity:** Low
- **Customization:** Easy to understand and modify

**Why choose Minimal?**
- You're new to zsh and want to keep things simple
- You value speed and don't need many features
- You prefer to add plugins one-by-one as needed
- You want a clean slate to build on

**Example:**
```bash
zprof create simple-shell --preset minimal
```

---

### ⚙️ Performance

**Best for:** Users with slow shell startup times

Optimized for blazing-fast startup with async loading and turbo mode. Uses modern, performance-focused tools.

**Configuration:**
- **Framework:** Zinit
- **Prompt:** Starship
- **Plugins (5):**
  - `git` - Git integration
  - `zsh-autosuggestions` - Smart suggestions
  - `fast-syntax-highlighting` - Zinit-optimized syntax highlighting
  - `fzf` - Fuzzy file finder
  - `history-substring-search` - Better history navigation

**Characteristics:**
- **Startup time:** < 100ms (optimized with turbo mode)
- **Memory footprint:** Light
- **Complexity:** Medium
- **Customization:** Advanced features available

**Why choose Performance?**
- Your current shell takes too long to start
- You work on remote servers or older hardware
- You want modern async features (like Starship)
- You value efficiency and speed

**Example:**
```bash
zprof create fast-shell --preset performance
```

---

### 🎨 Fancy

**Best for:** Users who want a feature-rich, beautiful terminal

The complete oh-my-zsh experience with a stunning prompt and comprehensive plugin collection. Make your terminal Instagram-worthy!

**Configuration:**
- **Framework:** Oh-My-Zsh
- **Prompt:** Powerlevel10k
- **Plugins (12):**
  - `git` - Advanced git integration
  - `docker` - Docker completions and aliases
  - `kubectl` - Kubernetes shortcuts
  - `node` - Node.js helpers and completions
  - `npm` - npm command completions
  - `zsh-autosuggestions` - Command suggestions
  - `zsh-syntax-highlighting` - Syntax validation
  - `colored-man-pages` - Beautiful man pages
  - `web-search` - Search the web from terminal
  - `jsontools` - JSON formatting and validation
  - `extract` - Universal archive extractor
  - `command-not-found` - Suggest packages for missing commands

**Characteristics:**
- **Startup time:** ~200-300ms (feature-rich)
- **Memory footprint:** Medium
- **Complexity:** Medium-High
- **Customization:** Extensive options via oh-my-zsh ecosystem

**Requirements:**
- **Nerd Font** required for Powerlevel10k icons and symbols

**Why choose Fancy?**
- You want the full-featured terminal experience
- Visual appearance and icons matter to you
- You use many different tools (Docker, Kubernetes, etc.)
- You're willing to trade some speed for features

**Example:**
```bash
zprof create beautiful-shell --preset fancy
```

---

### 👨‍💻 Developer

**Best for:** Professional developers who code daily

Tailored for software engineers with tools for version control, containers, cloud platforms, and development workflows.

**Configuration:**
- **Framework:** Zimfw
- **Prompt:** Starship
- **Plugins (8):**
  - `git` - Git shortcuts and info
  - `docker` - Docker completions
  - `kubectl` - Kubernetes helpers
  - `fzf` - Fuzzy file/history search
  - `ripgrep` - Fast recursive grep tool
  - `node` - Node.js helpers and completions
  - `zsh-autosuggestions` - Smart suggestions
  - `zsh-syntax-highlighting` - Syntax validation

**Characteristics:**
- **Startup time:** ~150ms (balanced)
- **Memory footprint:** Medium
- **Complexity:** Medium
- **Customization:** Highly modular and extensible

**Why choose Developer?**
- You work with multiple programming languages
- You use Docker, Kubernetes, or cloud tools regularly
- You manage different project environments
- You want professional-grade tooling without bloat

**Example:**
```bash
zprof create dev-shell --preset developer
```

---

## Comparison Table

| Preset        | Framework  | Prompt         | Plugin Count | Startup Time | Nerd Font | Best For                      |
|---------------|------------|----------------|--------------|--------------|-----------|-------------------------------|
| **Minimal**   | Zap        | Pure           | 3            | < 50ms       | No        | Beginners, minimalists        |
| **Performance** | Zinit    | Starship       | 5            | < 100ms      | Yes       | Speed-focused users           |
| **Fancy**     | Oh-My-Zsh  | Powerlevel10k  | 12           | ~250ms       | Yes       | Feature lovers, aesthetics    |
| **Developer** | Zimfw      | Starship       | 8            | ~150ms       | Yes       | Professional developers       |

## Using Presets

### Interactive Selection

Run `zprof create` and choose **Quick Setup**:

```bash
zprof create myprofile
```

You'll see a menu with all preset options. Navigate with arrow keys and press Enter to select.

### Command-Line Flag

Skip the TUI and create directly:

```bash
zprof create myprofile --preset minimal
zprof create work --preset developer
zprof create fancy-term --preset fancy
```

### Customizing After Creation

Presets are starting points, not locked configurations. You can always customize:

```bash
# Edit your profile's configuration
zprof edit myprofile
```

Then modify the `profile.toml` to add/remove plugins, change themes, etc.

## Frequently Asked Questions

### Can I switch presets after creating a profile?

Not directly, but you can:
1. Create a new profile with the desired preset
2. Delete your old profile
3. Or manually edit `profile.toml` to match the preset's configuration

### Do presets include Nerd Font installation?

No. If your chosen preset requires a Nerd Font (Performance, Fancy, Developer), you need to install one separately. See [Nerd Fonts](https://www.nerdfonts.com/) for installation.

### Can I create my own preset?

Currently, presets are built into zprof. Custom presets are planned for a future release. For now, use Custom Setup mode or edit a profile after creation.

### What if I want to combine features from multiple presets?

Start with the preset closest to your needs, then use `zprof edit` to add/remove plugins from other presets.

### Are presets updated?

Yes! When you update zprof, preset configurations may improve. Use `zprof regenerate` to apply updates to existing profiles.

## What's Next?

After creating a profile with a preset:

1. **Activate it:** `zprof use myprofile && exec zsh`
2. **Explore installed plugins:** `zprof current`
3. **Customize if needed:** `zprof edit myprofile`
4. **Learn shortcuts:** Check framework-specific documentation

## Related Documentation

- [Quick Start](quick-start.md) - Get started with zprof
- [Command Reference](commands.md) - All available commands
- [Supported Frameworks](frameworks.md) - Framework details
- [Understanding Profiles](profiles.md) - How profiles work
