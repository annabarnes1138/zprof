# Epic 5: Remove Deprecated Rollback Command

**Priority:** P2 (Nice to Have)
**Estimated Effort:** 1 day
**Owner:** TBD

## Overview

Remove the deprecated `zprof rollback` command now that it's been superseded by the comprehensive `zprof uninstall` command with restoration options.

## Problem Statement

The original `rollback` command was a simple mechanism to restore pre-zprof state, but it has several limitations:
- Limited functionality compared to new uninstall system
- Confusing naming (implies undoing recent changes, not full removal)
- Code duplication with new uninstall implementation
- Users don't know which command to use

With Epic 3 (Complete Uninstall System) providing comprehensive restoration options, the `rollback` command is redundant and should be removed.

## Goals

1. **Clean codebase**: Remove deprecated code
2. **Clear migration path**: Guide users to `uninstall` command
3. **Backward compatibility period**: Graceful transition with helpful error messages
4. **Documentation updates**: Remove rollback references, document migration

## User Stories

### Story 5.1: Add Deprecation Warning to Rollback Command

**As a** user running `zprof rollback`
**I want** a clear message directing me to the new command
**So that** I know what to use instead

**Acceptance Criteria:**
- [ ] Update `src/cli/rollback.rs` to show deprecation warning:
  ```
  ⚠ Warning: 'zprof rollback' is deprecated and will be removed in v0.3.0

  Use 'zprof uninstall' instead, which provides more options:
  • Restore your original pre-zprof configuration
  • Promote a profile to become your root config
  • Clean removal without restoration

  Run 'zprof uninstall --help' for details.

  Continue with legacy rollback? [y/N]
  ```
- [ ] Default to "No" (user must explicitly confirm)
- [ ] Add `--force` flag to skip warning (for scripts)
- [ ] Log deprecation warning
- [ ] Track usage metrics if analytics enabled
- [ ] Add to CLI help text: "(deprecated, use 'uninstall')"

**Files:**
- `src/cli/rollback.rs`

---

### Story 5.2: Update CLI Help Text

**As a** user running `zprof --help`
**I want** to see that rollback is deprecated
**So that** I use the correct command

**Acceptance Criteria:**
- [ ] Update `src/main.rs` command list:
  ```
  COMMANDS:
    init         Initialize zprof
    create       Create a new profile
    use          Switch to a profile
    list         List all profiles
    delete       Delete a profile
    uninstall    Uninstall zprof and restore configuration
    rollback     (deprecated) Use 'uninstall' instead
  ```
- [ ] Update `zprof rollback --help`:
  ```
  zprof-rollback (deprecated)

  This command is deprecated. Use 'zprof uninstall' instead.

  The new uninstall command provides:
  - Original configuration restoration
  - Profile promotion to root
  - Flexible cleanup options

  See 'zprof uninstall --help' for details.
  ```
- [ ] Ensure rollback still appears in command list (for discoverability)
- [ ] Add "(deprecated)" marker in all help output

**Files:**
- `src/main.rs`
- `src/cli/rollback.rs`

---

### Story 5.3: Add Migration Guide

**As a** user of the rollback command
**I want** documentation on migrating to uninstall
**So that** I understand the differences and benefits

**Acceptance Criteria:**
- [ ] Create `docs/user-guide/migration-rollback-to-uninstall.md`:
  - Why rollback is being removed
  - Comparison table of rollback vs uninstall features
  - Migration examples (rollback → equivalent uninstall command)
  - FAQ about the change
- [ ] Add migration guide to documentation index
- [ ] Include in v0.2.0 release notes
- [ ] Comparison table format:
  ```markdown
  | Feature | rollback | uninstall |
  |---------|----------|-----------|
  | Restore original config | ✓ | ✓ |
  | Promote profile to root | ✗ | ✓ |
  | Clean removal only | ✗ | ✓ |
  | Safety backup | ✗ | ✓ |
  | Configurable cleanup | ✗ | ✓ |
  ```

**Files:**
- `docs/user-guide/migration-rollback-to-uninstall.md` (NEW)
- `docs/README.md`

---

### Story 5.4: Update All Documentation References

**As a** user reading documentation
**I want** no references to the deprecated command
**So that** I'm not confused about which to use

**Acceptance Criteria:**
- [ ] Search all docs for "rollback" references
- [ ] Replace with "uninstall" where appropriate
- [ ] Update file list:
  - `docs/user-guide/quick-start.md`
  - `docs/user-guide/commands.md`
  - `docs/user-guide/troubleshooting.md`
  - `docs/user-guide/faq.md`
  - `docs/user-guide/installation.md`
  - `README.md`
- [ ] Add deprecation notice to commands.md:
  ```markdown
  ## rollback (deprecated)

  **Deprecated:** This command will be removed in v0.3.0.
  Use `zprof uninstall` instead.

  [See migration guide](migration-rollback-to-uninstall.md)
  ```
- [ ] Verify no broken examples or outdated instructions

**Files:**
- `docs/user-guide/*.md`
- `README.md`

---

### Story 5.5: Remove Rollback Code (v0.3.0 Placeholder)

**As a** developer
**I want** a plan for complete removal
**So that** we cleanly remove the code in v0.3.0

