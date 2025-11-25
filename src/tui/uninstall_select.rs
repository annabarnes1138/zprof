//! Uninstall restoration option selection TUI
//!
//! Provides interactive menus for:
//! 1. Choosing restoration method (Original / Promote / Clean)
//! 2. Selecting profile to promote (if Promote chosen)

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

use crate::core::profile::ProfileInfo;
use crate::tui::{restore_terminal, setup_terminal};

/// Restoration option for display
#[derive(Debug, Clone)]
struct RestorationOption {
    id: RestorationId,
    name: &'static str,
    description: &'static str,
    details: &'static str,
    enabled: bool,
    disabled_reason: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RestorationId {
    Original,
    Promote,
    Clean,
    Cancel,
}

impl RestorationOption {
    /// Get all restoration options with enabled/disabled states
    fn all(backup_available: bool, profiles_available: bool) -> Vec<Self> {
        vec![
            RestorationOption {
                id: RestorationId::Original,
                name: "Restore Original",
                description: "Restore pre-zprof backup to HOME directory",
                details: "Returns shell config to state before zprof installation",
                enabled: backup_available,
                disabled_reason: if backup_available {
                    None
                } else {
                    Some("Pre-zprof backup not found")
                },
            },
            RestorationOption {
                id: RestorationId::Promote,
                name: "Promote Profile",
                description: "Promote a profile to become root configuration",
                details: "Selected profile configs will become your main shell config",
                enabled: profiles_available,
                disabled_reason: if profiles_available {
                    None
                } else {
                    Some("No profiles available")
                },
            },
            RestorationOption {
                id: RestorationId::Clean,
                name: "Clean Removal",
                description: "Remove zprof without restoring any configuration",
                details: "HOME directory left clean for manual shell setup",
                enabled: true,
                disabled_reason: None,
            },
            RestorationOption {
                id: RestorationId::Cancel,
                name: "Cancel",
                description: "Do not uninstall zprof",
                details: "Exit without making any changes",
                enabled: true,
                disabled_reason: None,
            },
        ]
    }
}

/// Select restoration option interactively
///
/// # Arguments
///
/// * `backup_available` - Whether pre-zprof backup exists
/// * `profiles_available` - Whether any profiles exist
pub fn select_restoration_option(
    backup_available: bool,
    profiles_available: bool,
) -> Result<crate::cli::uninstall::RestoreOption> {
    use crate::cli::uninstall::RestoreOption;

    // Initialize terminal
    let mut terminal = setup_terminal().context("Failed to initialize terminal for TUI")?;

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

    // Run selection loop
    let result = run_restoration_selection_loop(&mut terminal, backup_available, profiles_available);

    // Always restore terminal
    restore_terminal()?;

    // Convert result
    match result? {
        RestorationId::Original => Ok(RestoreOption::Original),
        RestorationId::Promote => {
            // Need to get profiles and select one
            let profiles_dir = crate::core::profile::get_profiles_dir()?;
            let config = crate::core::config::load_config()?;
            let profiles = crate::core::profile::scan_profiles(
                &profiles_dir,
                config.active_profile.as_deref(),
            )?;

            let profile_name = select_profile_to_promote(&profiles)?;
            Ok(RestoreOption::Promote(profile_name))
        }
        RestorationId::Clean => Ok(RestoreOption::Clean),
        RestorationId::Cancel => bail!("Uninstall cancelled by user"),
    }
}

/// Main event loop for restoration option selection
fn run_restoration_selection_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    backup_available: bool,
    profiles_available: bool,
) -> Result<RestorationId> {
    let options = RestorationOption::all(backup_available, profiles_available);
    let mut state = ListState::default();

    // Select first enabled option by default
    let first_enabled = options.iter().position(|opt| opt.enabled).unwrap_or(0);
    state.select(Some(first_enabled));

    loop {
        // Render UI
        terminal.draw(|f| {
            render_restoration_ui(f, &options, &mut state);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => select_previous_enabled(&mut state, &options),
                KeyCode::Down | KeyCode::Char('j') => select_next_enabled(&mut state, &options),
                KeyCode::Enter => {
                    if let Some(selected) = state.selected() {
                        let option = &options[selected];
                        if option.enabled {
                            return Ok(option.id);
                        }
                    }
                }
                KeyCode::Esc => {
                    return Ok(RestorationId::Cancel);
                }
                _ => {}
            }
        }
    }
}

