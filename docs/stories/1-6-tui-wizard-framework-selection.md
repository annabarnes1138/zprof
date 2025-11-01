# Story 1.6: TUI Wizard Framework Selection

Status: ready-for-dev

## Story

As a developer,
I want an interactive menu to select a zsh framework,
so that I can easily choose the framework for my new profile.

## Acceptance Criteria

1. TUI displays list of supported frameworks (oh-my-zsh, zimfw, prezto, zinit, zap)
2. Each framework shows brief description and key characteristics
3. Keyboard navigation with arrow keys and enter to select
4. Selected framework is highlighted visually
5. TUI is responsive and works in 80x24 terminal minimum
6. Supports both light and dark terminal themes

## Tasks / Subtasks

- [ ] Set up Ratatui TUI infrastructure (AC: #3, #4, #5, #6)
  - [ ] Create `tui/mod.rs` module with common TUI utilities
  - [ ] Add Ratatui 0.29.0 and Crossterm 0.29.0 dependencies per architecture
  - [ ] Implement terminal initialization and cleanup functions
  - [ ] Handle terminal raw mode enable/disable properly
  - [ ] Test TUI works on 80x24 minimum terminal size (AC: #5)
- [ ] Define framework selection data model (AC: #1, #2)
  - [ ] Create FrameworkOption struct with name, description, characteristics
  - [ ] Define all 5 frameworks with descriptions:
    - oh-my-zsh: "Most popular, 200+ plugins, large community"
    - zimfw: "Fast and minimal, modular design, low overhead"
    - prezto: "Feature-rich, well-organized, intermediate complexity"
    - zinit: "Ultra-fast plugin manager, advanced features, steep learning curve"
    - zap: "Minimalist, simple configuration, good for beginners"
  - [ ] Store framework options in vector for list rendering
- [ ] Implement framework selection TUI screen (AC: #1, #2, #3, #4)
  - [ ] Create `tui/framework_select.rs` module
  - [ ] Implement `run_framework_selection() -> Result<FrameworkType>` function
  - [ ] Use Ratatui List widget to display frameworks
  - [ ] Show framework name and description for each item (AC: #2)
  - [ ] Implement keyboard navigation (up/down arrows) per AC: #3
  - [ ] Highlight selected framework with visual indicator (AC: #4)
  - [ ] Use consistent colors/styles from Ratatui themes
  - [ ] Return selected FrameworkType on Enter key press
  - [ ] Support Esc key to cancel selection
- [ ] Integrate TUI into create command (AC: All)
  - [ ] Modify `cli/create.rs` to call TUI when no framework detected
  - [ ] Call TUI when user chooses 'n' on import prompt (from Story 1.5)
  - [ ] Handle TUI cancellation gracefully (Esc key -> abort creation)
  - [ ] Store selected framework for use in subsequent wizard steps
  - [ ] Pass selected framework to next step (Story 1.7 - plugin browser)
- [ ] Implement responsive layout (AC: #5, #6)
  - [ ] Use Ratatui Block widget for borders and title
  - [ ] Create layout that fits 80x24 terminal (minimum size)
  - [ ] Add title: "Select Framework for Profile '<name>'"
  - [ ] Add footer with help text: "↑↓: Navigate | Enter: Select | Esc: Cancel"
  - [ ] Test on multiple terminal emulators (iTerm2, Alacritty, Terminal.app)
  - [ ] Verify readability in both light and dark themes (AC: #6)
- [ ] Handle edge cases and errors (AC: All)
  - [ ] Handle terminal too small gracefully (show error, don't crash)
  - [ ] Restore terminal state on panic (use panic handler)
  - [ ] Handle terminal resize events during selection
  - [ ] Use anyhow::Context for error messages per Pattern 2
  - [ ] Log TUI events with env_logger for debugging
- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test FrameworkOption data structure
  - [ ] Unit test keyboard event handling (up/down/enter/esc)
  - [ ] Integration test TUI initialization and cleanup
  - [ ] Test terminal state restoration after TUI exit
  - [ ] Test selection returns correct FrameworkType
  - [ ] Test cancellation returns error (user aborted)
  - [ ] Manual test in 80x24 terminal size (AC: #5)
  - [ ] Manual test in light and dark themes (AC: #6)
  - [ ] Test on macOS Terminal, iTerm2, Alacritty

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `tui/framework_select.rs`, `tui/mod.rs`
- Secondary: `cli/create.rs` (integration point)
- New dependencies: Ratatui 0.29.0, Crossterm 0.29.0 per architecture decision ADR-003
- Follow Pattern 2 (Error Handling) with anyhow::Result
- TUI must work on minimum 80x24 terminal per design constraints (NFR003)

**Ratatui Architecture Pattern:**
```rust
// tui/framework_select.rs structure
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

pub fn run_framework_selection(profile_name: &str) -> Result<FrameworkType> {
    // 1. Initialize terminal
    let mut terminal = setup_terminal()?;

    // 2. Run event loop
    let result = run_selection_loop(&mut terminal, profile_name);

    // 3. Cleanup terminal (always runs)
    restore_terminal()?;

    result
}

fn run_selection_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>, profile_name: &str) -> Result<FrameworkType> {
    let mut state = ListState::default();
    state.select(Some(0)); // Start with first item selected

    loop {
        // Render UI
        terminal.draw(|f| {
            render_framework_list(f, &state, profile_name);
        })?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => select_previous(&mut state),
                KeyCode::Down => select_next(&mut state),
                KeyCode::Enter => return Ok(get_selected_framework(&state)),
                KeyCode::Esc => bail!("Framework selection cancelled"),
                _ => {}
            }
        }
    }
}
```

**Framework Options Data:**
```rust
struct FrameworkOption {
    framework_type: FrameworkType,
    name: &'static str,
    description: &'static str,
    characteristics: &'static str,
}

fn get_framework_options() -> Vec<FrameworkOption> {
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
```

**TUI Layout Example:**
```
┌─ Select Framework for Profile 'experimental' ──────────┐
│                                                         │
│  ▸ oh-my-zsh                                           │
│    Most popular zsh framework                          │
│    200+ plugins, large community, extensive docs       │
│                                                         │
│    zimfw                                               │
│    Fast and minimal framework                          │
│    Modular design, low overhead, quick startup         │
│                                                         │
│    prezto                                              │
│    Feature-rich configuration framework                │
│    Well-organized modules, intermediate complexity     │
│                                                         │
│    zinit                                               │
│    Ultra-fast plugin manager                           │
│    Advanced features, powerful, steep learning curve   │
│                                                         │
│    zap                                                 │
│    Minimalist plugin manager                           │
│    Simple configuration, beginner-friendly             │
│                                                         │
└─────────────────────────────────────────────────────────┘
 ↑↓: Navigate | Enter: Select | Esc: Cancel
```

**Terminal Initialization Pattern:**
```rust
use std::io;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

// CRITICAL: Use panic handler to restore terminal on crashes
fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        restore_terminal().ok(); // Best effort
        original_hook(panic);
    }));
}
```

**Integration with create command:**
```rust
// In cli/create.rs
use crate::tui::framework_select;

pub fn execute(args: CreateArgs) -> Result<()> {
    // ... framework detection code from Story 1.5 ...

    let selected_framework = if let Some(detected) = detected_framework {
        // Prompt to import (Story 1.5 logic)
        if user_confirms_import() {
            detected.framework_type
        } else {
            // User declined import, launch TUI wizard
            framework_select::run_framework_selection(&args.name)?
        }
    } else {
        // No framework detected, launch TUI wizard
        framework_select::run_framework_selection(&args.name)?
    };

    // Continue with selected framework...
    // Next: Story 1.7 (plugin browser)
    Ok(())
}
```

**Dependencies to Add:**
```toml
[dependencies]
ratatui = "0.29.0"
crossterm = "0.29.0"
```

**Error Handling:**
```rust
// Terminal too small
if terminal.size()?.width < 80 || terminal.size()?.height < 24 {
    restore_terminal()?;
    bail!("✗ Error: Terminal too small\n  → Minimum size: 80x24\n  → Current size: {}x{}",
        terminal.size()?.width, terminal.size()?.height);
}

// TUI cancelled
framework_select::run_framework_selection(&name)
    .context("Framework selection cancelled. Profile creation aborted.")?;
```

### Project Structure Notes

**New Files Created:**
- `src/tui/mod.rs` - TUI module initialization, terminal setup/cleanup utilities
- `src/tui/framework_select.rs` - Framework selection screen implementation
- `tests/tui_framework_select_test.rs` - Integration tests for TUI

**Modified Files:**
- `src/cli/create.rs` - Integrate TUI wizard when no framework detected or import declined
- `Cargo.toml` - Add Ratatui and Crossterm dependencies
- `src/main.rs` - Install panic hook for terminal restoration

**Integration Points:**
- Called from `cli/create.rs` when framework detection returns None (no framework found)
- Called from `cli/create.rs` when user declines import prompt (Story 1.5)
- Passes selected FrameworkType to Story 1.7 (plugin browser)
- Uses FrameworkType enum from `frameworks/mod.rs` (Story 1.4)

**TUI Flow in Context:**
```
User: zprof create experimental
  ↓
Story 1.5: Detect framework → None found
  ↓
Story 1.6: TUI Framework Selection → User selects "zimfw"
  ↓
Story 1.7: TUI Plugin Browser (next story)
  ↓
Story 1.8: TUI Theme Selection (next story)
  ↓
Profile created with zimfw
```

### Learnings from Previous Story

**From Story 1.5 (Status: drafted)**

Story 1.5 implements the import flow for detected frameworks. Story 1.6 handles the alternative path when:
1. No framework is detected by Story 1.4
2. User declines import prompt in Story 1.5

**Integration Requirements:**
- Story 1.6 must use the same `FrameworkType` enum from `frameworks/mod.rs`
- Return value from TUI must be compatible with Story 1.5's code flow
- Both paths (import vs TUI wizard) should converge to same profile creation logic
- Error handling must follow same patterns (anyhow::Result, user-friendly messages)

**Expected Call Pattern from Story 1.5:**
```rust
// cli/create.rs (from Story 1.5)
let selected_framework = match detected_framework {
    Some(info) => {
        if prompt_import_confirmation()? {
            info.framework_type  // Import path
        } else {
            framework_select::run_framework_selection(&args.name)?  // Story 1.6 path
        }
    }
    None => {
        framework_select::run_framework_selection(&args.name)?  // Story 1.6 path
    }
};
```

**Critical Handoff:**
After Story 1.6 completes and returns selected FrameworkType, execution continues to:
- Story 1.7: Plugin selection TUI (next wizard step)
- Story 1.8: Theme selection TUI and profile generation (final wizard step)

The wizard is a multi-step flow where each story builds on the previous selection.

### References

- [Source: docs/epics.md#Story-1.6]
- [Source: docs/PRD.md#FR007-interactive-TUI-wizard]
- [Source: docs/architecture.md#Ratatui-0.29.0-Crossterm-0.29.0]
- [Source: docs/architecture.md#ADR-003-Use-Ratatui-for-TUI]
- [Source: docs/architecture.md#Design-Constraints-terminal-80x24]
- [Source: docs/architecture.md#NFR003-TUI-responsive]
- [Source: docs/architecture.md#Epic-1-Story-1.6-Mapping]
- [Source: docs/architecture.md#Pattern-2-Error-Handling]

## Dev Agent Record

### Context Reference

- docs/stories/1-6-tui-wizard-framework-selection.context.xml

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
