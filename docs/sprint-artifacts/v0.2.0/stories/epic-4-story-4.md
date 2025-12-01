# Story 4.4: Create Font Selection TUI

Status: ready-for-dev

## Story

As a user who needs a Nerd Font,
I want to choose which font to install,
so that I can pick one that matches my preferences.

## Acceptance Criteria

1. Create `src/tui/font_select.rs`
2. Show when Nerd Font is required but not detected
3. Display font options as cards with details
4. Highlight recommended fonts
5. Show preview characters for each font
6. Allow skipping installation
7. Keyboard navigation (↑↓, Enter, Esc)
8. Add help text explaining why Nerd Fonts are needed

## Dev Agent Context

### Story Requirements from Epic

This story creates the TUI interface for font selection when a Nerd Font is required but not detected. It must integrate seamlessly into the profile creation workflow after prompt engine selection.

**Key User Flow:**
1. User selects a prompt engine that requires Nerd Fonts (Starship, Powerlevel10k, Oh-My-Posh, Spaceship)
2. Font detection runs and finds no Nerd Fonts installed
3. This TUI displays showing available fonts with rich details
4. User selects a font or skips
5. Selection returns to calling code for download/installation

**Expected TUI Output Format:**
```
Select a Nerd Font to install:

> ✨ FiraCode Nerd Font (recommended)
  Programming ligatures, clean and modern
  Preview: ⚡ ⬢  →  ✓

  JetBrainsMono Nerd Font (recommended)
  Designed for developers, excellent readability
  Preview: ⚡ ⬢  →  ✓

  Meslo Nerd Font (recommended for Powerlevel10k)
  Optimized for Powerlevel10k prompt
  Preview: ⚡ ⬢  →  ✓

  Skip font installation (prompt may not display correctly)
```

### Architecture Compliance

**Module Location:** `src/tui/font_select.rs` (NEW)
- Follows existing TUI pattern established in `src/tui/preset_select.rs`, `src/tui/framework_select.rs`
- Uses `ratatui` crate with CrosstermBackend (already in dependencies)
- Imports from `crate::fonts::nerd_fonts` for font registry
- Must export public API through `src/tui/mod.rs`

**TUI Architectural Pattern** (from existing code):
```rust
// 1. Public function returns Result with selection or error
pub fn select_font() -> Result<FontChoice>

// 2. Enum for user choice (selected font or skip)
pub enum FontChoice {
    Font(&'static NerdFont),
    Skip,
}

// 3. Terminal setup/restore pattern
let mut terminal = setup_terminal()?;
let result = run_selection_loop(&mut terminal);
restore_terminal()?;
result

// 4. Event loop with ListState
fn run_selection_loop(terminal: &mut Terminal) -> Result<FontChoice>
// - ListState for tracking selection
// - Event polling with crossterm
// - Up/Down navigation, Enter to select, Esc to cancel

// 5. Render function
fn render(frame: &mut Frame, state: &ListState, options: &[SelectionOption])
// - Layout with Constraint::Percentage
// - List widget with highlighted selection
// - Rich formatting with Span/Line
```

**Size Requirements:**
- Minimum terminal size check: 80x24 (same as preset_select.rs)
- Clear error message if too small

### Library and Framework Requirements

**Dependencies (already in Cargo.toml):**
```toml
ratatui = "0.28"
crossterm = "0.28"
anyhow = "1.0"
```

**Imports needed:**
```rust
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

use crate::fonts::nerd_fonts::{get_all_fonts, NerdFont};
use crate::tui::{restore_terminal, setup_terminal};
```

**Font Registry API** (from src/fonts/nerd_fonts.rs):
```rust
// Get all 6 fonts
get_all_fonts() -> &'static [NerdFont]

// NerdFont struct has these fields:
pub struct NerdFont {
    pub id: &'static str,              // "firacode"
    pub name: &'static str,            // "FiraCode Nerd Font"
    pub display_name: &'static str,    // "FiraCode Nerd Font Mono"
    pub description: &'static str,     // "Programming ligatures..."
    pub preview_chars: &'static str,   // "⚡ ⬢  →  ✓   λ ≡"
    pub download_url: &'static str,
    pub file_format: FontFormat,
    pub recommended: bool,             // true for 3 fonts
    pub recommended_for: &'static [PromptEngine],
}
```

