# Epic Technical Specification: Remove Deprecated Rollback Command

Date: 2025-11-24
Author: Anna
Epic ID: 5
Status: Draft

---

## Overview

This epic focuses on the deprecation and eventual removal of the `zprof rollback` command, which has been superseded by the more comprehensive `zprof uninstall` command introduced in Epic 3. The rollback command was the original mechanism for restoring pre-zprof configuration state, but it lacks the flexibility and features of the new uninstall system. This epic establishes a graceful deprecation path that maintains backward compatibility while guiding users toward the superior replacement.

The work involves adding clear deprecation warnings to the rollback command, updating all documentation to reference the uninstall command, creating migration guides, and laying groundwork for complete removal in v0.3.0. This ensures users have adequate notice and support during the transition period.

## Objectives and Scope

**In Scope:**
- Add deprecation warning to `zprof rollback` command with interactive confirmation
- Update CLI help text to mark rollback as deprecated across all output
- Create comprehensive migration guide comparing rollback and uninstall features
- Audit and update all documentation to replace rollback references with uninstall
- Add `--force` flag to rollback for backward compatibility in scripts
- Create release notes communicating the deprecation clearly
- Add integration tests for deprecation warning behavior
- Add TODO comments in code marking areas for v0.3.0 removal
- Maintain full backward compatibility (command still works if user confirms)

**Out of Scope:**
- Actual removal of rollback code (deferred to v0.3.0 epic)
- Automatic migration of user scripts using rollback
- Telemetry or analytics tracking rollback usage
- Changes to uninstall command functionality

## System Architecture Alignment

This epic aligns with the existing CLI architecture pattern where all commands reside in `src/cli/` and follow consistent argument parsing and execution patterns. The deprecation approach follows Rust best practices for API deprecation while maintaining the modular CLI structure.

**Architecture Components Referenced:**
- **CLI Module (`src/cli/rollback.rs`)**: Modified to add deprecation warning flow before executing legacy behavior
- **CLI Main (`src/main.rs`)**: Updated to reflect deprecated status in command registration and help output
- **Core Business Logic**: No changes required; deprecation is purely at CLI interface layer
- **Testing Infrastructure**: Extended with new integration tests following existing patterns in `tests/` directory

**Constraints from Architecture:**
- Must maintain non-breaking change guarantee (command still functional)
- Follow anyhow error handling patterns
- Use clap for argument parsing with backward-compatible flags
- Preserve existing test coverage patterns using tempfile and serial_test
- Documentation must align with existing user guide structure

## Detailed Design

### Services and Modules

| Module | Responsibility | Inputs | Outputs | Owner |
|--------|---------------|--------|---------|-------|
| `src/cli/rollback.rs` | Rollback command implementation with deprecation warning | `RollbackArgs` (with new `--force` flag) | Success/error messages, deprecation warnings | CLI Team |
| `src/main.rs` | CLI command registration and help text | Command definitions | Clap-generated help output | CLI Team |
| `tests/rollback_deprecation_test.rs` | Deprecation behavior validation | Test scenarios | Test pass/fail results | QA Team |
| `docs/user-guide/migration-rollback-to-uninstall.md` | Migration documentation | User migration needs | Step-by-step guide | Documentation Team |
| `docs/user-guide/*.md` | User documentation updates | Existing docs with rollback references | Updated docs with uninstall references | Documentation Team |

**No new modules created** - all work occurs within existing CLI structure.

### Data Models and Contracts

**Modified: RollbackArgs Structure**

```rust
#[derive(Debug, Args)]
pub struct RollbackArgs {
    /// Skip deprecation warning (for scripts and CI)
    #[arg(long, default_value = "false")]
    pub force: bool,
}
```

**Fields:**
- `force`: Boolean flag to bypass interactive deprecation confirmation
  - Type: `bool`
  - Default: `false`
  - Purpose: Allow automated scripts to continue using rollback during deprecation period

**No database schema changes** - this epic is purely code and documentation updates.

**Deprecation Warning Message Contract:**

```text
⚠ Warning: 'zprof rollback' is deprecated and will be removed in v0.3.0

Use 'zprof uninstall' instead, which provides more options:
  • Restore your original pre-zprof configuration
  • Promote a profile to become your root config
  • Clean removal without restoration

Run 'zprof uninstall --help' for details.

Continue with legacy rollback? [y/N]
```

