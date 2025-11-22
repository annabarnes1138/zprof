# zprof Roadmap

Public-facing roadmap for zprof development.

## Released

### v0.1.1 (Current)

**Core Profile Management**
- ✅ Initialize zprof directory structure
- ✅ Create, list, switch, and delete profiles
- ✅ Support for 5 frameworks (oh-my-zsh, zimfw, prezto, zinit, zap)
- ✅ Interactive TUI wizard for profile creation
- ✅ TOML-based profile manifests
- ✅ Export/import profiles as portable archives
- ✅ GitHub repository imports
- ✅ Shared history and customizations
- ✅ Rollback to pre-zprof state

## In Planning

### v0.2.0 (Next Release)

**Focus**: Accessibility + Power User Features

**Smart UX**
- Prompt mode branching (standalone engines vs framework themes)
- Quick setup presets (Minimal, Performance, Fancy, Developer)
- Nerd Font auto-installation with terminal config instructions

**Complete Lifecycle Management**
- Comprehensive uninstall command with restoration options
- Enhanced init process (backup and clean root configs)
- Health check command (`zprof doctor`)

**Framework Expansion**
- Add 2-4 additional frameworks based on user demand
- Candidates: Antigen, Zplug, Zgenom, Antidote

**Technical**
- Upgrade to Rust 2024 edition
- Remove deprecated `rollback` command (replaced by `uninstall`)

## Future Versions

### v0.3.0

**Profile Management**
- Interactive profile editing TUI (add/remove plugins, change themes)
- Automatic terminal configuration for Nerd Fonts
- Profile templates and sharing marketplace

### v0.4.0

**Advanced Features**
- Plugin version management and pinning
- Startup time profiling and optimization suggestions
- Cloud sync for profiles (S3, GitHub Gists)

### v0.5.0

**Enterprise Features**
- Team profile sharing and standardization
- Policy enforcement (required plugins, approved frameworks)
- Audit logs for profile changes

## Feature Requests

Have an idea? [Open a discussion](https://github.com/annabarnes1138/zprof/discussions) or [request a feature](https://github.com/annabarnes1138/zprof/issues/new).

## Contributing

Want to help build these features? See our [Contributing Guide](../developer/contributing.md).
