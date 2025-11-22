# Story 4.7: Generate Terminal Configuration Instructions

**Epic:** Epic 4 - Nerd Font Auto-Installation
**Priority:** P1
**Status:** TODO

## User Story

**As a** user who installed a Nerd Font
**I want** clear instructions to configure my terminal
**So that** font displays correctly

## Acceptance Criteria

- [ ] Create src/fonts/terminal_config.rs
- [ ] Detect terminal via $TERM_PROGRAM
- [ ] Generate terminal-specific instructions (iTerm2, VS Code, Terminal.app, Alacritty, Kitty, etc.)
- [ ] Show exact font name
- [ ] Add troubleshooting tips
- [ ] Store in manifest

## Files

- src/fonts/terminal_config.rs (NEW)
- src/tui/terminal_instructions.rs (NEW)

## Dependencies

Epic 1 (for PromptEngine integration)