### APIs and Interfaces

**Modified CLI Command Interface:**

**Command: `zprof rollback [--force]`**

**Request Parameters:**
- `--force` (optional): Skip deprecation warning

**Response Codes:**
- Exit 0: Rollback successful or user cancelled
- Exit 1: Rollback failed (error in execution)

**User Interaction Flow:**

1. **Without `--force` flag:**
   ```bash
   $ zprof rollback
   ⚠ Warning: 'zprof rollback' is deprecated...
   Continue with legacy rollback? [y/N] █
   ```
   - User input: `y` → Proceed with rollback
   - User input: `n` or Enter → Cancel with message "Rollback cancelled. Use 'zprof uninstall' for more options."

2. **With `--force` flag:**
   ```bash
   $ zprof rollback --force
   [Rollback executes immediately without warning]
   ```

**Modified Help Output:**

```bash
$ zprof --help
...
COMMANDS:
  init         Initialize zprof
  create       Create a new profile
  use          Switch to a profile
  list         List all profiles
  delete       Delete a profile
  uninstall    Uninstall zprof and restore configuration
  rollback     (deprecated) Use 'uninstall' instead
...

$ zprof rollback --help
zprof-rollback (deprecated)

This command is deprecated. Use 'zprof uninstall' instead.

The new uninstall command provides:
  - Original configuration restoration
  - Profile promotion to root
  - Flexible cleanup options

See 'zprof uninstall --help' for details.

Usage: zprof rollback [OPTIONS]

Options:
      --force    Skip deprecation warning
  -h, --help     Print help
```

### Workflows and Sequencing

**Workflow 1: User runs `zprof rollback` (interactive)**

```
┌─────────────────────────────────────────┐
│ User executes: zprof rollback           │
└────────────┬────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────┐
│ rollback::execute(args)                 │
│ - Check args.force flag                 │
└────────────┬────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────┐
│ args.force == false?                    │
└──────┬──────────────────────┬───────────┘
       │ true                 │ false
       ▼                      ▼
┌─────────────────────┐  ┌───────────────────────┐
│ show_deprecation_   │  │ Skip to legacy        │
│ warning()           │  │ rollback execution    │
│ - Print warning     │  └───────────┬───────────┘
│ - Print migration   │              │
│   guidance          │              │
└──────┬──────────────┘              │
       │                             │
       ▼                             │
┌─────────────────────┐              │
│ confirm_continue()? │              │
│ [y/N] prompt        │              │
└──────┬──────────────┘              │
       │                             │
    ┌──┴──┐                          │
    │     │                          │
    y     n                          │
    │     │                          │
    │     ▼                          │
    │  ┌─────────────────────────┐  │
    │  │ Print "Rollback         │  │
    │  │ cancelled..."           │  │
    │  │ Return Ok(())           │  │
    │  └─────────────────────────┘  │
    │                                │
    └────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│ execute_legacy_rollback(args)           │
│ - Existing rollback implementation      │
│ - Restore pre-zprof backup              │
└────────────┬────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────┐
│ Display success message                 │
│ Exit with code 0                        │
└─────────────────────────────────────────┘
```

**Workflow 2: Automated script runs `zprof rollback --force`**

```
┌─────────────────────────────────────────┐
│ Script executes: zprof rollback --force │
└────────────┬────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────┐
│ rollback::execute(args)                 │
│ - args.force == true                    │
└────────────┬────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────┐
│ execute_legacy_rollback(args)           │
│ [No warning shown]                      │
└────────────┬────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────┐
│ Exit with code 0                        │
└─────────────────────────────────────────┘
```

**Workflow 3: Documentation update process**

```
1. Search all docs for "rollback" keyword
2. Review each occurrence:
   - Command example → Replace with "uninstall"
   - Conceptual reference → Add deprecation note
   - Migration scenario → Reference migration guide
3. Add migration guide document
4. Update command reference page
5. Add release notes entry
6. Review for consistency
```

## Non-Functional Requirements

### Performance

**Requirement:** Deprecation warning must not add perceptible latency to command execution.

**Measurable Targets:**
- Deprecation warning display: < 10ms overhead
- Interactive confirmation prompt: < 50ms to display (depends on terminal I/O)
- Overall command execution with confirmation: < 100ms additional overhead compared to current implementation
- `--force` flag execution: 0ms overhead (bypass warning entirely)

