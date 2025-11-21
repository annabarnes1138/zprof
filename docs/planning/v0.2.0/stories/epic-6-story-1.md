# Story 6.1: Detect Existing Shell Configuration

**Epic:** Epic 6 - Init Cleanup and Enhancement
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user running zprof init
**I want** my existing config detected
**So that** zprof can properly back it up

## Acceptance Criteria

- [ ] Create src/backup/shell_config.rs
- [ ] Detect all shell config files (.zshrc, .zshenv, etc.)
- [ ] Detect existing frameworks (oh-my-zsh, zimfw, etc.)
- [ ] Parse .zshrc to identify framework
- [ ] Return ShellConfigInfo struct
- [ ] Handle symlinks
- [ ] Unit tests with mock filesystem

## Files

- src/backup/shell_config.rs (NEW)
- src/backup/mod.rs

## Dependencies

Epic 3 (shares backup logic and manifest format)

## Dev Agent Record

### Context Reference
- [epic-6-story-1.context.xml](epic-6-story-1.context.xml)
