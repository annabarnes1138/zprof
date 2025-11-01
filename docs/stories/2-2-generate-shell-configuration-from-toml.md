# Story 2.2: Generate Shell Configuration from TOML

Status: ready-for-dev

## Story

As a developer,
I want zprof to automatically generate .zshrc and .zshenv from my TOML manifest,
so that my declarative configuration is translated into functional shell files.

## Acceptance Criteria

1. System generates .zshrc with framework initialization, plugin loading, and theme activation
2. System generates .zshenv with environment variables from manifest
3. Generated files include header comments indicating they're auto-generated from manifest
4. Re-generation from manifest overwrites previous generated files (manifest is source of truth)
5. Generated configuration is syntactically valid zsh code
6. Process completes in under 1 second for typical profiles

## Tasks / Subtasks

- [ ] Create shell generator module (AC: All)
  - [ ] Create `shell/generator.rs` module
  - [ ] Define generation functions for .zshrc and .zshenv
  - [ ] Follow Pattern 5 (Shell Integration) from architecture
  - [ ] Use ProfileManifest from Story 2.1 as input
  - [ ] Return generated content as strings
  - [ ] Add comprehensive logging for debugging
- [ ] Implement .zshenv generation (AC: #2, #3, #5)
  - [ ] Create generate_zshenv(manifest) function
  - [ ] Add header comment with warning: "Auto-generated from profile.toml - DO NOT EDIT"
  - [ ] Include generation timestamp and zprof version
  - [ ] Set HISTFILE to shared history: `~/.zsh-profiles/shared/.zsh_history`
  - [ ] Export all env vars from manifest.env as `export KEY="VALUE"`
  - [ ] Quote environment variable values to handle spaces
  - [ ] Escape special shell characters in values
  - [ ] Ensure syntactically valid zsh code (test with zsh -n)
  - [ ] Return complete .zshenv content as String
- [ ] Implement .zshrc generation for each framework (AC: #1, #3, #5)
  - [ ] Create generate_zshrc(manifest) function
  - [ ] Add header comment with warning: "Auto-generated from profile.toml - DO NOT EDIT"
  - [ ] Include generation timestamp and zprof version
  - [ ] Implement framework-specific initialization patterns
  - [ ] oh-my-zsh: Source oh-my-zsh.sh, set ZSH_THEME, set plugins array
  - [ ] zimfw: Source zimfw.zsh, load plugins with zmodule
  - [ ] prezto: Source init.zsh, set zpreztorc plugins
  - [ ] zinit: Source zinit.zsh, load plugins with zinit light
  - [ ] zap: Source zap.zsh, load plugins with plug
  - [ ] Set theme per framework conventions
  - [ ] Ensure syntactically valid zsh code for each framework
- [ ] Implement file writing operations (AC: #4, #6)
  - [ ] Create write_generated_files(profile_name, manifest) function
  - [ ] Get profile directory path from profile_name
  - [ ] Generate .zshenv content
  - [ ] Generate .zshrc content
  - [ ] Write .zshenv to profile directory
  - [ ] Write .zshrc to profile directory
  - [ ] Overwrite existing files without backup (manifest is source of truth per AC #4)
  - [ ] Set appropriate file permissions (0644 - readable/writable by user)
  - [ ] Complete in under 1 second (AC: #6)
  - [ ] Log file paths written
- [ ] Add regeneration command (AC: #4)
  - [ ] Create `cli/regenerate.rs` or add to `cli/edit.rs`
  - [ ] Define RegenerateArgs with profile_name
  - [ ] Load manifest with load_and_validate()
  - [ ] Call write_generated_files()
  - [ ] Display success message showing files regenerated
  - [ ] Use anyhow::Context for errors
- [ ] Handle edge cases and errors (AC: All)
  - [ ] Profile directory doesn't exist: create it
  - [ ] Manifest doesn't exist: clear error message
  - [ ] Invalid manifest: show validation errors
  - [ ] Framework not supported: show supported list
  - [ ] File write permission denied: helpful error
  - [ ] Generated file validation (zsh -n dry-run syntax check)
  - [ ] Empty plugin list: valid (minimal profile)
  - [ ] Empty env vars: valid (no custom env)
- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test .zshenv generation with various env vars
  - [ ] Unit test .zshrc generation for each framework
  - [ ] Unit test special character escaping in env values
  - [ ] Unit test header comments are present
  - [ ] Integration test write_generated_files()
  - [ ] Test regeneration overwrites files (AC: #4)
  - [ ] Test generated files are syntactically valid (zsh -n)
  - [ ] Test performance under 1 second (AC: #6)
  - [ ] Manual test generated .zshrc sources correctly in actual zsh
  - [ ] Manual test all 5 frameworks with real installations

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `shell/generator.rs`
- Secondary: `core/manifest.rs` (reads manifest), `cli/regenerate.rs` (CLI command)
- Follow Pattern 5 (Shell Integration)
- Follow Pattern 2 (Error Handling)
- Implements ADR-002 (TOML as source of truth)
- Meets NFR001 subset: < 1 second generation

**.zshenv Generation Pattern:**
```rust
// shell/generator.rs
use anyhow::{Context, Result};
use crate::core::manifest::ProfileManifest;
use chrono::Utc;

pub fn generate_zshenv(manifest: &ProfileManifest) -> Result<String> {
    let mut output = String::new();

    // Header comment
    output.push_str("# Auto-generated by zprof from profile.toml\n");
    output.push_str("# DO NOT EDIT THIS FILE DIRECTLY - Edit profile.toml instead\n");
    output.push_str(&format!("# Generated: {}\n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    output.push_str("# Profile: {}\n", manifest.profile.name);
    output.push_str("\n");

    // Shared history configuration
    output.push_str("# Shared command history across all profiles\n");
    output.push_str("export HISTFILE=\"$HOME/.zsh-profiles/shared/.zsh_history\"\n");
    output.push_str("export HISTSIZE=10000\n");
    output.push_str("export SAVEHIST=10000\n");
    output.push_str("\n");

    // Environment variables from manifest
    if !manifest.env.is_empty() {
        output.push_str("# Custom environment variables\n");
        for (key, value) in &manifest.env {
            // Escape quotes and special characters
            let escaped_value = escape_shell_value(value);
            output.push_str(&format!("export {}=\"{}\"\n", key, escaped_value));
        }
        output.push_str("\n");
    }

    Ok(output)
}

fn escape_shell_value(value: &str) -> String {
    // Escape double quotes and backslashes for shell safety
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$")
        .replace('`', "\\`")
}
```

**.zshrc Generation Pattern (Framework-Specific):**
```rust
// shell/generator.rs (continued)

pub fn generate_zshrc(manifest: &ProfileManifest) -> Result<String> {
    let mut output = String::new();

    // Header comment
    output.push_str("# Auto-generated by zprof from profile.toml\n");
    output.push_str("# DO NOT EDIT THIS FILE DIRECTLY - Edit profile.toml instead\n");
    output.push_str(&format!("# Generated: {}\n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    output.push_str(&format!("# Profile: {}\n", manifest.profile.name));
    output.push_str(&format!("# Framework: {}\n", manifest.profile.framework));
    output.push_str("\n");

    // Generate framework-specific initialization
    match manifest.profile.framework.as_str() {
        "oh-my-zsh" => generate_oh_my_zsh_config(&mut output, manifest)?,
        "zimfw" => generate_zimfw_config(&mut output, manifest)?,
        "prezto" => generate_prezto_config(&mut output, manifest)?,
        "zinit" => generate_zinit_config(&mut output, manifest)?,
        "zap" => generate_zap_config(&mut output, manifest)?,
        _ => bail!("Unsupported framework: {}", manifest.profile.framework),
    }

    Ok(output)
}

fn generate_oh_my_zsh_config(output: &mut String, manifest: &ProfileManifest) -> Result<()> {
    output.push_str("# oh-my-zsh configuration\n");

    // Set ZSH path
    output.push_str("export ZSH=\"$ZDOTDIR/.oh-my-zsh\"\n");
    output.push_str("\n");

    // Set theme
    if !manifest.profile.theme.is_empty() {
        output.push_str(&format!("ZSH_THEME=\"{}\"\n", manifest.profile.theme));
    } else {
        output.push_str("ZSH_THEME=\"robbyrussell\"\n");
    }
    output.push_str("\n");

    // Set plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("plugins=(\n");
        for plugin in &manifest.plugins.enabled {
            output.push_str(&format!("  {}\n", plugin));
        }
        output.push_str(")\n");
        output.push_str("\n");
    }

    // Source oh-my-zsh
    output.push_str("source $ZSH/oh-my-zsh.sh\n");

    Ok(())
}

fn generate_zimfw_config(output: &mut String, manifest: &ProfileManifest) -> Result<()> {
    output.push_str("# zimfw configuration\n");

    // Set ZIM_HOME
    output.push_str("export ZIM_HOME=\"$ZDOTDIR/.zim\"\n");
    output.push_str("\n");

    // Load plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("# Plugins\n");
        for plugin in &manifest.plugins.enabled {
            output.push_str(&format!("zmodule {}\n", plugin));
        }
        output.push_str("\n");
    }

    // Set theme
    if !manifest.profile.theme.is_empty() {
        output.push_str(&format!("zmodule {}\n", manifest.profile.theme));
        output.push_str("\n");
    }

    // Source zimfw
    output.push_str("source $ZIM_HOME/init.zsh\n");

    Ok(())
}

fn generate_prezto_config(output: &mut String, manifest: &ProfileManifest) -> Result<()> {
    output.push_str("# prezto configuration\n");

    // Set ZDOTDIR for prezto
    output.push_str("export PREZTO_DIR=\"$ZDOTDIR/.zprezto\"\n");
    output.push_str("\n");

    // Load plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("# Prezto modules\n");
        output.push_str("zstyle ':prezto:load' pmodule \\\n");
        for (idx, plugin) in manifest.plugins.enabled.iter().enumerate() {
            if idx == manifest.plugins.enabled.len() - 1 {
                output.push_str(&format!("  '{}'\n", plugin));
            } else {
                output.push_str(&format!("  '{}' \\\n", plugin));
            }
        }
        output.push_str("\n");
    }

    // Set theme
    if !manifest.profile.theme.is_empty() {
        output.push_str(&format!("zstyle ':prezto:module:prompt' theme '{}'\n", manifest.profile.theme));
        output.push_str("\n");
    }

    // Source prezto
    output.push_str("source $PREZTO_DIR/init.zsh\n");

    Ok(())
}

fn generate_zinit_config(output: &mut String, manifest: &ProfileManifest) -> Result<()> {
    output.push_str("# zinit configuration\n");

    // Set zinit home
    output.push_str("export ZINIT_HOME=\"$ZDOTDIR/.zinit\"\n");
    output.push_str("\n");

    // Source zinit
    output.push_str("source $ZINIT_HOME/zinit.zsh\n");
    output.push_str("\n");

    // Load plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("# Plugins\n");
        for plugin in &manifest.plugins.enabled {
            output.push_str(&format!("zinit light {}\n", plugin));
        }
        output.push_str("\n");
    }

    // Load theme
    if !manifest.profile.theme.is_empty() {
        output.push_str(&format!("zinit light {}\n", manifest.profile.theme));
        output.push_str("\n");
    }

    Ok(())
}

fn generate_zap_config(output: &mut String, manifest: &ProfileManifest) -> Result<()> {
    output.push_str("# zap configuration\n");

    // Set zap home
    output.push_str("export ZAP_DIR=\"$ZDOTDIR/.zap\"\n");
    output.push_str("\n");

    // Source zap
    output.push_str("source $ZAP_DIR/zap.zsh\n");
    output.push_str("\n");

    // Load plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("# Plugins\n");
        for plugin in &manifest.plugins.enabled {
            output.push_str(&format!("plug \"{}\"\n", plugin));
        }
        output.push_str("\n");
    }

    // Load theme
    if !manifest.profile.theme.is_empty() {
        output.push_str(&format!("plug \"{}\"\n", manifest.profile.theme));
        output.push_str("\n");
    }

    Ok(())
}
```

**File Writing Pattern:**
```rust
// shell/generator.rs (continued)

use std::path::Path;
use std::fs;
use std::time::Instant;

pub fn write_generated_files(profile_name: &str, manifest: &ProfileManifest) -> Result<()> {
    let start = Instant::now();

    // Get profile directory
    let profile_dir = dirs::home_dir()
        .context("Could not find home directory")?
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name);

    // Ensure profile directory exists
    fs::create_dir_all(&profile_dir)
        .context(format!("Failed to create profile directory: {:?}", profile_dir))?;

    // Generate .zshenv
    let zshenv_content = generate_zshenv(manifest)?;
    let zshenv_path = profile_dir.join(".zshenv");
    fs::write(&zshenv_path, zshenv_content)
        .context(format!("Failed to write .zshenv to {:?}", zshenv_path))?;
    log::info!("Generated: {:?}", zshenv_path);

    // Generate .zshrc
    let zshrc_content = generate_zshrc(manifest)?;
    let zshrc_path = profile_dir.join(".zshrc");
    fs::write(&zshrc_path, zshrc_content)
        .context(format!("Failed to write .zshrc to {:?}", zshrc_path))?;
    log::info!("Generated: {:?}", zshrc_path);

    // Validate syntax (optional, requires zsh binary)
    validate_zsh_syntax(&zshrc_path)?;

    let duration = start.elapsed();
    log::debug!("Generation completed in {:?}", duration);

    // Should complete in under 1 second (AC: #6)
    if duration.as_secs() >= 1 {
        log::warn!("Generation took longer than expected: {:?}", duration);
    }

    Ok(())
}

fn validate_zsh_syntax(file_path: &Path) -> Result<()> {
    // Run 'zsh -n <file>' to check syntax without executing
    let output = std::process::Command::new("zsh")
        .arg("-n")
        .arg(file_path)
        .output();

    match output {
        Ok(result) if result.status.success() => {
            log::debug!("Syntax validation passed: {:?}", file_path);
            Ok(())
        }
        Ok(result) => {
            let stderr = String::from_utf8_lossy(&result.stderr);
            bail!("Generated zsh file has syntax errors:\n{}", stderr);
        }
        Err(e) => {
            log::warn!("Could not validate syntax (zsh not available): {}", e);
            Ok(()) // Don't fail if zsh not available
        }
    }
}
```

**Example Generated .zshenv:**
```zsh
# Auto-generated by zprof from profile.toml
# DO NOT EDIT THIS FILE DIRECTLY - Edit profile.toml instead
# Generated: 2025-10-31 15:30:00
# Profile: work

# Shared command history across all profiles
export HISTFILE="$HOME/.zsh-profiles/shared/.zsh_history"
export HISTSIZE=10000
export SAVEHIST=10000

# Custom environment variables
export EDITOR="vim"
export GOPATH="$HOME/go"
export PATH="$HOME/bin:$PATH"
```

**Example Generated .zshrc (oh-my-zsh):**
```zsh
# Auto-generated by zprof from profile.toml
# DO NOT EDIT THIS FILE DIRECTLY - Edit profile.toml instead
# Generated: 2025-10-31 15:30:00
# Profile: work
# Framework: oh-my-zsh

# oh-my-zsh configuration
export ZSH="$ZDOTDIR/.oh-my-zsh"

ZSH_THEME="robbyrussell"

plugins=(
  git
  docker
  kubectl
  fzf
)

source $ZSH/oh-my-zsh.sh
```

**CLI Command for Regeneration:**
```rust
// cli/regenerate.rs (or part of edit.rs)
use anyhow::{Context, Result};
use clap::Args;
use crate::core::manifest;
use crate::shell::generator;

#[derive(Debug, Args)]
pub struct RegenerateArgs {
    /// Name of the profile to regenerate
    pub profile_name: String,
}

pub fn execute(args: RegenerateArgs) -> Result<()> {
    // Load and validate manifest
    let manifest = manifest::load_and_validate(&args.profile_name)
        .context("Cannot regenerate from invalid manifest")?;

    // Generate shell files
    generator::write_generated_files(&args.profile_name, &manifest)
        .context("Failed to generate shell configuration files")?;

    println!("✓ Shell configuration regenerated successfully");
    println!();
    println!("  Profile: {}", args.profile_name);
    println!("  Framework: {}", manifest.profile.framework);
    println!("  Files updated:");
    println!("    - .zshrc");
    println!("    - .zshenv");
    println!();
    println!("  → Run 'zprof use {}' to activate changes", args.profile_name);

    Ok(())
}
```

**Usage Example:**
```bash
# After editing profile.toml manually
$ vim ~/.zsh-profiles/profiles/work/profile.toml
# ... add new plugin ...

# Regenerate shell files from manifest
$ zprof regenerate work
✓ Shell configuration regenerated successfully

  Profile: work
  Framework: oh-my-zsh
  Files updated:
    - .zshrc
    - .zshenv

  → Run 'zprof use work' to activate changes

# Switch to profile to load new configuration
$ zprof use work
```

**Performance Considerations (AC: #6):**
```rust
// Generation is extremely fast:
// 1. Load manifest: ~1-5ms (parse TOML)
// 2. Generate .zshenv: ~0.1ms (string formatting)
// 3. Generate .zshrc: ~0.5ms (framework-specific template)
// 4. Write files: ~5-10ms (two file writes)
// 5. Validate syntax: ~10-50ms (zsh -n execution)
// Total: ~20-70ms << 1 second requirement ✓
```

### Project Structure Notes

**New Files Created:**
- `src/shell/generator.rs` - Shell configuration generation from manifests
- `src/cli/regenerate.rs` - CLI command to regenerate shell files

**Modified Files:**
- `src/main.rs` - Register `regenerate` subcommand (optional - may add later)
- `src/shell/mod.rs` - Export generator module
- `src/cli/mod.rs` - Export regenerate module

**Generated Files (User Space):**
- `~/.zsh-profiles/profiles/<name>/.zshrc` - Auto-generated from profile.toml
- `~/.zsh-profiles/profiles/<name>/.zshenv` - Auto-generated from profile.toml

**Learnings from Previous Stories:**

**From Story 2.1: Parse and Validate TOML Manifests (Status: drafted)**

Story 2.1 provides the manifest parsing foundation that Story 2.2 builds upon:

- **Manifest Loading**: Use `load_and_validate()` to get ProfileManifest
- **Validation**: Ensure manifest is valid before generation
- **Framework Field**: manifest.profile.framework determines generation template
- **Plugins Array**: manifest.plugins.enabled lists plugins to load
- **Env Vars**: manifest.env provides environment variable definitions
- **Error Handling**: Use anyhow::Context for all errors

**Critical Integration:**
Story 2.2 is the consumer of Story 2.1's output. The ProfileManifest struct is the contract between parsing and generation.

**From Story 1.9: Switch Active Profile (Status: drafted)**

Story 1.9 establishes shared history pattern that Story 2.2 implements:

- **Shared History**: .zshenv sets HISTFILE to shared location
- **ZDOTDIR**: Generated files assume they're in ZDOTDIR
- **Framework Paths**: Framework installations at `$ZDOTDIR/.<framework-dir>`

**From Story 1.8: TUI Wizard Theme Selection and Profile Generation (Status: drafted)**

Story 1.8 originally generates manifests and shell files. Story 2.2 provides the core generation logic that 1.8 can use:

- **Generation Patterns**: Story 2.2 establishes canonical generation templates
- **Framework Support**: All 5 frameworks have generation templates
- **Reusability**: Story 1.8 can call `write_generated_files()` after wizard completes

**Integration Requirements:**
- Story 2.1 provides manifest parsing
- Story 2.2 provides generation from manifest
- Story 2.3 will use regeneration after manual edits
- Stories 1.5-1.8 can use this module for initial profile creation
- Manifest is single source of truth (ADR-002)

**Manifest as Source of Truth (AC #4):**

This story implements the principle that profile.toml is the authoritative source:

- **Regeneration Overwrites**: No merge, just overwrite from manifest
- **No Manual Edits**: Header warns users not to edit generated files
- **Edit → Regenerate Flow**: Edit manifest, regenerate shell files
- **Consistency**: Shell files always match manifest exactly

**Benefits:**
- No drift between manifest and shell files
- Simple mental model: TOML → shell files
- Easy to version control (track manifest, ignore generated files)
- Reproducible profiles (manifest fully describes configuration)

### References

- [Source: docs/epics.md#Story-2.2]
- [Source: docs/PRD.md#FR013-generate-shell-from-manifest]
- [Source: docs/PRD.md#NFR001-sub-500ms] (subset: < 1s generation)
- [Source: docs/architecture.md#ADR-002-TOML-not-YAML]
- [Source: docs/architecture.md#Pattern-4-TOML-Manifest-Schema]
- [Source: docs/architecture.md#Pattern-5-Shell-Integration]
- [Source: docs/architecture.md#Epic-2-Story-2.2-Mapping]
- [Source: docs/stories/2-1-parse-and-validate-yaml-manifests.md#manifest-structure]
- [Source: docs/stories/1-9-switch-active-profile.md#shared-history]

## Dev Agent Record

### Context Reference

- [2-2-generate-shell-configuration-from-yaml.context.xml](docs/stories/2-2-generate-shell-configuration-from-yaml.context.xml)

### Agent Model Used

<!-- Will be populated by dev agent during implementation -->

### Debug Log References

<!-- Will be populated by dev agent during implementation -->

### Completion Notes List

<!-- Will be populated by dev agent during implementation -->

### File List

<!-- Will be populated by dev agent during implementation -->

## Change Log

- 2025-10-31: Story drafted by SM agent (Bob)