**Rationale:** Users should not experience performance degradation during deprecation period. The warning is a simple stdout write operation with minimal computational overhead.

**Source:** Existing architecture performance target of < 500ms for profile switching operations; deprecation adds negligible overhead.

### Security

**Requirement:** Deprecation changes must not introduce security vulnerabilities.

**Security Considerations:**
1. **No new attack surface**: Deprecation warning is purely informational output with no new user input beyond y/n confirmation
2. **Input validation**: User confirmation input limited to single character (y/n), validated before processing
3. **No privilege escalation**: Deprecation flow maintains same permission model as existing rollback command
4. **Force flag safety**: `--force` flag doesn't bypass any security checks, only user confirmation

**Compliance:**
- Maintain existing shell injection prevention in rollback command implementation
- Preserve path traversal validation in backup restoration logic
- No changes to file permission handling

**Source:** Architecture document security section on shell injection and path traversal prevention.

### Reliability/Availability

**Requirement:** Backward compatibility must be maintained throughout deprecation period.

**Reliability Targets:**
1. **100% backward compatibility**: All existing rollback functionality continues to work when user confirms
2. **Graceful degradation**: If confirmation prompt fails (non-interactive terminal), command should fail safely with clear error message
3. **Script compatibility**: `--force` flag ensures CI/CD pipelines and automated scripts continue to function
4. **Zero data loss**: Deprecation changes must not affect backup restoration reliability

**Error Handling:**
- Non-interactive terminal detection: Exit with error code 1 and message "Cannot prompt in non-interactive terminal. Use --force flag for automated execution."
- Invalid confirmation input: Re-prompt user (handle typos gracefully)
- Maintain existing rollback error handling for actual restore operations

**Availability:** No downtime or service interruption - pure client-side CLI change.

### Observability

**Requirement:** Deprecation usage should be visible in logs for migration tracking (optional, low priority).

**Logging Strategy:**
1. **Deprecation warning shown**: Log at INFO level when warning is displayed
   - Message: "Displayed rollback deprecation warning to user"
2. **User confirmation**: Log user's choice
   - "User confirmed rollback execution despite deprecation"
   - "User cancelled rollback, directed to uninstall command"
3. **Force flag usage**: Log when `--force` is used
   - Message: "Rollback executed with --force flag (deprecation warning skipped)"

**Log Format:** Use existing `log` crate with standard format
```rust
log::info!("Rollback deprecation warning displayed");
log::info!("User {} rollback execution", if confirmed { "confirmed" } else { "cancelled" });
```

**Metrics (Future Enhancement - Out of Scope for this Epic):**
- Could track deprecation warning display count
- Could track force flag usage frequency
- Deferred to v0.3.0 epic if needed for migration planning

**Source:** Architecture uses `log` crate for application logging; extend existing patterns.

## Dependencies and Integrations

**External Dependencies (from Cargo.toml):**

| Dependency | Version | Purpose | Changes Required |
|------------|---------|---------|------------------|
| `clap` | 4.5.51 | CLI argument parsing | Add `--force` flag to `RollbackArgs` |
| `dialoguer` | 0.11 | Interactive prompts | Use `Confirm` for y/n confirmation prompt |
| `anyhow` | 1.0 | Error handling | No changes (existing patterns) |
| `log` | 0.4 | Logging | Add info-level logs for deprecation events |

**No new dependencies required** - all functionality uses existing crates.

**Integration Points:**

1. **CLI Module Integration:**
   - `src/main.rs` registers rollback command with updated help text
   - `src/cli/rollback.rs` implements deprecation flow
   - No changes to other CLI commands

2. **Core Business Logic:**
   - No integration changes required
   - Deprecation is purely CLI presentation layer
   - Existing rollback logic (`execute_legacy_rollback`) called unchanged

3. **Testing Integration:**
   - New test file: `tests/rollback_deprecation_test.rs`
   - Integrates with existing test infrastructure (`tempfile`, `serial_test`, `insta`)
   - Uses snapshot testing for warning message validation

4. **Documentation Integration:**
   - Migration guide links to existing uninstall command documentation
   - User guide updates reference consolidated command list
   - Release notes integrate with v0.2.0 changelog structure

**Version Constraints:**
- No changes to dependency versions
- Maintains compatibility with Rust 1.70+ (existing requirement)
- No breaking changes to public API (CLI interface)