/// Render the restoration option TUI
fn render_restoration_ui(
    f: &mut Frame,
    options: &[RestorationOption],
    state: &mut ListState,
) {
    // Create main layout: title, list, footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(15),    // List
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Choose Restoration Option")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Restoration options list
    let selected_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            let is_selected = i == selected_idx;
            let indicator = if is_selected { "▸ " } else { "  " };

            let name_style = if !option.enabled {
                // Disabled option - gray out
                Style::default().fg(Color::DarkGray)
            } else if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let mut lines = vec![
                Line::from(vec![
                    Span::styled(
                        indicator,
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(option.name, name_style),
                ]),
                Line::from(Span::styled(
                    format!("  {}", option.description),
                    if option.enabled {
                        Style::default().fg(Color::Gray)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                )),
                Line::from(Span::styled(
                    format!("  {}", option.details),
                    Style::default().fg(Color::DarkGray),
                )),
            ];

            // Add disabled reason if applicable
            if let Some(reason) = option.disabled_reason {
                lines.push(Line::from(Span::styled(
                    format!("  ⚠ {}", reason),
                    Style::default().fg(Color::Red),
                )));
            }

            lines.push(Line::from("")); // Spacing

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Restoration Options"));

    f.render_stateful_widget(list, chunks[1], state);

    // Footer with help text
    let footer = Paragraph::new("↑↓/j/k: Navigate | Enter: Select | Esc: Cancel")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

/// Select profile to promote interactively
pub fn select_profile_to_promote(profiles: &[ProfileInfo]) -> Result<String> {
    // Initialize terminal
    let mut terminal = setup_terminal().context("Failed to initialize terminal for TUI")?;

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

    // Run selection loop
    let result = run_profile_selection_loop(&mut terminal, profiles);

    // Always restore terminal
    restore_terminal()?;

    result
}

/// Main event loop for profile selection
fn run_profile_selection_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    profiles: &[ProfileInfo],
) -> Result<String> {
    let mut state = ListState::default();
    state.select(Some(0)); // Start with first profile selected

    loop {
        // Render UI
        terminal.draw(|f| {
            render_profile_ui(f, profiles, &mut state);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => select_previous(&mut state, profiles.len()),
                KeyCode::Down | KeyCode::Char('j') => select_next(&mut state, profiles.len()),
                KeyCode::Enter => {
                    if let Some(selected) = state.selected() {
                        return Ok(profiles[selected].name.clone());
                    }
                }
                KeyCode::Esc => {
                    bail!("Profile selection cancelled by user")
                }
                _ => {}
            }
        }
    }
}

/// Render the profile selection TUI
fn render_profile_ui(
    f: &mut Frame,
    profiles: &[ProfileInfo],
    state: &mut ListState,
) {
    // Create main layout: title, list, footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // List
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Select Profile to Promote")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Profile list
    let selected_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = profiles
        .iter()
        .enumerate()
        .map(|(i, profile)| {
            let is_selected = i == selected_idx;
            let indicator = if is_selected { "▸ " } else { "  " };

            let active_marker = if profile.is_active { " (active)" } else { "" };

            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        indicator,
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{}{}", profile.name, active_marker),
                        if is_selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        },
                    ),
                ]),
                Line::from(Span::styled(
                    format!("  Framework: {}", profile.framework),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(""), // Spacing
            ];

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Profiles"));

    f.render_stateful_widget(list, chunks[1], state);

    // Footer with help text
    let footer = Paragraph::new("↑↓/j/k: Navigate | Enter: Select | Esc: Cancel")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

/// Move selection to previous enabled item
fn select_previous_enabled(state: &mut ListState, options: &[RestorationOption]) {
    let current = state.selected().unwrap_or(0);
    let mut i = current;

    loop {
        i = if i == 0 {
            options.len() - 1 // Wrap to bottom
        } else {
            i - 1
        };

        if options[i].enabled || i == current {
            state.select(Some(i));
            break;
        }
    }
}

/// Move selection to next enabled item
fn select_next_enabled(state: &mut ListState, options: &[RestorationOption]) {
    let current = state.selected().unwrap_or(0);
    let mut i = current;

    loop {
        i = if i >= options.len() - 1 {
            0 // Wrap to top
        } else {
            i + 1
        };

        if options[i].enabled || i == current {
            state.select(Some(i));
            break;
        }
    }
}

/// Move selection to previous item (with wrapping)
fn select_previous(state: &mut ListState, len: usize) {
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

    #[test]
    fn test_restoration_option_count() {
        let options = RestorationOption::all(true, true);
        assert_eq!(options.len(), 4, "Should have 4 options (Original, Promote, Clean, Cancel)");
    }

    #[test]
    fn test_restoration_option_disabled_states() {
        // No backup, no profiles - only Clean and Cancel should be enabled
        let options = RestorationOption::all(false, false);

        let original = options.iter().find(|o| matches!(o.id, RestorationId::Original)).unwrap();
        let promote = options.iter().find(|o| matches!(o.id, RestorationId::Promote)).unwrap();
        let clean = options.iter().find(|o| matches!(o.id, RestorationId::Clean)).unwrap();
        let cancel = options.iter().find(|o| matches!(o.id, RestorationId::Cancel)).unwrap();

        assert!(!original.enabled, "Original should be disabled when backup unavailable");
        assert!(!promote.enabled, "Promote should be disabled when no profiles");
        assert!(clean.enabled, "Clean should always be enabled");
        assert!(cancel.enabled, "Cancel should always be enabled");
    }

    #[test]
    fn test_restoration_option_enabled_states() {
        // With backup and profiles - all should be enabled
        let options = RestorationOption::all(true, true);

        let original = options.iter().find(|o| matches!(o.id, RestorationId::Original)).unwrap();
        let promote = options.iter().find(|o| matches!(o.id, RestorationId::Promote)).unwrap();

        assert!(original.enabled, "Original should be enabled when backup available");
        assert!(promote.enabled, "Promote should be enabled when profiles available");
    }

    #[test]
    fn test_select_next_wrapping() {
        let mut state = ListState::default();
        state.select(Some(2)); // Last item (assuming 3 items)

        select_next(&mut state, 3);
        assert_eq!(state.selected(), Some(0), "Should wrap to first item");
    }

    #[test]
    fn test_select_previous_wrapping() {
        let mut state = ListState::default();
        state.select(Some(0)); // First item

        select_previous(&mut state, 3);
        assert_eq!(state.selected(), Some(2), "Should wrap to last item");
    }
}
