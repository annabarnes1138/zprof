//! Prompt mode selection TUI screen
//!
//! Provides a binary choice interface for selecting between standalone prompt
//! engines (Starship, Powerlevel10k, etc.) and framework built-in themes.

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

/// Prompt mode choice options
#[derive(Debug, Clone)]
struct PromptModeChoice {
    title: &'static str,
    description: &'static str,
    details: &'static str,
    mode: PromptModeType,
}

/// Prompt mode type selection result
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromptModeType {
    PromptEngine,
    FrameworkTheme,
}

/// Run interactive prompt mode selection TUI
///
/// Displays a binary choice between standalone prompt engines and framework themes
/// with clear help text explaining the differences.
///
/// # Returns
///
/// Selected `PromptModeType` on Enter, or error if cancelled (Esc) or failed
///
/// # Keyboard Controls
///
/// - Up/Down arrows: Navigate between options
/// - Enter: Select highlighted option
/// - Esc: Cancel selection
///
/// # Errors
///
/// - Terminal too small (minimum 80x24)
/// - User cancels with Esc key
/// - Terminal initialization fails
pub fn run_prompt_mode_selection() -> Result<PromptModeType> {
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

/// Main event loop for prompt mode selection
fn run_selection_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<PromptModeType> {
    let choices = get_prompt_mode_choices();

    let mut state = ListState::default();
    state.select(Some(0)); // Start with first item selected

    loop {
        // Render UI
        terminal.draw(|f| {
            render_ui(f, &choices, &mut state);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => select_previous(&mut state, choices.len()),
                KeyCode::Down => select_next(&mut state, choices.len()),
                KeyCode::Enter => {
                    if let Some(selected) = state.selected() {
                        return Ok(choices[selected].mode);
                    }
                }
                KeyCode::Esc => {
                    bail!("Prompt mode selection cancelled by user")
                }
                _ => {}
            }
        }
    }
}

/// Render the TUI interface
fn render_ui(
    f: &mut Frame,
    choices: &[PromptModeChoice],
    state: &mut ListState,
) {
    // Create main layout: title, help text, list, footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(8),  // Help text
            Constraint::Min(8),     // List
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("How do you want to handle your prompt?")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Help text explaining the difference
    let help_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Prompt Engines:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" Independent tools (Starship, Powerlevel10k) that", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled("replace the framework's theme system with advanced features.", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Framework Themes:", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" Built-in themes from your framework", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled("(robbyrussell, agnoster, etc.) - simpler, integrated approach.", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
    ];

    let help = Paragraph::new(help_lines)
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("About"));
    f.render_widget(help, chunks[1]);

    // Choice list
    let selected_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = choices
        .iter()
        .enumerate()
        .map(|(i, choice)| {
            let is_selected = i == selected_idx;
            let indicator = if is_selected { "▸ " } else { "  " };

            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        indicator,
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        choice.title,
                        if is_selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        },
                    ),
                ]),
                Line::from(vec![Span::styled(
                    format!("    {}", choice.description),
                    Style::default().fg(Color::Gray),
                )]),
                Line::from(vec![Span::styled(
                    format!("    {}", choice.details),
                    Style::default().fg(Color::DarkGray),
                )]),
                Line::from(""), // Blank line for spacing
            ];

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Choose Your Approach"))
        .style(Style::default().fg(Color::White));
    f.render_stateful_widget(list, chunks[2], state);

    // Footer
    let footer = Paragraph::new("↑/↓: Navigate | Enter: Select | Esc: Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}

/// Get the available prompt mode choices
fn get_prompt_mode_choices() -> Vec<PromptModeChoice> {
    vec![
        PromptModeChoice {
            title: "Standalone prompt engine",
            description: "Use a separate prompt tool (Starship, Powerlevel10k, Pure...)",
            details: "Advanced, cross-shell compatible, highly customizable",
            mode: PromptModeType::PromptEngine,
        },
        PromptModeChoice {
            title: "Framework's built-in themes",
            description: "Use your framework's theme system (robbyrussell, agnoster...)",
            details: "Simple, integrated, framework-specific",
            mode: PromptModeType::FrameworkTheme,
        },
    ]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_prompt_mode_choices() {
        let choices = get_prompt_mode_choices();
        assert_eq!(choices.len(), 2);
        assert_eq!(choices[0].mode, PromptModeType::PromptEngine);
        assert_eq!(choices[1].mode, PromptModeType::FrameworkTheme);
    }

    #[test]
    fn test_select_previous_wrapping() {
        let mut state = ListState::default();
        state.select(Some(0));
        select_previous(&mut state, 2);
        assert_eq!(state.selected(), Some(1)); // Wraps to end
    }

    #[test]
    fn test_select_next_wrapping() {
        let mut state = ListState::default();
        state.select(Some(1));
        select_next(&mut state, 2);
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
}
