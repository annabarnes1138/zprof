# Story 1.7: Integrate Prompt Mode into Create Workflow (GUI)

**Epic:** Epic 1 - Smart Prompt Selection
**Priority:** P0 (Must Have)
**Status:** ready-for-dev

## Dev Agent Record

**Context Reference:**
- [epic-1-story-7.context.xml](epic-1-story-7.context.xml)

## User Story

**As a** user creating a profile via GUI
**I want** the prompt mode selection flow to be seamless and visual
**So that** profile creation is intuitive and I can preview my choices

## Acceptance Criteria

- [ ] Create Prompt Mode Selection view: `src-ui/src/views/CreateWizard/PromptModeStep.svelte`
  - Visual binary choice between prompt engine and framework theme
  - Clear explanations with icons/graphics
  - Help text explaining the difference
  - "Why choose this?" expandable sections
- [ ] Create Prompt Engine Selection view: `src-ui/src/views/CreateWizard/EngineSelectStep.svelte`
  - Grid of engine cards with:
    - Engine logo/icon
    - Name and tagline
    - Key features (async, cross-shell, etc.)
    - Nerd Font requirement badge
    - Preview screenshot/GIF
  - Filterable by requirements (cross-shell, no Nerd Font, etc.)
  - Search functionality
- [ ] Create Theme Selection view updates: `src-ui/src/views/CreateWizard/ThemeSelectStep.svelte`
  - Show only if user chose "Framework Theme" mode
  - Visual theme previews (screenshots or live preview)
  - Filter by framework
  - Search by theme name
- [ ] Implement wizard flow logic:
  1. Framework selection (existing or new)
  2. **Prompt mode selection** (NEW - engine vs theme)
  3. **Conditional branching:**
     - IF engine mode → Engine selection → Plugin selection
     - IF theme mode → Theme selection → Plugin selection
  4. Configuration review
  5. Profile creation with installation progress
- [ ] Add installation progress UI:
  - Progress modal/overlay during engine installation
  - Step indicators: "Downloading...", "Installing...", "Configuring..."
  - Progress bar (determinate if possible, indeterminate otherwise)
  - Error handling with retry option
  - Success message with "View Profile" action
- [ ] Update configuration review screen:
  - Show selected prompt mode prominently
  - If engine: Show engine name, logo, init command preview
  - If theme: Show theme name, preview screenshot
  - Framework, plugins, env vars (existing)
  - "Edit" buttons to go back to specific steps
- [ ] Add IPC commands for prompt engines:
  - `get_prompt_engines()` → `Vec<PromptEngineInfo>`
  - `install_prompt_engine(engine: String)` → `Result<InstallProgress>`
  - `check_engine_installed(engine: String)` → `bool`
- [ ] Integration tests for full workflow:
  - Test engine mode path end-to-end
  - Test theme mode path (no regression)
  - Test installation error handling
  - Test back navigation through wizard
- [ ] Update user-facing documentation:
  - GUI user guide with screenshots
  - Prompt mode selection guidance
  - Troubleshooting installation issues

## Technical Details

### Wizard State Management

```typescript
// src-ui/src/lib/stores/wizard.ts

import { writable, derived } from 'svelte/store';

export interface WizardState {
  step: number;
  framework: string | null;
  promptMode: 'engine' | 'theme' | null;
  promptEngine: string | null;
  frameworkTheme: string | null;
  plugins: string[];
  envVars: Record<string, string>;
}

export const wizardState = writable<WizardState>({
  step: 0,
  framework: null,
  promptMode: null,
  promptEngine: null,
  frameworkTheme: null,
  plugins: [],
  envVars: {},
});

// Computed: Which steps to show based on mode
export const wizardSteps = derived(wizardState, ($state) => {
  const baseSteps = [
    { id: 'framework', title: 'Framework', component: FrameworkStep },
    { id: 'prompt-mode', title: 'Prompt Mode', component: PromptModeStep },
  ];

  if ($state.promptMode === 'engine') {
    baseSteps.push(
      { id: 'engine', title: 'Prompt Engine', component: EngineSelectStep },
      { id: 'plugins', title: 'Plugins', component: PluginStep },
    );
  } else if ($state.promptMode === 'theme') {
    baseSteps.push(
      { id: 'theme', title: 'Theme', component: ThemeSelectStep },
      { id: 'plugins', title: 'Plugins', component: PluginStep },
    );
  }

  baseSteps.push({ id: 'review', title: 'Review', component: ReviewStep });

  return baseSteps;
});
```

