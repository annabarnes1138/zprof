# zprof v0.2.0 Planning

**Release Target**: TBD
**Status**: Planning in progress

## Overview

Version 0.2.0 focuses on making zprof more accessible while maintaining its power-user capabilities through smart UX and comprehensive installation workflows.

## Key Features

### 1. Smart TUI (Prompt Mode Branching)
Enable users to choose between standalone prompt engines (Starship, Powerlevel10k) and framework-built-in themes with intelligent UX that doesn't overwhelm beginners.

### 2. Quick Setup Presets
Provide curated preset configurations (Minimal, Performance, Fancy, Developer) for one-click profile creation while still offering advanced customization.

### 3. Complete Uninstall System
Allow users to safely try zprof with confidence they can completely remove it and restore either their original configuration or any profile as their root config.

### 4. Nerd Font Auto-Installation
Automatically download and install Nerd Fonts when users select prompts that require them, with clear terminal configuration instructions.

## Epics

Epics will be created in this directory as planning progresses:

- `epic-smart-tui.md` - Prompt mode branching and intelligent selection flows
- `epic-presets.md` - Quick setup preset system
- `epic-uninstall.md` - Complete uninstall with restoration options
- `epic-nerd-fonts.md` - Automatic font installation
- `epic-framework-expansion.md` - Add 2-4 additional frameworks based on demand

## Stories

Story files will be created in `stories/` as epics are finalized.

## Design Decisions

Key architectural and UX decisions will be documented here as they're made.

## Timeline

Planning phase: Current
Implementation: TBD
Release: TBD

## Success Criteria

- [ ] New users can create a working profile in < 2 minutes
- [ ] Users can uninstall zprof cleanly with confidence
- [ ] Prompt selection doesn't confuse beginners
- [ ] Power users still have full control
- [ ] 2+ new frameworks supported based on user demand
