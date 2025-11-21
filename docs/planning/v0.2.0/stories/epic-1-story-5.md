# Story 1.5: Refactor Theme Selection for Conditional Display

**Epic:** Epic 1 - Smart TUI (Prompt Mode Branching)
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user who chose "Framework themes"
**I want** to see only themes compatible with my framework
**So that** I don't pick incompatible options

## Acceptance Criteria

- [ ] Modify `src/tui/theme_select.rs` to accept `PromptMode`
- [ ] If mode = `PromptEngine`: Skip theme selection entirely
- [ ] If mode = `FrameworkTheme`: Show framework-specific themes
- [ ] Filter theme registry by selected framework
- [ ] Update tests

## Files

- `src/tui/theme_select.rs`
- `src/tui/wizard.rs`

## Dependencies

- Story 1.2 (Prompt Mode Selection TUI)
## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-5.context.xml](epic-1-story-5.context.xml)
