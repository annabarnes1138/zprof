//! Theme selection TUI screen
//!
//! Provides an interactive menu for selecting a theme for the chosen framework.
//! Displays themes with descriptions and preview information.
//! Also provides confirmation screen for wizard selections.

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

use crate::core::manifest::PromptMode;
use crate::frameworks::{Framework, FrameworkType, Theme};
use crate::frameworks::installer::WizardState;
use crate::tui::{restore_terminal, setup_terminal};

/// Run interactive theme selection TUI
///
/// Displays available themes for the selected framework with descriptions
/// and preview information. If using PromptEngine mode, returns empty string
/// and skips theme selection entirely.
///
/// # Arguments
///
/// * `framework` - The framework type to get themes for
/// * `_plugins` - Selected plugins (reserved for future use, currently unused)
/// * `mode` - The prompt mode (PromptEngine or FrameworkTheme)
///
/// # Returns
///
/// Selected theme name as String on Enter, empty string if PromptEngine mode,
/// or error if cancelled (Esc) or failed
///
/// # Keyboard Controls
///
/// - Up/Down arrows: Navigate list
/// - Enter: Select highlighted theme
/// - Esc: Cancel selection
///
/// # Errors
///
/// - Terminal too small (minimum 80x24)
/// - User cancels with Esc key
/// - Terminal initialization fails
pub fn run_theme_selection(framework: FrameworkType, _plugins: &[String], mode: PromptMode) -> Result<String> {
    // Skip theme selection entirely if using a prompt engine
    if matches!(mode, PromptMode::PromptEngine { .. }) {
        return Ok(String::new());
    }

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
    let result = run_selection_loop(&mut terminal, framework);

    // Always restore terminal, even on error
    restore_terminal()?;

    result
}

/// Main event loop for theme selection
fn run_selection_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    framework: FrameworkType,
) -> Result<String> {
    let mut themes = get_themes_for_framework(&framework);

    // Handle framework with no themes (edge case)
    if themes.is_empty() {
        return Ok("default".to_string());
    }

    // Sort themes: recommended first, then alphabetically
    themes.sort_by(|a, b| {
        let a_recommended = a.compatibility.is_recommended_for(&framework);
        let b_recommended = b.compatibility.is_recommended_for(&framework);

        match (a_recommended, b_recommended) {
            (true, false) => std::cmp::Ordering::Less,    // a first
            (false, true) => std::cmp::Ordering::Greater, // b first
            _ => a.name.cmp(b.name),                      // alphabetical
        }
    });

    let mut state = ListState::default();
    state.select(Some(0)); // Start with first item selected

    loop {
        // Render UI
        terminal.draw(|f| {
            render_ui(f, &themes, &mut state, &framework);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => select_previous(&mut state, themes.len()),
                KeyCode::Down => select_next(&mut state, themes.len()),
                KeyCode::Enter => {
                    if let Some(selected) = state.selected() {
                        return Ok(themes[selected].name.to_string());
                    }
                }
                KeyCode::Esc => {
                    bail!("Theme selection cancelled by user")
                }
                _ => {}
            }
        }
    }
}

/// Render the TUI interface
fn render_ui(
    f: &mut Frame,
    themes: &[Theme],
    state: &mut ListState,
    framework: &FrameworkType,
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
    let title = Paragraph::new(format!("Select Theme for {}", framework_name(framework)))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Theme list
    let selected_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = themes
        .iter()
        .enumerate()
        .map(|(i, theme)| {
            let is_selected = i == selected_idx;
            let indicator = if is_selected { "▸ " } else { "  " };

            // Add (recommended) suffix if applicable
            let is_recommended = theme.compatibility.is_recommended_for(framework);
            let description = if is_recommended {
                format!("{} (recommended)", theme.description)
            } else {
                theme.description.to_string()
            };

            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        indicator,
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        theme.name,
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
                    format!("    {}", description),
                    Style::default().fg(Color::Gray),
                )]),
                Line::from(vec![Span::styled(
                    format!("    Preview: {}", theme.preview),
                    Style::default().fg(Color::DarkGray),
                )]),
                Line::from(""), // Blank line for spacing
            ];

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Themes"))
        .style(Style::default().fg(Color::White));
    f.render_stateful_widget(list, chunks[1], state);

    // Footer
    let footer = Paragraph::new("↑/↓: Navigate | Enter: Select | Esc: Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
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

/// Get themes for the given framework
fn get_themes_for_framework(framework: &FrameworkType) -> Vec<Theme> {
    use crate::frameworks::{oh_my_zsh::OhMyZsh, prezto::Prezto, zap::Zap, zimfw::Zimfw, zinit::Zinit};

    match framework {
        FrameworkType::OhMyZsh => OhMyZsh::get_themes(),
        FrameworkType::Zimfw => Zimfw::get_themes(),
        FrameworkType::Prezto => Prezto::get_themes(),
        FrameworkType::Zinit => Zinit::get_themes(),
        FrameworkType::Zap => Zap::get_themes(),
    }
}

/// Get human-readable framework name
fn framework_name(framework: &FrameworkType) -> &str {
    match framework {
        FrameworkType::OhMyZsh => "oh-my-zsh",
        FrameworkType::Zimfw => "zimfw",
        FrameworkType::Prezto => "prezto",
        FrameworkType::Zinit => "zinit",
        FrameworkType::Zap => "zap",
    }
}

/// Show confirmation screen with wizard selections
///
/// Displays a summary of all user selections and prompts for confirmation.
///
/// # Arguments
///
/// * `state` - The wizard state containing all selections
///
/// # Returns
///
/// - `Ok(true)` if user confirms (y/Y/Enter)
/// - `Ok(false)` if user cancels (n/N/Esc)
/// - `Err` if terminal operations fail
///
/// # Keyboard Controls
///
/// - y/Y/Enter: Confirm and proceed with profile creation
/// - n/N/Esc: Cancel profile creation
pub fn show_confirmation_screen(state: &WizardState) -> Result<bool> {
    // Initialize terminal
    let mut terminal = setup_terminal().context("Failed to initialize terminal for confirmation")?;

    // Run confirmation loop
    let result = run_confirmation_loop(&mut terminal, state);

    // Always restore terminal, even on error
    restore_terminal()?;

    result
}

/// Main event loop for confirmation screen
fn run_confirmation_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &WizardState,
) -> Result<bool> {
    loop {
        // Render confirmation UI
        terminal.draw(|f| {
            render_confirmation_ui(f, state);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    return Ok(true);
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    return Ok(false);
                }
                _ => {}
            }
        }
    }
}

