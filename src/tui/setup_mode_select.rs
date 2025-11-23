//! Setup mode selection TUI screen
//!
//! Provides an interactive menu for choosing between Quick Setup (presets)
//! and Custom Setup (manual component selection).

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

use crate::tui::{restore_terminal, setup_terminal};

/// Setup mode selection - Quick (presets) or Custom (manual)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupMode {
    /// Quick setup using recommended presets
    Quick,
    /// Custom setup with manual component selection
    Custom,
}

/// Setup mode option with display information
#[derive(Debug, Clone)]
struct SetupModeOption {
    mode: SetupMode,
    name: &'static str,
    description: &'static str,
    details: &'static str,
}

impl SetupModeOption {
    /// Get all available setup mode options
    fn all() -> Vec<Self> {
        vec![
            SetupModeOption {
                mode: SetupMode::Quick,
                name: "Quick Setup",
                description: "Use recommended presets for common workflows",
                details: "Fast configuration with curated plugin collections",
            },
            SetupModeOption {
                mode: SetupMode::Custom,
                name: "Custom Setup",
                description: "Choose your own components manually",
                details: "Full control over framework, plugins, and theme",
            },
        ]
    }
}

/// Run interactive setup mode selection TUI
///
/// Displays a binary choice between Quick Setup (presets) and Custom Setup (manual).
/// Quick Setup is the default selection.
///
/// # Returns
///
/// Selected SetupMode on Enter, or error if cancelled (Esc) or failed
///
/// # Keyboard Controls
///
/// - Up/Down arrows or j/k: Navigate list
/// - Enter: Select highlighted mode
/// - Esc: Cancel selection
///
/// # Errors
///
/// - Terminal too small (minimum 80x24)
/// - User cancels with Esc key
/// - Terminal initialization fails
pub fn select_setup_mode() -> Result<SetupMode> {
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
    let result = run_selection_loop(&mut terminal);

    // Always restore terminal, even on error
    restore_terminal()?;

    result
}

/// Main event loop for setup mode selection
fn run_selection_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<SetupMode> {
    let modes = SetupModeOption::all();
    let mut state = ListState::default();
    state.select(Some(0)); // Start with Quick Setup (first item) selected

    loop {
        // Render UI
        terminal.draw(|f| {
            render_ui(f, &modes, &mut state);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => select_previous(&mut state, modes.len()),
                KeyCode::Down | KeyCode::Char('j') => select_next(&mut state, modes.len()),
                KeyCode::Enter => {
                    if let Some(selected) = state.selected() {
                        return Ok(modes[selected].mode);
                    }
                }
                KeyCode::Esc => {
                    bail!("Setup mode selection cancelled by user")
                }
                _ => {}
            }
        }
    }
}

/// Render the TUI interface
fn render_ui(
    f: &mut Frame,
    modes: &[SetupModeOption],
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
    let title = Paragraph::new("Choose Setup Mode")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Setup mode list
    let selected_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = modes
        .iter()
        .enumerate()
        .map(|(i, mode)| {
            let is_selected = i == selected_idx;
            let indicator = if is_selected { "▸ " } else { "  " };

            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        indicator,
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        mode.name,
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
                    format!("  {}", mode.description),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(Span::styled(
                    format!("  {}", mode.details),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""), // Spacing between items
            ];

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Setup Modes"));

    f.render_stateful_widget(list, chunks[1], state);

    // Footer with help text
    let footer = Paragraph::new("↑↓/j/k: Navigate | Enter: Select | Esc: Cancel")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
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
    fn test_setup_mode_enum_variants() {
        // Test that SetupMode has both Quick and Custom variants
        let quick = SetupMode::Quick;
        let custom = SetupMode::Custom;

        assert_ne!(quick, custom);
    }

    #[test]
    fn test_setup_mode_match() {
        // Test that SetupMode can be used in match statements
        let mode = SetupMode::Quick;
        let result = match mode {
            SetupMode::Quick => "quick",
            SetupMode::Custom => "custom",
        };
        assert_eq!(result, "quick");
    }

    #[test]
    fn test_setup_mode_option_count() {
        let modes = SetupModeOption::all();
        assert_eq!(modes.len(), 2, "Should have exactly 2 setup mode options");
    }

    #[test]
    fn test_setup_mode_option_names() {
        let modes = SetupModeOption::all();
        let names: Vec<&str> = modes.iter().map(|m| m.name).collect();

        assert_eq!(names, vec!["Quick Setup", "Custom Setup"]);
    }

    #[test]
    fn test_setup_mode_option_has_descriptions() {
        let modes = SetupModeOption::all();

        for mode in modes {
            assert!(!mode.description.is_empty(), "Mode {} missing description", mode.name);
            assert!(!mode.details.is_empty(), "Mode {} missing details", mode.name);
        }
    }

    #[test]
    fn test_default_selection_is_quick() {
        // Test that the first item (index 0) is Quick Setup
        let modes = SetupModeOption::all();
        assert_eq!(modes[0].mode, SetupMode::Quick, "First item should be Quick Setup");
    }

    #[test]
    fn test_select_next_wrapping() {
        let mut state = ListState::default();
        state.select(Some(1)); // Last item

        select_next(&mut state, 2);
        assert_eq!(state.selected(), Some(0), "Should wrap to first item");
    }

    #[test]
    fn test_select_previous_wrapping() {
        let mut state = ListState::default();
        state.select(Some(0)); // First item

        select_previous(&mut state, 2);
        assert_eq!(state.selected(), Some(1), "Should wrap to last item");
    }

    #[test]
    fn test_select_navigation() {
        let mut state = ListState::default();
        state.select(Some(0));

        select_next(&mut state, 2);
        assert_eq!(state.selected(), Some(1));

        select_previous(&mut state, 2);
        assert_eq!(state.selected(), Some(0));
    }
}
