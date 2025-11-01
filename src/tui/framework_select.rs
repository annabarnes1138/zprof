//! Framework selection TUI screen
//!
//! Provides an interactive menu for selecting a zsh framework using Ratatui.
//! Supports keyboard navigation and displays framework descriptions.

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

use crate::frameworks::FrameworkType;
use crate::tui::{restore_terminal, setup_terminal};

/// Framework option with display information
#[derive(Debug, Clone)]
struct FrameworkOption {
    framework_type: FrameworkType,
    name: &'static str,
    description: &'static str,
    characteristics: &'static str,
}

impl FrameworkOption {
    /// Get all available framework options
    fn all() -> Vec<Self> {
        vec![
            FrameworkOption {
                framework_type: FrameworkType::OhMyZsh,
                name: "oh-my-zsh",
                description: "Most popular zsh framework",
                characteristics: "200+ plugins, large community, extensive documentation",
            },
            FrameworkOption {
                framework_type: FrameworkType::Zimfw,
                name: "zimfw",
                description: "Fast and minimal framework",
                characteristics: "Modular design, low overhead, quick startup",
            },
            FrameworkOption {
                framework_type: FrameworkType::Prezto,
                name: "prezto",
                description: "Feature-rich configuration framework",
                characteristics: "Well-organized modules, intermediate complexity",
            },
            FrameworkOption {
                framework_type: FrameworkType::Zinit,
                name: "zinit",
                description: "Ultra-fast plugin manager",
                characteristics: "Advanced features, powerful, steep learning curve",
            },
            FrameworkOption {
                framework_type: FrameworkType::Zap,
                name: "zap",
                description: "Minimalist plugin manager",
                characteristics: "Simple configuration, beginner-friendly, lightweight",
            },
        ]
    }
}

/// Run interactive framework selection TUI
///
/// Displays a list of supported zsh frameworks with descriptions and
/// allows keyboard navigation to select one.
///
/// # Arguments
///
/// * `profile_name` - Name of the profile being created (shown in title)
///
/// # Returns
///
/// Selected FrameworkType on Enter, or error if cancelled (Esc) or failed
///
/// # Keyboard Controls
///
/// - Up/Down arrows: Navigate list
/// - Enter: Select highlighted framework
/// - Esc: Cancel selection
///
/// # Errors
///
/// - Terminal too small (minimum 80x24)
/// - User cancels with Esc key
/// - Terminal initialization fails
pub fn run_framework_selection(profile_name: &str) -> Result<FrameworkType> {
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
    let result = run_selection_loop(&mut terminal, profile_name);

    // Always restore terminal, even on error
    restore_terminal()?;

    result
}

/// Main event loop for framework selection
fn run_selection_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    profile_name: &str,
) -> Result<FrameworkType> {
    let frameworks = FrameworkOption::all();
    let mut state = ListState::default();
    state.select(Some(0)); // Start with first item selected

    loop {
        // Render UI
        terminal.draw(|f| {
            render_ui(f, &frameworks, &mut state, profile_name);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => select_previous(&mut state, frameworks.len()),
                KeyCode::Down => select_next(&mut state, frameworks.len()),
                KeyCode::Enter => {
                    if let Some(selected) = state.selected() {
                        return Ok(frameworks[selected].framework_type.clone());
                    }
                }
                KeyCode::Esc => {
                    bail!("Framework selection cancelled by user")
                }
                _ => {}
            }
        }
    }
}

/// Render the TUI interface
fn render_ui(
    f: &mut Frame,
    frameworks: &[FrameworkOption],
    state: &mut ListState,
    profile_name: &str,
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
    let title = Paragraph::new(format!("Select Framework for Profile '{}'", profile_name))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Framework list
    let selected_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = frameworks
        .iter()
        .enumerate()
        .map(|(i, fw)| {
            let is_selected = i == selected_idx;
            let indicator = if is_selected { "▸ " } else { "  " };

            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        indicator,
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        fw.name,
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
                    format!("  {}", fw.description),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(Span::styled(
                    format!("  {}", fw.characteristics),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""), // Spacing between items
            ];

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Frameworks"));

    f.render_stateful_widget(list, chunks[1], state);

    // Footer with help text
    let footer = Paragraph::new("↑↓: Navigate | Enter: Select | Esc: Cancel")
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
    fn test_framework_option_count() {
        let frameworks = FrameworkOption::all();
        assert_eq!(frameworks.len(), 5, "Should have exactly 5 framework options");
    }

    #[test]
    fn test_framework_option_names() {
        let frameworks = FrameworkOption::all();
        let names: Vec<&str> = frameworks.iter().map(|f| f.name).collect();

        assert_eq!(names, vec!["oh-my-zsh", "zimfw", "prezto", "zinit", "zap"]);
    }

    #[test]
    fn test_framework_option_has_descriptions() {
        let frameworks = FrameworkOption::all();

        for fw in frameworks {
            assert!(!fw.description.is_empty(), "Framework {} missing description", fw.name);
            assert!(!fw.characteristics.is_empty(), "Framework {} missing characteristics", fw.name);
        }
    }

    #[test]
    fn test_select_next_wrapping() {
        let mut state = ListState::default();
        state.select(Some(4)); // Last item

        select_next(&mut state, 5);
        assert_eq!(state.selected(), Some(0), "Should wrap to first item");
    }

    #[test]
    fn test_select_previous_wrapping() {
        let mut state = ListState::default();
        state.select(Some(0)); // First item

        select_previous(&mut state, 5);
        assert_eq!(state.selected(), Some(4), "Should wrap to last item");
    }

    #[test]
    fn test_select_navigation() {
        let mut state = ListState::default();
        state.select(Some(2));

        select_next(&mut state, 5);
        assert_eq!(state.selected(), Some(3));

        select_previous(&mut state, 5);
        assert_eq!(state.selected(), Some(2));
    }
}
