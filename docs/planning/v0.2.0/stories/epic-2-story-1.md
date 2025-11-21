# Story 2.1: Define Preset Data Model

**Epic:** Epic 2 - Quick Setup Presets
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** developer
**I want** a data-driven preset system
**So that** adding new presets requires no code changes

## Acceptance Criteria

- [ ] Create `src/presets/mod.rs` module
- [ ] Define `Preset` struct with all config fields:
  - id, name, icon, description
  - target_user (who it's for)
  - framework, prompt_mode, plugins
  - env vars, shell options
- [ ] Create `PresetConfig` that can generate a `Manifest`
- [ ] Add `PRESET_REGISTRY` constant with 4-5 presets
- [ ] Implement `Manifest::from_preset()` method
- [ ] Add unit tests

## Files

- `src/presets/mod.rs` (NEW)
- `src/core/manifest.rs`

## Dependencies

- Epic 1 complete (requires PromptMode)

## Dev Agent Record

### Context Reference
- [epic-2-story-1.context.xml](epic-2-story-1.context.xml)
