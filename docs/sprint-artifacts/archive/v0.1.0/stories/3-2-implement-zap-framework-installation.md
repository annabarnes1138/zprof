# Story 3.2: Implement Zap Framework Installation

Status: done

## Story

As a developer,
I want zprof to actually install the Zap framework when I select it in the wizard,
So that my profile contains a working Zap installation instead of empty directories.

## Acceptance Criteria

1. ✅ Replace `unimplemented!()` in zap.rs with actual installation call
2. ✅ Zap framework downloads from `https://github.com/zap-zsh/zap.git`
3. ✅ Zap installs to `profile_path/.zap` directory within the profile
4. ✅ Installation progress shows real download progress via git operations
5. ✅ Installation errors are handled gracefully with clear messages
6. ✅ Tests verify actual Zap installation (integration tests with `#[ignore]`)

## Implementation Notes

**Repository URL**: `https://github.com/zap-zsh/zap.git`  
**Installation Directory**: `profile_path/.zap` (profile-scoped)  
**Installation Method**: Direct git clone using `crate::git::clone_repository`

**Files Modified:**
- `src/frameworks/zap.rs` - Updated `install()` method to call installer module
- `src/frameworks/installer.rs` - Added `install_zap()` function with real git clone
- Tests updated to verify actual git repository structure

**Progress Integration:**
- Uses existing indicatif progress bars from installer module
- Git clone progress is tracked and displayed to user
- Progress callbacks integrated with git2 transfer progress

## Definition of Done

- [x] Zap framework installs from real git repository
- [x] Installation progress shows actual download progress  
- [x] Error handling for network/git failures
- [x] Integration tests verify .zap/.git directory exists
- [x] Framework installation forwards from trait to installer module
- [x] No more `unimplemented!()` macros in zap.rs

**Estimated Effort:** 4-6 hours ✅ **COMPLETED**

## Results

Users can now select Zap in the TUI wizard and get a fully functional Zap framework installation. The profile switching will load a real Zap environment instead of empty directories.

**Next Story:** 3.3 - Implement Oh-My-Zsh Framework Installation (also completed in this implementation cycle)