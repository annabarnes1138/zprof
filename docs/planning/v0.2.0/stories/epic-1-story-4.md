# Story 1.4: Create Prompt Engine Selection TUI

**Epic:** Epic 1 - Smart TUI (Prompt Mode Branching)
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user who chose "Standalone prompt engine"
**I want** to select which engine to use
**So that** I can pick the prompt that fits my needs

## Acceptance Criteria

- [ ] Create `src/tui/prompt_engine_select.rs`
- [ ] Display list of engines with descriptions:
  ```
  Select a prompt engine:

  > Starship (cross-shell, Rust-powered, async)
    Powerlevel10k (Zsh-only, highly customizable)
    Oh-My-Posh (cross-shell, many themes)
    Pure (minimal, async, fast)
    Spaceship (feature-rich, pretty)
  ```
- [ ] Show warning if engine requires Nerd Font
- [ ] Return selected `PromptEngine`
- [ ] Keyboard navigation

## Files

- `src/tui/prompt_engine_select.rs` (NEW)

## Dependencies

- Story 1.3 (Prompt Engine Registry)
## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-4.context.xml](epic-1-story-4.context.xml)