**Downstream Impact:**
- **User Scripts**: Scripts using `zprof rollback` must add `--force` flag to suppress warnings
- **CI/CD Pipelines**: Same as user scripts - add `--force` for automated execution
- **Documentation Sites**: Any external tutorials or guides referencing rollback should update to uninstall

## Acceptance Criteria (Authoritative)

**AC-1: Deprecation Warning Display**
- GIVEN a user runs `zprof rollback` without `--force` flag
- WHEN the command executes
- THEN a deprecation warning is displayed with:
  - Warning symbol and "deprecated" keyword
  - Clear message that command will be removed in v0.3.0
  - List of uninstall command benefits (restore, promote, clean removal)
  - Reference to `zprof uninstall --help`
  - Interactive y/N prompt for continuation

**AC-2: Interactive Confirmation Behavior**
- GIVEN the deprecation warning is shown
- WHEN user enters 'y' or 'Y'
- THEN rollback executes normally with existing functionality
- WHEN user enters 'n', 'N', or presses Enter (default)
- THEN command exits gracefully with message directing to uninstall command
- AND exit code is 0 (not an error, intentional cancellation)

**AC-3: Force Flag Bypass**
- GIVEN a user or script runs `zprof rollback --force`
- WHEN the command executes
- THEN deprecation warning is completely skipped
- AND rollback executes immediately with no user interaction
- AND behavior is identical to pre-deprecation rollback

**AC-4: Help Text Updates - Main Command List**
- GIVEN a user runs `zprof --help`
- WHEN viewing command list
- THEN rollback command appears with "(deprecated) Use 'uninstall' instead" suffix
- AND rollback still appears in list (not hidden)
- AND uninstall command is listed prominently

**AC-5: Help Text Updates - Rollback Specific**
- GIVEN a user runs `zprof rollback --help`
- WHEN viewing command help
- THEN help output shows:
  - "zprof-rollback (deprecated)" header
  - Clear deprecation message
  - Migration guidance to uninstall command
  - Benefits of uninstall over rollback
  - `--force` flag documentation
  - Reference to `zprof uninstall --help`

**AC-6: Migration Guide Exists**
- GIVEN documentation is updated
- WHEN reviewing user guide
- THEN `docs/user-guide/migration-rollback-to-uninstall.md` exists with:
  - Why rollback is deprecated
  - Feature comparison table (rollback vs uninstall)
  - Migration examples showing equivalent commands
  - FAQ addressing common concerns

**AC-7: Documentation Audit Complete**
- GIVEN all user-facing documentation
- WHEN searching for "rollback" references
- THEN all occurrences are either:
  - Replaced with "uninstall" (command examples)
  - Updated with deprecation note (historical references)
  - Linked to migration guide (transition scenarios)
- AND no broken examples or outdated instructions remain

**AC-8: Release Notes Entry**
- GIVEN v0.2.0 release notes
- WHEN reviewing CHANGELOG.md
- THEN deprecation is documented in "Deprecated" section with:
  - Clear statement of deprecation
  - Removal timeline (v0.3.0)
  - Migration guidance link
  - Rationale for change (uninstall benefits)

**AC-9: Code Marked for Future Removal**
- GIVEN `src/cli/rollback.rs` and related files
- WHEN reviewing code
- THEN TODO comments exist marking v0.3.0 removal:
  - `// TODO(v0.3.0): Remove this entire module`
  - Comments at module and function level
  - Clear indication this is temporary code

**AC-10: Integration Tests Pass**
- GIVEN `tests/rollback_deprecation_test.rs`
- WHEN running test suite
- THEN all tests pass verifying:
  - Deprecation warning displays correctly (snapshot test)
  - User can decline and command exits gracefully
  - `--force` flag skips warning
  - Help text shows "(deprecated)" marker
  - Rollback still works when user confirms (backward compatibility)

**AC-11: Non-Interactive Terminal Handling**
- GIVEN rollback is run in non-interactive terminal (CI/CD)
- WHEN deprecation prompt would appear
- THEN command fails with clear error message
- AND error message instructs user to add `--force` flag
- AND exit code is 1 (error state)

**AC-12: Backward Compatibility Maintained**
- GIVEN existing rollback functionality
- WHEN user confirms deprecation prompt
- THEN rollback executes with identical behavior to v0.1.x
- AND no functionality is removed or altered
- AND existing tests continue to pass

