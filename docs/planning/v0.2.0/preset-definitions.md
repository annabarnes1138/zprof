# Preset Definitions - Rationale and Design

This document explains the design decisions behind each of zprof's curated preset configurations.

## Overview

Presets provide opinionated, pre-configured profile setups for common use cases. Each preset is carefully designed to serve a specific target audience with a balance of features, performance, and usability.

## Design Principles

1. **Clear Differentiation**: Each preset serves a distinct user persona with minimal overlap
2. **Proven Components**: All frameworks, prompts, and plugins are well-tested and actively maintained
3. **Realistic Plugin Counts**: Plugin counts reflect practical usage patterns, not arbitrary numbers
4. **Framework Alignment**: Framework choice matches the preset's performance/feature goals

---

## Preset Catalog

### 1. Minimal ✨

**Target User**: Beginners who want simplicity

**Philosophy**: Fast startup, clean interface, essential features only. Perfect for new zsh users who want something better than the default shell without overwhelming complexity.

**Framework**: **Zap**
- Rationale: Lightest-weight framework with virtually no overhead
- Single-file installation, no complex bootstrapping
- Ideal for users new to zsh frameworks

**Prompt**: **Pure**
- Rationale: Clean, minimalist prompt that shows only essential info
- No complex configuration required
- Fast rendering, no async complications

**Plugins** (3):
1. `zsh-autosuggestions` - Essential productivity feature, shows command history suggestions
2. `zsh-syntax-highlighting` - Immediate feedback on command validity
3. `git` - Basic git aliases and functions for version control

**Shell Options**:
- `HIST_IGNORE_DUPS` - Cleaner history
- `AUTO_CD` - Navigate directories without `cd` command

**Environment Variables**: None (keep it simple)

**Why This Works**:
- New users get immediate value without complexity
- Sub-100ms startup time
- Clear visual feedback from syntax highlighting
- Just enough features to feel modern without being overwhelming

---

### 2. Performance 🚀

**Target User**: Users with slow shells

**Philosophy**: Maximum speed with aggressive optimization. For users who've been frustrated by slow shell startup or laggy prompts and prioritize performance above all else.

**Framework**: **Zinit**
- Rationale: Fastest framework available with advanced features
- Turbo mode for deferred plugin loading
- Async plugin loading capabilities
- Best for users who understand optimization tradeoffs

**Prompt**: **Starship**
- Rationale: Async rendering prevents prompt lag
- Highly optimized Rust implementation
- Smart caching of git status and other expensive operations

**Plugins** (5 optimized):
1. `git` - Core version control support
2. `zsh-autosuggestions` - Command history suggestions
3. `fast-syntax-highlighting` - Zinit-optimized syntax highlighting variant
4. `fzf` - Fuzzy finding for history/files (huge productivity boost)
5. `history-substring-search` - Better history navigation

**Shell Options**:
- `HIST_IGNORE_DUPS` - Reduce history bloat
- `HIST_FIND_NO_DUPS` - Faster history searches

**Why This Works**:
- Zinit's turbo mode defers non-critical plugin loading
- Fast-syntax-highlighting is specifically optimized for Zinit
- Starship's async rendering prevents any prompt lag
- Plugin count kept minimal for fastest possible startup
- Typical startup time: < 100ms

---

### 3. Fancy ✨

**Target User**: "Make my terminal Instagram-worthy"

**Philosophy**: Beautiful, feature-rich environment with lots of visual enhancements and convenience features. For users who want their terminal to look amazing and don't mind slightly slower startup.

**Framework**: **Oh-My-Zsh**
- Rationale: Largest plugin ecosystem, most mature framework
- Best documentation and community support
- Wide variety of built-in plugins
- Standard choice for feature-rich setups

**Prompt**: **Powerlevel10k**
- Rationale: Most visually customizable prompt available
- Beautiful out-of-the-box appearance
- Extensive icon support, segment customization
- Configuration wizard for easy setup

**Plugins** (12 feature-rich):
1. `git` - Enhanced git integration
2. `docker` - Docker command completion and aliases
3. `kubectl` - Kubernetes CLI completion
4. `node` - Node.js/npm utilities
5. `npm` - npm command completion
6. `zsh-autosuggestions` - Command suggestions
7. `zsh-syntax-highlighting` - Syntax highlighting
8. `colored-man-pages` - Beautiful colorized man pages
9. `web-search` - Quick web searches from terminal
10. `jsontools` - JSON formatting and manipulation
11. `extract` - Universal archive extraction
12. `command-not-found` - Suggests package installation for missing commands

**Shell Options**:
- `HIST_IGNORE_DUPS` - Clean history
- `HIST_IGNORE_SPACE` - Commands starting with space aren't saved
- `SHARE_HISTORY` - Share history across sessions
- `AUTO_CD` - Navigate without `cd`

