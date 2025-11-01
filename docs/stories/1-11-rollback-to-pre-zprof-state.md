# Story 1.11: Rollback to Pre-zprof State

Status: ready-for-dev

## Story

As a developer who wants to uninstall zprof,
I want to restore my original shell configuration,
so that I can revert to my pre-zprof setup if needed.

## Acceptance Criteria

1. `zprof rollback` command checks for backup file (`.zshrc.pre-zprof`) in profiles
2. Shows what will be restored and what will be moved back:
   - Restore: `~/.zshrc` from backup
   - Move: Framework back to home directory (if applicable)
   - Keep: `~/.zsh-profiles/` directory for reference
3. Requires explicit confirmation: "Continue? [y/N]"
4. On confirmation:
   - Restores original `.zshrc` from backup
   - Moves framework back to original location (e.g., `~/.oh-my-zsh`)
   - Leaves `~/.zsh-profiles/` intact but inactive
5. Success message confirms rollback and provides instructions:
   - Restart shell or run: `source ~/.zshrc`
   - Can safely delete `~/.zsh-profiles/` manually if desired
6. If no backup found, provides clear error message
7. Cannot rollback if backup was modified or deleted

## Tasks / Subtasks

### Task 1: Implement Rollback Detection and Validation (AC: 1, 6, 7)
- [ ] 1.1: Add `rollback` subcommand to CLI
- [ ] 1.2: Search for `.zshrc.pre-zprof` backup file in profile directories
- [ ] 1.3: Validate backup file exists and is readable
- [ ] 1.4: Detect framework type from backup metadata or profile manifest
- [ ] 1.5: Validate backup integrity (not modified/corrupted)
- [ ] 1.6: Generate error message if backup missing or invalid
- [ ] 1.7: Write unit tests for backup detection logic

### Task 2: Implement Rollback Preview Display (AC: 2)
- [ ] 2.1: Create preview display showing restoration plan
- [ ] 2.2: List what will be restored (`.zshrc` from backup)
- [ ] 2.3: List what will be moved (framework to original location)
- [ ] 2.4: List what will remain (`~/.zsh-profiles/` for reference)
- [ ] 2.5: Format preview with clear headings and visual structure
- [ ] 2.6: Write tests for preview generation

### Task 3: Implement User Confirmation Flow (AC: 3)
- [ ] 3.1: Add confirmation prompt with explicit "Continue? [y/N]" message
- [ ] 3.2: Default to "No" for safety (require explicit "y" or "yes")
- [ ] 3.3: Handle various input formats (Y, y, yes, YES)
- [ ] 3.4: Exit gracefully on "No" without changes
- [ ] 3.5: Write tests for confirmation handling

### Task 4: Implement Rollback Execution (AC: 4)
- [ ] 4.1: Backup current `.zshrc` to `.zshrc.pre-rollback` for safety
- [ ] 4.2: Restore `.zshrc` from `.zshrc.pre-zprof` backup
- [ ] 4.3: Detect framework location in profile directory
- [ ] 4.4: Move framework back to home directory (e.g., `~/.oh-my-zsh`)
- [ ] 4.5: Preserve `~/.zsh-profiles/` directory structure
- [ ] 4.6: Set appropriate file permissions on restored files
- [ ] 4.7: Handle errors during file operations (rollback on failure)
- [ ] 4.8: Write integration tests for rollback execution

### Task 5: Implement Success Messaging and Instructions (AC: 5)
- [ ] 5.1: Create success message confirming rollback completion
- [ ] 5.2: Include instruction to restart shell or run `source ~/.zshrc`
- [ ] 5.3: Suggest manual deletion of `~/.zsh-profiles/` if desired
- [ ] 5.4: Display paths of restored files for reference
- [ ] 5.5: Write tests for success message formatting

### Task 6: Integration Testing and Documentation
- [ ] 6.1: Test rollback on system with oh-my-zsh migration
- [ ] 6.2: Test rollback on system with zinit migration
- [ ] 6.3: Test rollback on system with prezto migration
- [ ] 6.4: Test rollback on system with multiple profiles
- [ ] 6.5: Test error cases (missing backup, corrupted backup)
- [ ] 6.6: Update README with rollback documentation
- [ ] 6.7: Add rollback examples to user guide

## Dev Notes

### Dependencies

This story depends on:
- Story 1.1b: Migrate Existing Configuration During Init (provides backup file `.zshrc.pre-zprof`)
- Story 1.4: Framework Detection (framework detection logic for identifying what to move back)

### Architecture Patterns

**Backup Detection:**
- Search all profile directories for `.zshrc.pre-zprof`
- Validate file exists and has expected structure
- Extract framework type from profile manifest or backup metadata

**Restoration Strategy:**
- Atomic operations: create safety backup before restoration
- Fail-safe: rollback changes if any operation fails
- Preserve user data: keep `~/.zsh-profiles/` for manual cleanup

**Framework Relocation:**
- Detect framework type from profile manifest
- Map profile framework location to home directory location
- Move framework directory tree preserving permissions

### Project Structure Notes

**Files to Modify:**
- `src/commands/rollback.rs` - New rollback command implementation
- `src/backup/restore.rs` - Restoration logic (may need to create)
- `src/framework/detection.rs` - Reuse framework detection
- `tests/integration/rollback_test.rs` - Integration tests

**Expected File Operations:**
- Read: `~/.zsh-profiles/profiles/*/profile.toml` (manifest)
- Read: `~/.zsh-profiles/profiles/*/.zshrc.pre-zprof` (backup)
- Write: `~/.zshrc.pre-rollback` (safety backup)
- Write: `~/.zshrc` (restored from backup)
- Move: `~/.zsh-profiles/profiles/*/.<framework>/` → `~/.<framework>/`

### Testing Standards

- Unit tests for backup detection and validation
- Integration tests for complete rollback flow
- Test error cases (missing backup, corrupted files, permission issues)
- Test with different frameworks (oh-my-zsh, zinit, prezto, zimfw)
- Manual testing on live system recommended before release

### References

- [Source: docs/epics.md#Story-1.11]
- [Source: docs/PRD.md] - User safety and reversibility requirements
- [Source: docs/architecture.md] - File system operations patterns

## Dev Agent Record

### Context Reference

- [docs/stories/1-11-rollback-to-pre-zprof-state.context.xml](docs/stories/1-11-rollback-to-pre-zprof-state.context.xml)

### Agent Model Used

<!-- To be filled by dev agent -->

### Debug Log References

<!-- To be filled by dev agent -->

### Completion Notes List

<!-- To be filled by dev agent -->

### File List

<!-- To be filled by dev agent -->