### Prompt Mode Selection Component

```svelte
<!-- src-ui/src/views/CreateWizard/PromptModeStep.svelte -->
<script lang="ts">
  import { wizardState } from '$lib/stores/wizard';
  import { Sparkles, Palette, ChevronRight } from 'lucide-svelte';

  let selectedMode: 'engine' | 'theme' | null = $wizardState.promptMode;

  function selectMode(mode: 'engine' | 'theme') {
    selectedMode = mode;
    wizardState.update(s => ({ ...s, promptMode: mode }));
  }

  function next() {
    if (!selectedMode) return;
    wizardState.update(s => ({ ...s, step: s.step + 1 }));
  }
</script>

<div class="p-6 max-w-4xl mx-auto">
  <h2 class="text-2xl font-bold mb-2">Choose Your Prompt Style</h2>
  <p class="text-muted-foreground mb-8">
    How do you want to configure your shell prompt?
  </p>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Prompt Engine Option -->
    <button
      class="card p-6 text-left transition-all hover:shadow-lg {selectedMode === 'engine' ? 'ring-2 ring-primary' : ''}"
      on:click={() => selectMode('engine')}
    >
      <div class="flex items-start gap-4 mb-4">
        <Sparkles class="h-8 w-8 text-primary" />
        <div>
          <h3 class="text-xl font-semibold">Standalone Prompt Engine</h3>
          <p class="text-sm text-muted-foreground">Advanced, customizable prompts</p>
        </div>
      </div>

      <ul class="space-y-2 text-sm mb-4">
        <li class="flex items-start gap-2">
          <ChevronRight class="h-4 w-4 mt-0.5 text-success" />
          <span>Highly customizable (Starship, P10k, etc.)</span>
        </li>
        <li class="flex items-start gap-2">
          <ChevronRight class="h-4 w-4 mt-0.5 text-success" />
          <span>Cross-shell compatible (some engines)</span>
        </li>
        <li class="flex items-start gap-2">
          <ChevronRight class="h-4 w-4 mt-0.5 text-success" />
          <span>Asynchronous rendering (faster)</span>
        </li>
        <li class="flex items-start gap-2">
          <ChevronRight class="h-4 w-4 mt-0.5 text-warning" />
          <span>Requires separate installation</span>
        </li>
      </ul>

      <details class="text-sm">
        <summary class="cursor-pointer text-primary font-medium">Why choose this?</summary>
        <p class="mt-2 text-muted-foreground">
          Prompt engines like Starship and Powerlevel10k offer advanced features like
          async rendering, extensive customization, and rich visual elements. They replace
          your framework's built-in theme system entirely.
        </p>
      </details>
    </button>

    <!-- Framework Theme Option -->
    <button
      class="card p-6 text-left transition-all hover:shadow-lg {selectedMode === 'theme' ? 'ring-2 ring-primary' : ''}"
      on:click={() => selectMode('theme')}
    >
      <div class="flex items-start gap-4 mb-4">
        <Palette class="h-8 w-8 text-primary" />
        <div>
          <h3 class="text-xl font-semibold">Framework Theme</h3>
          <p class="text-sm text-muted-foreground">Simple, built-in themes</p>
        </div>
      </div>

      <ul class="space-y-2 text-sm mb-4">
        <li class="flex items-start gap-2">
          <ChevronRight class="h-4 w-4 mt-0.5 text-success" />
          <span>Zero installation required</span>
        </li>
        <li class="flex items-start gap-2">
          <ChevronRight class="h-4 w-4 mt-0.5 text-success" />
          <span>Curated by framework maintainers</span>
        </li>
        <li class="flex items-start gap-2">
          <ChevronRight class="h-4 w-4 mt-0.5 text-success" />
          <span>Simple, reliable, fast</span>
        </li>
        <li class="flex items-start gap-2">
          <ChevronRight class="h-4 w-4 mt-0.5 text-warning" />
          <span>Less customizable than engines</span>
        </li>
      </ul>

      <details class="text-sm">
        <summary class="cursor-pointer text-primary font-medium">Why choose this?</summary>
        <p class="mt-2 text-muted-foreground">
          Framework themes like "robbyrussell" or "agnoster" come bundled with your chosen
          framework. They're simple, well-tested, and require no extra setup. Great for
          getting started quickly.
        </p>
      </details>
    </button>
  </div>

  <div class="mt-8 flex justify-end">
    <button
      class="btn btn-primary"
      disabled={!selectedMode}
      on:click={next}
    >
      Continue
      <ChevronRight class="ml-2 h-4 w-4" />
    </button>
  </div>
</div>
```

