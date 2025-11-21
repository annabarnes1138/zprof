# Story 1.1: Add Prompt Mode to Manifest Schema

**Epic:** Epic 1 - Smart TUI (Prompt Mode Branching)
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** developer
**I want** the manifest to support prompt mode discrimination
**So that** configs can represent either engine-based or theme-based prompts

## Acceptance Criteria

- [ ] Add `prompt_mode` field to `[profile]` section (enum: "prompt_engine" | "framework_theme")
- [ ] Add `prompt_engine` field (optional, used when mode = "prompt_engine")
- [ ] Rename `theme` field to `framework_theme` (optional, used when mode = "framework_theme")
- [ ] Implement backward compatibility (old manifests default to `framework_theme`)
- [ ] Update validation to ensure only one is set based on mode
- [ ] Update tests for new schema

## Files

- `src/core/manifest.rs`
- `tests/manifest_validation_test.rs`

## Dependencies

None

## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-1.context.xml](epic-1-story-1.context.xml)