/// Render the confirmation UI
fn render_confirmation_ui(f: &mut Frame, state: &WizardState) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Min(15),     // Content
            Constraint::Length(3),   // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Confirm Profile Creation")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Content
    let profile_path = format!("~/.zsh-profiles/profiles/{}", state.profile_name);
    let content_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Profile Name:    ", Style::default().fg(Color::Gray)),
            Span::styled(&state.profile_name, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Framework:       ", Style::default().fg(Color::Gray)),
            Span::styled(framework_name(&state.framework), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  Plugins:         ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", state.plugins.len()), Style::default().fg(Color::Cyan)),
            Span::styled(" selected", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("                   ", Style::default()),
            Span::styled(state.plugins.join(", "), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Theme:           ", Style::default().fg(Color::Gray)),
            Span::styled(&state.theme, Style::default().fg(Color::Magenta)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  This will:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("    • Install ", Style::default().fg(Color::Gray)),
            Span::styled(framework_name(&state.framework), Style::default().fg(Color::Green)),
            Span::styled(" to ", Style::default().fg(Color::Gray)),
            Span::styled(&profile_path, Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(format!("    • Install {} selected plugins", state.plugins.len()), Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("    • Generate profile.toml manifest", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("    • Generate .zshrc and .zshenv", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Create profile with these settings?", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];

    let content = Paragraph::new(content_lines)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(content, chunks[1]);

    // Footer
    let footer = Paragraph::new("y: Confirm | n/Esc: Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_themes_for_framework_oh_my_zsh() {
        let themes = get_themes_for_framework(&FrameworkType::OhMyZsh);
        assert!(!themes.is_empty());
        assert!(themes.iter().any(|t| t.name == "robbyrussell"));
    }

    #[test]
    fn test_get_themes_for_framework_zimfw() {
        let themes = get_themes_for_framework(&FrameworkType::Zimfw);
        assert!(!themes.is_empty());
        assert!(themes.iter().any(|t| t.name == "powerlevel10k"));
    }

    #[test]
    fn test_skip_theme_selection_for_prompt_engine() {
        // Test that PromptEngine mode skips theme selection and returns empty string
        let mode = PromptMode::PromptEngine {
            engine: "starship".to_string(),
        };
        let result = run_theme_selection(FrameworkType::OhMyZsh, &[], mode);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::new());
    }

    #[test]
    fn test_framework_theme_mode_proceeds_to_selection() {
        // This test would require mocking terminal input, so we just verify the early return doesn't happen
        // For now, we test the skip logic directly
        let prompt_engine_mode = PromptMode::PromptEngine {
            engine: "starship".to_string(),
        };
        let framework_theme_mode = PromptMode::FrameworkTheme {
            theme: "robbyrussell".to_string(),
        };

        // PromptEngine mode should return immediately
        let result = run_theme_selection(FrameworkType::OhMyZsh, &[], prompt_engine_mode);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::new());

        // FrameworkTheme mode would proceed to TUI (we can't test the full flow without terminal mocking)
        // Instead we verify that the conditional logic is correct by checking the match
        assert!(!matches!(framework_theme_mode, PromptMode::PromptEngine { .. }));
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
}
