# Story 1.7: Integrate Prompt Mode into Create Workflow

**Epic:** Epic 1 - Smart TUI (Prompt Mode Branching)
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user running `zprof create`
**I want** the new prompt mode flow to be seamless
**So that** profile creation is intuitive

## Acceptance Criteria

- [ ] Update `src/cli/create.rs` to use new TUI flow
- [ ] New flow order:
  1. Framework selection
  2. **Prompt mode selection** (NEW)
  3. **IF engine**: Prompt engine selection
  4. **IF theme**: Theme selection (existing)
  5. Plugin selection
  6. Confirmation
- [ ] Update confirmation screen to show prompt info correctly
- [ ] Integration test for full workflow
- [ ] Update user-facing docs

## Files

- `src/cli/create.rs`
- `tests/create_workflow_test.rs`
- `docs/user-guide/quick-start.md`

## Dependencies

- All previous Epic 1 stories (full integration)
## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-7.context.xml](epic-1-story-7.context.xml)
