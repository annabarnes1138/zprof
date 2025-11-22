# Story 1.6: Update Generator for Prompt Engines

**Epic:** Epic 1 - Smart Prompt Selection
**Priority:** P0 (Must Have)
**Status:** ready-for-dev

## Dev Agent Record

**Context Reference:**
- [epic-1-story-6.context.xml](epic-1-story-6.context.xml)

## User Story

**As a** developer
**I want** the generator to handle prompt engines correctly
**So that** shell configs initialize engines instead of framework themes

## Acceptance Criteria

- [ ] Modify `src/shell/generator.rs` to handle prompt modes:
  - If `prompt_mode = PromptEngine`:
    - Disable framework theme (`ZSH_THEME=""` for oh-my-zsh)
    - Add engine initialization (e.g., `eval "$(starship init zsh)"`)
    - Handle framework-specific syntax (oh-my-zsh, zimfw, prezto)
  - If `prompt_mode = FrameworkTheme`:
    - Use existing theme logic (no changes)
- [ ] Create prompt engine installer: `src/prompts/installer.rs`
  - Support installation methods:
    - Binary download (e.g., Starship via official installer)
    - Git clone (e.g., Pure, Spaceship)
    - Framework plugin (e.g., Powerlevel10k for oh-my-zsh)
  - Handle errors gracefully (network failures, permission issues)
  - Verify installation success
- [ ] Add engine initialization during profile creation:
  - Install selected prompt engine
  - Configure engine path in shell config
  - Add initialization command to .zshrc
- [ ] Validate generated configs:
  - Use `zsh -n` to check syntax
  - Ensure no conflicts between engine and framework theme
  - Verify initialization order (framework → plugins → engine)
- [ ] Add comprehensive tests:
  - Test each engine × framework combination (5 engines × 3 frameworks = 15 tests)
  - Snapshot tests for generated configs
  - Integration tests with actual shell execution
  - Error handling tests (failed installation, missing dependencies)
- [ ] Handle edge cases:
  - Engine already installed (skip installation)
  - Nerd Font requirement warnings
  - Cross-shell compatibility checks
- [ ] Add rollback mechanism:
  - If engine installation fails, offer framework theme fallback
  - Clean up partial installations
  - Log errors for debugging

## Technical Details

### Generator Modification

```rust
// src/shell/generator.rs

use crate::core::manifest::{Manifest, PromptMode};
use crate::prompts::engine::PromptEngine;
use crate::prompts::installer::EngineInstaller;
use anyhow::{Context, Result};

pub struct ShellGenerator {
    manifest: Manifest,
}

impl ShellGenerator {
    pub fn generate_zshrc(&self) -> Result<String> {
        let mut config = String::new();

        // Framework setup
        config.push_str(&self.generate_framework_header()?);

        // Prompt configuration (MODE-DEPENDENT)
        match &self.manifest.profile.prompt_mode {
            PromptMode::PromptEngine { engine } => {
                config.push_str(&self.generate_engine_config(engine)?);
            }
            PromptMode::FrameworkTheme { theme } => {
                config.push_str(&self.generate_theme_config(theme)?);
            }
        }

        // Plugins
        config.push_str(&self.generate_plugin_config()?);

        // Framework initialization
        config.push_str(&self.generate_framework_source()?);

        // Engine initialization (AFTER framework)
        if let PromptMode::PromptEngine { engine } = &self.manifest.profile.prompt_mode {
            config.push_str(&self.generate_engine_init(engine)?);
        }

        Ok(config)
    }

    fn generate_engine_config(&self, engine: &PromptEngine) -> Result<String> {
        match &self.manifest.profile.framework.as_str() {
            "oh-my-zsh" => Ok("ZSH_THEME=\"\"\n".to_string()),
            "zimfw" => Ok("# Prompt engine mode - framework theme disabled\n".to_string()),
            "prezto" => Ok("zstyle ':prezto:module:prompt' theme 'off'\n".to_string()),
            _ => Ok(String::new()),
        }
    }

    fn generate_engine_init(&self, engine: &PromptEngine) -> Result<String> {
        let init_cmd = engine.init_command();
        Ok(format!(
            "\n# {} initialization\n{}\n",
            engine.name(),
            init_cmd
        ))
    }
}
```

### Prompt Engine Installer