**Acceptance Criteria:**
- [ ] Create tracking issue for v0.3.0: "Remove deprecated rollback command"
- [ ] Document files to delete:
  - `src/cli/rollback.rs`
  - `tests/rollback_test.rs`
- [ ] Document code to update:
  - `src/main.rs` (remove from command list)
  - `src/cli/mod.rs` (remove module)
- [ ] Add TODO comments in code:
  ```rust
  // TODO(v0.3.0): Remove this entire module
  // Deprecated in v0.2.0, scheduled for removal in v0.3.0
  ```
- [ ] Add to v0.3.0 planning epic (when created)
- [ ] Include removal in v0.3.0 breaking changes documentation

**Files:**
- `docs/planning/v0.3.0/remove-rollback.md` (placeholder for future)
- `src/cli/rollback.rs` (add TODO comments)

---

### Story 5.6: Update Release Notes

**As a** user upgrading to v0.2.0
**I want** clear communication about the change
**So that** I'm not surprised by deprecation warnings

**Acceptance Criteria:**
- [ ] Add to v0.2.0 release notes:
  ```markdown
  ### Deprecated

  #### `zprof rollback` command

  The `rollback` command is deprecated in favor of the more flexible
  `zprof uninstall` command. The rollback command will show a deprecation
  warning and will be completely removed in v0.3.0.

  **Migration:** Use `zprof uninstall` instead. See the
  [migration guide](docs/user-guide/migration-rollback-to-uninstall.md)
  for details.

  **Why:** The new `uninstall` command provides more options:
  - Restore your original pre-zprof configuration
  - Promote any profile to become your root config
  - Clean removal without restoration
  - Safety backups before cleanup
  ```
- [ ] Add to "Breaking Changes" section (with note about v0.3.0 removal)
- [ ] Include in upgrade instructions

**Files:**
- `CHANGELOG.md`
- `docs/releases/v0.2.0.md`

---

### Story 5.7: Add Integration Test for Deprecation Warning

**As a** developer
**I want** tests verifying the deprecation warning
**So that** we ensure users see helpful messages

**Acceptance Criteria:**
- [ ] Create test in `tests/rollback_deprecation_test.rs`
- [ ] Test deprecation warning is shown
- [ ] Test user can decline and command exits
- [ ] Test `--force` flag skips warning
- [ ] Test help text shows "(deprecated)"
- [ ] Snapshot test for warning message format
- [ ] Verify rollback still works if user confirms (backward compatibility)

**Files:**
- `tests/rollback_deprecation_test.rs` (NEW)

---

## Technical Design

### Deprecation Warning Implementation

```rust
// src/cli/rollback.rs

pub fn execute(args: RollbackArgs) -> Result<()> {
    if !args.force {
        show_deprecation_warning()?;

        if !confirm_continue()? {
            println!("Rollback cancelled. Use 'zprof uninstall' for more options.");
            return Ok(());
        }
    }

    // Legacy rollback implementation
    // TODO(v0.3.0): Remove this entire function
    execute_legacy_rollback(args)
}

fn show_deprecation_warning() -> Result<()> {
    eprintln!("⚠ Warning: 'zprof rollback' is deprecated and will be removed in v0.3.0");
    eprintln!();
    eprintln!("Use 'zprof uninstall' instead, which provides more options:");
    eprintln!("  • Restore your original pre-zprof configuration");
    eprintln!("  • Promote a profile to become your root config");
    eprintln!("  • Clean removal without restoration");
    eprintln!();
    eprintln!("Run 'zprof uninstall --help' for details.");
    eprintln!();
    Ok(())
}
```

### Migration Mapping

| rollback behavior | uninstall equivalent |
|-------------------|----------------------|
| `zprof rollback` | `zprof uninstall` → choose "Restore original" |
| Restores pre-zprof backup | Same in uninstall with "Restore original" option |
| No profile promotion | Now available with "Promote profile" option |
| No safety backup | Automatic in uninstall |

## Dependencies

- **Epic 3 (Complete Uninstall System)**: Must be complete before removing rollback

## Risks & Mitigations

**Risk:** Users don't see deprecation warning
**Mitigation:** Warning shown on every rollback execution, updated help text, release notes

**Risk:** Scripts using rollback break
**Mitigation:** `--force` flag allows legacy usage, deprecation period gives time to update scripts

**Risk:** Documentation still references rollback
**Mitigation:** Comprehensive documentation audit, search and replace

## Testing Strategy

- Integration test for deprecation warning display
- Test backward compatibility (rollback still works with confirmation)
- Snapshot test for help text
- Manual testing of warning in real terminal
- Documentation review for rollback references

## Success Criteria

- [ ] Rollback command shows clear deprecation warning
- [ ] Users understand migration path to uninstall
- [ ] All documentation updated
- [ ] Backward compatibility maintained (command still works)
- [ ] Release notes clearly communicate the change
- [ ] Tests verify deprecation behavior
- [ ] No confusion about which command to use

## Out of Scope

- Actual removal of rollback code (deferred to v0.3.0)
- Automatic migration of user scripts
- Telemetry/analytics of rollback usage