### Engine Selection Component

```svelte
<!-- src-ui/src/views/CreateWizard/EngineSelectStep.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { wizardState } from '$lib/stores/wizard';
  import { getPromptEngines, checkEngineInstalled } from '$lib/api';
  import { Sparkles, Search, Filter } from 'lucide-svelte';

  let engines = [];
  let loading = true;
  let searchQuery = '';
  let filterNerdFont = false;
  let filterCrossShell = false;
  let selectedEngine = $wizardState.promptEngine;

  onMount(async () => {
    engines = await getPromptEngines();

    // Check which engines are already installed
    for (let engine of engines) {
      engine.installed = await checkEngineInstalled(engine.name);
    }

    loading = false;
  });

  $: filteredEngines = engines
    .filter(e => e.name.toLowerCase().includes(searchQuery.toLowerCase()))
    .filter(e => !filterNerdFont || !e.requires_nerd_font)
    .filter(e => !filterCrossShell || e.cross_shell);

  function selectEngine(engineName: string) {
    selectedEngine = engineName;
    wizardState.update(s => ({ ...s, promptEngine: engineName }));
  }

  function next() {
    if (!selectedEngine) return;
    wizardState.update(s => ({ ...s, step: s.step + 1 }));
  }
</script>

<div class="p-6 max-w-5xl mx-auto">
  <h2 class="text-2xl font-bold mb-2">Select a Prompt Engine</h2>
  <p class="text-muted-foreground mb-6">
    Choose the prompt engine that fits your workflow
  </p>

  <!-- Search & Filters -->
  <div class="flex gap-4 mb-6">
    <div class="flex-1 relative">
      <Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
      <input
        type="text"
        placeholder="Search engines..."
        class="w-full pl-10 pr-4 py-2 border rounded-lg"
        bind:value={searchQuery}
      />
    </div>

    <button
      class="btn btn-secondary"
      class:active={filterNerdFont}
      on:click={() => filterNerdFont = !filterNerdFont}
    >
      <Filter class="h-4 w-4 mr-2" />
      No Nerd Font Required
    </button>

    <button
      class="btn btn-secondary"
      class:active={filterCrossShell}
      on:click={() => filterCrossShell = !filterCrossShell}
    >
      <Filter class="h-4 w-4 mr-2" />
      Cross-Shell
    </button>
  </div>

  <!-- Engine Grid -->
  {#if loading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each [1, 2, 3] as _}
        <div class="card p-4 animate-pulse">
          <div class="h-8 w-8 bg-muted rounded mb-3"></div>
          <div class="h-6 w-32 bg-muted rounded mb-2"></div>
          <div class="h-4 w-full bg-muted rounded mb-1"></div>
          <div class="h-4 w-24 bg-muted rounded"></div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filteredEngines as engine (engine.name)}
        <button
          class="card p-4 text-left transition-all hover:shadow-lg {selectedEngine === engine.name ? 'ring-2 ring-primary' : ''}"
          on:click={() => selectEngine(engine.name)}
        >
          <!-- Engine Icon/Logo -->
          <div class="flex items-start justify-between mb-3">
            <Sparkles class="h-8 w-8 text-primary" />
            {#if engine.installed}
              <span class="text-xs bg-success/10 text-success px-2 py-1 rounded">Installed</span>
            {/if}
          </div>

          <!-- Engine Name & Description -->
          <h3 class="text-lg font-semibold mb-1">{engine.name}</h3>
          <p class="text-sm text-muted-foreground mb-3">{engine.description}</p>

          <!-- Badges -->
          <div class="flex flex-wrap gap-2 mb-3">
            {#if engine.requires_nerd_font}
              <span class="badge badge-warning">Nerd Font Required</span>
            {/if}
            {#if engine.cross_shell}
              <span class="badge badge-info">Cross-Shell</span>
            {/if}
            {#if engine.async_rendering}
              <span class="badge badge-success">Async</span>
            {/if}
          </div>

          <!-- Preview Image -->
          {#if engine.preview_url}
            <img src={engine.preview_url} alt="{engine.name} preview" class="rounded border" />
          {/if}
        </button>
      {/each}
    </div>

    {#if filteredEngines.length === 0}
      <div class="text-center py-12">
        <p class="text-muted-foreground">No engines match your filters</p>
        <button class="btn btn-secondary mt-4" on:click={() => { searchQuery = ''; filterNerdFont = false; filterCrossShell = false; }}>
          Clear Filters
        </button>
      </div>
    {/if}
  {/if}

  <!-- Navigation -->
  <div class="mt-8 flex justify-between">
    <button class="btn btn-secondary" on:click={() => wizardState.update(s => ({ ...s, step: s.step - 1 }))}>
      Back
    </button>
    <button class="btn btn-primary" disabled={!selectedEngine} on:click={next}>
      Continue
    </button>
  </div>
</div>
```

