# Story 2.3: Create Quick vs Custom Selection Screen

**Epic:** Epic 2 - Quick Setup Presets
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user running `zprof create`
**I want** to choose between preset and custom setup
**So that** I can pick the path that fits my expertise

## Acceptance Criteria

- [ ] Create `src/tui/setup_mode_select.rs`
- [ ] Display initial binary choice (Quick Setup vs Custom Setup)
- [ ] Return `SetupMode` enum (Quick | Custom)
- [ ] Default selection: Quick Setup
- [ ] Keyboard navigation
- [ ] Help text explaining each option

## Files

- `src/tui/setup_mode_select.rs` (NEW)

## Dependencies

None

## Dev Agent Record

### Context Reference
- [epic-2-story-3.context.xml](epic-2-story-3.context.xml)
