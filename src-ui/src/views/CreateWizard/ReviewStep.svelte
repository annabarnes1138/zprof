<script lang="ts">
  import { wizardState, toast } from '$lib/stores';
  import { createProfile } from '$lib/api';
  import type { ProfileConfig } from '$lib/types';
  import Button from '../../components/ui/button.svelte';
  import InstallProgress from '../../components/InstallProgress.svelte';

  export let onComplete: () => void;

  let profileName = '';
  let creating = false;
  let installing = false;
  let installEngine = '';

  function back() {
    wizardState.prevStep();
  }

  async function create() {
    if (!profileName.trim()) {
      toast.error('Please enter a profile name');
      return;
    }

    creating = true;

    try {
      // Check if we need to install an engine first
      if ($wizardState.promptMode === 'engine' && $wizardState.promptEngine) {
        installing = true;
        installEngine = $wizardState.promptEngine;
        // Installation handled by InstallProgress component
        return;
      }

      await createProfileDirectly();
    } catch (error: any) {
      console.error('Failed to create profile:', error);
      toast.error(error.message || 'Failed to create profile');
      creating = false;
    }
  }

  async function createProfileDirectly() {
    try {
      const config: ProfileConfig = {
        name: profileName,
        framework: $wizardState.framework || '',
        prompt_mode: $wizardState.promptMode === 'engine' ? 'prompt_engine' : 'framework_theme',
        prompt_engine: $wizardState.promptMode === 'engine' ? $wizardState.promptEngine || undefined : undefined,
        framework_theme: $wizardState.promptMode === 'theme' ? $wizardState.frameworkTheme || undefined : undefined,
        plugins: $wizardState.plugins,
        env_vars: $wizardState.envVars,
      };

      await createProfile(config);
      toast.success(`Profile '${profileName}' created successfully!`);
      wizardState.reset();
      creating = false;
      onComplete();
    } catch (error: any) {
      console.error('Failed to create profile:', error);
      toast.error(error.message || 'Failed to create profile');
      creating = false;
    }
  }

  function handleInstallComplete() {
    installing = false;
    // Continue with profile creation
    createProfileDirectly();
  }

  function handleInstallError(error: string) {
    installing = false;
    creating = false;
    toast.error(`Engine installation failed: ${error}`);
  }
</script>

{#if installing}
  <InstallProgress
    engineName={installEngine}
    onComplete={handleInstallComplete}
    onError={handleInstallError}
  />
{/if}

<div class="p-6 max-w-3xl mx-auto">
  <h2 class="text-2xl font-bold mb-2">Review & Create</h2>
  <p class="text-muted-foreground mb-8">
    Review your configuration and create the profile
  </p>

  <!-- Profile Name -->
  <div class="mb-6">
    <label class="block text-sm font-medium mb-2">Profile Name</label>
    <input
      type="text"
      placeholder="e.g., work, personal, dev"
      class="w-full px-4 py-2 border rounded-lg"
      bind:value={profileName}
    />
    <p class="text-xs text-muted-foreground mt-1">
      Use lowercase letters, numbers, and hyphens
    </p>
  </div>

  <!-- Configuration Summary -->
  <div class="card p-6 mb-6">
    <h3 class="text-lg font-semibold mb-4">Configuration Summary</h3>

    <div class="space-y-3">
      <!-- Framework -->
      <div class="flex justify-between">
        <span class="text-muted-foreground">Framework:</span>
        <span class="font-medium">{$wizardState.framework}</span>
      </div>

      <!-- Prompt Mode -->
      <div class="flex justify-between">
        <span class="text-muted-foreground">Prompt Style:</span>
        <span class="font-medium">
          {$wizardState.promptMode === 'engine' ? 'Standalone Engine' : 'Framework Theme'}
        </span>
      </div>

      <!-- Prompt Engine or Theme -->
      {#if $wizardState.promptMode === 'engine'}
        <div class="flex justify-between">
          <span class="text-muted-foreground">Engine:</span>
          <span class="font-medium">{$wizardState.promptEngine}</span>
        </div>
      {:else}
        <div class="flex justify-between">
          <span class="text-muted-foreground">Theme:</span>
          <span class="font-medium">{$wizardState.frameworkTheme || 'Default'}</span>
        </div>
      {/if}

      <!-- Plugins -->
      <div class="flex justify-between">
        <span class="text-muted-foreground">Plugins:</span>
        <span class="font-medium">{$wizardState.plugins.length} selected</span>
      </div>
    </div>
  </div>

  <!-- Navigation -->
  <div class="flex justify-between">
    <Button variant="secondary" on:click={back} disabled={creating}>
      ← Back
    </Button>
    <Button variant="primary" on:click={create} disabled={creating || !profileName.trim()}>
      {creating ? 'Creating...' : 'Create Profile'}
    </Button>
  </div>
</div>

<style>
  .card {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
  }

  .text-muted-foreground {
    color: var(--muted-foreground);
  }

  input {
    background: var(--background);
    color: var(--foreground);
    border-color: var(--border);
  }

  input:focus {
    outline: none;
    border-color: var(--primary);
  }

  label {
    color: var(--foreground);
  }

  .space-y-3 > * + * {
    margin-top: 0.75rem;
  }
</style>
