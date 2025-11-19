# History Sharing Fix

**Issue**: History doesn't follow when switching between profiles

**Root Cause**: The shared history directory (`~/.zsh-profiles/shared/.zsh_history`) was only created during `zprof init`, but not when creating profiles via `zprof create` or the wizard. This meant:

1. If users ran `zprof init` first, the shared directory existed and everything worked
2. If users created profiles directly (without `zprof init`), the shared directory never got created
3. Old profiles imported during init might not have `.zshenv` files generated

## Changes Made

### 1. Made `create_shared_history()` Idempotent
**File**: `src/core/filesystem.rs`

```rust
/// Create the shared history file with appropriate permissions
///
/// This function is idempotent - safe to call multiple times.
/// If the history file already exists, it will not be modified.
pub fn create_shared_history() -> Result<PathBuf> {
    let base_dir = get_zprof_dir()?;
    let shared_dir = base_dir.join("shared");
    let history_file = shared_dir.join(".zsh_history");

    // Ensure shared directory exists
    fs::create_dir_all(&shared_dir)
        .with_context(|| format!("Failed to create shared directory at {}", shared_dir.display()))?;

    // Create empty history file if it doesn't exist
    if !history_file.exists() {
        fs::write(&history_file, "")
            .with_context(|| format!("Failed to create history file at {}", history_file.display()))?;

        // Set permissions to 0600 (user read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&history_file, permissions)
                .with_context(|| format!("Failed to set permissions on history file at {}", history_file.display()))?;
        }
    }

    Ok(history_file)
}
```

**Changes**:
- Explicitly creates the `shared` directory with `fs::create_dir_all()`
- Only creates the history file if it doesn't exist (idempotent)
- Safe to call multiple times without overwriting existing history

### 2. Added Shared History Creation to Profile Creation
**File**: `src/cli/create.rs`

Added import:
```rust
use crate::core::filesystem::{copy_dir_recursive, create_shared_history, get_zprof_dir};
```

Added call in `execute()` function:
```rust
// 4. Create profile directory and ensure shared history exists
fs::create_dir_all(&profile_dir).with_context(|| {
    format!(
        "Failed to create profile directory at {}",
        profile_dir.display()
    )
})?;

// Ensure shared history file exists for cross-profile history sharing
create_shared_history()
    .context("Failed to create shared history file")?;
```

### 3. Added Shared History Creation to Profile Switching
**File**: `src/cli/use_cmd.rs`

Added import:
```rust
use crate::core::{config, filesystem, manifest, profile};
```

Added call in `execute()` function:
```rust
// Step 1c: Ensure shared history file exists for cross-profile history sharing
filesystem::create_shared_history()
    .context("Failed to create shared history file")?;
```

## How It Works Now

1. **Profile Creation** (`zprof create <name>`):
   - Creates profile directory
   - **Ensures shared history directory and file exist**
   - Installs framework
   - Generates `.zshenv` with `HISTFILE="$HOME/.zsh-profiles/shared/.zsh_history"`

2. **Profile Switching** (`zprof use <name>`):
   - Validates profile
   - **Ensures shared history directory and file exist**
   - Updates active profile in config
   - Sets ZDOTDIR to point to the profile

3. **Initialization** (`zprof init`):
   - Creates directory structure including `shared/`
   - Creates shared history file
   - (Already worked correctly)

## Testing

Created test script `test_history_sharing.sh` that verifies:
- ✓ Shared history file exists
- ✓ File has correct permissions (0600)
- ✓ All profile `.zshenv` files point to shared history
- ✓ New profiles will create shared history if missing

## Migration for Existing Users

If you have existing profiles created before this fix:

1. **Option 1: Recreate profiles** (recommended for clean state)
   ```bash
   zprof delete <profile-name>
   zprof create <profile-name>
   # Go through wizard to recreate
   ```

2. **Option 2: Manually regenerate shell configs**
   ```bash
   zprof regenerate <profile-name>
   # This will create the .zshenv file with shared history config
   ```

3. **Option 3: Switch profiles to trigger fix**
   ```bash
   zprof use <profile-name>
   # The use command now ensures shared history exists
   ```

## Files Modified

- `src/core/filesystem.rs` - Made `create_shared_history()` idempotent
- `src/cli/create.rs` - Added shared history creation to profile creation
- `src/cli/use_cmd.rs` - Added shared history creation to profile switching
- `test_history_sharing.sh` - Created test script (new file)

## Verification

After this fix:
```bash
# Create a test profile
cargo build
./target/debug/zprof create test-profile-1
# (Go through wizard)

# Create another test profile
./target/debug/zprof create test-profile-2
# (Go through wizard)

# Both profiles will share history
# Commands run in profile-1 will be available in profile-2
# Shared history location: ~/.zsh-profiles/shared/.zsh_history
```

## Benefits

1. **Seamless history sharing**: All profiles share the same command history
2. **No manual setup**: Users don't need to run `zprof init` first
3. **Idempotent**: Safe to call multiple times, won't break existing setups
4. **Automatic migration**: Switching to any profile ensures shared history exists
5. **Maintains user data**: Never overwrites existing history file
