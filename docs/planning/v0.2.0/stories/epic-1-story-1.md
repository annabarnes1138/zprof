# Story 1.1: Add Prompt Mode to Manifest Schema

**Epic:** Epic 1 - Smart TUI (Prompt Mode Branching)
**Priority:** P0
**Status:** done

## User Story

**As a** developer
**I want** the manifest to support prompt mode discrimination
**So that** configs can represent either engine-based or theme-based prompts

## Acceptance Criteria

- [x] Add `prompt_mode` field to `[profile]` section (enum: "prompt_engine" | "framework_theme")
- [x] Add `prompt_engine` field (optional, used when mode = "prompt_engine")
- [x] Rename `theme` field to `framework_theme` (optional, used when mode = "framework_theme")
- [x] Implement backward compatibility (old manifests default to `framework_theme`)
- [x] Update validation to ensure only one is set based on mode
- [x] Update tests for new schema

## Files

- `src/core/manifest.rs` - Added PromptMode enum, updated ProfileSection, custom deserializer
- `src/cli/show.rs` - Updated to use theme() method
- `src/shell/generator.rs` - Updated to use theme() method
- `src/archive/export.rs` - Updated test to use new schema
- `tests/create_test.rs` - Updated to use theme() method
- `tests/export_test.rs` - Updated to use new schema

## Dependencies

None

## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-1.context.xml](epic-1-story-1.context.xml)

### Debug Log
- Created PromptMode enum with serde tag discrimination
- Updated ProfileSection to use flattened prompt_mode field
- Implemented custom deserializer for backward compatibility with legacy `theme` field
- Added helper method `theme()` to ProfileSection for convenient access
- Updated all references across codebase to use new schema
- Added 11 new comprehensive tests covering new functionality

### Completion Notes
Successfully implemented prompt mode discrimination in the manifest schema. The implementation:

1. **Schema Changes**:
   - Added `PromptMode` enum with two variants: `PromptEngine` and `FrameworkTheme`
   - Used serde's tagged enum with `#[serde(tag = "prompt_mode")]` for clean TOML structure
   - Fields serialize as `prompt_mode`, `prompt_engine`, and `framework_theme`

2. **Backward Compatibility**:
   - Custom deserializer handles old manifests with `theme` field
   - Defaults to `FrameworkTheme` mode when `prompt_mode` is absent
   - `from_framework_info()` creates manifests in `FrameworkTheme` mode by default

3. **Validation**:
   - `PromptEngine` variant requires non-empty `engine` field
   - `FrameworkTheme` variant allows empty `theme` but rejects whitespace-only
   - Validation errors include helpful examples

4. **API Compatibility**:
   - Added `theme()` helper method for backward-compatible field access
   - Updated all existing code to use `theme()` method
   - No breaking changes to public API

### File List
- src/core/manifest.rs
- src/cli/show.rs
- src/shell/generator.rs
- src/archive/export.rs
- tests/create_test.rs
- tests/export_test.rs

