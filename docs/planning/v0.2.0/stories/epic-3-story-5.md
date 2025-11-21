# Story 3.5: Implement Cleanup Logic

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** TODO

## User Story

**As a** user completing uninstall
**I want** all zprof files removed cleanly
**So that** system is back to pre-zprof state

## Acceptance Criteria

- [ ] Remove .zsh-profiles/ directory
- [ ] Remove zprof .zshenv
- [ ] Remove symlinks
- [ ] Handle errors gracefully
- [ ] Show progress during cleanup
- [ ] Add --keep-backups flag

## Files

- src/cli/uninstall.rs
- src/cleanup/mod.rs (NEW)

## Dependencies

Epic 6 (shares backup logic)