## Traceability Mapping

| Acceptance Criteria | Spec Section(s) | Component(s) / Module(s) | Test Coverage |
|---------------------|----------------|--------------------------|---------------|
| AC-1: Deprecation Warning Display | APIs & Interfaces → User Interaction Flow | `src/cli/rollback.rs::show_deprecation_warning()` | `rollback_deprecation_test::test_warning_display` (snapshot) |
| AC-2: Interactive Confirmation | Workflows → Workflow 1 | `src/cli/rollback.rs::confirm_continue()` | `rollback_deprecation_test::test_user_decline`, `test_user_confirm` |
| AC-3: Force Flag Bypass | APIs & Interfaces → Request Parameters, Workflows → Workflow 2 | `src/cli/rollback.rs::execute()` with `args.force` check | `rollback_deprecation_test::test_force_flag_skips_warning` |
| AC-4: Help Text - Main | APIs & Interfaces → Modified Help Output | `src/main.rs` Clap command registration | `rollback_deprecation_test::test_main_help_shows_deprecated` (snapshot) |
| AC-5: Help Text - Rollback | APIs & Interfaces → Modified Help Output | `src/cli/rollback.rs` Clap `#[command(about = "...")]` | `rollback_deprecation_test::test_rollback_help_shows_migration` (snapshot) |
| AC-6: Migration Guide | Services & Modules → Documentation | `docs/user-guide/migration-rollback-to-uninstall.md` | Manual review checklist |
| AC-7: Documentation Audit | Services & Modules → Documentation | All `docs/**/*.md` files | Manual review checklist + grep validation |
| AC-8: Release Notes | Dependencies → Documentation Integration | `CHANGELOG.md`, `docs/releases/v0.2.0.md` | Manual review checklist |
| AC-9: Code Removal Markers | Data Models → RollbackArgs, all modified code | `src/cli/rollback.rs`, `src/main.rs` | Code review checklist (grep for TODO comments) |
| AC-10: Integration Tests | Services & Modules → Testing | `tests/rollback_deprecation_test.rs` | Test suite execution (`cargo test rollback_deprecation`) |
| AC-11: Non-Interactive Handling | NFR → Reliability → Error Handling | `src/cli/rollback.rs::confirm_continue()` with terminal detection | `rollback_deprecation_test::test_non_interactive_fails_gracefully` |
| AC-12: Backward Compatibility | NFR → Reliability → Backward Compatibility | All existing rollback code paths | Existing `tests/rollback_test.rs` continues passing |

**Test Strategy Notes:**
- **Snapshot Tests**: Used for help text and warning message validation (using `insta` crate)
- **Integration Tests**: Cover all user interaction paths (confirm, decline, force flag, non-interactive)
- **Manual Checklists**: Required for documentation review (not automatable)
- **Regression Tests**: Existing rollback tests ensure functionality unchanged when confirmed

## Risks, Assumptions, Open Questions

### Risks

**RISK-1: Users Don't See Deprecation Warning**
- **Severity**: Medium
- **Probability**: Low
- **Impact**: Users unaware of migration path, surprised by removal in v0.3.0
- **Mitigation**:
  - Warning shown on every execution (impossible to miss)
  - Updated help text visible in `--help` output
  - Release notes prominently communicate deprecation
  - Migration guide published before v0.2.0 release
  - Consider announcement in project README

**RISK-2: Automated Scripts Break**
- **Severity**: High
- **Probability**: Medium
- **Impact**: CI/CD pipelines fail, automation breaks
- **Mitigation**:
  - `--force` flag allows immediate bypass
  - Non-interactive terminals get clear error message with fix
  - Release notes include script migration instructions
  - Deprecation period (v0.2.0 → v0.3.0) gives time to update
  - Backward compatible - scripts still work if updated

**RISK-3: Documentation References Missed**
- **Severity**: Low
- **Probability**: Medium
- **Impact**: Confusing documentation with mixed rollback/uninstall references
- **Mitigation**:
  - Comprehensive grep/search for "rollback" keyword
  - Manual review of all user guide files
  - Story 5.4 dedicated to documentation audit
  - Checklist-based verification process

