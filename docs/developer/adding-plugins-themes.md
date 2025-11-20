# Plugin & Theme Contributor Guide

This guide explains how to add new plugins and themes to the zprof registry.

## Overview

Plugins and themes are managed through central registries with metadata-driven compatibility. Each plugin/theme specifies which framework managers it supports and whether it's recommended.

## File Locations

- **Plugin Registry**: [src/frameworks/plugin.rs](src/frameworks/plugin.rs)
- **Theme Registry**: [src/frameworks/theme.rs](src/frameworks/theme.rs)

## Adding a New Plugin

### Step 1: Add Plugin to Registry

Edit `src/frameworks/plugin.rs` and add a new entry to `PLUGIN_REGISTRY`:

```rust
Plugin {
    name: "your-plugin-name",
    description: "Brief description of what this plugin does",
    category: PluginCategory::Utility, // Or Git, Docker, Kubernetes, Language
    compatibility: PluginCompatibility {
        supported_managers: &[
            // Add entries for each framework that supports this plugin
            ManagerSupport {
                framework: FrameworkType::OhMyZsh,
                repo_url: None, // oh-my-zsh uses plugin names directly
                recommended: false, // Set to true if this is a commonly-used plugin
            },
            ManagerSupport {
                framework: FrameworkType::Zap,
                repo_url: Some("username/repo-name"), // Zap requires GitHub repo URL
                recommended: false,
            },
            // Add more frameworks as needed...
        ],
    },
},
```

### Step 2: Follow Recommendation Criteria

Mark a plugin as `recommended: true` if it meets these criteria:

- **GitHub stars > 10k** OR widely included in framework defaults
- Included in oh-my-zsh/prezto/zimfw default or popular plugin lists
- Referenced in "awesome-zsh-plugins" curated lists
- Known to be stable and actively maintained
- Commonly used based on community surveys and adoption

### Step 3: Handle Framework-Specific Requirements

#### For Zap
- **MUST** provide `repo_url: Some("owner/repo")`
- Use the GitHub repository path (e.g., `"zsh-users/zsh-autosuggestions"`)
- If no Zap-compatible plugin exists, omit Zap from `supported_managers`

#### For other frameworks (oh-my-zsh, zimfw, prezto, zinit)
- Set `repo_url: None` (they use plugin names directly)
- Ensure the plugin name matches what the framework expects

### Step 4: Run Property Tests

After adding your plugin, run the metadata validation tests:

```bash
cargo test --test plugin_metadata_test
```

These tests verify:
- ✅ Plugins only appear if they're supported
- ✅ Zap plugins have repo URLs
- ✅ Recommended plugins are also supported
- ✅ Framework support is symmetric

## Adding a New Theme

### Step 1: Add Theme to Registry

Edit `src/frameworks/theme.rs` and add a new entry to `THEME_REGISTRY`:

```rust
Theme {
    name: "your-theme-name",
    description: "Brief description of the theme",
    preview: "Visual description or example prompt",
    compatibility: ThemeCompatibility {
        supported_managers: &[
            ManagerSupport {
                framework: FrameworkType::OhMyZsh,
                repo_url: None,
                recommended: false,
            },
            ManagerSupport {
                framework: FrameworkType::Zap,
                repo_url: Some("username/theme-repo"),
                recommended: false,
            },
            // Add more frameworks...
        ],
    },
},
```

### Step 2: Follow Theme Recommendation Criteria

Mark a theme as `recommended: true` if it meets these criteria:

- **GitHub stars > 5k** OR widely used in the community
- Fast startup time and good performance
- Active maintenance and broad compatibility
- Good documentation and customization options
- Popular in framework defaults or community recommendations

### Step 3: Run Tests

```bash
cargo test --test plugin_metadata_test
```

The same property tests apply to themes.

## Example: Adding a Popular Plugin

Here's a complete example adding the `zsh-autocomplete` plugin:

```rust
Plugin {
    name: "zsh-autocomplete",
    description: "Real-time type-ahead completion",
    category: PluginCategory::Utility,
    compatibility: PluginCompatibility {
        supported_managers: &[
            ManagerSupport {
                framework: FrameworkType::OhMyZsh,
                repo_url: None,
                recommended: true, // Popular and widely-used
            },
            ManagerSupport {
                framework: FrameworkType::Zimfw,
                repo_url: None,
                recommended: true,
            },
            ManagerSupport {
                framework: FrameworkType::Zinit,
                repo_url: None,
                recommended: true,
            },
            ManagerSupport {
                framework: FrameworkType::Zap,
                repo_url: Some("marlonrichert/zsh-autocomplete"),
                recommended: true,
            },
        ],
    },
},
```

## Testing Your Changes

1. **Build the project:**
   ```bash
   cargo build
   ```

2. **Run property tests:**
   ```bash
   cargo test --test plugin_metadata_test
   ```

3. **Test in the TUI:**
   ```bash
   cargo run -- init test-profile
   # Navigate through plugin selection
   # Verify your plugin appears with correct "(recommended)" suffix
   ```

## Common Issues

### "Plugin doesn't show up in TUI"
- Check that `supports_framework()` returns true for the target framework
- Verify the plugin is in the registry array
- Ensure there are no compilation errors

### "Zap plugin fails to install"
- Verify `repo_url` is a valid GitHub repository path
- Test the repo URL manually: `https://github.com/<your-repo-url>`
- Ensure the URL doesn't include `https://github.com/` prefix

### "Property tests failing"
- **Error**: "Plugin supports Zap but has no repo URL"
  - **Fix**: Add `repo_url: Some("owner/repo")` for Zap
- **Error**: "Plugin is recommended but not supported"
  - **Fix**: Ensure the framework with `recommended: true` is actually in the `supported_managers` list

## Questions?

If you have questions about adding plugins or themes, please:
1. Check existing plugin/theme entries in the registries for examples
2. Review the property tests in `tests/plugin_metadata_test.rs`
3. Open an issue on GitHub with the `question` label
