# Story 3.6: Add Uninstall Confirmation Screen

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** ready-for-dev

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

## Dev Agent Record

**Context File:** epic-3-story-6.context.xml
**Status:** Ready for development
**Generated:** 2025-11-24
