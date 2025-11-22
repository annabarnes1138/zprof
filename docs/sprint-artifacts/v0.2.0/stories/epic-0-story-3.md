# Story 0.3: Implement IPC Command Layer

**Epic:** Epic 0 - GUI Foundation
**Priority:** P0 (Blocking)
**Status:** ready-for-dev

## Dev Agent Record

**Context Reference:**
- [epic-0-story-3.context.xml](epic-0-story-3.context.xml)

## User Story

**As a** developer
**I want** a robust IPC layer between frontend and backend
**So that** the GUI can interact with zprof's business logic

## Acceptance Criteria

- [ ] Create Tauri command module: `src-tauri/src/commands.rs`
- [ ] Implement core IPC commands:
  - `list_profiles()` → `Vec<ProfileInfo>`
  - `get_profile(name: String)` → `Result<ProfileDetails>`
  - `get_active_profile()` → `Option<String>`
  - `create_profile(config: ProfileConfig)` → `Result<String>`
  - `delete_profile(name: String)` → `Result<()>`
  - `activate_profile(name: String)` → `Result<()>`
  - `get_frameworks()` → `Vec<FrameworkInfo>`
  - `get_plugins(framework: String)` → `Vec<PluginInfo>`
  - `get_themes(framework: String)` → `Vec<ThemeInfo>`
  - `get_prompt_engines()` → `Vec<PromptEngineInfo>`
- [ ] Define shared types in `src-tauri/src/types.rs`:
  - `ProfileInfo` (id, name, framework, active, created_at)
  - `ProfileDetails` (full profile data with plugins, theme, etc.)
  - `ProfileConfig` (creation parameters: framework, prompt_mode, plugins, etc.)
  - `FrameworkInfo` (name, description, supports_themes, supports_plugins)
  - `PluginInfo` (name, description, category, framework)
  - `ThemeInfo` (name, description, framework, preview_url)
  - `PromptEngineInfo` (name, description, nerd_font_required)
- [ ] Reuse existing business logic from `src/`:
  - Import core modules: `use crate::core::{profile, manifest, config};`
  - Import framework modules: `use crate::frameworks;`
  - Convert between GUI types and core types
  - Handle errors and convert to JSON-serializable format
- [ ] Create frontend API client: `src-ui/src/lib/api.ts`
  - Wrapper functions for all IPC commands
  - TypeScript type definitions matching backend
  - Error handling with user-friendly messages
  - Loading state management
- [ ] Add error handling:
  - Return `Result<T, String>` from all commands
  - Log errors to console with context
  - Convert `anyhow::Error` to string messages
  - Include error codes for frontend handling
- [ ] Create error types: `src-tauri/src/error.rs`
  - `IpcError` enum with variants for different error types
  - Implement `From<anyhow::Error>` for `IpcError`
  - Serialize errors to JSON with error codes
- [ ] Register commands in `src-tauri/src/main.rs`:
  - Add all commands to Tauri builder
  - Enable proper error handling
- [ ] Add integration tests:
  - Test each command handler in isolation
  - Mock filesystem operations
  - Verify JSON serialization/deserialization

## Technical Details

### Command Implementation Pattern

```rust
// src-tauri/src/commands.rs

use crate::types::*;
use crate::error::IpcError;

#[tauri::command]
pub fn list_profiles() -> Result<Vec<ProfileInfo>, String> {
    // Call existing business logic
    let profiles = crate::core::profile::list_all()
        .map_err(|e| format!("Failed to list profiles: {}", e))?;

    // Convert to GUI types
    Ok(profiles
        .into_iter()
        .map(|p| ProfileInfo::from(p))
        .collect())
}

#[tauri::command]
pub fn activate_profile(name: String) -> Result<(), String> {
    crate::core::profile::activate(&name)
        .map_err(|e| format!("Failed to activate profile '{}': {}", name, e))
}

#[tauri::command]
pub fn create_profile(config: ProfileConfig) -> Result<String, String> {
    // Validate config
    config.validate()
        .map_err(|e| format!("Invalid profile configuration: {}", e))?;

    // Convert to core types and create
    let manifest = crate::core::manifest::Manifest::from(config);
    let profile_name = crate::core::profile::create(manifest)
        .map_err(|e| format!("Failed to create profile: {}", e))?;

    Ok(profile_name)
}
```

### Type Definitions

```rust
// src-tauri/src/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub name: String,
    pub framework: String,
    pub prompt_mode: String,  // "prompt_engine" | "framework_theme"
    pub active: bool,
    pub created_at: String,
    pub plugin_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub framework: String,
    pub prompt_mode: String,
    pub prompt_engine: Option<String>,
    pub framework_theme: Option<String>,
    pub plugins: Vec<String>,
    pub env_vars: std::collections::HashMap<String, String>,
}

impl ProfileConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validation logic
        Ok(())
    }
}
```