```rust
// src/prompts/installer.rs

use crate::prompts::engine::{PromptEngine, InstallMethod};
use anyhow::{anyhow, Context, Result};
use std::process::Command;
use std::path::PathBuf;

pub struct EngineInstaller {
    home_dir: PathBuf,
}

impl EngineInstaller {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?;
        Ok(Self { home_dir })
    }

    pub fn install(&self, engine: &PromptEngine) -> Result<()> {
        // Check if already installed
        if self.is_installed(engine)? {
            println!("{} is already installed, skipping...", engine.name());
            return Ok(());
        }

        println!("Installing {}...", engine.name());

        match engine.install_method() {
            InstallMethod::Binary { url } => self.install_binary(engine, url),
            InstallMethod::GitClone { repo } => self.install_git(engine, repo),
            InstallMethod::FrameworkPlugin { plugin_name } => {
                self.install_framework_plugin(plugin_name)
            }
        }
    }

    fn install_binary(&self, engine: &PromptEngine, url: &str) -> Result<()> {
        // Example: Starship installation
        let output = Command::new("sh")
            .arg("-c")
            .arg(url)
            .output()
            .context("Failed to run installer script")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!(
                "Installation failed for {}: {}",
                engine.name(),
                stderr
            ));
        }

        Ok(())
    }

    fn install_git(&self, engine: &PromptEngine, repo: &str) -> Result<()> {
        let install_dir = self.home_dir.join(format!(".zprof/engines/{}", engine.name()));

        if install_dir.exists() {
            println!("{} already cloned, skipping...", engine.name());
            return Ok(());
        }

        let output = Command::new("git")
            .args(&["clone", "--depth=1", repo, install_dir.to_str().unwrap()])
            .output()
            .context("Failed to clone repository")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Git clone failed for {}: {}", engine.name(), stderr));
        }

        Ok(())
    }

    fn install_framework_plugin(&self, plugin_name: &str) -> Result<()> {
        // Framework-specific plugin installation
        // For oh-my-zsh: clone to custom plugins directory
        let custom_plugins_dir = self
            .home_dir
            .join(".oh-my-zsh/custom/plugins")
            .join(plugin_name);

        if custom_plugins_dir.exists() {
            println!("Plugin {} already installed", plugin_name);
            return Ok(());
        }

        // Clone plugin repo
        // (Implementation depends on framework and plugin)
        Ok(())
    }

    fn is_installed(&self, engine: &PromptEngine) -> Result<bool> {
        match engine {
            PromptEngine::Starship => {
                // Check if starship binary exists in PATH
                Command::new("which")
                    .arg("starship")
                    .output()
                    .map(|o| o.status.success())
                    .context("Failed to check for starship")
            }
            PromptEngine::Powerlevel10k => {
                // Check if p10k directory exists
                let p10k_dir = self.home_dir.join(".oh-my-zsh/custom/themes/powerlevel10k");
                Ok(p10k_dir.exists())
            }
            PromptEngine::Pure => {
                let pure_dir = self.home_dir.join(".zprof/engines/pure");
                Ok(pure_dir.exists())
            }
            // ... other engines
            _ => Ok(false),
        }
    }
}
```

### Installation Method Configuration

```rust
// src/prompts/engine.rs (additions)

impl PromptEngine {
    pub fn install_method(&self) -> InstallMethod {
        match self {
            PromptEngine::Starship => InstallMethod::Binary {
                url: "curl -sS https://starship.rs/install.sh | sh",
            },
            PromptEngine::Powerlevel10k => InstallMethod::GitClone {
                repo: "https://github.com/romkatv/powerlevel10k.git",
            },
            PromptEngine::OhMyPosh => InstallMethod::Binary {
                url: "brew install jandedobbeleer/oh-my-posh/oh-my-posh",
            },
            PromptEngine::Pure => InstallMethod::GitClone {
                repo: "https://github.com/sindresorhus/pure.git",
            },
            PromptEngine::Spaceship => InstallMethod::GitClone {
                repo: "https://github.com/spaceship-prompt/spaceship-prompt.git",
            },
        }
    }

    pub fn init_command(&self) -> &'static str {
        match self {
            PromptEngine::Starship => "eval \"$(starship init zsh)\"",
            PromptEngine::Powerlevel10k => "source ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k/powerlevel10k.zsh-theme",
            PromptEngine::OhMyPosh => "eval \"$(oh-my-posh init zsh)\"",
            PromptEngine::Pure => {
                "fpath+=$HOME/.zprof/engines/pure\nautoload -U promptinit; promptinit\nprompt pure"
            }
            PromptEngine::Spaceship => "source $HOME/.zprof/engines/spaceship-prompt/spaceship.zsh",
        }
    }
}
```

### Generated Config Examples

**Starship with oh-my-zsh:**
```bash
# Framework: oh-my-zsh
export ZSH="$ZDOTDIR/.oh-my-zsh"
ZSH_THEME=""  # Disabled for external prompt engine

# Plugins
plugins=(git zsh-autosuggestions zsh-syntax-highlighting)

# Initialize framework
source $ZSH/oh-my-zsh.sh

# Starship initialization
eval "$(starship init zsh)"
```

**Pure with zimfw:**
```bash
# Framework: zimfw
# Prompt engine mode - framework theme disabled

# zimfw initialization
source ${ZDOTDIR:-${HOME}}/.zim/init.zsh

# Pure initialization
fpath+=$HOME/.zprof/engines/pure
autoload -U promptinit; promptinit
prompt pure
```

**robbyrussell theme (no engine):**
```bash
# Framework: oh-my-zsh
export ZSH="$ZDOTDIR/.oh-my-zsh"
ZSH_THEME="robbyrussell"

# Plugins
plugins=(git zsh-autosuggestions)

# Initialize framework
source $ZSH/oh-my-zsh.sh
```

