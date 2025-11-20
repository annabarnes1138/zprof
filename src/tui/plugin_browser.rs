//! Plugin browser TUI screen with multi-select capability
//!
//! Provides an interactive multi-select menu for browsing and selecting plugins
//! for a chosen zsh framework. Includes search/filter functionality.

use anyhow::{bail, Context, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::io;

use crate::frameworks::{Framework, FrameworkType, Plugin};
use crate::tui::{restore_terminal, setup_terminal};

// Import framework implementations
use crate::frameworks::{
    oh_my_zsh::OhMyZsh,
    prezto::Prezto,
    zap::Zap,
    zimfw::Zimfw,
    zinit::Zinit,
};

/// Run interactive plugin selection TUI
///
/// Displays a multi-select list of available plugins for the selected framework.
/// Users can browse, search/filter, and select multiple plugins.
///
/// # Arguments
///
/// * `framework` - The framework type to load plugins for
///
/// # Returns
///
/// Vec<String> of selected plugin names on Enter (may be empty),
/// or error if cancelled (Esc) or failed
///
/// # Keyboard Controls
///
/// - Up/Down arrows: Navigate list
/// - Space: Toggle plugin selection (checkbox)
/// - /: Enter search mode
/// - Enter: Confirm selection and continue
/// - Esc: Cancel selection
///
/// # Errors
///
/// - Terminal too small (minimum 80x24)
/// - User cancels with Esc key
/// - Terminal initialization fails
pub fn run_plugin_selection(framework: FrameworkType) -> Result<Vec<String>> {
    // Initialize terminal
    let mut terminal = setup_terminal().context("Failed to initialize terminal for plugin browser")?;

    // Check minimum terminal size
    let size = terminal.size()?;
    if size.width < 80 || size.height < 24 {
        restore_terminal()?;
        bail!(
            "✗ Error: Terminal too small\n  → Minimum size: 80x24\n  → Current size: {}x{}",
            size.width,
            size.height
        );
    }

    // Get plugins for framework
    let mut plugins = get_plugins_for_framework(&framework);

    // If no plugins available, show warning and return empty vec (AC: #6)
    if plugins.is_empty() {
        restore_terminal()?;
        log::warn!("No plugins defined for framework: {:?}", framework);
        return Ok(vec![]);
    }

    // Sort plugins: recommended first, then alphabetically
    plugins.sort_by(|a, b| {
        let a_recommended = a.compatibility.is_recommended_for(&framework);
        let b_recommended = b.compatibility.is_recommended_for(&framework);

        match (a_recommended, b_recommended) {
            (true, false) => std::cmp::Ordering::Less,    // a first
            (false, true) => std::cmp::Ordering::Greater, // b first
            _ => a.name.cmp(b.name),                      // alphabetical
        }
    });

    // Run selection loop
    let result = run_plugin_loop(&mut terminal, framework, plugins);

    // Always restore terminal, even on error
    restore_terminal()?;

    result
}

/// Get plugins for a specific framework
fn get_plugins_for_framework(framework: &FrameworkType) -> Vec<Plugin> {
    match framework {
        FrameworkType::OhMyZsh => OhMyZsh::get_plugins(),
        FrameworkType::Zimfw => Zimfw::get_plugins(),
        FrameworkType::Prezto => Prezto::get_plugins(),
        FrameworkType::Zinit => Zinit::get_plugins(),
        FrameworkType::Zap => Zap::get_plugins(),
    }
}

/// Main event loop for plugin selection
fn run_plugin_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    framework: FrameworkType,
    plugins: Vec<Plugin>,
) -> Result<Vec<String>> {
    let mut selected = vec![false; plugins.len()]; // Track selections
    let mut state = ListState::default();
    state.select(Some(0)); // Start with first item
    let mut search_query = String::new();
    let mut search_mode = false;

    loop {
        // Render UI
        terminal.draw(|f| {
            render_plugin_browser(
                f,
                &framework,
                &plugins,
                &selected,
                &mut state,
                &search_query,
                search_mode,
            );
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            if search_mode {
                // Search mode key handling
                match key.code {
                    KeyCode::Char(c) => {
                        search_query.push(c);
                    }
                    KeyCode::Backspace => {
                        search_query.pop();
                    }
                    KeyCode::Esc => {
                        search_mode = false;
                        search_query.clear();
                    }
                    KeyCode::Enter => {
                        search_mode = false;
                    }
                    _ => {}
                }
            } else {
                // Normal mode key handling
                match key.code {
                    KeyCode::Up => select_previous(&mut state, plugins.len()),
                    KeyCode::Down => select_next(&mut state, plugins.len()),
                    KeyCode::Char(' ') => toggle_selection(&mut selected, &state),
                    KeyCode::Char('/') => {
                        search_mode = true;
                    }
                    KeyCode::Enter => {
                        return Ok(get_selected_plugins(&plugins, &selected));
                    }
                    KeyCode::Esc => {
                        bail!("Plugin selection cancelled by user")
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Toggle selection for current item
fn toggle_selection(selected: &mut Vec<bool>, state: &ListState) {
    if let Some(idx) = state.selected() {
        selected[idx] = !selected[idx];
    }
}

/// Get list of selected plugin names
fn get_selected_plugins(plugins: &[Plugin], selected: &[bool]) -> Vec<String> {
    plugins
        .iter()
        .zip(selected.iter())
        .filter(|(_, &is_selected)| is_selected)
        .map(|(plugin, _)| plugin.name.to_string())
        .collect()
}

/// Filter plugins by search query (matches name or description)
fn filter_plugins(plugins: &[Plugin], query: &str) -> Vec<usize> {
    if query.is_empty() {
        return (0..plugins.len()).collect();
    }

    let query_lower = query.to_lowercase();
    plugins
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            p.name.to_lowercase().contains(&query_lower)
                || p.description.to_lowercase().contains(&query_lower)
        })
        .map(|(idx, _)| idx)
        .collect()
}

/// Render the plugin browser UI
fn render_plugin_browser(
    f: &mut Frame,
    framework: &FrameworkType,
    plugins: &[Plugin],
    selected: &[bool],
    state: &mut ListState,
    search_query: &str,
    search_mode: bool,
) {
    // Get filtered indices
    let filtered_indices = filter_plugins(plugins, search_query);

    // Count selected plugins
    let selected_count = selected.iter().filter(|&&s| s).count();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Plugin list
            Constraint::Length(3), // Footer/help
        ])
        .split(f.area());

    // Header with framework name, selection count, and search status
    let framework_name = match framework {
        FrameworkType::OhMyZsh => "oh-my-zsh",
        FrameworkType::Zimfw => "zimfw",
        FrameworkType::Prezto => "prezto",
        FrameworkType::Zinit => "zinit",
        FrameworkType::Zap => "zap",
    };

    let header_text = if search_mode {
        format!(
            "Select Plugins for '{}' - Selected: {} - Search: {}",
            framework_name, selected_count, search_query
        )
    } else if !search_query.is_empty() {
        format!(
            "Select Plugins for '{}' - Selected: {} - Filter: '{}'",
            framework_name, selected_count, search_query
        )
    } else {
        format!(
            "Select Plugins for '{}' - Selected: {}",
            framework_name, selected_count
        )
    };

    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Plugin list with checkboxes
    let current_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = filtered_indices
        .iter()
        .enumerate()
        .map(|(display_idx, &actual_idx)| {
            let plugin = &plugins[actual_idx];
            let is_current = display_idx == current_idx;
            let is_selected = selected[actual_idx];

            let checkbox = if is_selected { "[x]" } else { "[ ]" };
            let indicator = if is_current { "▸ " } else { "  " };

            // Add (recommended) suffix if applicable, then truncate if too long
            let is_recommended = plugin.compatibility.is_recommended_for(framework);
            let base_desc = if is_recommended {
                format!("{} (recommended)", plugin.description)
            } else {
                plugin.description.to_string()
            };

            let max_desc_len = 60;
            let description = if base_desc.len() > max_desc_len {
                format!("{}...", &base_desc[..max_desc_len - 3])
            } else {
                base_desc
            };

            let line = Line::from(vec![
                Span::styled(
                    indicator,
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    checkbox,
                    if is_selected {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
                Span::raw(" "),
                Span::styled(
                    plugin.name,
                    if is_current {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ),
                Span::raw(" - "),
                Span::styled(description, Style::default().fg(Color::Gray)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Plugins ({} available)", filtered_indices.len())),
    );

    f.render_stateful_widget(list, chunks[1], state);

    // Footer with help text
    let footer_text = if search_mode {
        "Type to search | Enter: Exit Search | Esc: Clear | Backspace: Delete"
    } else {
        "Space: Toggle | Enter: Continue | /: Search | Esc: Cancel"
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

/// Move selection to previous item (with wrapping)
fn select_previous(state: &mut ListState, len: usize) {
    if len == 0 {
        return;
    }
    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                len - 1 // Wrap to bottom
            } else {
                i - 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}

/// Move selection to next item (with wrapping)
fn select_next(state: &mut ListState, len: usize) {
    if len == 0 {
        return;
    }
    let i = match state.selected() {
        Some(i) => {
            if i >= len - 1 {
                0 // Wrap to top
            } else {
                i + 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Dummy compatibility for test fixtures
    const DUMMY_COMPAT: crate::frameworks::PluginCompatibility = crate::frameworks::PluginCompatibility {
        supported_managers: &[],
    };

    use super::*;

    #[test]
    fn test_toggle_selection() {
        let mut selected = vec![false, false, false];
        let mut state = ListState::default();
        state.select(Some(1));

        toggle_selection(&mut selected, &state);
        assert!(selected[1], "Should toggle selection to true");

        toggle_selection(&mut selected, &state);
        assert!(!selected[1], "Should toggle selection back to false");
    }

    #[test]
    fn test_get_selected_plugins() {
        let plugins = vec![
            Plugin { name: "git", description: "Git plugin", category: crate::frameworks::PluginCategory::Git, compatibility: DUMMY_COMPAT },
            Plugin { name: "docker", description: "Docker plugin", category: crate::frameworks::PluginCategory::Docker, compatibility: DUMMY_COMPAT },
            Plugin { name: "kubectl", description: "Kubectl plugin", category: crate::frameworks::PluginCategory::Kubernetes, compatibility: DUMMY_COMPAT },
        ];

        let selected = vec![true, false, true];
        let result = get_selected_plugins(&plugins, &selected);

        assert_eq!(result, vec!["git", "kubectl"]);
    }

    #[test]
    fn test_get_selected_plugins_empty() {
        let plugins = vec![
            Plugin { name: "git", description: "Git plugin", category: crate::frameworks::PluginCategory::Git, compatibility: DUMMY_COMPAT },
        ];

        let selected = vec![false];
        let result = get_selected_plugins(&plugins, &selected);

        assert!(result.is_empty(), "Should return empty vec when nothing selected");
    }

    #[test]
    fn test_filter_plugins_by_name() {
        let plugins = vec![
            Plugin { name: "git", description: "Version control", category: crate::frameworks::PluginCategory::Git, compatibility: DUMMY_COMPAT },
            Plugin { name: "docker", description: "Containerization", category: crate::frameworks::PluginCategory::Docker, compatibility: DUMMY_COMPAT },
            Plugin { name: "kubectl", description: "Kubernetes control", category: crate::frameworks::PluginCategory::Kubernetes, compatibility: DUMMY_COMPAT },
        ];

        let result = filter_plugins(&plugins, "git");
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_filter_plugins_by_description() {
        let plugins = vec![
            Plugin { name: "git", description: "Version control", category: crate::frameworks::PluginCategory::Git, compatibility: DUMMY_COMPAT },
            Plugin { name: "docker", description: "Container platform", category: crate::frameworks::PluginCategory::Docker, compatibility: DUMMY_COMPAT },
        ];

        let result = filter_plugins(&plugins, "container");
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_filter_plugins_case_insensitive() {
        let plugins = vec![
            Plugin { name: "Docker", description: "Container platform", category: crate::frameworks::PluginCategory::Docker, compatibility: DUMMY_COMPAT },
        ];

        let result = filter_plugins(&plugins, "docker");
        assert_eq!(result.len(), 1);

        let result = filter_plugins(&plugins, "DOCKER");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_filter_plugins_empty_query() {
        let plugins = vec![
            Plugin { name: "git", description: "VCS", category: crate::frameworks::PluginCategory::Git, compatibility: DUMMY_COMPAT },
            Plugin { name: "docker", description: "Containers", category: crate::frameworks::PluginCategory::Docker, compatibility: DUMMY_COMPAT },
        ];

        let result = filter_plugins(&plugins, "");
        assert_eq!(result, vec![0, 1], "Empty query should return all plugins");
    }

    #[test]
    fn test_select_navigation_wrapping() {
        let mut state = ListState::default();
        state.select(Some(0));

        select_previous(&mut state, 5);
        assert_eq!(state.selected(), Some(4), "Should wrap to last item");

        state.select(Some(4));
        select_next(&mut state, 5);
        assert_eq!(state.selected(), Some(0), "Should wrap to first item");
    }
}
