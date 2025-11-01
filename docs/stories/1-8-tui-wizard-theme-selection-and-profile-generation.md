# Story 1.8: TUI Wizard Theme Selection and Profile Generation

Status: ready-for-dev

## Story

As a developer,
I want to select a theme and finalize my profile creation,
so that I have a complete, working zsh profile.

## Acceptance Criteria

1. TUI displays available themes for selected framework with previews/descriptions
2. Single-select interface for choosing one theme
3. Final confirmation screen shows all selections (framework, plugins, theme)
4. On confirmation, system installs selected framework and plugins
5. Generates profile.toml manifest with all selections
6. Creates functional .zshrc and .zshenv in profile directory
7. Installation progress is displayed with clear status messages
8. Success message confirms profile is ready to use

## Tasks / Subtasks

- [ ] Define theme data model for all frameworks (AC: #1)
  - [ ] Create Theme struct with name, description, preview_example (color scheme info)
  - [ ] Define 10-15 popular themes per framework
  - [ ] Store theme definitions in `frameworks/*.rs` per framework
  - [ ] Create get_themes() method for each framework returning Vec<Theme>
  - [ ] Include default/minimal themes for each framework
- [ ] Implement theme selection TUI screen (AC: #1, #2)
  - [ ] Create `tui/theme_select.rs` module
  - [ ] Implement `run_theme_selection(framework: FrameworkType, plugins: &[String]) -> Result<String>` function
  - [ ] Use Ratatui List widget for single-select (similar to Story 1.6)
  - [ ] Display theme name and description for each theme (AC: #1)
  - [ ] Show visual indicator for selected theme (AC: #2)
  - [ ] Arrow key navigation (up/down)
  - [ ] Return selected theme name on Enter key press
  - [ ] Support Esc to cancel and abort wizard
- [ ] Implement confirmation screen (AC: #3)
  - [ ] Create summary view showing all wizard selections:
    - Profile name
    - Selected framework
    - Selected plugins (count + list)
    - Selected theme
  - [ ] Display confirmation prompt: "Create profile with these settings? (y/n)"
  - [ ] Allow editing by returning to previous steps (optional for MVP - can defer)
  - [ ] Proceed to installation on 'y', abort on 'n' or Esc
- [ ] Implement framework and plugin installation (AC: #4, #7)
  - [ ] Create `frameworks/installer.rs` module for installation orchestration
  - [ ] Implement install_framework() function per framework type
  - [ ] Download and install framework to profile directory
  - [ ] Install selected plugins for the framework
  - [ ] Use indicatif progress bars for installation steps (AC: #7)
  - [ ] Show clear status: "Installing oh-my-zsh...", "Installing plugin: git...", etc.
  - [ ] Handle installation failures gracefully (cleanup, error messages)
  - [ ] Log installation steps with env_logger
- [ ] Generate TOML manifest (AC: #5)
  - [ ] Create `core/manifest.rs` module (or use existing from architecture)
  - [ ] Define ProfileManifest struct matching TOML schema from architecture
  - [ ] Implement generate_manifest() function taking WizardState
  - [ ] Populate manifest with framework, plugins, theme, timestamps
  - [ ] Write manifest to `<profile_dir>/profile.toml`
  - [ ] Use serde + toml crate for serialization (per architecture)
  - [ ] Validate generated TOML is well-formed
- [ ] Generate shell configuration files (AC: #6)
  - [ ] Create `shell/generator.rs` module
  - [ ] Implement generate_zshrc() function using manifest data
  - [ ] Generate .zshrc with framework initialization, plugin loading, theme activation
  - [ ] Implement generate_zshenv() function for environment variables
  - [ ] Include header comments indicating auto-generated from manifest
  - [ ] Write files to profile directory
  - [ ] Ensure generated files are syntactically valid zsh
  - [ ] Handle framework-specific configuration differences
- [ ] Complete wizard integration and success messaging (AC: #8)
  - [ ] Update `cli/create.rs` to orchestrate full wizard flow (Stories 1.6, 1.7, 1.8)
  - [ ] Handle wizard completion and profile directory creation
  - [ ] Display success message with profile details (AC: #8)
  - [ ] Update active profile in `config.toml` if this is first profile
  - [ ] Provide next steps: "Run 'zprof use <name>' to activate profile"
- [ ] Handle edge cases and errors (AC: All)
  - [ ] Handle framework with no themes defined (use default theme)
  - [ ] Restore terminal state on errors/cancellation
  - [ ] Cleanup partial installations on failure (use Pattern 3 - Safe File Operations)
  - [ ] Handle network failures during framework downloads
  - [ ] Handle disk space issues
  - [ ] Use anyhow::Context for user-friendly error messages
  - [ ] Log all operations for troubleshooting
- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test Theme data structure
  - [ ] Unit test keyboard event handling for theme selection
  - [ ] Unit test manifest generation from WizardState
  - [ ] Unit test .zshrc and .zshenv generation
  - [ ] Integration test full wizard flow (framework -> plugins -> theme -> profile created)
  - [ ] Test installation process (may require mocking downloads)
  - [ ] Test error handling and cleanup on failures
  - [ ] Test confirmation screen displays correct selections
  - [ ] Manual test full wizard with all 5 frameworks
  - [ ] Verify generated profiles are functional (load in zsh)

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `tui/theme_select.rs`, `shell/generator.rs`, `core/manifest.rs`, `frameworks/installer.rs`
- Secondary: `cli/create.rs` (wizard orchestration), `frameworks/*.rs` (theme definitions)
- Reuse: Terminal setup from Story 1.6, TUI patterns from Stories 1.6 and 1.7
- Follow Pattern 2 (Error Handling) with anyhow::Result
- Follow Pattern 3 (Safe File Operations) for profile creation
- Use Pattern 4 (TOML Manifest Schema) for profile.toml generation

**Theme Data Model:**
```rust
// frameworks/mod.rs
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub preview: String,  // Visual description of color scheme
}

pub trait Framework {
    // ... existing methods ...
    fn get_themes(&self) -> Vec<Theme>;
}
```

**Example Theme Definitions (oh-my-zsh):**
```rust
// frameworks/oh_my_zsh.rs
impl Framework for OhMyZsh {
    fn get_themes(&self) -> Vec<Theme> {
        vec![
            Theme {
                name: "robbyrussell".to_string(),
                description: "Default oh-my-zsh theme, minimal and fast".to_string(),
                preview: "➜ user@host:~/dir (git:main)".to_string(),
            },
            Theme {
                name: "agnoster".to_string(),
                description: "Powerline-style theme with git info".to_string(),
                preview: "Powerline arrows, git status, dark background".to_string(),
            },
            Theme {
                name: "powerlevel10k".to_string(),
                description: "Fast, customizable, feature-rich theme".to_string(),
                preview: "Highly customizable, extensive git integration".to_string(),
            },
            Theme {
                name: "pure".to_string(),
                description: "Minimal, fast, asynchronous prompt".to_string(),
                preview: "Clean single-line prompt with git status".to_string(),
            },
            Theme {
                name: "spaceship".to_string(),
                description: "Minimalist, powerful prompt for astronauts".to_string(),
                preview: "🚀 Modern icons, git info, execution time".to_string(),
            },
            Theme {
                name: "bullet-train".to_string(),
                description: "Powerline-inspired with customizable cars".to_string(),
                preview: "Modular sections, colorful, git integration".to_string(),
            },
            Theme {
                name: "af-magic".to_string(),
                description: "Two-line theme with timestamp and git".to_string(),
                preview: "Timestamp, path, git status on separate lines".to_string(),
            },
            Theme {
                name: "bira".to_string(),
                description: "Two-line theme with user and time".to_string(),
                preview: "User@host, time, path, git on two lines".to_string(),
            },
            Theme {
                name: "cloud".to_string(),
                description: "Minimalist cloud-inspired theme".to_string(),
                preview: "☁️ Simple, clean, hostname and path".to_string(),
            },
            Theme {
                name: "avit".to_string(),
                description: "Clean theme with git status colors".to_string(),
                preview: "Colored git indicators, clean layout".to_string(),
            },
        ]
    }
}
```

**Theme Selection TUI Pattern:**
```rust
// tui/theme_select.rs
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

pub fn run_theme_selection(framework: FrameworkType, _plugins: &[String]) -> Result<String> {
    let mut terminal = setup_terminal()?;
    let result = run_theme_loop(&mut terminal, framework);
    restore_terminal()?;
    result
}

fn run_theme_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>, framework: FrameworkType) -> Result<String> {
    let themes = get_themes_for_framework(framework);
    let mut state = ListState::default();
    state.select(Some(0)); // Default to first theme

    loop {
        terminal.draw(|f| {
            render_theme_list(f, &themes, &state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => select_previous(&mut state, themes.len()),
                KeyCode::Down => select_next(&mut state, themes.len()),
                KeyCode::Enter => return Ok(get_selected_theme(&themes, &state)),
                KeyCode::Esc => bail!("Theme selection cancelled"),
                _ => {}
            }
        }
    }
}
```

**Confirmation Screen Pattern:**
```rust
// tui/theme_select.rs or wizard.rs
fn show_confirmation_screen(state: &WizardState) -> Result<bool> {
    let mut terminal = setup_terminal()?;

    loop {
        terminal.draw(|f| {
            render_confirmation(f, state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    restore_terminal()?;
                    return Ok(true);
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    restore_terminal()?;
                    return Ok(false);
                }
                _ => {}
            }
        }
    }
}
```

**Confirmation Screen Layout:**
```
┌─ Confirm Profile Creation ─────────────────────────────────────┐
│                                                                 │
│  Profile Name:    experimental                                 │
│  Framework:       zimfw                                        │
│  Plugins (3):     git, docker, fzf                             │
│  Theme:           pure                                         │
│                                                                 │
│  This will:                                                    │
│    • Install zimfw to ~/.zsh-profiles/profiles/experimental/   │
│    • Install 3 selected plugins                                │
│    • Generate profile.toml manifest                            │
│    • Generate .zshrc and .zshenv                               │
│                                                                 │
│  Create profile with these settings?                           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
 y: Confirm | n/Esc: Cancel
```

**Installation Progress Pattern:**
```rust
// frameworks/installer.rs
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;

pub fn install_profile(wizard_state: &WizardState, profile_path: &Path) -> Result<()> {
    let total_steps = 3 + wizard_state.plugins.len();
    let pb = ProgressBar::new(total_steps as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40}] {pos}/{len} {msg}")
            .unwrap()
    );

    // Step 1: Install framework
    pb.set_message(format!("Installing {}...", wizard_state.framework));
    install_framework(&wizard_state.framework, profile_path)?;
    pb.inc(1);

    // Step 2: Install plugins
    for plugin in &wizard_state.plugins {
        pb.set_message(format!("Installing plugin: {}...", plugin));
        install_plugin(&wizard_state.framework, plugin, profile_path)?;
        pb.inc(1);
    }

    // Step 3: Generate manifest
    pb.set_message("Generating profile.toml...");
    generate_manifest(wizard_state, profile_path)?;
    pb.inc(1);

    // Step 4: Generate shell configs
    pb.set_message("Generating .zshrc and .zshenv...");
    generate_shell_configs(wizard_state, profile_path)?;
    pb.inc(1);

    pb.finish_with_message("Profile created successfully!");
    Ok(())
}
```

**Manifest Generation Pattern (Pattern 4):**
```rust
// core/manifest.rs
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileManifest {
    pub profile: ProfileInfo,
    pub plugins: PluginsConfig,
    pub env: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub name: String,
    pub framework: String,
    pub theme: String,
    pub created: String,  // ISO 8601 timestamp
    pub modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginsConfig {
    pub enabled: Vec<String>,
}

pub fn generate_manifest(wizard_state: &WizardState, profile_path: &Path) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();

    let manifest = ProfileManifest {
        profile: ProfileInfo {
            name: wizard_state.profile_name.clone(),
            framework: wizard_state.framework.to_string(),
            theme: wizard_state.theme.clone(),
            created: now.clone(),
            modified: now,
        },
        plugins: PluginsConfig {
            enabled: wizard_state.plugins.clone(),
        },
        env: std::collections::HashMap::new(), // Empty for now
    };

    let toml_string = toml::to_string_pretty(&manifest)
        .context("Failed to serialize manifest to TOML")?;

    let manifest_path = profile_path.join("profile.toml");
    std::fs::write(&manifest_path, toml_string)
        .context(format!("Failed to write manifest to {:?}", manifest_path))?;

    Ok(())
}
```

**Shell Config Generation Pattern:**
```rust
// shell/generator.rs
use anyhow::Result;

pub fn generate_shell_configs(wizard_state: &WizardState, profile_path: &Path) -> Result<()> {
    generate_zshrc(wizard_state, profile_path)?;
    generate_zshenv(wizard_state, profile_path)?;
    Ok(())
}

fn generate_zshrc(wizard_state: &WizardState, profile_path: &Path) -> Result<()> {
    let mut content = String::new();

    // Header comment
    content.push_str("# Auto-generated by zprof from profile.toml\n");
    content.push_str("# DO NOT EDIT - Changes will be overwritten\n");
    content.push_str("# Edit profile.toml and run 'zprof edit <profile>' instead\n\n");

    // Framework initialization (framework-specific)
    match wizard_state.framework {
        FrameworkType::OhMyZsh => {
            content.push_str(&format!("export ZSH=\"$ZDOTDIR/.oh-my-zsh\"\n"));
            content.push_str(&format!("ZSH_THEME=\"{}\"\n", wizard_state.theme));
            content.push_str(&format!("plugins=({})\n", wizard_state.plugins.join(" ")));
            content.push_str("source $ZSH/oh-my-zsh.sh\n");
        },
        FrameworkType::Zimfw => {
            content.push_str("# zimfw initialization\n");
            content.push_str(&format!("zstyle ':zim:zmodule' home '$ZDOTDIR/.zimfw'\n"));
            content.push_str(&format!("prompt {}\n", wizard_state.theme));
            // ... zimfw-specific config
        },
        // ... other frameworks
    }

    let zshrc_path = profile_path.join(".zshrc");
    std::fs::write(&zshrc_path, content)
        .context(format!("Failed to write .zshrc to {:?}", zshrc_path))?;

    Ok(())
}

fn generate_zshenv(wizard_state: &WizardState, profile_path: &Path) -> Result<()> {
    let mut content = String::new();

    content.push_str("# Auto-generated by zprof from profile.toml\n\n");

    // Shared history
    content.push_str("export HISTFILE=\"$HOME/.zsh-profiles/shared/.zsh_history\"\n");
    content.push_str("export HISTSIZE=10000\n");
    content.push_str("export SAVEHIST=10000\n");

    let zshenv_path = profile_path.join(".zshenv");
    std::fs::write(&zshenv_path, content)
        .context(format!("Failed to write .zshenv to {:?}", zshenv_path))?;

    Ok(())
}
```

**Complete Wizard Orchestration:**
```rust
// cli/create.rs
use crate::tui::{framework_select, plugin_browser, theme_select};
use crate::frameworks::installer;
use crate::core::manifest;
use crate::shell::generator;

pub fn execute(args: CreateArgs) -> Result<()> {
    // ... existing initialization and framework detection from Story 1.5 ...

    // Run complete wizard flow
    let framework = if let Some(detected) = detected_framework {
        if prompt_import(&detected)? {
            import_existing_config(&args.name, &detected)?;
            return Ok(());
        } else {
            framework_select::run_framework_selection(&args.name)?  // Story 1.6
        }
    } else {
        framework_select::run_framework_selection(&args.name)?  // Story 1.6
    };

    let plugins = plugin_browser::run_plugin_selection(framework)?;  // Story 1.7
    let theme = theme_select::run_theme_selection(framework, &plugins)?;  // Story 1.8

    let wizard_state = WizardState {
        profile_name: args.name.clone(),
        framework,
        plugins,
        theme,
    };

    // Show confirmation (Story 1.8)
    if !theme_select::show_confirmation_screen(&wizard_state)? {
        bail!("Profile creation cancelled by user.");
    }

    // Create profile directory
    let profile_path = get_profile_path(&args.name)?;
    std::fs::create_dir_all(&profile_path)
        .context("Failed to create profile directory")?;

    // Install framework and plugins (Story 1.8)
    installer::install_profile(&wizard_state, &profile_path)?;

    // Success message (AC: #8)
    println!("✓ Profile '{}' created successfully!", args.name);
    println!();
    println!("  Framework: {}", wizard_state.framework);
    println!("  Plugins:   {} installed", wizard_state.plugins.len());
    println!("  Theme:     {}", wizard_state.theme);
    println!("  Location:  {:?}", profile_path);
    println!();
    println!("Next steps:");
    println!("  zprof use {}    # Activate this profile", args.name);
    println!("  zprof list      # See all profiles");

    Ok(())
}
```

**Dependencies to Add:**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"
chrono = "0.4"
indicatif = "0.18"
```

### Project Structure Notes

**New Files Created:**
- `src/tui/theme_select.rs` - Theme selection screen implementation
- `src/frameworks/installer.rs` - Framework and plugin installation orchestration
- `src/core/manifest.rs` - TOML manifest generation and parsing
- `src/shell/generator.rs` - .zshrc and .zshenv generation from manifest
- `tests/integration/wizard_flow_test.rs` - Full wizard integration test

**Modified Files:**
- `src/frameworks/mod.rs` - Add Theme struct and get_themes() trait method
- `src/frameworks/oh_my_zsh.rs` - Implement get_themes() with 10+ themes
- `src/frameworks/zimfw.rs` - Implement get_themes() with 10+ themes
- `src/frameworks/prezto.rs` - Implement get_themes() with 10+ themes
- `src/frameworks/zinit.rs` - Implement get_themes() with 10+ themes
- `src/frameworks/zap.rs` - Implement get_themes() with 10+ themes
- `src/cli/create.rs` - Complete wizard orchestration (Stories 1.6, 1.7, 1.8)
- `src/tui/mod.rs` - Export theme_select module
- `Cargo.toml` - Add serde, toml, chrono, indicatif dependencies

**Learnings from Previous Stories:**

**From Story 1.6: TUI Wizard Framework Selection (Status: drafted)**
- Reuse terminal setup/cleanup utilities from `tui/mod.rs`
- Follow established TUI layout patterns (Block widget, footer help text)
- Consistent keyboard navigation (arrows, Enter, Esc)
- Error handling with anyhow::Result and .context()

**From Story 1.7: TUI Wizard Plugin Browser (Status: drafted)**
- Integrate as third step in wizard flow after plugin selection
- Receive both FrameworkType and selected plugins Vec<String> as inputs
- Return selected theme String for manifest generation
- Maintain wizard state through all three steps

**Integration Requirements:**
- Story 1.8 is the final wizard step, completing the profile creation flow
- Must install framework and plugins (new functionality not in 1.6 or 1.7)
- Generates profile.toml manifest from wizard selections (new in 1.8)
- Generates .zshrc and .zshenv shell config files (new in 1.8)
- Shows progress indicators during installation per AC #7
- Displays comprehensive success message per AC #8

**Expected Complete Wizard Flow:**
```rust
// Full wizard sequence (Stories 1.6, 1.7, 1.8)
let framework = framework_select::run_framework_selection(profile_name)?;     // Story 1.6
let plugins = plugin_browser::run_plugin_selection(framework)?;               // Story 1.7
let theme = theme_select::run_theme_selection(framework, &plugins)?;          // Story 1.8

// Story 1.8: Confirmation, installation, manifest, shell config generation
let confirmed = show_confirmation_screen(&wizard_state)?;
if confirmed {
    installer::install_profile(&wizard_state, &profile_path)?;
    // Success!
}
```

**Critical Handoff:**
Story 1.8 completes the wizard flow started in Stories 1.6 and 1.7. After Story 1.8, the profile is fully created and ready to use. Next stories (1.9, 1.10) focus on profile management (switching, deleting), not creation.

**Architecture Pattern Usage:**
- **Pattern 2 (Error Handling)**: All fallible operations use anyhow::Result with .context()
- **Pattern 3 (Safe File Operations)**: Profile directory creation with backups, cleanup on failure
- **Pattern 4 (TOML Manifest Schema)**: Follow exact schema from architecture.md for profile.toml

**Profile Directory After Story 1.8:**
```
~/.zsh-profiles/profiles/experimental/
├── profile.toml          # Generated manifest (Story 1.8)
├── .zshrc                # Generated shell config (Story 1.8)
├── .zshenv               # Generated shell config (Story 1.8)
└── .zimfw/               # Installed framework (Story 1.8)
    └── ... framework files ...
```

### References

- [Source: docs/epics.md#Story-1.8]
- [Source: docs/PRD.md#FR009-theme-selection]
- [Source: docs/PRD.md#FR010-generate-manifest]
- [Source: docs/PRD.md#FR011-install-framework]
- [Source: docs/architecture.md#Pattern-4-TOML-Manifest-Schema]
- [Source: docs/architecture.md#Epic-1-Story-1.8-Mapping]
- [Source: docs/architecture.md#indicatif-0.18-progress-bars]
- [Source: docs/architecture.md#serde-toml-manifest]
- [Source: docs/stories/1-6-tui-wizard-framework-selection.md#TUI-patterns]
- [Source: docs/stories/1-7-tui-wizard-plugin-browser.md#wizard-flow]

## Dev Agent Record

### Context Reference

- docs/stories/1-8-tui-wizard-theme-selection-and-profile-generation.context.xml

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