### Installation Progress Component

```svelte
<!-- src-ui/src/components/InstallProgress.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { installPromptEngine } from '$lib/api';
  import { Loader, CheckCircle, XCircle } from 'lucide-svelte';

  export let engineName: string;
  export let onComplete: () => void;
  export let onError: (error: string) => void;

  let status: 'installing' | 'success' | 'error' = 'installing';
  let progress = 0;
  let currentStep = 'Downloading...';
  let error = '';

  onMount(async () => {
    try {
      // Call IPC to install engine (with progress updates)
      const result = await installPromptEngine(engineName);

      // Simulate progress updates (in real implementation, use event streaming)
      const steps = ['Downloading...', 'Installing...', 'Configuring...'];
      for (let i = 0; i < steps.length; i++) {
        currentStep = steps[i];
        progress = ((i + 1) / steps.length) * 100;
        await new Promise(resolve => setTimeout(resolve, 1000));
      }

      status = 'success';
      setTimeout(onComplete, 1500);
    } catch (e) {
      status = 'error';
      error = e.message;
      onError(error);
    }
  });
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
  <div class="bg-background rounded-lg shadow-xl p-8 max-w-md w-full">
    {#if status === 'installing'}
      <div class="text-center">
        <Loader class="h-12 w-12 text-primary mx-auto mb-4 animate-spin" />
        <h3 class="text-xl font-semibold mb-2">Installing {engineName}</h3>
        <p class="text-sm text-muted-foreground mb-4">{currentStep}</p>

        <!-- Progress Bar -->
        <div class="w-full bg-muted rounded-full h-2">
          <div
            class="bg-primary h-2 rounded-full transition-all duration-300"
            style="width: {progress}%"
          ></div>
        </div>
      </div>
    {:else if status === 'success'}
      <div class="text-center">
        <CheckCircle class="h-12 w-12 text-success mx-auto mb-4" />
        <h3 class="text-xl font-semibold mb-2">Installation Complete!</h3>
        <p class="text-sm text-muted-foreground">
          {engineName} has been successfully installed and configured.
        </p>
      </div>
    {:else if status === 'error'}
      <div class="text-center">
        <XCircle class="h-12 w-12 text-destructive mx-auto mb-4" />
        <h3 class="text-xl font-semibold mb-2">Installation Failed</h3>
        <p class="text-sm text-muted-foreground mb-4">{error}</p>

        <button class="btn btn-primary" on:click={() => window.location.reload()}>
          Retry
        </button>
      </div>
    {/if}
  </div>
</div>
```

