# Story 3.1: Git Operations Infrastructure

Status: done

## Story

As a developer,
I want zprof to be able to download frameworks from GitHub,
So that framework installation can work with real repositories instead of creating empty directories.

## Acceptance Criteria

1. Add `git2` dependency to Cargo.toml
2. Create `src/git.rs` module with `clone_repository(url, destination)` function
3. Function shows download progress using existing indicatif progress bars
4. Function handles basic git errors (network, permissions, invalid URLs)
5. Function validates destination directory before cloning
6. Integration tests verify git clone operations work

## Implementation Notes

**Dependencies to Add:**
```toml
git2 = "0.18"
```

**Files to Create:**
- `src/git.rs` - Core git operations module

**Files to Modify:**
- `Cargo.toml` - Add git2 dependency
- `src/lib.rs` - Export git module
- `src/frameworks/installer.rs` - Use git::clone_repository instead of fs::create_dir_all

**Testing Strategy:**
- Unit tests with temporary directories
- Integration tests with real git repositories (use small test repos)
- Mock network failures for error handling tests

## Definition of Done

- [ ] git2 dependency added and compiling
- [ ] git::clone_repository() function implemented
- [ ] Progress indicators integrated with existing installer progress bars
- [ ] Basic error handling for network/permission issues
- [ ] Unit tests pass
- [ ] Integration tests with real git clone pass
- [ ] installer.rs updated to use git operations (ready for Story 3.2)

**Estimated Effort:** 6-8 hours