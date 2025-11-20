# Supported Frameworks

zprof supports five popular zsh frameworks, each with different philosophies and strengths.

## Quick Comparison

| Framework | Speed | Plugins | Complexity | Best For |
|-----------|-------|---------|------------|----------|
| **oh-my-zsh** | ⭐⭐ | ⭐⭐⭐⭐⭐ | Low | Beginners, feature-rich setups |
| **zimfw** | ⭐⭐⭐⭐ | ⭐⭐⭐ | Low | Fast startup, balanced features |
| **prezto** | ⭐⭐⭐ | ⭐⭐⭐⭐ | Medium | Curated, polished configurations |
| **zinit** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | High | Power users, maximum performance |
| **zap** | ⭐⭐⭐⭐ | ⭐⭐ | Low | Minimalists, simple setups |

## oh-my-zsh

**Philosophy**: "Give me everything, I'll choose what I want."

**Official Site**: https://ohmyz.sh/

### Strengths
- **Massive ecosystem**: 200+ plugins, 100+ themes
- **Community**: Largest user base, extensive documentation
- **Beginner-friendly**: Simple to use, lots of examples
- **Comprehensive**: Plugins for almost every tool imaginable

### Weaknesses
- **Slower startup**: ~500ms-1s depending on plugins
- **Bloat**: Ships with everything, even if you don't need it
- **Dated patterns**: Older codebase, not optimized for modern zsh

### When to Choose
- You're new to zsh customization
- You want maximum plugin availability
- Startup time isn't a concern
- You value community support

### Example Profile

```toml
[profile]
framework = "oh-my-zsh"
theme = "robbyrussell"

[plugins]
enabled = [
    "git",
    "docker",
    "kubectl",
    "zsh-autosuggestions",
    "zsh-syntax-highlighting"
]
```

---

## zimfw

**Philosophy**: "Fast, modular, minimal."

**Official Site**: https://zimfw.sh/

### Strengths
- **Speed**: One of the fastest frameworks (~200ms startup)
- **Modular**: Clean module system, only load what you need
- **Well-organized**: Sensible defaults, good documentation
- **Active development**: Modern features, maintained regularly

### Weaknesses
- **Smaller ecosystem**: Fewer plugins than oh-my-zsh
- **Less known**: Smaller community
- **Manual configuration**: Requires editing `.zimrc`

### When to Choose
- You want speed without complexity
- You like modular, minimal setups
- You're comfortable with configuration files
- Startup time matters

### Example Profile

```toml
[profile]
framework = "zimfw"
theme = "starship"  # Works great with external prompts

[plugins]
enabled = [
    "git",
    "zsh-autosuggestions",
    "zsh-syntax-highlighting",
    "fzf"
]
```

**Note**: zimfw uses `.zimrc` for module configuration. zprof generates this automatically from your manifest.

---

## prezto

**Philosophy**: "Curated, elegant, feature-rich."

**Official Site**: https://github.com/sorin-ionescu/prezto

### Strengths
- **Polished**: Well-tested, refined modules
- **Comprehensive**: Good balance of features vs simplicity
- **Elegant**: Consistent APIs, thoughtful design
- **Batteries included**: Useful defaults out of the box

### Weaknesses
- **Moderate speed**: Slower than zimfw/zinit
- **Rigid structure**: Less flexible than oh-my-zsh
- **Smaller ecosystem**: Fewer third-party plugins
- **Slower updates**: Less frequent releases

### When to Choose
- You want a curated, refined experience
- You prefer quality over quantity
- You like sensible defaults
- You don't need bleeding-edge features

### Example Profile

```toml
[profile]
framework = "prezto"
theme = "sorin"

[plugins]
enabled = [
    "git",
    "syntax-highlighting",
    "autosuggestions",
    "history-substring-search"
]
```

**Note**: prezto calls plugins "modules" and uses `.zpreztorc` for configuration.

---

## zinit

**Philosophy**: "Maximum performance, maximum control."

**Official Site**: https://github.com/zdharma-continuum/zinit

### Strengths
- **Fastest**: Turbo mode, lazy loading, async (~50ms startup possible)
- **Powerful**: Advanced features (snippets, patching, ice modifiers)
- **Flexible**: Load anything from anywhere (GitHub, OMZ plugins, local)
- **Active**: Modern codebase, actively maintained

### Weaknesses
- **Complex**: Steep learning curve
- **Verbose config**: More configuration required
- **Overwhelming**: Too many options for beginners
- **Less hand-holding**: You need to know what you want

### When to Choose
- You're an advanced user
- Maximum performance is critical
- You want fine-grained control
- You enjoy tinkering

### Example Profile

```toml
[profile]
framework = "zinit"
theme = "starship"

[plugins]
enabled = [
    "git",
    "zsh-autosuggestions",
    "fast-syntax-highlighting",  # zinit's optimized fork
    "fzf",
    "history-substring-search"
]
```

**Note**: zinit has a unique syntax. zprof translates your plugin list to zinit's format automatically.

---

## zap

**Philosophy**: "Minimal, fast, beginner-friendly."

**Official Site**: https://github.com/zap-zsh/zap