**RISK-4: Uninstall Command Doesn't Cover All Rollback Use Cases**
- **Severity**: High
- **Probability**: Low
- **Impact**: Users lose functionality when rollback removed in v0.3.0
- **Mitigation**:
  - Assumption validated: Epic 3 uninstall is superset of rollback
  - Feature comparison table in migration guide shows equivalence
  - Deprecation period allows user feedback
  - If gaps found, defer v0.3.0 removal until addressed

### Assumptions

**ASSUMPTION-1: Epic 3 (Complete Uninstall System) is Complete**
- **Statement**: The `zprof uninstall` command is fully implemented and stable
- **Validation**: Check Epic 3 completion status in sprint planning
- **Impact if False**: Cannot deprecate rollback without replacement ready
- **Confidence**: High (Epic 3 listed as dependency in epic document)

**ASSUMPTION-2: Users Prefer Interactive Warnings**
- **Statement**: Users appreciate being informed of deprecations during command execution
- **Validation**: Industry standard practice (npm, cargo, git all use similar warnings)
- **Impact if False**: Warning could be perceived as annoying
- **Confidence**: High (with `--force` opt-out, users have control)

**ASSUMPTION-3: One Version Deprecation Period is Sufficient**
- **Statement**: v0.2.0 → v0.3.0 gives users adequate migration time
- **Validation**: Typical software deprecation cycles, project release cadence
- **Impact if False**: Users feel rushed, negative experience
- **Confidence**: Medium (depends on v0.3.0 release timeline - recommend 3+ months)

**ASSUMPTION-4: Rollback is Lightly Used Compared to Other Commands**
- **Statement**: Most users use profiles, not rollback (rollback is "escape hatch")
- **Validation**: No usage metrics available (telemetry out of scope)
- **Impact if False**: High churn from deprecation
- **Confidence**: Medium (based on command purpose - one-time restoration vs frequent profile switching)

**ASSUMPTION-5: No External Documentation References Rollback**
- **Statement**: Only in-repo documentation needs updating
- **Validation**: Project is early-stage, limited external adoption
- **Impact if False**: External guides become outdated
- **Confidence**: Medium (recommend web search for "zprof rollback" before release)

### Open Questions

**QUESTION-1: Should We Add Telemetry for Rollback Usage?**
- **Context**: Tracking rollback usage could inform v0.3.0 removal decision
- **Options**:
  - A) Add opt-in telemetry tracking deprecation warning displays
  - B) No telemetry (keep it simple, current approach)
- **Decision Needed By**: Story prioritization (currently out of scope)
- **Recommendation**: Defer to v0.3.0 epic; not critical for deprecation announcement

**QUESTION-2: Should `--force` Be Documented Prominently or Hidden?**
- **Context**: Balance between script compatibility and encouraging migration
- **Options**:
  - A) Document `--force` prominently (helps script users, might delay migration)
  - B) Document minimally (encourages migration, might frustrate automation users)
- **Decision Needed By**: Story 5.2 (help text updates)
- **Recommendation**: Document clearly but not prominently - include in `--help` but emphasize migration in main warning

**QUESTION-3: Should Rollback Help Show Full Legacy Help or Just Migration Message?**
- **Context**: `zprof rollback --help` could show full original help or focus on migration
- **Current Spec**: Shows migration-focused message (implemented in AC-5)
- **Validation**: Aligns with deprecation goal - users should migrate, not continue using
- **Status**: Resolved in spec (migration message approach)

**QUESTION-4: What Happens If User Finds Bug in Uninstall During Deprecation Period?**
- **Context**: If uninstall has issues, rollback might be needed as fallback
- **Options**:
  - A) Keep rollback fully functional (current spec)
  - B) Direct users to file bug, defer to support case
- **Decision**: Current spec maintains full rollback functionality, mitigates this risk
- **Status**: Resolved (backward compatibility approach)

**QUESTION-5: Should We Automate Documentation Audit?**
- **Context**: Story 5.4 requires auditing all docs for "rollback" references
- **Options**:
  - A) Manual review (current spec)
  - B) Automated script + manual validation
- **Decision Needed By**: Story 5.4 implementation
- **Recommendation**: Use `grep -r "rollback" docs/` to identify, manual review to validate replacements

## Test Strategy Summary

### Test Levels

**1. Unit Tests** (Low Priority - Minimal New Logic)
- Simple deprecation warning is straightforward, limited unit test value
- Focus on integration tests for end-to-end behavior