### IPC Commands (Backend)

```rust
// src-tauri/src/commands.rs (additions)

use crate::prompts::engine::{PromptEngine, PromptEngineInfo};
use crate::prompts::installer::EngineInstaller;

#[tauri::command]
pub fn get_prompt_engines() -> Result<Vec<PromptEngineInfo>, String> {
    let engines = vec![
        PromptEngine::Starship,
        PromptEngine::Powerlevel10k,
        PromptEngine::OhMyPosh,
        PromptEngine::Pure,
        PromptEngine::Spaceship,
    ];

    Ok(engines.iter().map(|e| e.to_info()).collect())
}

#[tauri::command]
pub fn check_engine_installed(engine: String) -> Result<bool, String> {
    let engine = PromptEngine::from_str(&engine)
        .map_err(|e| format!("Unknown engine: {}", e))?;

    let installer = EngineInstaller::new()
        .map_err(|e| format!("Installer error: {}", e))?;

    installer.is_installed(&engine)
        .map_err(|e| format!("Check failed: {}", e))
}

#[tauri::command]
pub async fn install_prompt_engine(engine: String) -> Result<(), String> {
    let engine = PromptEngine::from_str(&engine)
        .map_err(|e| format!("Unknown engine: {}", e))?;

    let installer = EngineInstaller::new()
        .map_err(|e| format!("Installer error: {}", e))?;

    installer.install(&engine)
        .map_err(|e| format!("Installation failed: {}", e))
}
```

### Frontend API (TypeScript)

```typescript
// src-ui/src/lib/api.ts (additions)

export interface PromptEngineInfo {
  name: string;
  description: string;
  requires_nerd_font: boolean;
  cross_shell: boolean;
  async_rendering: boolean;
  preview_url?: string;
  installed: boolean;
}

export async function getPromptEngines(): Promise<PromptEngineInfo[]> {
  try {
    return await invoke<PromptEngineInfo[]>('get_prompt_engines');
  } catch (error) {
    console.error('Failed to get prompt engines:', error);
    throw new Error(`Failed to load prompt engines: ${error}`);
  }
}

export async function checkEngineInstalled(engine: string): Promise<boolean> {
  try {
    return await invoke<boolean>('check_engine_installed', { engine });
  } catch (error) {
    console.error(`Failed to check if ${engine} is installed:`, error);
    return false;
  }
}

export async function installPromptEngine(engine: string): Promise<void> {
  try {
    await invoke('install_prompt_engine', { engine });
  } catch (error) {
    console.error(`Failed to install ${engine}:`, error);
    throw new Error(`Installation failed: ${error}`);
  }
}
```

## Files Created/Modified

**New Files:**
- `src-ui/src/views/CreateWizard/PromptModeStep.svelte`
- `src-ui/src/views/CreateWizard/EngineSelectStep.svelte`
- `src-ui/src/components/InstallProgress.svelte`
- `src-ui/src/lib/stores/wizard.ts`

**Modified Files:**
- `src-ui/src/views/CreateWizard.svelte` (integrate new steps)
- `src-ui/src/views/CreateWizard/ThemeSelectStep.svelte` (conditional display)
- `src-ui/src/views/CreateWizard/ReviewStep.svelte` (show prompt mode)
- `src-ui/src/lib/api.ts` (add engine IPC calls)
- `src-tauri/src/commands.rs` (add engine commands)
- `src-tauri/src/main.rs` (register commands)
- `docs/user-guide/gui-guide.md` (NEW - GUI user documentation)