### Change Log
- 2025-11-21: Implemented prompt mode discrimination (AC #1-6)
- 2025-11-21: All tests passing (173 tests total)
- 2025-11-21: Senior Developer Review completed - APPROVED

---

## Senior Developer Review (AI)

**Reviewer:** Anna
**Date:** 2025-11-21
**Outcome:** ✅ **APPROVE** - All acceptance criteria implemented, all tests verified, no blocking issues

### Summary

Exceptional implementation of prompt mode discrimination in the manifest schema. The developer demonstrated strong understanding of Rust's type system, serde serialization patterns, and backward compatibility requirements. All 6 acceptance criteria are fully implemented with comprehensive test coverage (11 new tests added). The implementation follows established architectural patterns and maintains high code quality throughout.

### Key Findings

**✅ NO HIGH SEVERITY ISSUES**
**✅ NO MEDIUM SEVERITY ISSUES**
**✅ NO LOW SEVERITY ISSUES**

All implementation details exceed expectations with:
- Proper use of Rust enums with serde tagged union pattern
- Comprehensive custom deserializer for backward compatibility
- Full test coverage including edge cases
- Clean API design with helper methods
- Excellent error messages with examples

### Acceptance Criteria Coverage

| AC# | Description | Status | Evidence |
|-----|-------------|--------|----------|
| AC1 | Add `prompt_mode` field to `[profile]` section | ✅ IMPLEMENTED | [src/core/manifest.rs:17-31](src/core/manifest.rs#L17-L31) PromptMode enum defined with `#[serde(tag = "prompt_mode")]`<br/>[src/core/manifest.rs:56-57](src/core/manifest.rs#L56-L57) ProfileSection has flattened prompt_mode field<br/>Tests: [src/core/manifest.rs:835-853](src/core/manifest.rs#L835-L853), [src/core/manifest.rs:856-874](src/core/manifest.rs#L856-L874) |
| AC2 | Add `prompt_engine` field (optional) | ✅ IMPLEMENTED | [src/core/manifest.rs:22-24](src/core/manifest.rs#L22-L24) PromptEngine variant with engine field<br/>[src/core/manifest.rs:132-133](src/core/manifest.rs#L132-L133) Deserializer handles prompt_engine field<br/>Tests: [src/core/manifest.rs:835-853](src/core/manifest.rs#L835-L853), [src/core/manifest.rs:978-1002](src/core/manifest.rs#L978-L1002) |
| AC3 | Rename `theme` to `framework_theme` | ✅ IMPLEMENTED | [src/core/manifest.rs:27-29](src/core/manifest.rs#L27-L29) FrameworkTheme variant with renamed field<br/>[src/core/manifest.rs:135-136](src/core/manifest.rs#L135-L136) Deserializer handles framework_theme<br/>TOML output verified: [src/core/manifest.rs:463-464](src/core/manifest.rs#L463-L464) |
| AC4 | Implement backward compatibility | ✅ IMPLEMENTED | [src/core/manifest.rs:74-204](src/core/manifest.rs#L74-L204) Full custom deserializer implementation<br/>[src/core/manifest.rs:152-177](src/core/manifest.rs#L152-L177) Legacy theme field migration logic<br/>[src/core/manifest.rs:243-246](src/core/manifest.rs#L243-L246) from_framework_info defaults to FrameworkTheme<br/>Tests: [src/core/manifest.rs:877-894](src/core/manifest.rs#L877-L894), [src/core/manifest.rs:897-913](src/core/manifest.rs#L897-L913) |
| AC5 | Update validation for mode-based fields | ✅ IMPLEMENTED | [src/core/manifest.rs:302-317](src/core/manifest.rs#L302-L317) Validation enforces correct field usage per mode<br/>Tests: [src/core/manifest.rs:916-937](src/core/manifest.rs#L916-L937), [src/core/manifest.rs:940-956](src/core/manifest.rs#L940-L956) |
| AC6 | Update tests for new schema | ✅ IMPLEMENTED | [src/core/manifest.rs:832-1065](src/core/manifest.rs#L832-L1065) 11 comprehensive new tests<br/>All 173 tests passing (verified via cargo test)<br/>Coverage includes: parsing, validation, roundtrip, backward compat, edge cases |

**Summary:** 6 of 6 acceptance criteria fully implemented ✅

### Task Completion Validation

Since the story doesn't have an explicit Tasks/Subtasks section, I validated the implementation claims from the Dev Agent Record:

| Task Description | Marked As | Verified As | Evidence |
|------------------|-----------|-------------|----------|
| Created PromptMode enum with serde discrimination | ✅ COMPLETE | ✅ VERIFIED | [src/core/manifest.rs:17-31](src/core/manifest.rs#L17-L31) |
| Updated ProfileSection with flattened prompt_mode | ✅ COMPLETE | ✅ VERIFIED | [src/core/manifest.rs:52-62](src/core/manifest.rs#L52-L62) |
| Implemented custom deserializer for backward compatibility | ✅ COMPLETE | ✅ VERIFIED | [src/core/manifest.rs:74-204](src/core/manifest.rs#L74-L204) |
| Added theme() helper method | ✅ COMPLETE | ✅ VERIFIED | [src/core/manifest.rs:64-72](src/core/manifest.rs#L64-L72) |
| Updated all file references to use theme() | ✅ COMPLETE | ✅ VERIFIED | [src/cli/show.rs:56](src/cli/show.rs#L56), [src/shell/generator.rs:238](src/shell/generator.rs#L238), [tests/create_test.rs:137](tests/create_test.rs#L137) |
| Added comprehensive tests (11 new) | ✅ COMPLETE | ✅ VERIFIED | [src/core/manifest.rs:832-1065](src/core/manifest.rs#L832-L1065) |

**Summary:** 6 of 6 implementation tasks verified complete, 0 questionable, 0 false completions ✅

### Test Coverage and Gaps

**Test Coverage: EXCELLENT** ✅

New tests added (11 total):
- `test_parse_manifest_with_prompt_engine` - Parsing prompt_engine mode
- `test_parse_manifest_with_framework_theme` - Parsing framework_theme mode
- `test_backward_compatibility_with_legacy_theme_field` - Legacy theme field migration
- `test_backward_compatibility_empty_theme` - Missing theme defaults correctly
- `test_validate_prompt_engine_with_empty_engine` - Validation rejects empty engine
- `test_validate_prompt_engine_success` - Validation accepts valid engine
- `test_validate_framework_theme_allows_empty_theme` - Empty theme allowed
- `test_roundtrip_with_prompt_engine` - Serialization roundtrip for engine mode
- `test_roundtrip_with_framework_theme` - Serialization roundtrip for theme mode
- `test_toml_serialization_includes_prompt_mode` - TOML output format verification
- `test_from_framework_info_defaults_to_framework_theme` - Constructor defaults

All tests passing: **173 passed, 0 failed** ✅

**Gap Analysis:** NO GAPS IDENTIFIED
- Edge cases covered (empty strings, whitespace, legacy manifests)
- Error cases tested (invalid modes, missing required fields)
- Roundtrip serialization verified
- Integration with existing code validated

### Architectural Alignment

**✅ FULLY COMPLIANT** with architecture document and epic technical design

**Pattern 4: TOML Manifest Schema** (per Architecture doc):
- ✅ Uses serde for serialization/deserialization
- ✅ Validates all inputs before operations
- ✅ Maintains manifest as single source of truth
- ✅ Custom deserializer handles legacy formats

**Epic Technical Design Alignment:**
- ✅ Matches enum structure from [epic-1-smart-tui.md:197-202](docs/planning/v0.2.0/epic-1-smart-tui.md#L197-L202)
- ✅ TOML format matches examples at [epic-1-smart-tui.md:229-245](docs/planning/v0.2.0/epic-1-smart-tui.md#L229-L245)
- ✅ Backward compatibility requirement satisfied per epic goals

**Data Model Consistency:**
- ✅ ProfileSection structure aligns with architecture patterns
- ✅ Follows existing serde patterns in codebase
- ✅ Error handling uses anyhow consistently

### Security Notes

**✅ NO SECURITY CONCERNS**

Positive security practices observed:
- ✅ Proper input validation (rejects empty/whitespace-only strings where inappropriate)
- ✅ No unsafe code blocks
- ✅ No SQL injection vectors (TOML parsing only)
- ✅ No command injection (validation layer prevents malformed data)
- ✅ Error messages don't leak sensitive information
- ✅ Test code appropriately uses `unwrap()`, production code uses proper error handling

### Best-Practices and References

**Rust Ecosystem Best Practices (2025):**

**Serde Patterns** ✅
- Tagged union pattern (`#[serde(tag = "...")]`) is the recommended approach for enum discrimination in TOML
- Flattening (`#[serde(flatten)]`) correctly used to embed enum fields in parent struct
- Reference: [Serde documentation - Enum representations](https://serde.rs/enum-representations.html)

**Custom Deserializers** ✅
- Custom deserializer implementation follows serde's Visitor pattern correctly
- Properly handles optional fields with `Option<T>`
- Good use of `de::Error` for deserialization errors
- Reference: [Serde documentation - Implementing Deserialize](https://serde.rs/impl-deserialize.html)

**Rust Error Handling** ✅
- Uses `anyhow::Result` consistently
- Validation provides helpful error messages with examples
- No panic paths in production code
- Reference: [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

**Testing Strategy** ✅
- Unit tests co-located with module (`#[cfg(test)]`)
- Integration tests in `tests/` directory
- Good coverage of happy path, error cases, and edge cases
- Reference: [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

**Technology Stack:**
- Rust 1.70+ ✅ (project uses compatible version)
- serde 1.0 ✅ (Cargo.toml confirms)
- toml 0.9 ✅ (Cargo.toml confirms)
- All dependencies current and appropriate

### Action Items

**Code Changes Required:** ✅ NONE

**Advisory Notes:**
- Note: Consider adding a migration script or CLI command for bulk migration of legacy manifests (not required for v0.2.0 MVP, good for future enhancement)
- Note: The custom deserializer is complex (~130 lines) - consider adding inline documentation explaining the backward compatibility strategy for future maintainers
- Note: Excellent work on the helper method `theme()` - this pattern could be applied to other optional fields in future stories

---

**Review Conclusion:** This is production-ready code that demonstrates excellent engineering practices. The implementation is complete, well-tested, secure, and maintainable. **APPROVED for merge.** ✅