### File Structure Requirements

**New File:** `src/tui/font_select.rs`

**Module Export:** Add to `src/tui/mod.rs`:
```rust
pub mod font_select;
```

**Naming Conventions:**
- Function names: snake_case (`select_font`, `run_selection_loop`)
- Struct names: PascalCase (`FontChoice`, `SelectionOption`)
- Constants: SCREAMING_SNAKE_CASE (if any)

### Testing Requirements

**Unit Tests** (in same file under `#[cfg(test)] mod tests`):
1. Test `SelectionOption::all()` returns 7 options (6 fonts + skip)
2. Test recommended fonts appear first in list
3. Test skip option is last
4. Test that font details include name, description, preview chars
5. Test keyboard controls enum parsing

**Integration Tests** (manual for TUI):
- Visual verification of layout in 80x24 terminal
- Test navigation with arrow keys
- Test Enter on each option
- Test Esc cancellation
- Test with terminal < 80x24 (should error gracefully)

**Testing Pattern from Existing Code:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_options_count() {
        let options = SelectionOption::all();
        // 6 fonts + 1 skip option = 7 total
        assert_eq!(options.len(), 7);
    }

    #[test]
    fn test_recommended_fonts_first() {
        let options = SelectionOption::all();
        // First 3 should be recommended (from nerd_fonts.rs registry)
        // FiraCode, JetBrainsMono, Meslo
        assert!(matches!(options[0].kind, SelectionKind::Font(idx) if is_recommended(idx)));
    }
}
```

### Previous Story Intelligence (Story 4.3)

**What Story 4.3 Completed:**
- Font detection system (`src/fonts/detector.rs`)
- `detect_nerd_fonts()` function returns `DetectionResult` enum
- Platform-specific directory scanning (macOS: `~/Library/Fonts/`, Linux: `~/.local/share/fonts/`)
- Pattern matching for `*Nerd*.{ttf,otf}` files
- Thread-safe caching with `OnceLock`
- 21/21 tests passing

**Integration Point with Story 4.3:**
```rust
use crate::fonts::detector::{detect_nerd_fonts, DetectionResult};

// Story 4.4 gets called only when:
if let DetectionResult::NotInstalled = detect_nerd_fonts()? {
    // Show font selection TUI
    let choice = select_font()?;
    // ...
}
```

**Key Learnings:**
- Code review identified importance of recursion depth limits and symlink handling
- Module re-exports in `src/fonts/mod.rs` for clean API
- Comprehensive test coverage (unit + integration) is expected
- Zero clippy warnings enforced

### Git Intelligence (Recent Commits)

**Recent Work Pattern:**
```
880cc58 Apply code review fixes to font detection (Story 4.3)
afd2dde Implement Nerd Font detection system (Story 4.3)
02afd3c Implement Nerd Font registry (Story 4.1)
```

**Code Patterns Observed:**
1. **Two-commit pattern:** Initial implementation, then code review fixes
2. **Comprehensive docs:** All new modules have detailed doc comments (`//!` module-level, `///` item-level)
3. **Test-first approach:** Tests written alongside implementation
4. **Clean commits:** Each story is self-contained with passing tests

**File Creation Pattern:**
```rust
// Module-level documentation
//! Brief description
//!
//! Longer explanation of purpose and usage

use statements...

// Public API
pub fn main_function() -> Result<T>

// Helper functions (private)
fn helper() -> Result<T>

// Tests at end
#[cfg(test)]
mod tests { ... }
```

### Latest Tech Information

**Ratatui v0.28 (Current Stable):**
- List widget with ListState is standard pattern for selection UIs
- `Span` and `Line` for rich text formatting
- `Modifier::BOLD` for highlighting recommended fonts
- `Color::Green` for success indicators
- Layout system uses `Constraint::Percentage` for responsive design

**CrosstermBackend:**
- Standard event loop: `event::poll()` then `event::read()`
- KeyCode enum for key handling
- Terminal must be setup/restored properly (handled by `crate::tui` module)

