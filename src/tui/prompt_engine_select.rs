//! Prompt engine selection TUI screen
//!
//! Provides an interactive menu for selecting a standalone prompt engine.
//! Displays engines with descriptions and shows warnings for Nerd Font requirements.

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

use crate::prompts::PromptEngine;
use crate::tui::{restore_terminal, setup_terminal};

/// Run interactive prompt engine selection TUI
///
/// Displays available standalone prompt engines with descriptions and requirements.
/// Shows warning indicators for engines requiring Nerd Fonts.
///
/// # Returns
///
/// Selected PromptEngine on Enter, or error if cancelled (Esc) or failed
///
/// # Keyboard Controls
///
/// - Up/Down arrows: Navigate list
/// - Enter: Select highlighted engine
/// - Esc: Cancel selection
///
/// # Errors
///
/// - Terminal too small (minimum 80x24)
/// - User cancels with Esc key
/// - Terminal initialization fails
pub fn run_prompt_engine_selection() -> Result<PromptEngine> {
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

/// Main event loop for prompt engine selection
fn run_selection_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<PromptEngine> {
    let engines = get_all_engines();
    let mut state = ListState::default();
    state.select(Some(0)); // Start with first item selected

    loop {
        // Render UI
        terminal.draw(|f| {
            render_ui(f, &engines, &mut state);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => select_previous(&mut state, engines.len()),
                KeyCode::Down => select_next(&mut state, engines.len()),
                KeyCode::Enter => {
                    if let Some(selected) = state.selected() {
                        return Ok(engines[selected].clone());
                    }
                }
                KeyCode::Esc => {
                    bail!("Prompt engine selection cancelled by user")
                }
                _ => {}
            }
        }
    }
}

/// Render the TUI interface
fn render_ui(
    f: &mut Frame,
    engines: &[PromptEngine],
    state: &mut ListState,
) {
    // Create main layout: title, list, footer, warning
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // List
            Constraint::Length(5),  // Warning
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Select a Prompt Engine")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Engine list
    let selected_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = engines
        .iter()
        .enumerate()
        .map(|(i, engine)| {
            let is_selected = i == selected_idx;
            let indicator = if is_selected { "▸ " } else { "  " };
            let metadata = engine.metadata();

            // Build description with features
            let mut features = vec![];
            if metadata.cross_shell {
                features.push("cross-shell");
            } else {
                features.push("zsh-only");
            }
            features.push("async");

            let description = format!("{} ({})", metadata.description, features.join(", "));

            // Nerd Font indicator
            let nerd_font_indicator = if metadata.requires_nerd_font {
                " ⚠ Nerd Font"
            } else {
                ""
            };

            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        indicator,
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        metadata.name,
                        if is_selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        },
                    ),
                    Span::styled(
                        nerd_font_indicator,
                        Style::default().fg(Color::Red),
                    ),
                ]),
                Line::from(vec![Span::styled(
                    format!("    {}", description),
                    Style::default().fg(Color::Gray),
                )]),
                Line::from(""), // Blank line for spacing
            ];

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Engines"))
        .style(Style::default().fg(Color::White));
    f.render_stateful_widget(list, chunks[1], state);

    // Warning box
    let selected_engine = &engines[selected_idx];
    let warning_text = if selected_engine.requires_nerd_font() {
        vec![
            Line::from(vec![
                Span::styled("⚠ ", Style::default().fg(Color::Red)),
                Span::styled(
                    "This engine requires Nerd Fonts",
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![Span::styled(
                "  Install from: https://www.nerdfonts.com/",
                Style::default().fg(Color::Gray),
            )]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("✓ ", Style::default().fg(Color::Green)),
                Span::styled(
                    "No special font requirements",
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ]
    };

    let warning = Paragraph::new(warning_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Requirements"));
    f.render_widget(warning, chunks[2]);

    // Footer
    let footer = Paragraph::new("↑/↓: Navigate | Enter: Select | Esc: Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}

/// Move selection to previous item (with wrapping)
fn select_previous(state: &mut ListState, list_len: usize) {
    if list_len == 0 {
        return;
    }
    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                list_len - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}

/// Move selection to next item (with wrapping)
fn select_next(state: &mut ListState, list_len: usize) {
    if list_len == 0 {
        return;
    }
    let i = match state.selected() {
        Some(i) => {
            if i >= list_len - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}

/// Get all available prompt engines in display order
fn get_all_engines() -> Vec<PromptEngine> {
    vec![
        PromptEngine::Starship,
        PromptEngine::Powerlevel10k,
        PromptEngine::OhMyPosh,
        PromptEngine::Pure,
        PromptEngine::Spaceship,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_engines() {
        let engines = get_all_engines();
        assert_eq!(engines.len(), 5);
        assert_eq!(engines[0], PromptEngine::Starship);
        assert_eq!(engines[1], PromptEngine::Powerlevel10k);
        assert_eq!(engines[2], PromptEngine::OhMyPosh);
        assert_eq!(engines[3], PromptEngine::Pure);
        assert_eq!(engines[4], PromptEngine::Spaceship);
    }

    #[test]
    fn test_select_previous_wrapping() {
        let mut state = ListState::default();
        state.select(Some(0));
        select_previous(&mut state, 5);
        assert_eq!(state.selected(), Some(4)); // Wraps to end
    }

    #[test]
    fn test_select_next_wrapping() {
        let mut state = ListState::default();
        state.select(Some(4));
        select_next(&mut state, 5);
        assert_eq!(state.selected(), Some(0)); // Wraps to beginning
    }

    #[test]
    fn test_select_empty_list() {
        let mut state = ListState::default();
        select_previous(&mut state, 0);
        assert_eq!(state.selected(), None);
        select_next(&mut state, 0);
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn test_engine_order() {
        // Verify engines appear in the expected order
        let engines = get_all_engines();

        // Starship should be first (cross-shell, popular)
        assert_eq!(engines[0].name(), "Starship");

        // P10k second (popular, zsh-only)
        assert_eq!(engines[1].name(), "Powerlevel10k");

        // Pure should not require Nerd Font
        let pure = engines.iter().find(|e| e.name() == "Pure").unwrap();
        assert!(!pure.requires_nerd_font());
    }

    #[test]
    fn test_nerd_font_warnings() {
        let engines = get_all_engines();

        // Verify Nerd Font requirements for each engine
        let starship = &engines[0];
        assert!(starship.requires_nerd_font());

        let p10k = &engines[1];
        assert!(p10k.requires_nerd_font());

        let pure = engines.iter().find(|e| e.name() == "Pure").unwrap();
        assert!(!pure.requires_nerd_font());
    }
}