## Dependencies

- **Blocks:** Epic 0 (GUI foundation), Stories 1.1, 1.3, 1.6
- **External:**
  - lucide-svelte (icons)
  - Tauri IPC layer
  - Engine installation dependencies (git, curl, etc.)

## Testing

**Unit Tests:**

```typescript
// src-ui/tests/wizard-state.test.ts

import { get } from 'svelte/store';
import { wizardState, wizardSteps } from '$lib/stores/wizard';

describe('Wizard State', () => {
  it('should show engine step when prompt_mode is engine', () => {
    wizardState.set({
      ...get(wizardState),
      promptMode: 'engine',
    });

    const steps = get(wizardSteps);
    expect(steps.some(s => s.id === 'engine')).toBe(true);
    expect(steps.some(s => s.id === 'theme')).toBe(false);
  });

  it('should show theme step when prompt_mode is theme', () => {
    wizardState.set({
      ...get(wizardState),
      promptMode: 'theme',
    });

    const steps = get(wizardSteps);
    expect(steps.some(s => s.id === 'theme')).toBe(true);
    expect(steps.some(s => s.id === 'engine')).toBe(false);
  });
});
```

**Integration Tests:**

```rust
// tests/gui_wizard_integration_test.rs

#[test]
fn test_full_engine_workflow() {
    // 1. Select framework
    let framework = "oh-my-zsh";

    // 2. Select prompt mode
    let prompt_mode = "engine";

    // 3. Select engine
    let engine = "starship";

    // 4. Create profile
    let config = ProfileConfig {
        name: "test-wizard".to_string(),
        framework: framework.to_string(),
        prompt_mode: PromptMode::PromptEngine {
            engine: PromptEngine::Starship,
        },
        plugins: vec![],
        env_vars: HashMap::new(),
    };

    let result = create_profile(config);
    assert!(result.is_ok());

    // 5. Verify generated config
    let zshrc = std::fs::read_to_string(
        format!("{}/.zprof/profiles/test-wizard/home/.zshrc", env::var("HOME").unwrap())
    ).unwrap();

    assert!(zshrc.contains("ZSH_THEME=\"\""));
    assert!(zshrc.contains("eval \"$(starship init zsh)\""));
}
```

**E2E Tests (manual or automated with Playwright):**

1. Launch GUI
2. Click "Create Profile"
3. Select "oh-my-zsh" framework
4. Select "Standalone prompt engine"
5. Select "Starship"
6. Skip plugins
7. Confirm creation
8. Verify installation progress shows
9. Verify success message
10. Verify profile appears in list with "Starship" badge

## Success Criteria

- [ ] Wizard flow works for both engine and theme modes
- [ ] Engine installation shows progress and handles errors
- [ ] Theme selection is skipped when engine mode selected
- [ ] Configuration review shows correct prompt mode
- [ ] All IPC commands work correctly
- [ ] No regression: theme mode still works
- [ ] GUI tests passing
- [ ] User documentation updated with screenshots

## Notes

- Consider adding theme/engine preview capabilities in future
- Installation progress could use Tauri events for real-time updates
- Add analytics/telemetry to track which engines are popular
- Future: Allow switching prompt mode after profile creation

## References

- UX Design: [docs/ux-design-specification.md](../../../ux-design-specification.md) (Section 5.1: Create Wizard)
- Epic 1: [docs/planning/v0.2.0/epic-1-smart-tui.md](../epic-1-smart-tui.md) (Story 1.7)
- Story 1.6: [epic-1-story-6.md](epic-1-story-6.md) (Engine installation backend)
- Tauri IPC: https://tauri.app/develop/calling-rust/

## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-7.context.xml](epic-1-story-7.context.xml)
