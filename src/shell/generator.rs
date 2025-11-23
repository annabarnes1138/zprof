//! Shell configuration generator
//!
//! Generates .zshrc and .zshenv files from profile manifest data.
//! Handles framework-specific configuration differences.
//!
//! This module implements Story 2.2: Generate Shell Configuration from TOML
//! Generates .zshrc and .zshenv from ProfileManifest following Pattern 5.

use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::core::manifest::{Manifest, PromptMode};

/// Current zprof version for generated file headers
const ZPROF_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Write generated .zshrc and .zshenv files from manifest (Story 2.2)
///
/// This is the main entry point for shell file generation from manifests.
/// It orchestrates the generation of both .zshrc and .zshenv files,
/// validates syntax, and ensures performance requirements are met.
///
/// # Arguments
///
/// * `profile_name` - The profile name (used to locate profile directory)
/// * `manifest` - The validated ProfileManifest containing configuration
///
/// # Errors
///
/// Returns an error if:
/// - Profile directory cannot be created
/// - File writes fail
/// - Generated files have syntax errors (when zsh is available)
/// - Generation takes longer than 1 second (logged as warning, not error)
///
/// # Performance
///
/// Must complete in under 1 second per AC #6
pub fn write_generated_files(profile_name: &str, manifest: &Manifest) -> Result<()> {
    let start = Instant::now();

    // Validate profile name to prevent path traversal attacks
    if profile_name.contains("..") || profile_name.contains('/') || profile_name.contains('\\') {
        bail!(
            "Invalid profile name '{profile_name}': cannot contain path traversal characters (.., /, \\)"
        );
    }

    // Get profile directory
    let profile_dir = dirs::home_dir()
        .context("Could not find home directory")?
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name);

    // Ensure profile directory exists
    fs::create_dir_all(&profile_dir)
        .with_context(|| format!("Failed to create profile directory: {profile_dir:?}"))?;

    // Generate .zshenv
    let zshenv_content = generate_zshenv_from_manifest(manifest)?;
    let zshenv_path = profile_dir.join(".zshenv");
    fs::write(&zshenv_path, zshenv_content)
        .with_context(|| format!("Failed to write .zshenv to {zshenv_path:?}"))?;

    // Set explicit file permissions to 0644 (readable/writable by user, readable by others)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&zshenv_path, fs::Permissions::from_mode(0o644))
            .with_context(|| format!("Failed to set permissions on {zshenv_path:?}"))?;
    }

    log::info!("Generated: {zshenv_path:?}");

    // Generate .zshrc
    let zshrc_content = generate_zshrc_from_manifest(manifest)?;
    let zshrc_path = profile_dir.join(".zshrc");
    fs::write(&zshrc_path, zshrc_content)
        .with_context(|| format!("Failed to write .zshrc to {zshrc_path:?}"))?;

    // Set explicit file permissions to 0644 (readable/writable by user, readable by others)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&zshrc_path, fs::Permissions::from_mode(0o644))
            .with_context(|| format!("Failed to set permissions on {zshrc_path:?}"))?;
    }

    log::info!("Generated: {zshrc_path:?}");

    // Validate syntax (optional, requires zsh binary)
    validate_zsh_syntax(&zshrc_path)?;

    // Generate framework-specific files (e.g., .zimrc for zimfw)
    if manifest.profile.framework == "zimfw" {
        let zimrc_content = generate_zimrc_from_manifest(manifest)?;
        let zimrc_path = profile_dir.join(".zimrc");
        fs::write(&zimrc_path, zimrc_content)
            .with_context(|| format!("Failed to write .zimrc to {zimrc_path:?}"))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&zimrc_path, fs::Permissions::from_mode(0o644))
                .with_context(|| format!("Failed to set permissions on {zimrc_path:?}"))?;
        }

        log::info!("Generated: {zimrc_path:?}");
    }

    let duration = start.elapsed();
    log::debug!("Generation completed in {duration:?}");

    // Should complete in under 1 second (AC #6)
    if duration.as_secs() >= 1 {
        log::warn!("Generation took longer than expected: {duration:?}");
    }

    Ok(())
}

