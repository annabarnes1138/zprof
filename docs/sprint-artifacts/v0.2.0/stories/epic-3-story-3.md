# Story 3.3: Create Uninstall Command with Restoration Options

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user who wants to remove zprof
**I want** choose what happens to my shell config
**So that** restore original or promote a profile

## Acceptance Criteria

- [ ] Create zprof uninstall command
- [ ] Show restoration options TUI (Restore / Promote / Clean removal)
- [ ] Implement restore original option
- [ ] Implement promote profile option
- [ ] Implement clean removal option
- [ ] Add --yes flag for non-interactive

## Files

- src/cli/uninstall.rs (NEW)
- src/tui/uninstall_select.rs (NEW)

## Dependencies

Epic 6 (shares backup logic)

## Dev Agent Record

**Context File:** epic-3-story-3.context.xml
**Status:** Ready for development
**Generated:** 2025-11-24