**Why This Works**:
- Oh-My-Zsh provides robust plugin management for 12+ plugins
- Powerlevel10k makes the terminal visually stunning
- Mix of utility plugins (jsontools, extract) and dev tools (docker, kubectl, node)
- Accepts ~200-300ms startup time for maximum features
- Perfect for users who want to show off their terminal

---

### 4. Developer 👨‍💻

**Target User**: Professional devs who code daily

**Philosophy**: Balanced setup optimized for software development workflows. Fast enough for daily use, with essential dev tools pre-configured.

**Framework**: **Zimfw**
- Rationale: Fast framework with good plugin support
- Better performance than Oh-My-Zsh
- More features than minimal frameworks
- Sweet spot for professional development

**Prompt**: **Starship**
- Rationale: Shows relevant dev context (git branch, language versions)
- Async rendering keeps terminal responsive
- Smart detection of project context
- Highly customizable via TOML config

**Plugins** (8 dev-focused):
1. `git` - Essential for version control workflows
2. `docker` - Container management commands and completion
3. `kubectl` - Kubernetes orchestration
4. `fzf` - Fuzzy finding for files, history, processes
5. `ripgrep` - Fast code search integration
6. `node` - Node.js/npm support
7. `zsh-autosuggestions` - Command history suggestions
8. `zsh-syntax-highlighting` - Command validation feedback

**Shell Options**:
- `HIST_IGNORE_DUPS` - Clean history
- `HIST_IGNORE_SPACE` - Private commands
- `SHARE_HISTORY` - Sync across tmux/terminal sessions
- `AUTO_CD` - Fast directory navigation

**Why This Works**:
- Zimfw provides speed while supporting 8 plugins comfortably
- Plugin selection focuses on daily dev tasks (git, containers, search)
- Starship shows context-aware info (node version in node projects, etc.)
- fzf + ripgrep combo enables powerful code navigation
- Typical startup time: ~150ms (fast enough for frequent terminal use)
- No language-specific plugins beyond Node (users add their own)

---

## Comparison Matrix

| Preset | Framework | Prompt | Plugins | Startup | Target |
|--------|-----------|--------|---------|---------|--------|
| Minimal | Zap | Pure | 3 | < 100ms | Beginners |
| Performance | Zinit | Starship | 5 | < 100ms | Speed seekers |
| Fancy | Oh-My-Zsh | Powerlevel10k | 12 | ~250ms | Aesthetics |
| Developer | Zimfw | Starship | 8 | ~150ms | Professional devs |

## Framework Selection Rationale

### Why Zap for Minimal?
- Simplest installation (single command)
- Virtually zero overhead
- Perfect for users who've never used frameworks

### Why Zinit for Performance?
- Fastest framework with turbo mode
- Advanced async capabilities
- Best for users who want maximum optimization

### Why Oh-My-Zsh for Fancy?
- Largest plugin ecosystem (150+ built-in plugins)
- Best documentation and community
- Standard for feature-rich setups

### Why Zimfw for Developer?
- Good balance of speed and features
- Fast startup even with multiple plugins
- Professional-grade without the Oh-My-Zsh overhead

## Plugin Selection Guidelines

### Core Plugins (All Presets)
- `git` - Universal version control need
- `zsh-autosuggestions` - Massive productivity boost
- `zsh-syntax-highlighting` - Immediate command feedback

### Performance-Specific
- `fast-syntax-highlighting` - Zinit-optimized variant
- `fzf` - Fast fuzzy finding
- `history-substring-search` - Optimized history search

### Developer-Specific
- `docker` + `kubectl` - Container/orchestration tools
- `ripgrep` - Code search
- `node` - JavaScript development

### Fancy-Specific
- Utility plugins: `jsontools`, `extract`, `web-search`
- Enhancement plugins: `colored-man-pages`, `command-not-found`
- Additional dev tools: `npm`, `docker`, `kubectl`

## Future Considerations

### Potential Additions (v0.3.0+)
- **Data Engineer** preset: Python, Spark, SQL tools
- **Security** preset: Security-focused tools and hardening
- **Minimalist** preset: Even lighter than Minimal (no framework)

### Customization
- Story 2.6+ will allow users to preview and customize presets before installation
- Users can clone presets and modify them
- Advanced users can create and share custom presets

## Maintenance Notes

### Plugin Version Stability
- All plugins use stable, well-maintained versions
- No experimental or abandoned plugins
- Prefer plugins with 1000+ GitHub stars

### Framework Compatibility
- All plugins tested with their respective frameworks
- No framework-specific hacks or workarounds needed
- Clean, standard configurations

### Update Strategy
- Review preset definitions quarterly
- Update plugin lists based on ecosystem changes
- Monitor framework development for optimization opportunities

---

**Last Updated**: 2025-11-22
**Version**: v0.2.0
**Author**: BMad Development Team
