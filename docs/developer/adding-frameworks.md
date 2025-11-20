# Adding Framework Support

Step-by-step guide to adding support for a new zsh framework to zprof.

## Overview

Adding a framework involves:
1. Creating a framework module
2. Implementing the `Framework` trait
3. Adding detection logic
4. Updating the generator
5. Adding to registries
6. Writing tests

**Estimated time**: 4-6 hours for a simple framework

## Step 1: Create Framework Module

Create `src/frameworks/myframework.rs`:

```rust
use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::frameworks::{Framework, FrameworkInfo, FrameworkType, Plugin, Theme};

pub struct MyFramework;

impl Framework for MyFramework {
    fn name(&self) -> &str {
        "myframework"
    }

    fn detect() -> Option<FrameworkInfo> {
        // Detection logic (Step 2)
        None
    }

    fn install(profile_path: &Path) -> Result<()> {
        // Installation logic (Step 3)
        Ok(())
    }

    fn get_plugins() -> Vec<Plugin> {
        // Plugin list (Step 4)
        vec![]
    }

    fn get_themes() -> Vec<Theme> {
        // Theme list (Step 5)
        vec![]
    }
}
```

## Step 2: Implement Detection

Detection finds existing installations in the user's home directory:

```rust
fn detect() -> Option<FrameworkInfo> {
    use std::fs;

    let home = dirs::home_dir()?;

    // Check for framework directory
    let framework_dir = home.join(".myframework");
    if !framework_dir.exists() {
        return None;
    }

    // Check for config file
    let config_path = home.join(".myframeworkrc");
    if !config_path.exists() {
        return None;
    }

    // Parse config for plugins and theme
    let content = fs::read_to_string(&config_path).ok()?;
    let plugins = parse_plugins(&content);
    let theme = parse_theme(&content).unwrap_or_else(|| "default".to_string());

    Some(FrameworkInfo {
        framework_type: FrameworkType::MyFramework,
        plugins,
        theme,
        config_path,
        install_path: framework_dir,
    })
}

fn parse_plugins(content: &str) -> Vec<String> {
    // Framework-specific parsing logic
    // Example: Look for lines like "plugin git" or "load plugin1 plugin2"
    vec![]
}

fn parse_theme(content: &str) -> Option<String> {
    // Framework-specific parsing logic
    None
}
```

## Step 3: Implement Installation

```rust
fn install(profile_path: &Path) -> Result<()> {
    use std::process::Command;

    let framework_dir = profile_path.join(".myframework");

    // Clone repository
    let repo_url = "https://github.com/myframework/myframework.git";

    Command::new("git")
        .args(&["clone", "--depth=1", repo_url, framework_dir.to_str().unwrap()])
        .output()
        .context("Failed to clone myframework repository")?;

    // Run any post-installation scripts if needed
    // ...

    Ok(())
}
```

## Step 4: Update Framework Registry

Add to `src/frameworks/mod.rs`:

```rust
pub enum FrameworkType {
    OhMyZsh,
    Zimfw,
    Prezto,
    Zinit,
    Zap,
    MyFramework,  // Add here
}

impl FrameworkType {
    pub fn name(&self) -> &str {
        match self {
            // ... existing
            Self::MyFramework => "myframework",
        }
    }
}
```

Add to detector:

```rust
// In src/frameworks/detector.rs
use crate::frameworks::myframework::MyFramework;

pub fn detect_existing_framework() -> Option<FrameworkInfo> {
    // Try each framework
    if let Some(info) = MyFramework::detect() {
        return Some(info);
    }

    // ... existing frameworks
    None
}
```

## Step 5: Update Generator

Add config generation in `src/shell/generator.rs`:

```rust
fn generate_framework_config(manifest: &Manifest, output: &mut String) -> Result<()> {
    match manifest.profile.framework.as_str() {
        // ... existing frameworks
        "myframework" => generate_myframework_config(manifest, output)?,
        _ => bail!("Unsupported framework: {}", manifest.profile.framework),
    }
    Ok(())
}

fn generate_myframework_config(manifest: &Manifest, output: &mut String) -> Result<()> {
    // Set framework home
    writeln!(output, "export MYFRAMEWORK_HOME=\"$ZDOTDIR/.myframework\"")?;

    // Source framework initialization
    writeln!(output, "source $MYFRAMEWORK_HOME/myframework.sh")?;

    // Load plugins
    for plugin in &manifest.plugins.enabled {
        writeln!(output, "myframework_plugin {}", plugin)?;
    }

    // Set theme
    if let Some(theme) = &manifest.profile.theme {
        writeln!(output, "myframework_theme {}", theme)?;
    }

    Ok(())
}
```

