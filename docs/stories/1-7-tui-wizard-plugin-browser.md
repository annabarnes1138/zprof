# Story 1.7: TUI Wizard Plugin Browser

Status: ready-for-dev

## Story

As a developer,
I want to browse and select plugins for my profile,
so that I can customize my shell with useful tools.

## Acceptance Criteria

1. TUI displays popular plugins with descriptions for selected framework
2. Multi-select interface allows checking/unchecking plugins
3. At least 10-15 popular plugins per framework with recommendations
4. Search/filter capability for finding specific plugins
5. Selected plugins are highlighted and counted
6. Can proceed with no plugins selected (minimal setup)

## Tasks / Subtasks

- [ ] Define plugin data model for all frameworks (AC: #1, #3)
  - [ ] Create Plugin struct with name, description, category, framework_type
  - [ ] Define 10-15 popular plugins per framework (oh-my-zsh, zimfw, prezto, zinit, zap)
  - [ ] Organize plugins by category (git, docker, kubernetes, language-specific, utilities)
  - [ ] Store plugin definitions in `frameworks/*.rs` per framework
  - [ ] Create get_plugins() method for each framework returning Vec<Plugin>
- [ ] Implement multi-select TUI screen (AC: #2, #5, #6)
  - [ ] Create `tui/plugin_browser.rs` module
  - [ ] Implement `run_plugin_selection(framework: FrameworkType) -> Result<Vec<String>>` function
  - [ ] Use Ratatui List widget with checkbox indicators [x] / [ ]
  - [ ] Track selected plugins in Vec<bool> matching plugin list indices
  - [ ] Toggle selection on Space key press (AC: #2)
  - [ ] Show selection count in header: "Selected: 5 plugins" (AC: #5)
  - [ ] Allow proceeding with empty selection (AC: #6)
  - [ ] Return Vec<String> of selected plugin names on Enter
- [ ] Implement search/filter functionality (AC: #4)
  - [ ] Add input mode for search query entry
  - [ ] Toggle search mode with `/` key (vim-style)
  - [ ] Filter plugin list by name or description matching search query
  - [ ] Display search query in UI with visual indicator
  - [ ] Clear filter with Esc or empty search string
  - [ ] Maintain selection state when filtering/unfiltering
- [ ] Design responsive layout (AC: #1, #2, #5)
  - [ ] Use Ratatui Block widget for borders and sections
  - [ ] Create header showing: framework name, selection count, search status
  - [ ] Main area: scrollable plugin list with checkboxes and descriptions
  - [ ] Footer: help text "Space: Toggle | Enter: Continue | /: Search | Esc: Cancel"
  - [ ] Fit layout in 80x24 terminal minimum (NFR003)
  - [ ] Handle long plugin descriptions gracefully (truncate or wrap)
- [ ] Integrate with wizard flow (AC: All)
  - [ ] Modify `tui/wizard.rs` to call plugin browser after framework selection
  - [ ] Pass selected FrameworkType from Story 1.6 to plugin browser
  - [ ] Store selected plugins in wizard state for Story 1.8
  - [ ] Handle cancellation (Esc) by returning error and aborting wizard
  - [ ] Pass selected plugins to theme selection step (Story 1.8)
- [ ] Handle edge cases and errors (AC: All)
  - [ ] Handle framework with no plugins defined gracefully (show message, allow continue)
  - [ ] Restore terminal state on panic (reuse panic handler from Story 1.6)
  - [ ] Handle terminal resize during plugin browsing
  - [ ] Use anyhow::Context for error messages per Pattern 2
  - [ ] Log plugin selection events with env_logger
- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test Plugin data structure
  - [ ] Unit test keyboard event handling (space/enter/esc/slash)
  - [ ] Unit test search/filter logic (query matching)
  - [ ] Integration test plugin selection returns correct Vec<String>
  - [ ] Test empty selection returns empty Vec (AC: #6)
  - [ ] Test multi-select toggle state management
  - [ ] Test filter preserves selection state
  - [ ] Manual test in 80x24 terminal
  - [ ] Test with all 5 frameworks' plugin lists

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `tui/plugin_browser.rs`, `frameworks/*.rs` (plugin definitions)
- Secondary: `tui/wizard.rs` (orchestration), `tui/mod.rs` (shared utilities)
- Reuse: Terminal setup/cleanup from Story 1.6 (`tui/mod.rs`)
- Follow Pattern 2 (Error Handling) with anyhow::Result
- Must work on 80x24 terminal minimum per NFR003

**Plugin Data Model:**
```rust
// frameworks/mod.rs
#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub description: String,
    pub category: PluginCategory,
}

#[derive(Debug, Clone)]
pub enum PluginCategory {
    Git,
    Docker,
    Kubernetes,
    Language,  // Programming language support
    Utility,   // General utilities
}

pub trait Framework {
    // ... existing methods from Story 1.4 ...
    fn get_plugins(&self) -> Vec<Plugin>;
}
```

**Plugin Browser TUI Pattern:**
```rust
// tui/plugin_browser.rs
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

pub fn run_plugin_selection(framework: FrameworkType) -> Result<Vec<String>> {
    let mut terminal = setup_terminal()?;
    let result = run_plugin_loop(&mut terminal, framework);
    restore_terminal()?;
    result
}

fn run_plugin_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>, framework: FrameworkType) -> Result<Vec<String>> {
    let plugins = get_plugins_for_framework(framework);
    let mut selected = vec![false; plugins.len()]; // Track selections
    let mut state = ListState::default();
    state.select(Some(0));
    let mut search_query = String::new();
    let mut search_mode = false;

    loop {
        // Render UI
        terminal.draw(|f| {
            render_plugin_browser(f, &plugins, &selected, &state, &search_query, search_mode);
        })?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            if search_mode {
                match key.code {
                    KeyCode::Char(c) => search_query.push(c),
                    KeyCode::Backspace => { search_query.pop(); },
                    KeyCode::Esc => { search_mode = false; search_query.clear(); },
                    KeyCode::Enter => search_mode = false,
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Up => select_previous(&mut state, plugins.len()),
                    KeyCode::Down => select_next(&mut state, plugins.len()),
                    KeyCode::Char(' ') => toggle_selection(&mut selected, &state),
                    KeyCode::Char('/') => search_mode = true,
                    KeyCode::Enter => return Ok(get_selected_plugins(&plugins, &selected)),
                    KeyCode::Esc => bail!("Plugin selection cancelled"),
                    _ => {}
                }
            }
        }
    }
}

fn toggle_selection(selected: &mut Vec<bool>, state: &ListState) {
    if let Some(idx) = state.selected() {
        selected[idx] = !selected[idx];
    }
}

fn get_selected_plugins(plugins: &[Plugin], selected: &[bool]) -> Vec<String> {
    plugins.iter()
        .zip(selected.iter())
        .filter(|(_, &is_selected)| is_selected)
        .map(|(plugin, _)| plugin.name.clone())
        .collect()
}
```

**Example Plugin Definitions (oh-my-zsh):**
```rust
// frameworks/oh_my_zsh.rs
impl Framework for OhMyZsh {
    fn get_plugins(&self) -> Vec<Plugin> {
        vec![
            Plugin {
                name: "git".to_string(),
                description: "Git aliases and functions".to_string(),
                category: PluginCategory::Git,
            },
            Plugin {
                name: "docker".to_string(),
                description: "Docker aliases and completion".to_string(),
                category: PluginCategory::Docker,
            },
            Plugin {
                name: "kubectl".to_string(),
                description: "Kubernetes kubectl aliases and completion".to_string(),
                category: PluginCategory::Kubernetes,
            },
            Plugin {
                name: "rust".to_string(),
                description: "Rust language support and cargo aliases".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "fzf".to_string(),
                description: "Fuzzy file finder integration".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "zsh-autosuggestions".to_string(),
                description: "Fish-like autosuggestions for zsh".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "zsh-syntax-highlighting".to_string(),
                description: "Syntax highlighting for commands".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "z".to_string(),
                description: "Jump to frequently used directories".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "nvm".to_string(),
                description: "Node Version Manager integration".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "brew".to_string(),
                description: "Homebrew completion and aliases".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "sudo".to_string(),
                description: "Prefix last command with sudo using ESC-ESC".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "history".to_string(),
                description: "History search and aliases".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "aws".to_string(),
                description: "AWS CLI completion".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "terraform".to_string(),
                description: "Terraform completion and aliases".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "node".to_string(),
                description: "Node.js aliases and utilities".to_string(),
                category: PluginCategory::Language,
            },
        ]
    }
}
```

**TUI Layout Example:**
```
┌─ Select Plugins for 'experimental' (zimfw) ─ Selected: 3 ────┐
│                                                                │
│  [x] git - Git aliases and functions                          │
│  [ ] docker - Docker aliases and completion                   │
│  [x] kubectl - Kubernetes kubectl aliases and completion      │
│  [ ] rust - Rust language support and cargo aliases           │
│  [x] fzf - Fuzzy file finder integration                      │
│  [ ] zsh-autosuggestions - Fish-like autosuggestions          │
│  [ ] zsh-syntax-highlighting - Syntax highlighting            │
│  [ ] z - Jump to frequently used directories                  │
│  [ ] nvm - Node Version Manager integration                   │
│  [ ] brew - Homebrew completion and aliases                   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
 Space: Toggle | Enter: Continue | /: Search | Esc: Cancel
```

**Search Mode Example:**
```
┌─ Select Plugins (Search: "docker") ─ Selected: 1 ─────────────┐
│                                                                │
│  [x] docker - Docker aliases and completion                   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
 Search: docker_
 Enter: Exit Search | Esc: Clear | Backspace: Delete
```

**Integration with Wizard:**
```rust
// tui/wizard.rs (new file for orchestrating multi-step wizard)
use anyhow::Result;

pub struct WizardState {
    pub profile_name: String,
    pub framework: FrameworkType,
    pub plugins: Vec<String>,
    pub theme: String,
}

pub fn run_profile_wizard(profile_name: &str) -> Result<WizardState> {
    // Step 1: Framework selection (Story 1.6)
    let framework = framework_select::run_framework_selection(profile_name)?;

    // Step 2: Plugin selection (Story 1.7)
    let plugins = plugin_browser::run_plugin_selection(framework)?;

    // Step 3: Theme selection (Story 1.8)
    let theme = theme_select::run_theme_selection(framework, &plugins)?;

    Ok(WizardState {
        profile_name: profile_name.to_string(),
        framework,
        plugins,
        theme,
    })
}
```

**Error Handling:**
```rust
// Empty plugin list for framework
if plugins.is_empty() {
    restore_terminal()?;
    log::warn!("No plugins defined for {:?}", framework);
    return Ok(vec![]); // Allow continuing with no plugins (AC: #6)
}

// User cancellation
plugin_browser::run_plugin_selection(framework)
    .context("Plugin selection cancelled. Profile creation aborted.")?;
```

### Project Structure Notes

**New Files Created:**
- `src/tui/plugin_browser.rs` - Plugin multi-select screen implementation
- `src/tui/wizard.rs` - Multi-step wizard orchestration (optional, can be in create.rs)

**Modified Files:**
- `src/frameworks/mod.rs` - Add Plugin struct and PluginCategory enum
- `src/frameworks/oh_my_zsh.rs` - Implement get_plugins() with 15 popular plugins
- `src/frameworks/zimfw.rs` - Implement get_plugins() with 15 popular plugins
- `src/frameworks/prezto.rs` - Implement get_plugins() with 15 popular plugins
- `src/frameworks/zinit.rs` - Implement get_plugins() with 15 popular plugins
- `src/frameworks/zap.rs` - Implement get_plugins() with 15 popular plugins
- `src/cli/create.rs` - Call plugin browser after framework selection
- `src/tui/mod.rs` - Export plugin_browser module

**Learnings from Previous Story:**

**From Story 1.6: TUI Wizard Framework Selection (Status: drafted)**

Story 1.6 established the TUI infrastructure and terminal setup/cleanup utilities that Story 1.7 reuses:

- **Reuse Terminal Setup**: Use `setup_terminal()` and `restore_terminal()` from `tui/mod.rs` - don't reimplement
- **Reuse Panic Handler**: Terminal restoration panic hook already installed in `main.rs` (Story 1.6)
- **Follow Same Layout Patterns**: Use Block widget for borders, footer for help text, 80x24 minimum size
- **Consistent Error Handling**: Return anyhow::Result, use .context() for user-friendly errors
- **Keyboard Navigation**: Arrow keys for navigation, Enter for confirm, Esc for cancel (established pattern)

**Integration Requirements:**
- Story 1.7 receives `FrameworkType` from Story 1.6's return value
- Must load appropriate plugin list based on selected framework
- Return `Vec<String>` of plugin names for Story 1.8 to use
- All three wizard steps (1.6, 1.7, 1.8) form continuous flow

**Expected Call Pattern from Wizard:**
```rust
// Wizard flow sequence
let framework = framework_select::run_framework_selection(profile_name)?;  // Story 1.6
let plugins = plugin_browser::run_plugin_selection(framework)?;            // Story 1.7
let theme = theme_select::run_theme_selection(framework, &plugins)?;       // Story 1.8
```

**Critical Handoff:**
After Story 1.7 returns selected plugins Vec<String>, execution continues to:
- Story 1.8: Theme selection TUI and profile generation (final wizard step)

The plugin selection is stored in wizard state and later written to profile.toml manifest:
```toml
[plugins]
enabled = ["git", "docker", "kubectl"]  # From Story 1.7 selections
```

### References

- [Source: docs/epics.md#Story-1.7]
- [Source: docs/PRD.md#FR008-plugin-browser]
- [Source: docs/architecture.md#Ratatui-0.29.0]
- [Source: docs/architecture.md#Epic-1-Story-1.7-Mapping]
- [Source: docs/architecture.md#Pattern-2-Error-Handling]
- [Source: docs/stories/1-6-tui-wizard-framework-selection.md#TUI-infrastructure]

## Dev Agent Record

### Context Reference

- docs/stories/1-7-tui-wizard-plugin-browser.context.xml

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