## Files Created/Modified

**New Files:**
- `src/prompts/installer.rs`

**Modified Files:**
- `src/shell/generator.rs` (add prompt mode branching)
- `src/prompts/engine.rs` (add installation metadata)
- `src/cli/create.rs` (integrate engine installation)
- `tests/generator_test.rs` (add engine × framework tests)

## Dependencies

- **Blocks:** Stories 1.1, 1.3 (requires PromptMode enum and PromptEngine registry)
- **External dependencies:**
  - `dirs` crate for home directory detection
  - Git (for clone-based installations)
  - curl/wget (for binary downloads)

## Testing

**Unit Tests:**

```rust
// tests/generator_test.rs

#[test]
fn test_generate_starship_config() {
    let manifest = Manifest {
        profile: Profile {
            framework: "oh-my-zsh".to_string(),
            prompt_mode: PromptMode::PromptEngine {
                engine: PromptEngine::Starship,
            },
            plugins: vec!["git".to_string()],
        },
    };

    let generator = ShellGenerator::new(manifest);
    let config = generator.generate_zshrc().unwrap();

    assert!(config.contains("ZSH_THEME=\"\""));
    assert!(config.contains("eval \"$(starship init zsh)\""));
    assert!(!config.contains("ZSH_THEME=")); // Should not have theme assignment
}

#[test]
fn test_generate_theme_config() {
    let manifest = Manifest {
        profile: Profile {
            framework: "oh-my-zsh".to_string(),
            prompt_mode: PromptMode::FrameworkTheme {
                theme: "robbyrussell".to_string(),
            },
            plugins: vec![],
        },
    };

    let generator = ShellGenerator::new(manifest);
    let config = generator.generate_zshrc().unwrap();

    assert!(config.contains("ZSH_THEME=\"robbyrussell\""));
    assert!(!config.contains("starship"));
    assert!(!config.contains("powerlevel10k"));
}

#[test]
fn test_all_engine_framework_combinations() {
    let engines = vec![
        PromptEngine::Starship,
        PromptEngine::Powerlevel10k,
        PromptEngine::OhMyPosh,
        PromptEngine::Pure,
        PromptEngine::Spaceship,
    ];

    let frameworks = vec!["oh-my-zsh", "zimfw", "prezto"];

    for engine in &engines {
        for framework in &frameworks {
            let manifest = create_test_manifest(framework, engine);
            let generator = ShellGenerator::new(manifest);
            let config = generator.generate_zshrc().unwrap();

            // Validate syntax
            assert!(validate_zsh_syntax(&config).is_ok());

            // Verify engine init exists
            assert!(config.contains(engine.init_command()));
        }
    }
}
```

**Integration Tests:**

```rust
// tests/installer_integration_test.rs

#[test]
#[ignore] // Run manually (requires network)
fn test_install_starship() {
    let installer = EngineInstaller::new().unwrap();
    let result = installer.install(&PromptEngine::Starship);

    assert!(result.is_ok());
    assert!(installer.is_installed(&PromptEngine::Starship).unwrap());
}

#[test]
fn test_install_failure_handling() {
    // Mock network failure
    let installer = EngineInstaller::new().unwrap();
    // ... test error handling
}
```

**Manual Verification:**

1. **Test each engine installation:**
   ```bash
   zprof create test-starship
   # Select Starship → verify installation
   # Activate profile → verify prompt changes
   ```

2. **Test framework theme (no regression):**
   ```bash
   zprof create test-theme
   # Select framework theme → verify works as before
   ```

3. **Test generated configs:**
   ```bash
   cat ~/.zprof/profiles/test-starship/home/.zshrc
   # Verify ZSH_THEME="" and eval "$(starship init zsh)"
   ```

4. **Test syntax validation:**
   ```bash
   zsh -n ~/.zprof/profiles/test-starship/home/.zshrc
   # Should exit 0 (no syntax errors)
   ```

## Success Criteria

- [ ] All 15 engine × framework combinations generate valid configs
- [ ] Syntax validation passes for all generated configs
- [ ] Engine installation works for all supported engines
- [ ] Error handling gracefully handles failures
- [ ] No regression: framework themes still work
- [ ] All tests passing

## Notes

- Installation may require user interaction (sudo password, confirmations)
- Consider caching installations (shared across profiles)
- Nerd Font warnings should be shown before installation
- Future: Add progress indicators for long installations
- Future: Support custom engine configurations (e.g., Starship TOML)

## References

- Starship Installation: https://starship.rs/guide/#%F0%9F%9A%80-installation
- Powerlevel10k: https://github.com/romkatv/powerlevel10k#installation
- Pure: https://github.com/sindresorhus/pure#install
- Epic 1: [docs/planning/v0.2.0/epic-1-smart-tui.md](../epic-1-smart-tui.md) (Story 1.6)
- Architecture: [docs/developer/architecture.md](../../../developer/architecture.md) (Shell Generation)

## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-6.context.xml](epic-1-story-6.context.xml)