## Step 6: Update Plugin Registry

Add framework-compatible plugins in `src/frameworks/plugin.rs`:

```rust
Plugin {
    name: "git",
    description: "Git integration",
    compatibility: PluginCompatibility {
        supported_managers: &[
            // ... existing
            ManagerSupport {
                framework: FrameworkType::MyFramework,
                repo_url: None,  // Built-in to framework
                recommended: true,
            },
        ],
        dependencies: &[],
    },
},
```

## Step 7: Add Tests

Create `tests/myframework_test.rs`:

```rust
use serial_test::serial;
use tempfile::TempDir;

#[test]
#[serial]
fn test_myframework_detection() {
    // Create fake home directory
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Create framework files
    std::fs::create_dir(temp.path().join(".myframework")).unwrap();
    std::fs::write(
        temp.path().join(".myframeworkrc"),
        "plugin git\ntheme default\n"
    ).unwrap();

    // Test detection
    let detected = detect_existing_framework();
    assert!(detected.is_some());

    let info = detected.unwrap();
    assert_eq!(info.framework_type, FrameworkType::MyFramework);
    assert!(info.plugins.contains(&"git".to_string()));
}

#[test]
fn test_myframework_config_generation() {
    let manifest = create_test_manifest("myframework", vec!["git"], "default");
    let config = generate_zshrc(&manifest).unwrap();

    assert!(config.contains("MYFRAMEWORK_HOME"));
    assert!(config.contains("myframework_plugin git"));
}
```

## Step 8: Update Documentation

Update `docs/user-guide/frameworks.md`:

```markdown
## myframework

**Philosophy**: "Describe the framework's approach"

**Official Site**: https://myframework.sh/

### Strengths
- Fast
- Simple
- Feature X

### Weaknesses
- Limited plugins
- New/unproven

### When to Choose
- When you want X
- When you need Y

### Example Profile

\`\`\`toml
[profile]
framework = "myframework"
theme = "default"

[plugins]
enabled = ["git", "docker"]
\`\`\`
```

## Step 9: Submit PR

1. Run tests: `cargo test`
2. Format code: `cargo fmt`
3. Run clippy: `cargo clippy`
4. Commit changes
5. Open pull request with description

## Framework-Specific Considerations

### oh-my-zsh-style Frameworks

If the framework uses oh-my-zsh's plugin/theme structure:
- Plugins in `plugins/` directory
- Themes in `themes/` directory
- Uses `source` command for loading

### Plugin Manager-style Frameworks

If it's a plugin manager (zinit, zap):
- Different loading syntax for each plugin
- May need repo URLs
- Support for snippets/scripts

### Module-based Frameworks

If it uses modules (prezto, zimfw):
- Need `.frameworkrc` or equivalent
- Module-specific configuration
- Dependency handling

## Common Pitfalls

1. **Forgetting to add to `FrameworkType` enum**: Causes compilation errors
2. **Not handling missing config files**: Detection should be robust
3. **Shell syntax errors**: Always test generated configs with `zsh -n`
4. **Not escaping special characters**: Use proper shell escaping
5. **Hardcoded paths**: Use `$ZDOTDIR`, `$HOME` variables

## Testing Checklist

- [ ] Detection works with real framework installation
- [ ] Detection returns `None` when framework not installed
- [ ] Installation succeeds in fresh profile
- [ ] Generated config has correct syntax (`zsh -n`)
- [ ] Plugins load correctly
- [ ] Theme applies correctly
- [ ] Works on macOS and Linux
- [ ] Integration test passes

## Getting Help

- Check existing framework implementations for examples
- Ask in [GitHub Discussions](https://github.com/annabarnes1138/zprof/discussions)
- Reference the [Architecture Overview](architecture.md)