### Strengths
- **Simple**: Easiest to understand
- **Fast**: Very quick startup (~100ms)
- **Clean syntax**: Readable configuration
- **Lightweight**: Minimal overhead

### Weaknesses
- **Limited ecosystem**: Fewer official plugins
- **Less features**: Minimal plugin manager, that's it
- **Newer**: Smaller community, less proven
- **Manual URLs**: Need to know GitHub URLs for plugins

### When to Choose
- You want simplicity above all
- You're comfortable finding plugins yourself
- You like minimal, transparent tools
- You don't need oh-my-zsh's ecosystem

### Example Profile

```toml
[profile]
framework = "zap"
theme = "pure"

[plugins]
enabled = [
    "zsh-autosuggestions",
    "zsh-syntax-highlighting",
    "git"
]
```

**Note**: zap uses `plug` commands with GitHub URLs. zprof's plugin registry provides the correct URLs automatically.

---

## Choosing a Framework

### Decision Tree

**Start here**: Are you new to zsh customization?
- **Yes** → **oh-my-zsh** (most beginner-friendly)
- **No** → Continue...

**Do you prioritize speed?**
- **Yes** → **zinit** (if advanced) or **zimfw** (if intermediate)
- **No** → Continue...

**Do you want maximum plugins/themes?**
- **Yes** → **oh-my-zsh**
- **No** → Continue...

**Do you prefer minimal, simple tools?**
- **Yes** → **zap**
- **No** → **prezto** (refined, curated)

### Recommendations by Use Case

**First-time zsh users**: oh-my-zsh
- Gentlest learning curve
- Most documentation
- Largest community

**Performance enthusiasts**: zinit
- Fastest possible startup
- Advanced optimization
- Worth the complexity

**Balanced users**: zimfw
- Good speed
- Sufficient features
- Clean design

**Aesthetic perfectionists**: prezto
- Polished modules
- Consistent experience
- Elegant defaults

**Minimalists**: zap
- Bare bones
- Transparent
- Just enough

## Mixing Frameworks with Prompt Engines

All frameworks work with standalone prompt engines:

**Compatible Prompt Engines:**
- **Starship**: Cross-shell, async, Rust-powered
- **Powerlevel10k**: Zsh-only, highly customizable
- **Pure**: Minimal, async, fast
- **Spaceship**: Feature-rich, pretty
- **Oh-My-Posh**: Cross-shell, many themes

**Example**: zimfw + Starship
```toml
[profile]
framework = "zimfw"
theme = "starship"  # External prompt engine
```

zprof automatically:
- Disables framework's built-in theme system
- Installs and initializes the prompt engine
- Configures proper loading order

## Switching Frameworks

Want to try a different framework? Create a new profile:

```bash
# Your current setup (oh-my-zsh)
zprof current
# → oh-my-zsh with robbyrussell theme

# Try zimfw in a new profile
zprof create test-zimfw --framework zimfw

# Switch and test
zprof use test-zimfw
exec zsh

# Like it? Keep it. Don't like it? Delete it.
zprof delete test-zimfw
zprof use default  # Back to original
```

**No risk**: Your original profile remains untouched!

## Performance Tips

### oh-my-zsh
- Use fewer plugins (< 10)
- Avoid heavy themes
- Consider switching to zimfw or zinit

### zimfw
- Only enable needed modules
- Use async themes (Starship, Pure)
- Keep plugin count low

### prezto
- Disable unused modules in `.zpreztorc`
- Use lightweight themes

### zinit
- Enable turbo mode for all plugins
- Use `wait` ice modifier
- Lazy-load heavy plugins

### zap
- Already minimal, just avoid heavy plugins

### Measure Startup Time

```bash
time zsh -i -c exit
```

**Good targets:**
- < 100ms: Excellent
- 100-300ms: Good
- 300-500ms: Acceptable
- > 500ms: Consider optimization

## Framework-Specific Notes

### oh-my-zsh Updates

oh-my-zsh auto-updates by default. To disable:

```bash
# Add to ~/.zsh-profiles/shared/custom.zsh
DISABLE_AUTO_UPDATE="true"
```

### zimfw Installation

zimfw bootstrap happens automatically when you create a profile. To update:

```bash
zimfw update
```

### prezto Modules

prezto modules in `profile.toml` map to `zstyle` directives in `.zpreztorc`. zprof handles this translation.

### zinit Turbo Mode

zprof enables turbo mode automatically for compatible plugins to maximize performance.

### zap Plugin URLs

The zprof plugin registry maintains correct GitHub URLs for popular plugins. For custom plugins:

```toml
[plugins]
enabled = ["username/custom-plugin"]  # GitHub short form
```

## Getting Help

Each framework has its own documentation:

- **oh-my-zsh**: https://github.com/ohmyzsh/ohmyzsh/wiki
- **zimfw**: https://zimfw.sh/docs/
- **prezto**: https://github.com/sorin-ionescu/prezto
- **zinit**: https://zdharma-continuum.github.io/zinit/wiki/
- **zap**: https://github.com/zap-zsh/zap

For zprof-specific framework issues, see [Troubleshooting](troubleshooting.md).