**Best Practices:**
1. **Minimum size check:** Always verify terminal >= 80x24
2. **Restore terminal:** Use `restore_terminal()` even on error paths
3. **Clear error messages:** User-friendly explanations, not raw errors
4. **Keyboard shortcuts:** Document in help text and function docs

### Implementation Guidance

**Data Structure for Selection:**
```rust
struct SelectionOption {
    kind: SelectionKind,
    display_name: String,
    description: String,
    preview_chars: String,
    is_recommended: bool,
}

enum SelectionKind {
    Font(usize),  // index into NERD_FONTS
    Skip,
}

impl SelectionOption {
    fn all() -> Vec<Self> {
        let mut options = Vec::new();

        // Add all fonts from registry
        for (idx, font) in get_all_fonts().iter().enumerate() {
            options.push(SelectionOption {
                kind: SelectionKind::Font(idx),
                display_name: font.name.to_string(),
                description: font.description.to_string(),
                preview_chars: font.preview_chars.to_string(),
                is_recommended: font.recommended,
            });
        }

        // Sort: recommended first, then others
        options.sort_by_key(|opt| !opt.is_recommended);

        // Add skip option at end
        options.push(SelectionOption {
            kind: SelectionKind::Skip,
            display_name: "Skip font installation".to_string(),
            description: "Prompt may not display correctly without Nerd Fonts".to_string(),
            preview_chars: String::new(),
            is_recommended: false,
        });

        options
    }
}
```

**Rendering Logic:**
```rust
fn render(frame: &mut Frame, state: &ListState, options: &[SelectionOption]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Title
            Constraint::Min(10),        // List
            Constraint::Length(3),      // Help text
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Select a Nerd Font to install:")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Font list
    let items: Vec<ListItem> = options
        .iter()
        .map(|opt| {
            let mut lines = vec![
                Line::from(if opt.is_recommended {
                    Span::styled(
                        format!("✨ {}", opt.display_name),
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(format!("  {}", opt.display_name))
                }),
                Line::from(format!("  {}", opt.description)),
            ];

            if !opt.preview_chars.is_empty() {
                lines.push(Line::from(format!("  Preview: {}", opt.preview_chars)));
            }

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(list, chunks[1], state);

    // Help
    let help = Paragraph::new("↑↓: Navigate  Enter: Select  Esc: Cancel")
        .alignment(Alignment::Center);
    frame.render_widget(help, chunks[2]);
}
```

**Event Loop:**
```rust
fn run_selection_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<FontChoice> {
    let options = SelectionOption::all();
    let mut state = ListState::default();

    // Default to first option (first recommended font)
    state.select(Some(0));

    loop {
        terminal.draw(|frame| render(frame, &state, &options))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        let selected = state.selected().unwrap_or(0);
                        if selected > 0 {
                            state.select(Some(selected - 1));
                        }
                    }
                    KeyCode::Down => {
                        let selected = state.selected().unwrap_or(0);
                        if selected < options.len() - 1 {
                            state.select(Some(selected + 1));
                        }
                    }
                    KeyCode::Enter => {
                        let selected = state.selected().unwrap_or(0);
                        let option = &options[selected];

                        return match option.kind {
                            SelectionKind::Font(idx) => {
                                let font = &get_all_fonts()[idx];
                                Ok(FontChoice::Font(font))
                            }
                            SelectionKind::Skip => Ok(FontChoice::Skip),
                        };
                    }
                    KeyCode::Esc => {
                        bail!("Font selection cancelled by user");
                    }
                    _ => {}
                }
            }
        }
    }
}
```

### Critical Reminders

**DO:**
- ✅ Follow exact TUI pattern from `preset_select.rs` and `framework_select.rs`
- ✅ Use `get_all_fonts()` from nerd_fonts registry (6 fonts total)
- ✅ Sort to show recommended fonts first (FiraCode, JetBrainsMono, Meslo)
- ✅ Display all font metadata: name, description, preview characters
- ✅ Add skip option as last item
- ✅ Check terminal size >= 80x24
- ✅ Always restore terminal, even on error
- ✅ Write comprehensive tests
- ✅ Document keyboard shortcuts in function docs
- ✅ Use `anyhow::Result` for error handling
- ✅ Add module-level and function-level documentation

