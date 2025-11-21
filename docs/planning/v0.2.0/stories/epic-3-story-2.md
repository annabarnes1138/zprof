# Story 3.2: Move Root Configs After Backup

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** TODO

## User Story

**As a** user completing zprof init
**I want** my root configs moved out of the way
**So that** don't get confused about which config is active

## Acceptance Criteria

- [ ] Move root configs to backup location
- [ ] Leave only zprof .zshenv in root
- [ ] Create .zshrc symlink to active profile
- [ ] Handle edge cases (symlinks, read-only)
- [ ] Update init integration test

## Files

- src/cli/init.rs
- src/backup/pre_zprof.rs

## Dependencies

Epic 6 (shares backup logic)