**2. Integration Tests** (PRIMARY TEST FOCUS)
- **File**: `tests/rollback_deprecation_test.rs`
- **Coverage Areas**:
  1. Deprecation warning display and format (snapshot test)
  2. User confirmation flow (accept/decline)
  3. Force flag bypasses warning
  4. Non-interactive terminal handling
  5. Help text updates (main and rollback-specific)
  6. Backward compatibility (rollback still works when confirmed)

**Test Cases:**
```rust
#[test]
fn test_deprecation_warning_displayed() {
    // Snapshot test for warning message format
    let output = run_command("rollback");
    insta::assert_snapshot!(output);
}

#[test]
fn test_user_declines_rollback() {
    // Simulate 'n' input, verify graceful exit
    let result = run_command_with_input("rollback", "n\n");
    assert!(result.contains("Rollback cancelled"));
    assert!(result.contains("Use 'zprof uninstall'"));
}

#[test]
fn test_user_confirms_rollback() {
    // Simulate 'y' input, verify rollback executes
    let result = run_command_with_input("rollback", "y\n");
    assert!(result.contains("successfully")); // Rollback success message
}

#[test]
fn test_force_flag_skips_warning() {
    // Verify --force bypasses all prompts
    let output = run_command("rollback --force");
    assert!(!output.contains("deprecated"));
    assert!(!output.contains("Continue with"));
}

#[test]
fn test_main_help_shows_deprecated() {
    // Snapshot test for main help output
    let output = run_command("--help");
    insta::assert_snapshot!(output);
    assert!(output.contains("rollback"));
    assert!(output.contains("(deprecated)"));
}

#[test]
fn test_rollback_help_shows_migration() {
    // Snapshot test for rollback-specific help
    let output = run_command("rollback --help");
    insta::assert_snapshot!(output);
    assert!(output.contains("deprecated"));
    assert!(output.contains("uninstall"));
}

#[test]
fn test_non_interactive_fails_gracefully() {
    // Test behavior in CI/CD (non-interactive terminal)
    let result = run_command_non_interactive("rollback");
    assert!(result.is_err());
    assert!(result.contains("--force"));
}
```

**3. Snapshot Tests** (Using `insta` Crate)
- Warning message format
- Help text output (main and rollback-specific)
- Error messages
- **Update Process**: `cargo insta review` after changes

**4. Manual Testing**
- Real terminal interaction (verify prompt UX)
- macOS and Linux terminal emulators
- Copy-paste warning message for clarity validation

**5. Documentation Testing**
- **Checklist-Based Review**:
  - [ ] Search all docs for "rollback" keyword
  - [ ] Verify each replacement/update is appropriate
  - [ ] Check migration guide completeness
  - [ ] Validate release notes clarity
  - [ ] Test all code examples in docs
  - [ ] Verify no broken links

### Test Execution

**CI Pipeline:**
```bash
# Run all tests including new deprecation tests
cargo test

# Review snapshot changes (manual step in PR)
cargo insta review

# Clippy for code quality
cargo clippy --all-targets --all-features -- -D warnings

# Check documentation links
# (Manual or markdown-link-check tool)
```

**Pre-Release Checklist:**
- [ ] All integration tests pass
- [ ] Snapshot tests reviewed and approved
- [ ] Manual terminal testing complete
- [ ] Documentation audit checklist complete
- [ ] Release notes reviewed
- [ ] Migration guide validated

### Test Frameworks and Tools

| Tool | Purpose | Usage |
|------|---------|-------|
| `insta` | Snapshot testing | Help text, warning message validation |
| `tempfile` | Isolated test environments | Existing rollback tests |
| `serial_test` | Serialize tests modifying HOME | Existing pattern |
| `cargo test` | Test execution | Standard Rust testing |
| `grep` | Documentation audit | Find "rollback" references |

### Edge Cases to Test

1. **Empty input** (user presses Enter) → Should default to 'N' (cancel)
2. **Invalid input** (user types random characters) → Should re-prompt
3. **Ctrl+C during prompt** → Should exit gracefully
4. **Multiple sequential rollback calls** → Each shows warning (no caching)
5. **Help text in different terminal widths** → Should format reasonably

### Success Criteria

- **100% of integration tests pass** (new and existing)
- **0 regressions** in existing rollback functionality
- **All snapshots reviewed and approved** by developer and reviewer
- **Documentation checklist 100% complete** with no rollback references missed
- **Manual testing successful** on macOS and Linux