**DON'T:**
- ❌ Don't create new font data structures (use existing `NerdFont` from registry)
- ❌ Don't implement font download/installation here (that's Story 4.5/4.6)
- ❌ Don't hardcode font list (use `get_all_fonts()`)
- ❌ Don't skip terminal size check
- ❌ Don't forget to export in `src/tui/mod.rs`
- ❌ Don't use different TUI pattern than existing code
- ❌ Don't add dependencies (all needed crates already present)

### Acceptance Criteria Expanded

1. **Create `src/tui/font_select.rs`**
   - New file in `src/tui/` directory
   - Export `pub mod font_select;` in `src/tui/mod.rs`

2. **Show when Nerd Font required but not detected**
   - Called from profile creation workflow
   - Triggered by `if let DetectionResult::NotInstalled = detect_nerd_fonts()?`

3. **Display font options as cards**
   - Use `List` widget with `ListItem` for each font
   - Each item shows: name, description, preview chars
   - Multi-line formatting with `Line::from(vec![...])`

4. **Highlight recommended fonts**
   - First 3 fonts (recommended=true) show ✨ emoji
   - Green color + bold style for recommended
   - Sort order: recommended first, then others

5. **Show preview characters**
   - `font.preview_chars` field: "⚡ ⬢  →  ✓   λ ≡"
   - Display on third line of each card
   - Format: "Preview: {chars}"

6. **Allow skipping installation**
   - Add "Skip" option as last item
   - Returns `FontChoice::Skip`
   - Show warning: "Prompt may not display correctly without Nerd Fonts"

7. **Keyboard navigation**
   - Up arrow: Move selection up (stop at 0)
   - Down arrow: Move selection down (stop at last)
   - Enter: Confirm selection, return choice
   - Esc: Cancel, return error

8. **Help text**
   - Bottom of screen: "↑↓: Navigate  Enter: Select  Esc: Cancel"
   - Centered alignment
   - Always visible

## Tasks / Subtasks

- [ ] Create `src/tui/font_select.rs` (AC: 1)
  - [ ] Add module-level documentation
  - [ ] Define `FontChoice` enum (Font variant, Skip variant)
  - [ ] Define `SelectionOption` struct
  - [ ] Implement `SelectionOption::all()` method
- [ ] Implement `select_font()` public function (AC: 2)
  - [ ] Terminal setup with size check (>= 80x24)
  - [ ] Call `run_selection_loop()`
  - [ ] Restore terminal on success and error
- [ ] Implement `run_selection_loop()` function (AC: 3, 4, 5, 6, 7)
  - [ ] Create options list from `get_all_fonts()`
  - [ ] Initialize `ListState` with first item selected
  - [ ] Event loop with keyboard handling
  - [ ] Render function with font cards
- [ ] Implement rendering logic (AC: 3, 4, 5, 8)
  - [ ] Layout with title, list, help sections
  - [ ] Font list with highlighting for recommended
  - [ ] Preview characters display
  - [ ] Help text at bottom
- [ ] Add tests (AC: all)
  - [ ] Test option count (7 = 6 fonts + skip)
  - [ ] Test recommended fonts first
  - [ ] Test skip option last
  - [ ] Test font metadata in options
- [ ] Export module (AC: 1)
  - [ ] Add `pub mod font_select;` to `src/tui/mod.rs`
- [ ] Documentation (AC: all)
  - [ ] Module-level docs with usage example
  - [ ] Function docs with keyboard shortcuts
  - [ ] Struct/enum docs

## Dev Agent Record

### Context Reference

Epic: [epic-4-nerd-fonts.md](../epic-4-nerd-fonts.md)
Tech Spec: [tech-spec-epic-4.md](../../tech-spec-epic-4.md)
Previous Story: [epic-4-story-3.md](epic-4-story-3.md) (Font Detection - DONE)
Nerd Font Registry: [src/fonts/nerd_fonts.rs](../../../src/fonts/nerd_fonts.rs)
TUI Pattern Reference: [src/tui/preset_select.rs](../../../src/tui/preset_select.rs)

### Agent Model Used

<!-- Will be filled during implementation -->

### Debug Log References

<!-- Will be filled during implementation -->

### Completion Notes List

<!-- Will be filled during implementation -->

### File List

<!-- Will be filled during implementation -->
