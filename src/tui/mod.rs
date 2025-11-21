//! Terminal User Interface (TUI) module for interactive wizards
//!
//! This module provides TUI functionality using Ratatui for framework and
//! plugin selection, theme browsing, and other interactive workflows.

pub mod framework_select;
pub mod plugin_browser;
pub mod prompt_mode_select;
pub mod theme_select;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Setup terminal for TUI rendering
///
/// Enables raw mode and enters alternate screen buffer.
/// MUST be paired with restore_terminal() call, even on errors.
///
/// # Errors
///
/// Returns error if terminal cannot be initialized (e.g., not running in a terminal)
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore terminal to normal state
///
/// Disables raw mode and exits alternate screen buffer.
/// Should be called even if TUI encounters errors.
///
/// # Errors
///
/// Returns error if terminal state cannot be restored.
/// In practice, this is best-effort and should not fail.
pub fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

/// Install panic hook to restore terminal on crashes
///
/// CRITICAL: This ensures terminal state is restored even if the TUI panics.
/// Call this once at application startup.
pub fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = restore_terminal(); // Best effort - ignore errors
        original_hook(panic);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panic_hook_installation() {
        // Test that panic hook can be installed without panicking
        install_panic_hook();
        // Re-install to test idempotency
        install_panic_hook();
    }
}
