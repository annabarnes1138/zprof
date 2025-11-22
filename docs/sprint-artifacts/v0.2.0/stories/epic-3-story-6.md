# Story 3.6: Add Uninstall Confirmation Screen

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** TODO

## User Story

**As a** user about to uninstall
**I want** see a summary and confirm my choice
**So that** don't accidentally remove shell config

## Acceptance Criteria

- [ ] Show detailed summary (restoration plan, cleanup plan, safety backup)
- [ ] Default to No
- [ ] Add --yes flag to skip
- [ ] Include file counts and sizes
- [ ] Highlight destructive operations

## Files

- src/tui/uninstall_confirm.rs (NEW)
- src/cli/uninstall.rs

## Dependencies

Epic 6 (shares backup logic)