### Frontend API Client

```typescript
// src-ui/src/lib/api.ts

import { invoke } from '@tauri-apps/api/core';

export interface ProfileInfo {
  name: string;
  framework: string;
  prompt_mode: string;
  active: boolean;
  created_at: string;
  plugin_count: number;
}

export interface ProfileConfig {
  name: string;
  framework: string;
  prompt_mode: string;
  prompt_engine?: string;
  framework_theme?: string;
  plugins: string[];
  env_vars: Record<string, string>;
}

export async function listProfiles(): Promise<ProfileInfo[]> {
  try {
    return await invoke<ProfileInfo[]>('list_profiles');
  } catch (error) {
    console.error('Failed to list profiles:', error);
    throw new Error(`Failed to load profiles: ${error}`);
  }
}

export async function getActiveProfile(): Promise<string | null> {
  try {
    return await invoke<string | null>('get_active_profile');
  } catch (error) {
    console.error('Failed to get active profile:', error);
    return null;
  }
}

export async function activateProfile(name: string): Promise<void> {
  try {
    await invoke('activate_profile', { name });
  } catch (error) {
    console.error(`Failed to activate profile '${name}':`, error);
    throw new Error(`Failed to activate profile: ${error}`);
  }
}

export async function createProfile(config: ProfileConfig): Promise<string> {
  try {
    return await invoke<string>('create_profile', { config });
  } catch (error) {
    console.error('Failed to create profile:', error);
    throw new Error(`Failed to create profile: ${error}`);
  }
}

export async function deleteProfile(name: string): Promise<void> {
  try {
    await invoke('delete_profile', { name });
  } catch (error) {
    console.error(`Failed to delete profile '${name}':`, error);
    throw new Error(`Failed to delete profile: ${error}`);
  }
}
```

### Main Application Setup

```rust
// src-tauri/src/main.rs

mod commands;
mod types;
mod error;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::list_profiles,
            commands::get_profile,
            commands::get_active_profile,
            commands::create_profile,
            commands::delete_profile,
            commands::activate_profile,
            commands::get_frameworks,
            commands::get_plugins,
            commands::get_themes,
            commands::get_prompt_engines,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Files Created/Modified

**New Files:**
- `src-tauri/src/commands.rs`
- `src-tauri/src/types.rs`
- `src-tauri/src/error.rs`
- `src-ui/src/lib/api.ts`
- `src-ui/src/lib/types.ts`
- `tests/gui_commands_test.rs`

**Modified Files:**
- `src-tauri/src/main.rs` (register commands)
- `src-tauri/src/lib.rs` (export modules)
- `src-tauri/Cargo.toml` (add dependencies if needed)

## Dependencies

- **Blocks:** Stories 0.1, 0.2 (needs Tauri + UI setup)
- **Requires:** Existing business logic in `src/core/`, `src/frameworks/`

## Testing

**Unit Tests:**

```rust
// tests/gui_commands_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_profiles_empty() {
        // Test with no profiles
        let result = commands::list_profiles();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_activate_nonexistent_profile() {
        let result = commands::activate_profile("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_profile_config_validation() {
        let config = ProfileConfig {
            name: "test".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "prompt_engine".to_string(),
            prompt_engine: None,  // Invalid: missing engine
            framework_theme: None,
            plugins: vec![],
            env_vars: HashMap::new(),
        };

        assert!(config.validate().is_err());
    }
}
```

**Integration Tests:**

```typescript
// Frontend test (manual or E2E)
describe('API Client', () => {
  it('should list profiles', async () => {
    const profiles = await listProfiles();
    expect(Array.isArray(profiles)).toBe(true);
  });

  it('should handle errors gracefully', async () => {
    await expect(activateProfile('nonexistent')).rejects.toThrow();
  });
});
```

**Manual Verification:**
1. Open Tauri DevTools console
2. Call API methods from browser console:
   ```javascript
   window.__TAURI__.invoke('list_profiles').then(console.log);
   ```
3. Verify JSON serialization works correctly
4. Check error messages are user-friendly
5. Verify existing CLI commands still work (no regression)

## Notes

- **Keep it thin**: Commands should delegate to existing business logic
- **Type safety**: Use `serde` for JSON serialization/deserialization
- **Error messages**: Convert technical errors to user-friendly messages
- **No duplication**: GUI reuses 100% of CLI business logic
- **Future-proof**: Design types to be easily extended

## References

- Tauri Commands: https://tauri.app/develop/calling-rust/
- Serde JSON: https://docs.rs/serde_json/
- Architecture Doc: [docs/developer/architecture.md](../../../developer/architecture.md) (Section: GUI/IPC Communication Pattern)
- Epic 0: [docs/planning/v0.2.0/epic-0-gui-foundation.md](../epic-0-gui-foundation.md) (Story 0.3)
