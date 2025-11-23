//! Preset selection TUI screen
//!
//! Provides an interactive card-based menu for selecting presets or choosing custom setup.
//! Follows the same pattern as framework_select.rs for consistency.

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

use crate::presets::{Preset, PRESET_REGISTRY};
use crate::tui::{restore_terminal, setup_terminal};

/// User's choice from preset selection
#[derive(Debug, Clone, PartialEq)]
pub enum PresetChoice {
    /// User selected a specific preset
    Preset(&'static Preset),
    /// User wants to customize (go to full wizard)
    Custom,
}

/// Option in the preset selector (preset or custom)
#[derive(Debug, Clone)]
struct SelectionOption {
    kind: SelectionKind,
    display_name: String,
    icon: String,
    description: String,
    details: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum SelectionKind {
    Preset(usize), // index into PRESET_REGISTRY
    Custom,
}

impl SelectionOption {
    /// Get all selection options (presets + custom)
    fn all() -> Vec<Self> {
        let mut options: Vec<SelectionOption> = PRESET_REGISTRY
            .iter()
            .enumerate()
            .map(|(idx, preset)| {
                let details = vec![
                    format!("Framework: {}", preset.config.framework.name()),
                    format!(
                        "Prompt: {}",
                        preset
                            .config
                            .prompt_engine
                            .or(preset.config.framework_theme)
                            .unwrap_or("default")
                    ),
                    format!("Plugins: {} configured", preset.config.plugins.len()),
                    format!("Target: {}", preset.target_user),
                ];

                SelectionOption {
                    kind: SelectionKind::Preset(idx),
                    display_name: preset.name.to_string(),
                    icon: preset.icon.to_string(),
                    description: preset.description.to_string(),
                    details,
                }
            })
            .collect();

        // Add "Customize (advanced)" option at the end
        options.push(SelectionOption {
            kind: SelectionKind::Custom,
            display_name: "Customize (advanced)".to_string(),
            icon: "⚙️".to_string(),
            description: "Choose your own framework, plugins, and theme".to_string(),
            details: vec![
                "Full control over all options".to_string(),
                "Browse plugin catalog".to_string(),
                "Advanced configuration".to_string(),
            ],
        });

        options
    }
}

/// Run interactive preset selection TUI
///
/// Displays preset cards with full details and allows keyboard navigation.
/// Highlights the "Minimal" preset as recommended by default.
///
/// # Returns
///
/// PresetChoice indicating selected preset or custom setup, or error if cancelled
///
/// # Keyboard Controls
///
/// - Up/Down arrows: Navigate options
/// - Enter: Select highlighted option
/// - Esc: Cancel selection
///
/// # Errors
///
/// - Terminal too small (minimum 80x24)
/// - User cancels with Esc key
/// - Terminal initialization fails
pub fn select_preset() -> Result<PresetChoice> {
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

/// Main event loop for preset selection
fn run_selection_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<PresetChoice> {
    let options = SelectionOption::all();
    let mut state = ListState::default();

    // Default to "Minimal" preset (index 0 in PRESET_REGISTRY)
    // Find it in options (should be first)
    let minimal_idx = options
        .iter()
        .position(|opt| {
            if let SelectionKind::Preset(idx) = opt.kind {
                PRESET_REGISTRY[idx].id == "minimal"
            } else {
                false
            }
        })
        .unwrap_or(0);

    state.select(Some(minimal_idx));

    loop {
        // Render UI
        terminal.draw(|f| {
            render_ui(f, &options, &mut state);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => select_previous(&mut state, options.len()),
                KeyCode::Down => select_next(&mut state, options.len()),
                KeyCode::Enter => {
                    if let Some(selected) = state.selected() {
                        return match &options[selected].kind {
                            SelectionKind::Preset(idx) => {
                                Ok(PresetChoice::Preset(&PRESET_REGISTRY[*idx]))
                            }
                            SelectionKind::Custom => Ok(PresetChoice::Custom),
                        };
                    }
                }
                KeyCode::Esc => {
                    bail!("Preset selection cancelled by user")
                }
                _ => {}
            }
        }
    }
}

/// Render the TUI interface with preset cards
fn render_ui(
    f: &mut Frame,
    options: &[SelectionOption],
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
    let title = Paragraph::new("Select a Preset or Customize Your Setup")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Preset cards list
    let selected_idx = state.selected().unwrap_or(0);
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            let is_selected = i == selected_idx;
            let is_recommended = if let SelectionKind::Preset(idx) = option.kind {
                PRESET_REGISTRY[idx].id == "minimal"
            } else {
                false
            };

            // Build card lines
            let mut lines = Vec::new();

            // Card header with icon and name
            let indicator = if is_selected { "▸ " } else { "  " };
            let recommended_tag = if is_recommended { " (recommended)" } else { "" };

            lines.push(Line::from(vec![
                Span::styled(
                    indicator,
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::raw(&option.icon),
                Span::raw(" "),
                Span::styled(
                    format!("{}{}", option.display_name, recommended_tag),
                    if is_selected {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                    },
                ),
            ]));

            // Description
            lines.push(Line::from(Span::styled(
                format!("  {}", option.description),
                Style::default().fg(Color::Gray),
            )));

            // Empty line for spacing
            lines.push(Line::from(""));

            // Details
            for detail in &option.details {
                lines.push(Line::from(Span::styled(
                    format!("  {detail}"),
                    Style::default().fg(Color::DarkGray),
                )));
            }

            // Card separator
            lines.push(Line::from(""));

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Presets"));

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
    fn test_selection_options_count() {
        let options = SelectionOption::all();
        // Should have 4 presets + 1 custom option
        assert_eq!(options.len(), 5, "Should have 4 presets + 1 custom option");
    }

    #[test]
    fn test_last_option_is_custom() {
        let options = SelectionOption::all();
        let last = options.last().expect("Should have at least one option");

        assert_eq!(last.kind, SelectionKind::Custom);
        assert_eq!(last.display_name, "Customize (advanced)");
    }

    #[test]
    fn test_all_presets_included() {
        let options = SelectionOption::all();

        // Count preset options
        let preset_count = options
            .iter()
            .filter(|opt| matches!(opt.kind, SelectionKind::Preset(_)))
            .count();

        assert_eq!(
            preset_count,
            PRESET_REGISTRY.len(),
            "All presets should be included"
        );
    }

    #[test]
    fn test_minimal_preset_exists() {
        let options = SelectionOption::all();

        let minimal = options.iter().find(|opt| {
            if let SelectionKind::Preset(idx) = opt.kind {
                PRESET_REGISTRY[idx].id == "minimal"
            } else {
                false
            }
        });

        assert!(minimal.is_some(), "Minimal preset should be in options");
    }

    #[test]
    fn test_preset_details_format() {
        let options = SelectionOption::all();

        for option in options {
            // All options should have details
            assert!(
                !option.details.is_empty(),
                "Option {} should have details",
                option.display_name
            );

            // Icon and display name should not be empty
            assert!(!option.icon.is_empty());
            assert!(!option.display_name.is_empty());
            assert!(!option.description.is_empty());
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

    #[test]
    fn test_preset_choice_variants() {
        // Test that PresetChoice enum has expected variants
        let preset_choice = PresetChoice::Preset(&PRESET_REGISTRY[0]);
        assert!(matches!(preset_choice, PresetChoice::Preset(_)));

        let custom_choice = PresetChoice::Custom;
        assert!(matches!(custom_choice, PresetChoice::Custom));
    }
}
