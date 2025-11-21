# Story 4.6: Implement Platform-Specific Font Installation

**Epic:** Epic 4 - Nerd Font Auto-Installation
**Priority:** P1
**Status:** TODO

## User Story

**As a** user on macOS or Linux
**I want** fonts automatically installed to correct location
**So that** available system-wide

## Acceptance Criteria

- [ ] Create src/fonts/installer.rs
- [ ] macOS: Copy to ~/Library/Fonts/, run fc-cache
- [ ] Linux: Copy to ~/.local/share/fonts/, run fc-cache -fv
- [ ] Show installation progress
- [ ] Handle permission errors
- [ ] Offer manual instructions if auto-install fails
- [ ] Platform-specific tests

## Files

- src/fonts/installer.rs (NEW)
- tests/font_install_test.rs (NEW)

## Dependencies

Epic 1 (for PromptEngine integration)