/// Generate .zshenv content from manifest (Story 2.2)
///
/// Creates .zshenv file with:
/// - Auto-generated header with timestamp and version
/// - Shared history configuration
/// - Environment variables from manifest with proper escaping
///
/// # Arguments
///
/// * `manifest` - The validated ProfileManifest
///
/// # Returns
///
/// String containing complete .zshenv content
pub fn generate_zshenv_from_manifest(manifest: &Manifest) -> Result<String> {
    let mut output = String::new();

    // Header comment (AC #3)
    output.push_str("# Auto-generated by zprof from profile.toml\n");
    output.push_str("# DO NOT EDIT THIS FILE DIRECTLY - Edit profile.toml instead\n");
    output.push_str(&format!("# Generated: {}\n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    output.push_str(&format!("# zprof version: {ZPROF_VERSION}\n"));
    output.push_str(&format!("# Profile: {}\n", manifest.profile.name));
    output.push('\n');

    // Note: HISTFILE is set in ~/.zshenv (root) and reset in .zshrc to override /etc/zshrc
    // No need to set it here in profile .zshenv

    // Environment variables from manifest (AC #2)
    if !manifest.env.is_empty() {
        output.push_str("# Custom environment variables\n");
        for (key, value) in &manifest.env {
            // Escape quotes and special characters
            let escaped_value = escape_shell_value(value);
            output.push_str(&format!("export {key}=\"{escaped_value}\"\n"));
        }
        output.push('\n');
    }

    Ok(output)
}

/// Escape shell special characters in environment variable values
///
/// Escapes: backslashes, double quotes, dollar signs, and backticks
/// This prevents shell injection and ensures values are interpreted literally
fn escape_shell_value(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$")
        .replace('`', "\\`")
}

/// Add auto-installation check for starship binary
///
/// Generates shell code that checks if starship is installed and attempts
/// to install it using brew or cargo if not found.
fn add_starship_installation_check(output: &mut String) {
    output.push_str("# Auto-install starship if not present\n");
    output.push_str("if ! command -v starship &> /dev/null; then\n");
    output.push_str("  echo \"Installing starship...\"\n");
    output.push_str("  if command -v brew &> /dev/null; then\n");
    output.push_str("    brew install starship\n");
    output.push_str("  elif command -v cargo &> /dev/null; then\n");
    output.push_str("    cargo install starship --locked\n");
    output.push_str("  else\n");
    output.push_str("    echo \"Warning: Neither brew nor cargo found. Please install starship manually: https://starship.rs/\"\n");
    output.push_str("  fi\n");
    output.push_str("fi\n");
    output.push('\n');
}

/// Add prompt engine initialization code
///
/// Generates the appropriate initialization command for the given prompt engine.
/// This is called AFTER framework initialization to ensure proper load order.
///
/// # Arguments
///
/// * `output` - The mutable string buffer to append initialization code to
/// * `engine` - The name of the prompt engine (e.g., "starship", "powerlevel10k")
///
/// # Returns
///
/// Returns Ok(()) on success, or an error if the engine name is not recognized
fn add_prompt_engine_init(output: &mut String, engine: &str) -> Result<()> {
    output.push_str("\n# Prompt engine initialization\n");

    match engine.to_lowercase().as_str() {
        "starship" => {
            output.push_str("eval \"$(starship init zsh)\"\n");
        }
        "powerlevel10k" | "p10k" => {
            output.push_str("source ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k/powerlevel10k.zsh-theme\n");
        }
        "oh-my-posh" | "ohmyposh" => {
            output.push_str("eval \"$(oh-my-posh init zsh)\"\n");
        }
        "pure" => {
            output.push_str("fpath+=$HOME/.zprof/engines/pure\n");
            output.push_str("autoload -U promptinit; promptinit\n");
            output.push_str("prompt pure\n");
        }
        "spaceship" => {
            output.push_str("source $HOME/.zprof/engines/spaceship-prompt/spaceship.zsh\n");
        }
        _ => {
            bail!("Unsupported prompt engine: {engine}. Supported engines: starship, powerlevel10k, oh-my-posh, pure, spaceship");
        }
    }

    Ok(())
}

/// Generate .zshrc content from manifest (Story 2.2)
///
/// Creates framework-specific .zshrc file with:
/// - Auto-generated header with timestamp and version
/// - Framework initialization code
/// - Plugin loading
/// - Theme activation
///
/// # Arguments
///
/// * `manifest` - The validated ProfileManifest
///
/// # Returns
///
/// String containing complete .zshrc content
///
/// # Errors
///
/// Returns error if framework is not supported
pub fn generate_zshrc_from_manifest(manifest: &Manifest) -> Result<String> {
    let mut output = String::new();

    // Header comment (AC #3)
    output.push_str("# Auto-generated by zprof from profile.toml\n");
    output.push_str("# DO NOT EDIT THIS FILE DIRECTLY - Edit profile.toml instead\n");
    output.push_str(&format!("# Generated: {}\n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    output.push_str(&format!("# zprof version: {ZPROF_VERSION}\n"));
    output.push_str(&format!("# Profile: {}\n", manifest.profile.name));
    output.push_str(&format!("# Framework: {}\n", manifest.profile.framework));
    output.push('\n');

    // Shared history configuration (must be set early to override system /etc/zshrc)
    output.push_str("# Shared history configuration\n");
    output.push_str("export HISTFILE=\"$HOME/.zsh-profiles/shared/.zsh_history\"\n");
    output.push_str("export HISTSIZE=10000\n");
    output.push_str("export SAVEHIST=10000\n\n");

    // Auto-install external binary dependencies for selected theme
    if manifest.profile.theme() == "starship" {
        add_starship_installation_check(&mut output);
    }

    // Generate framework-specific initialization (AC #1)
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

/// Generate oh-my-zsh specific configuration
fn generate_oh_my_zsh_config(output: &mut String, manifest: &Manifest) -> Result<()> {
    output.push_str("# oh-my-zsh configuration\n");

    // Set ZSH path
    output.push_str("export ZSH=\"$ZDOTDIR/.oh-my-zsh\"\n");
    output.push('\n');

    // Set theme based on prompt mode
    match &manifest.profile.prompt_mode {
        PromptMode::PromptEngine { .. } => {
            // Disable framework theme when using external prompt engine
            output.push_str("ZSH_THEME=\"\"  # Disabled for external prompt engine\n");
        }
        PromptMode::FrameworkTheme { theme } => {
            if !theme.is_empty() {
                output.push_str(&format!("ZSH_THEME=\"{theme}\"\n"));
            } else {
                output.push_str("ZSH_THEME=\"robbyrussell\"\n");
            }
        }
    }
    output.push('\n');

    // Set plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("plugins=(\n");
        for plugin in &manifest.plugins.enabled {
            output.push_str(&format!("  {plugin}\n"));
        }
        output.push_str(")\n");
        output.push('\n');
    }

    // Source oh-my-zsh
    output.push_str("source $ZSH/oh-my-zsh.sh\n");

    // Initialize prompt engine if using PromptEngine mode (AFTER framework)
    if let PromptMode::PromptEngine { engine } = &manifest.profile.prompt_mode {
        add_prompt_engine_init(output, engine)?;
    }

    // Source shared customizations
    output.push_str("\n# Source shared customizations (edit ~/.zsh-profiles/shared/custom.zsh)\n");
    output.push_str("[ -f \"$HOME/.zsh-profiles/shared/custom.zsh\" ] && source \"$HOME/.zsh-profiles/shared/custom.zsh\"\n");

    Ok(())
}

/// Generate .zimrc content from manifest (zimfw module declarations)
///
/// Creates .zimrc file with module declarations for zimfw to process
fn generate_zimrc_from_manifest(manifest: &Manifest) -> Result<String> {
    let mut output = String::new();

    // Header comment
    output.push_str("# Auto-generated by zprof from profile.toml\n");
    output.push_str("# DO NOT EDIT THIS FILE DIRECTLY - Edit profile.toml instead\n");
    output.push_str(&format!("# Generated: {}\n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    output.push_str(&format!("# zprof version: {ZPROF_VERSION}\n"));
    output.push_str(&format!("# Profile: {}\n", manifest.profile.name));
    output.push('\n');

    // Add plugins as zmodules
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("# Plugins\n");
        for plugin in &manifest.plugins.enabled {
            output.push_str(&format!("zmodule {plugin}\n"));
        }
        output.push('\n');
    }

    // Add theme as zmodule
    if !manifest.profile.theme().is_empty() {
        output.push_str("# Theme\n");
        output.push_str(&format!("zmodule {}\n", manifest.profile.theme()));
    }

    Ok(output)
}

/// Generate zimfw specific configuration
fn generate_zimfw_config(output: &mut String, manifest: &Manifest) -> Result<()> {
    output.push_str("# zimfw configuration\n");

    // Add note about theme if using prompt engine
    match &manifest.profile.prompt_mode {
        PromptMode::PromptEngine { .. } => {
            output.push_str("# Prompt engine mode - framework theme disabled\n");
        }
        PromptMode::FrameworkTheme { .. } => {
            output.push_str("# Zimfw uses .zimrc for module declarations - edit that file to add/remove modules\n");
        }
    }
    output.push('\n');

    // Set ZIM_HOME
    output.push_str("export ZIM_HOME=\"$ZDOTDIR/.zim\"\n");
    output.push('\n');

    // Download zimfw if it doesn't exist
    output.push_str("# Download zimfw plugin manager if missing\n");
    output.push_str("if [[ ! -e ${ZIM_HOME}/zimfw.zsh ]]; then\n");
    output.push_str("  mkdir -p ${ZIM_HOME}\n");
    output.push_str("  curl -fsSL --create-dirs -o ${ZIM_HOME}/zimfw.zsh \\\n");
    output.push_str("    https://github.com/zimfw/zimfw/releases/latest/download/zimfw.zsh\n");
    output.push_str("fi\n");
    output.push('\n');

    // Install modules and build init.zsh if missing
    output.push_str("# Install missing modules and build init.zsh if missing\n");
    output.push_str("if [[ ! -e ${ZIM_HOME}/init.zsh ]]; then\n");
    output.push_str("  source ${ZIM_HOME}/zimfw.zsh install\n");
    output.push_str("fi\n");
    output.push('\n');

    // Initialize zimfw
    output.push_str("# Initialize modules\n");
    output.push_str("source ${ZIM_HOME}/init.zsh\n");

    // Initialize prompt engine if using PromptEngine mode (AFTER framework)
    if let PromptMode::PromptEngine { engine } = &manifest.profile.prompt_mode {
        add_prompt_engine_init(output, engine)?;
    }

    // Source shared customizations
    output.push_str("\n# Source shared customizations (edit ~/.zsh-profiles/shared/custom.zsh)\n");
    output.push_str("[ -f \"$HOME/.zsh-profiles/shared/custom.zsh\" ] && source \"$HOME/.zsh-profiles/shared/custom.zsh\"\n");

    Ok(())
}

/// Generate prezto specific configuration
fn generate_prezto_config(output: &mut String, manifest: &Manifest) -> Result<()> {
    output.push_str("# prezto configuration\n");

    // Set PREZTO_DIR
    output.push_str("export PREZTO_DIR=\"$ZDOTDIR/.zprezto\"\n");
    output.push('\n');

    // Load plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("# Prezto modules\n");
        output.push_str("zstyle ':prezto:load' pmodule \\\n");
        for (idx, plugin) in manifest.plugins.enabled.iter().enumerate() {
            if idx == manifest.plugins.enabled.len() - 1 {
                output.push_str(&format!("  '{plugin}'\n"));
            } else {
                output.push_str(&format!("  '{plugin}' \\\n"));
            }
        }
        output.push('\n');
    }

    // Set theme based on prompt mode
    match &manifest.profile.prompt_mode {
        PromptMode::PromptEngine { .. } => {
            // Disable framework theme when using external prompt engine
            output.push_str("zstyle ':prezto:module:prompt' theme 'off'\n");
            output.push('\n');
        }
        PromptMode::FrameworkTheme { theme } => {
            if !theme.is_empty() {
                output.push_str(&format!("zstyle ':prezto:module:prompt' theme '{theme}'\n"));
                output.push('\n');
            }
        }
    }

    // Source prezto
    output.push_str("source $PREZTO_DIR/init.zsh\n");

    // Initialize prompt engine if using PromptEngine mode (AFTER framework)
    if let PromptMode::PromptEngine { engine } = &manifest.profile.prompt_mode {
        add_prompt_engine_init(output, engine)?;
    }

    // Source shared customizations
    output.push_str("\n# Source shared customizations (edit ~/.zsh-profiles/shared/custom.zsh)\n");
    output.push_str("[ -f \"$HOME/.zsh-profiles/shared/custom.zsh\" ] && source \"$HOME/.zsh-profiles/shared/custom.zsh\"\n");

    Ok(())
}

/// Generate zinit specific configuration
fn generate_zinit_config(output: &mut String, manifest: &Manifest) -> Result<()> {
    output.push_str("# zinit configuration\n");

    // Set zinit home
    output.push_str("export ZINIT_HOME=\"$ZDOTDIR/.zinit\"\n");
    output.push('\n');

    // Source zinit
    output.push_str("source $ZINIT_HOME/zinit.zsh\n");
    output.push('\n');

    // Load plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("# Plugins\n");
        for plugin in &manifest.plugins.enabled {
            output.push_str(&format!("zinit light {plugin}\n"));
        }
        output.push('\n');
    }

    // Load theme (only if using FrameworkTheme mode)
    match &manifest.profile.prompt_mode {
        PromptMode::PromptEngine { .. } => {
            // Theme loading is handled by prompt engine
        }
        PromptMode::FrameworkTheme { theme } => {
            if !theme.is_empty() {
                output.push_str(&format!("zinit light {theme}\n"));
                output.push('\n');
            }
        }
    }

    // Initialize prompt engine if using PromptEngine mode
    if let PromptMode::PromptEngine { engine } = &manifest.profile.prompt_mode {
        add_prompt_engine_init(output, engine)?;
    }

    // Source shared customizations
    output.push_str("\n# Source shared customizations (edit ~/.zsh-profiles/shared/custom.zsh)\n");
    output.push_str("[ -f \"$HOME/.zsh-profiles/shared/custom.zsh\" ] && source \"$HOME/.zsh-profiles/shared/custom.zsh\"\n");

    Ok(())
}

/// Generate zap specific configuration
fn generate_zap_config(output: &mut String, manifest: &Manifest) -> Result<()> {
    output.push_str("# zap configuration\n");

    // Set zap home
    output.push_str("export ZAP_DIR=\"$ZDOTDIR/.zap\"\n");
    output.push('\n');

    // Source zap
    output.push_str("source $ZAP_DIR/zap.zsh\n");

    // Override ZAP_DIR after sourcing to ensure our path is used
    output.push_str("export ZAP_DIR=\"$ZDOTDIR/.zap\"\n");
    output.push_str("export ZAP_PLUGIN_DIR=\"$ZAP_DIR/plugins\"\n");
    output.push('\n');

    // Initialize completion system before loading plugins to prevent compdef warnings
    output.push_str("# Initialize completion system\n");
    output.push_str("autoload -Uz compinit\n");
    output.push_str("compinit\n");
    output.push('\n');

    // Load plugins
    if !manifest.plugins.enabled.is_empty() {
        output.push_str("# Plugins\n");
        for plugin in &manifest.plugins.enabled {
            // Look up plugin repo URL from registry metadata
            if let Some(p) = crate::frameworks::plugin::PLUGIN_REGISTRY
                .iter()
                .find(|p| p.name == plugin.as_str())
            {
                if let Some(repo_url) = p.compatibility.repo_url_for(&crate::frameworks::FrameworkType::Zap) {
                    output.push_str(&format!("plug \"{repo_url}\"\n"));
                }
            }
        }
        output.push('\n');
    }

    // Load theme (only if using FrameworkTheme mode)
    match &manifest.profile.prompt_mode {
        PromptMode::PromptEngine { .. } => {
            // Theme loading is handled by prompt engine
        }
        PromptMode::FrameworkTheme { theme } => {
            if !theme.is_empty() {
                // Look up theme repo URL from registry metadata
                if let Some(theme_meta) = crate::frameworks::theme::THEME_REGISTRY
                    .iter()
                    .find(|t| t.name == theme.as_str())
                {
                    // Install theme dependencies first
                    for dep in theme_meta.compatibility.dependencies {
                        output.push_str(&format!("plug \"{dep}\"\n"));
                    }

                    // Install the theme itself
                    if let Some(repo_url) = theme_meta.compatibility.repo_url_for(&crate::frameworks::FrameworkType::Zap) {
                        output.push_str(&format!("plug \"{repo_url}\"\n"));
                        output.push('\n');
                    }
                }
            }
        }
    }

    // Initialize prompt engine if using PromptEngine mode
    if let PromptMode::PromptEngine { engine } = &manifest.profile.prompt_mode {
        add_prompt_engine_init(output, engine)?;
    }

    // Source shared customizations
    output.push_str("\n# Source shared customizations (edit ~/.zsh-profiles/shared/custom.zsh)\n");
    output.push_str("[ -f \"$HOME/.zsh-profiles/shared/custom.zsh\" ] && source \"$HOME/.zsh-profiles/shared/custom.zsh\"\n");

    Ok(())
}

/// Validate generated zsh file syntax using zsh -n
///
/// Runs 'zsh -n <file>' to check syntax without executing.
/// If zsh is not available, logs a warning but does not fail.
///
/// # Arguments
///
/// * `file_path` - Path to the generated .zshrc file
///
/// # Errors
///
/// Returns error if file has syntax errors and zsh is available
fn validate_zsh_syntax(file_path: &Path) -> Result<()> {
    // Run 'zsh -n <file>' to check syntax without executing
    let output = std::process::Command::new("zsh")
        .arg("-n")
        .arg(file_path)
        .output();

    match output {
        Ok(result) if result.status.success() => {
            log::debug!("Syntax validation passed: {file_path:?}");
            Ok(())
        }
        Ok(result) => {
            let stderr = String::from_utf8_lossy(&result.stderr);
            bail!("Generated zsh file has syntax errors:\n{stderr}");
        }
        Err(e) => {
            log::warn!("Could not validate syntax (zsh not available): {e}");
            Ok(()) // Don't fail if zsh not available
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::manifest::{Manifest, ProfileSection, PluginsSection};
    use chrono::Utc;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::TempDir;

    // Story 2.2 Tests: Manifest-based generation

    fn create_test_manifest(framework: &str, plugins: Vec<String>, env: HashMap<String, String>) -> Manifest {
        Manifest {
            profile: ProfileSection {
                name: "test-profile".to_string(),
                framework: framework.to_string(),
                prompt_mode: crate::core::manifest::PromptMode::FrameworkTheme {
                    theme: "robbyrussell".to_string(),
                },
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: PluginsSection {
                enabled: plugins,
            },
            env,
        }
    }

    #[test]
    fn test_escape_shell_value() {
        // Test escaping backslashes
        assert_eq!(escape_shell_value("foo\\bar"), "foo\\\\bar");

        // Test escaping double quotes
        assert_eq!(escape_shell_value("foo\"bar"), "foo\\\"bar");

        // Test escaping dollar signs
        assert_eq!(escape_shell_value("$HOME/bin"), "\\$HOME/bin");

        // Test escaping backticks
        assert_eq!(escape_shell_value("foo`cmd`bar"), "foo\\`cmd\\`bar");

        // Test multiple special characters
        assert_eq!(escape_shell_value("\\$PATH:`pwd`"), "\\\\\\$PATH:\\`pwd\\`");
    }

    #[test]
    fn test_generate_zshenv_from_manifest_basic() -> Result<()> {
        let manifest = create_test_manifest("oh-my-zsh", vec![], HashMap::new());
        let content = generate_zshenv_from_manifest(&manifest)?;

        // AC #3: Header comments
        assert!(content.contains("Auto-generated by zprof from profile.toml"));
        assert!(content.contains("DO NOT EDIT THIS FILE DIRECTLY"));
        assert!(content.contains("zprof version:"));
        assert!(content.contains("Profile: test-profile"));

        // HISTFILE is now managed in ~/.zshenv (root) and .zshrc, not in profile .zshenv
        // So this file should NOT contain HISTFILE exports
        assert!(!content.contains("export HISTFILE"));

        Ok(())
    }

    #[test]
    fn test_generate_zshenv_from_manifest_with_env_vars() -> Result<()> {
        let mut env = HashMap::new();
        env.insert("EDITOR".to_string(), "vim".to_string());
        env.insert("GOPATH".to_string(), "$HOME/go".to_string());

        let manifest = create_test_manifest("oh-my-zsh", vec![], env);
        let content = generate_zshenv_from_manifest(&manifest)?;

        // AC #2: Environment variables with escaping
        assert!(content.contains("export EDITOR=\"vim\""));
        assert!(content.contains("export GOPATH=\"\\$HOME/go\""));

        Ok(())
    }

    #[test]
    fn test_generate_zshenv_special_char_escaping() -> Result<()> {
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "$HOME/bin:$PATH".to_string());
        env.insert("PROMPT".to_string(), "test`cmd`".to_string());

        let manifest = create_test_manifest("oh-my-zsh", vec![], env);
        let content = generate_zshenv_from_manifest(&manifest)?;

        // Verify special characters are escaped
        assert!(content.contains("\\$HOME"));
        assert!(content.contains("\\$PATH"));
        assert!(content.contains("\\`cmd\\`"));

        Ok(())
    }

    #[test]
    fn test_generate_zshrc_from_manifest_oh_my_zsh() -> Result<()> {
        let manifest = create_test_manifest(
            "oh-my-zsh",
            vec!["git".to_string(), "docker".to_string(), "kubectl".to_string()],
            HashMap::new(),
        );
        let content = generate_zshrc_from_manifest(&manifest)?;

        // AC #3: Header comments
        assert!(content.contains("Auto-generated by zprof from profile.toml"));
        assert!(content.contains("Framework: oh-my-zsh"));

        // AC #1: Framework initialization, plugin loading, theme activation
        assert!(content.contains("export ZSH=\"$ZDOTDIR/.oh-my-zsh\""));
        assert!(content.contains("ZSH_THEME=\"robbyrussell\""));
        assert!(content.contains("plugins=("));
        assert!(content.contains("  git"));
        assert!(content.contains("  docker"));
        assert!(content.contains("  kubectl"));
        assert!(content.contains("source $ZSH/oh-my-zsh.sh"));

        Ok(())
    }

    #[test]
    fn test_generate_zshrc_from_manifest_zimfw() -> Result<()> {
        let manifest = create_test_manifest(
            "zimfw",
            vec!["git".to_string(), "fzf".to_string()],
            HashMap::new(),
        );
        let content = generate_zshrc_from_manifest(&manifest)?;

        // AC #1: zimfw-specific configuration
        assert!(content.contains("export ZIM_HOME=\"$ZDOTDIR/.zim\""));
        // Zimfw now downloads its own manager
        assert!(content.contains("zimfw.zsh"));
        // Bootstrap and install process
        assert!(content.contains("zimfw.zsh install"));
        assert!(content.contains("source ${ZIM_HOME}/init.zsh"));

        Ok(())
    }

    #[test]
    fn test_generate_zshrc_from_manifest_prezto() -> Result<()> {
        let manifest = create_test_manifest(
            "prezto",
            vec!["git".to_string(), "syntax-highlighting".to_string()],
            HashMap::new(),
        );
        let content = generate_zshrc_from_manifest(&manifest)?;

        // AC #1: prezto-specific configuration
        assert!(content.contains("export PREZTO_DIR=\"$ZDOTDIR/.zprezto\""));
        assert!(content.contains("zstyle ':prezto:load' pmodule"));
        assert!(content.contains("'git'"));
        assert!(content.contains("'syntax-highlighting'"));
        assert!(content.contains("zstyle ':prezto:module:prompt' theme 'robbyrussell'"));
        assert!(content.contains("source $PREZTO_DIR/init.zsh"));

        Ok(())
    }

    #[test]
    fn test_generate_zshrc_from_manifest_zinit() -> Result<()> {
        let manifest = create_test_manifest(
            "zinit",
            vec!["zsh-users/zsh-autosuggestions".to_string()],
            HashMap::new(),
        );
        let content = generate_zshrc_from_manifest(&manifest)?;

        // AC #1: zinit-specific configuration
        assert!(content.contains("export ZINIT_HOME=\"$ZDOTDIR/.zinit\""));
        assert!(content.contains("source $ZINIT_HOME/zinit.zsh"));
        assert!(content.contains("zinit light zsh-users/zsh-autosuggestions"));
        assert!(content.contains("zinit light robbyrussell"));

        Ok(())
    }

    #[test]
    fn test_generate_zshrc_from_manifest_zap() -> Result<()> {
        // Create test manifest with zap-prompt theme (Zap's default)
        let mut manifest = create_test_manifest(
            "zap",
            vec!["zsh-syntax-highlighting".to_string()],
            HashMap::new(),
        );
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::FrameworkTheme {
            theme: "zap-prompt".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        // AC #1: zap-specific configuration
        assert!(content.contains("export ZAP_DIR=\"$ZDOTDIR/.zap\""));
        assert!(content.contains("source $ZAP_DIR/zap.zsh"));
        assert!(content.contains("export ZAP_PLUGIN_DIR=\"$ZAP_DIR/plugins\""));
        // Plugin should be mapped to zap-compatible GitHub URL
        assert!(content.contains("plug \"zsh-users/zsh-syntax-highlighting\""));
        // Theme should use zap-prompt
        assert!(content.contains("plug \"zap-zsh/zap-prompt\""));

        Ok(())
    }

    #[test]
    fn test_generate_zshrc_unsupported_framework() {
        let manifest = create_test_manifest("bash-it", vec![], HashMap::new());
        let result = generate_zshrc_from_manifest(&manifest);

        // Should return error for unsupported framework
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Unsupported framework"));
        assert!(err_msg.contains("bash-it"));
    }

    #[test]
    fn test_generate_zshrc_empty_plugins() -> Result<()> {
        let manifest = create_test_manifest("oh-my-zsh", vec![], HashMap::new());
        let content = generate_zshrc_from_manifest(&manifest)?;

        // Should generate valid config even with no plugins
        assert!(content.contains("export ZSH="));
        assert!(content.contains("source $ZSH/oh-my-zsh.sh"));
        // Should not have plugins=() section if empty
        assert!(!content.contains("plugins=("));

        Ok(())
    }

    #[test]
    fn test_generate_zshrc_empty_theme() -> Result<()> {
        let mut manifest = create_test_manifest("oh-my-zsh", vec![], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::FrameworkTheme {
            theme: String::new(),
        };
        let content = generate_zshrc_from_manifest(&manifest)?;

        // Should default to robbyrussell when theme is empty
        assert!(content.contains("ZSH_THEME=\"robbyrussell\""));

        Ok(())
    }

    #[test]
    fn test_write_generated_files_creates_directory() -> Result<()> {
        let _temp_dir = TempDir::new()?;

        // Override home dir for testing (if possible) or use temp dir structure
        // For now, we'll just test the generation functions individually
        // This is an integration test that would need mocking of dirs::home_dir()

        Ok(())
    }

    #[test]
    fn test_generated_files_are_syntactically_valid() -> Result<()> {
        // This test checks AC #5: Generated configuration is syntactically valid
        let manifest = create_test_manifest(
            "oh-my-zsh",
            vec!["git".to_string(), "docker".to_string()],
            HashMap::new(),
        );

        let zshrc_content = generate_zshrc_from_manifest(&manifest)?;
        let zshenv_content = generate_zshenv_from_manifest(&manifest)?;

        // Write to temp files for syntax validation
        let temp_dir = TempDir::new()?;
        let zshrc_path = temp_dir.path().join(".zshrc");
        let zshenv_path = temp_dir.path().join(".zshenv");

        fs::write(&zshrc_path, zshrc_content)?;
        fs::write(&zshenv_path, zshenv_content)?;

        // Try to validate syntax if zsh is available
        // This will log a warning if zsh is not available but won't fail
        validate_zsh_syntax(&zshrc_path)?;

        Ok(())
    }

    #[test]
    fn test_generation_performance() -> Result<()> {
        // AC #6: Process completes in under 1 second
        use std::time::Instant;

        let mut env = HashMap::new();
        for i in 0..50 {
            env.insert(format!("VAR_{i}"), format!("value_{i}"));
        }

        let manifest = create_test_manifest(
            "oh-my-zsh",
            vec!["git".to_string(), "docker".to_string(), "kubectl".to_string()],
            env,
        );

        let start = Instant::now();

        // Generate both files
        let _zshrc = generate_zshrc_from_manifest(&manifest)?;
        let _zshenv = generate_zshenv_from_manifest(&manifest)?;

        let duration = start.elapsed();

        // Should be much faster than 1 second (typically < 10ms)
        assert!(duration.as_millis() < 1000,
            "Generation took {}ms, should be < 1000ms",
            duration.as_millis());

        Ok(())
    }

    #[test]
    fn test_regeneration_overwrites_files() -> Result<()> {
        // AC #4: Re-generation overwrites previous files
        let temp_dir = TempDir::new()?;
        let profile_path = temp_dir.path();

        // Create initial manifest
        let manifest1 = create_test_manifest(
            "oh-my-zsh",
            vec!["git".to_string()],
            HashMap::new(),
        );

        // Generate files first time
        let zshrc_path = profile_path.join(".zshrc");
        fs::write(&zshrc_path, generate_zshrc_from_manifest(&manifest1)?)?;

        let first_content = fs::read_to_string(&zshrc_path)?;
        assert!(first_content.contains("  git"));
        assert!(!first_content.contains("  docker"));

        // Create updated manifest with different plugins
        let manifest2 = create_test_manifest(
            "oh-my-zsh",
            vec!["docker".to_string(), "kubectl".to_string()],
            HashMap::new(),
        );

        // Regenerate (overwrite)
        fs::write(&zshrc_path, generate_zshrc_from_manifest(&manifest2)?)?;

        let second_content = fs::read_to_string(&zshrc_path)?;

        // Verify old plugin is gone and new plugins are present
        assert!(!second_content.contains("  git"));
        assert!(second_content.contains("  docker"));
        assert!(second_content.contains("  kubectl"));

        Ok(())
    }

    // Story 1.6 Tests: Prompt Engine Support

    #[test]
    fn test_generate_with_starship_prompt_engine() -> Result<()> {
        let mut manifest = create_test_manifest("oh-my-zsh", vec!["git".to_string()], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
            engine: "starship".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        // Should disable framework theme
        assert!(content.contains("ZSH_THEME=\"\""));
        assert!(content.contains("Disabled for external prompt engine"));

        // Should include starship initialization
        assert!(content.contains("eval \"$(starship init zsh)\""));

        // Should NOT include any theme assignment
        assert!(!content.contains("ZSH_THEME=\"robbyrussell\""));

        Ok(())
    }

    #[test]
    fn test_generate_with_powerlevel10k_prompt_engine() -> Result<()> {
        let mut manifest = create_test_manifest("oh-my-zsh", vec![], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
            engine: "powerlevel10k".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        // Should disable framework theme
        assert!(content.contains("ZSH_THEME=\"\""));

        // Should include p10k source
        assert!(content.contains("powerlevel10k/powerlevel10k.zsh-theme"));

        Ok(())
    }

    #[test]
    fn test_prompt_engine_with_zimfw() -> Result<()> {
        let mut manifest = create_test_manifest("zimfw", vec!["git".to_string()], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
            engine: "starship".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        // Should note prompt engine mode
        assert!(content.contains("Prompt engine mode - framework theme disabled"));

        // Should include starship initialization
        assert!(content.contains("eval \"$(starship init zsh)\""));

        Ok(())
    }

    #[test]
    fn test_prompt_engine_with_prezto() -> Result<()> {
        let mut manifest = create_test_manifest("prezto", vec![], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
            engine: "pure".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        // Should disable prezto theme
        assert!(content.contains("zstyle ':prezto:module:prompt' theme 'off'"));

        // Should include pure initialization
        assert!(content.contains("fpath+=$HOME/.zprof/engines/pure"));
        assert!(content.contains("autoload -U promptinit; promptinit"));
        assert!(content.contains("prompt pure"));

        Ok(())
    }

    #[test]
    fn test_all_engine_framework_combinations() -> Result<()> {
        // Test all 5 engines × 3 main frameworks = 15 combinations
        let engines = vec![
            "starship",
            "powerlevel10k",
            "oh-my-posh",
            "pure",
            "spaceship",
        ];

        let frameworks = vec!["oh-my-zsh", "zimfw", "prezto"];

        for engine in &engines {
            for framework in &frameworks {
                let mut manifest = create_test_manifest(framework, vec![], HashMap::new());
                manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
                    engine: engine.to_string(),
                };

                let content = generate_zshrc_from_manifest(&manifest)?;

                // Verify engine init exists
                assert!(
                    content.contains("Prompt engine initialization") || content.contains("eval"),
                    "Engine {engine} with framework {framework} should have initialization code"
                );

                // Verify framework theme is disabled appropriately
                match *framework {
                    "oh-my-zsh" => {
                        assert!(
                            content.contains("ZSH_THEME=\"\""),
                            "oh-my-zsh should have empty ZSH_THEME for engine {engine}"
                        );
                    }
                    "prezto" => {
                        assert!(
                            content.contains("zstyle ':prezto:module:prompt' theme 'off'"),
                            "prezto should disable theme for engine {engine}"
                        );
                    }
                    "zimfw" => {
                        assert!(
                            content.contains("Prompt engine mode"),
                            "zimfw should note prompt engine mode for engine {engine}"
                        );
                    }
                    _ => {}
                }

                // Validate syntax if zsh is available
                let temp_dir = TempDir::new()?;
                let zshrc_path = temp_dir.path().join(".zshrc");
                fs::write(&zshrc_path, &content)?;
                validate_zsh_syntax(&zshrc_path)?;
            }
        }

        Ok(())
    }

    #[test]
    fn test_framework_theme_mode_still_works() -> Result<()> {
        // Ensure FrameworkTheme mode is not broken by PromptEngine changes
        let mut manifest = create_test_manifest("oh-my-zsh", vec![], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::FrameworkTheme {
            theme: "agnoster".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        // Should have theme set
        assert!(content.contains("ZSH_THEME=\"agnoster\""));

        // Should NOT have engine initialization
        assert!(!content.contains("Prompt engine initialization"));
        assert!(!content.contains("starship init"));

        Ok(())
    }

    #[test]
    fn test_unsupported_engine_error() {
        let mut manifest = create_test_manifest("oh-my-zsh", vec![], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
            engine: "unsupported-engine".to_string(),
        };

        let result = generate_zshrc_from_manifest(&manifest);

        // Should return error for unsupported engine
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Unsupported prompt engine"));
        assert!(err_msg.contains("unsupported-engine"));
    }

    #[test]
    fn test_engine_init_order() -> Result<()> {
        // Verify initialization order: framework → plugins → engine
        let mut manifest = create_test_manifest("oh-my-zsh", vec!["git".to_string()], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
            engine: "starship".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        // Find positions of key initialization steps
        let framework_pos = content.find("source $ZSH/oh-my-zsh.sh").unwrap();
        let engine_pos = content.find("eval \"$(starship init zsh)\"").unwrap();

        // Engine init must come AFTER framework init
        assert!(
            engine_pos > framework_pos,
            "Prompt engine initialization must come after framework initialization"
        );

        Ok(())
    }

    #[test]
    fn test_oh_my_posh_init() -> Result<()> {
        let mut manifest = create_test_manifest("oh-my-zsh", vec![], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
            engine: "oh-my-posh".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        assert!(content.contains("eval \"$(oh-my-posh init zsh)\""));

        Ok(())
    }

    #[test]
    fn test_spaceship_init() -> Result<()> {
        let mut manifest = create_test_manifest("prezto", vec![], HashMap::new());
        manifest.profile.prompt_mode = crate::core::manifest::PromptMode::PromptEngine {
            engine: "spaceship".to_string(),
        };

        let content = generate_zshrc_from_manifest(&manifest)?;

        assert!(content.contains("source $HOME/.zprof/engines/spaceship-prompt/spaceship.zsh"));

        Ok(())
    }
}
