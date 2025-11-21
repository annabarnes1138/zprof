# Story 2.7: Add Non-Interactive Preset Flag

**Epic:** Epic 2 - Quick Setup Presets
**Priority:** P0
**Status:** TODO

## User Story

**As a** power user
**I want** create profiles from presets via CLI
**So that** script profile creation

## Acceptance Criteria

Add --preset <name> flag
Skip TUI, use preset directly
Show summary after creation
Update CLI help text

## Files

- src/cli/create.rs
- tests/create_preset_test.rs (NEW)

## Dependencies

Previous Epic 2 stories
