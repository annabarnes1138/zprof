# Story 1.2: Create Prompt Mode Selection TUI

**Epic:** Epic 1 - Smart TUI (Prompt Mode Branching)
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user creating a profile
**I want** to choose how I want my prompt configured
**So that** I only see relevant options for my choice

## Acceptance Criteria

- [ ] Create `src/tui/prompt_mode_select.rs`
- [ ] Show binary choice screen after framework selection:
  ```
  How do you want to handle your prompt?

  > Standalone prompt engine (Starship, Powerlevel10k, Pure...)
    Framework's built-in themes (robbyrussell, agnoster...)
  ```
- [ ] Include help text explaining the difference
- [ ] Return selected `PromptMode` enum value
- [ ] Keyboard navigation (↑↓, Enter, Esc)

## Files

- `src/tui/prompt_mode_select.rs` (NEW)
- `src/tui/wizard.rs` (integrate new screen)

## Dependencies

- Story 1.1 (Manifest schema changes)
## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-2.context.xml](epic-1-story-2.context.xml)
