# Story 4.9: Add Font Management Command

**Epic:** Epic 4 - Nerd Font Auto-Installation
**Priority:** P1
**Status:** TODO

## User Story

**As a** user
**I want** install or change fonts later
**So that** not locked into initial choice

## Acceptance Criteria

- [ ] Create zprof font command with subcommands:
  - zprof font list (show installed)
  - zprof font install (install new)
  - zprof font info <name> (show terminal config)
- [ ] Update CLI help
- [ ] Integration tests
- [ ] Documentation

## Files

- src/cli/font.rs (NEW)
- src/cli/mod.rs
- src/main.rs

## Dependencies

Epic 1 (for PromptEngine integration)
