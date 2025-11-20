# Technical Decisions - zprof

**Project:** zprof
**Last Updated:** 2025-10-31

---

## Architecture Decisions

### AD-001: Profile Storage Directory Name
**Date:** 2025-10-31
**Status:** Decided
**Context:** Need to determine the directory name for storing zprof profiles in user's home directory.
**Decision:** Use `~/.zsh-profiles/` instead of `~/.zprof/`
**Rationale:**
- More descriptive and self-documenting - clearly indicates these are zsh profile configurations
- Avoids potential confusion with other "prof" tools (profilers, etc.)
- Follows Unix convention of descriptive hidden directory names
**Alternatives Considered:**
- `~/.zprof/` - Too abbreviated, not immediately clear what it contains
- `~/.zshprofiles/` - Missing hyphen makes it less readable
**Consequences:**
- All documentation and code must reference `~/.zsh-profiles/`
- Update FR001 to reflect correct directory structure

---

### AD-002: Profile Creation Smart Detection
**Date:** 2025-10-31
**Status:** Decided
**Context:** Need to determine behavior of `zprof create` command - should it always start from scratch, or intelligently detect existing configurations?
**Decision:** Implement smart detection with user choice (Option B)
- When `zprof create <name>` is run, system detects if existing zsh framework configuration exists
- If detected: Prompt user "Import current setup?" or "Start fresh?"
- If not detected or user chooses "Start fresh": Launch TUI wizard from scratch
**Rationale:**
- Provides best UX for first-time users who want to preserve existing setup
- Doesn't force assumptions - user maintains control
- Reduces friction for adoption (users don't have to manually recreate their current config)
- Still allows clean slate experimentation when desired
**Alternatives Considered:**
- Option A: Always launch wizard from scratch - loses user's existing config, high friction
- Option C: Separate commands (create/import-current/clone) - more complex CLI surface, steeper learning curve
**Consequences:**
- FR006 documents detection and prompt behavior
- User journey reflects two-path creation flow
- Implementation requires framework detection logic

---

## Technology Stack

_To be populated as technical decisions are made during PRD/Architecture phases_

---

## Integration Decisions

_To be populated as integration approaches are decided_

---

## Data & Storage Decisions

_To be populated as data storage approaches are decided_

---

## Notes

This document captures technical decisions, preferences, and constraints discussed during planning phases. It serves as input for the Architecture workflow and ensures consistency across implementation.
