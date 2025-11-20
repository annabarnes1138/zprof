# Epic 3: Real Framework Installation

**Status:** Planning
**Priority:** Critical
**Effort Estimate:** 20-30 hours (Phase 1)

## Problem Statement

The current implementation creates placeholder directories instead of actually installing frameworks. Users see "installation complete" but get non-functional profiles with empty framework directories.

## Epic Goal

Replace all `unimplemented!()` framework installation stubs with real git-based framework downloading and setup, starting with the most popular frameworks (Oh-My-Zsh and Zap).

## Stories

### Story 3.1: Git Operations Infrastructure
**Effort:** 6-8 hours  
**Priority:** Blocking (required for all other stories)  
**Summary:** Add git2 dependency and basic git clone operations with progress callbacks

### Story 3.2: Zap Framework Installation  
**Effort:** 4-6 hours  
**Priority:** High (simplest framework, good validation)  
**Summary:** Replace `unimplemented!()` in zap.rs with actual installation

### Story 3.3: Oh-My-Zsh Framework Installation
**Effort:** 8-12 hours  
**Priority:** High (most popular framework)  
**Summary:** Replace `unimplemented!()` in oh_my_zsh.rs with full installation

### Story 3.4: Installation Testing Updates
**Effort:** 4-6 hours  
**Priority:** Medium  
**Summary:** Update tests to verify real installations

### Story 3.5: Network Error Handling
**Effort:** 4-6 hours  
**Priority:** Medium  
**Summary:** Add connectivity checks and retry logic

## Success Criteria

- Users can create profiles with Oh-My-Zsh and Zap that actually work
- `zprof use profile` loads functional framework environments  
- Installation progress shows real download progress
- Network failures are handled gracefully

## Out of Scope (Phase 2)

- Zimfw, Prezto, Zinit implementations
- Advanced plugin installation features
- Framework version detection