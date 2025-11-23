<script lang="ts">
  import { onMount } from 'svelte';
  import { wizardState } from '$lib/stores';
  import { getPromptEngines, checkEngineInstalled } from '$lib/api';
  import type { PromptEngineInfo } from '$lib/types';
  import Button from '../../components/ui/button.svelte';

  let engines: PromptEngineInfo[] = [];
  let loading = true;
  let searchQuery = '';
  let filterNerdFont = false;
  let filterCrossShell = false;
  let selectedEngine = $wizardState.promptEngine;

  onMount(async () => {
    try {
      const fetchedEngines = await getPromptEngines();

      // Check which engines are already installed and create new array
      engines = await Promise.all(
        fetchedEngines.map(async (engine) => ({
          ...engine,
          installed: await checkEngineInstalled(engine.name),
        }))
      );

      loading = false;
    } catch (error) {
      console.error('Failed to load engines:', error);
      loading = false;
    }
  });

  $: filteredEngines = engines
    .filter(e => e.name.toLowerCase().includes(searchQuery.toLowerCase()))
    .filter(e => !filterNerdFont || !e.nerd_font_required)
    .filter(e => !filterCrossShell || e.cross_shell);

  function selectEngine(engineName: string) {
    selectedEngine = engineName;
    wizardState.update(s => ({ ...s, promptEngine: engineName }));
  }

  function next() {
    if (!selectedEngine) return;
    wizardState.nextStep();
  }

  function back() {
    wizardState.prevStep();
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
      <svg class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
      <input
        type="text"
        placeholder="Search engines..."
        class="w-full pl-10 pr-4 py-2 border rounded-lg"
        bind:value={searchQuery}
      />
    </div>

    <Button
      variant={filterNerdFont ? 'primary' : 'secondary'}
      on:click={() => filterNerdFont = !filterNerdFont}
    >
      <svg class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
      </svg>
      No Nerd Font Required
    </Button>

    <Button
      variant={filterCrossShell ? 'primary' : 'secondary'}
      on:click={() => filterCrossShell = !filterCrossShell}
    >
      <svg class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
      </svg>
      Cross-Shell
    </Button>
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
            <svg class="h-8 w-8 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
            </svg>
            {#if engine.installed}
              <span class="text-xs bg-success/10 text-success px-2 py-1 rounded">Installed</span>
            {/if}
          </div>

          <!-- Engine Name & Description -->
          <h3 class="text-lg font-semibold mb-1">{engine.name}</h3>
          <p class="text-sm text-muted-foreground mb-3">{engine.description}</p>

          <!-- Badges -->
          <div class="flex flex-wrap gap-2 mb-3">
            {#if engine.nerd_font_required}
              <span class="badge badge-warning">Nerd Font Required</span>
            {/if}
            {#if engine.cross_shell}
              <span class="badge badge-info">Cross-Shell</span>
            {/if}
            {#if engine.async_rendering}
              <span class="badge badge-success">Async</span>
            {/if}
          </div>
        </button>
      {/each}
    </div>

    {#if filteredEngines.length === 0}
      <div class="text-center py-12">
        <p class="text-muted-foreground">No engines match your filters</p>
        <Button variant="secondary" on:click={() => { searchQuery = ''; filterNerdFont = false; filterCrossShell = false; }} class="mt-4">
          Clear Filters
        </Button>
      </div>
    {/if}
  {/if}

  <!-- Navigation -->
  <div class="mt-8 flex justify-between">
    <Button variant="secondary" on:click={back}>
      ← Back
    </Button>
    <Button variant="primary" disabled={!selectedEngine} on:click={next}>
      Continue →
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

  .text-primary {
    color: var(--primary);
  }

  .text-success {
    color: var(--success);
  }

  .ring-2 {
    box-shadow: 0 0 0 2px var(--primary);
  }

  .ring-primary {
    --tw-ring-color: var(--primary);
  }

  .badge {
    display: inline-block;
    padding: 0.125rem 0.5rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 0.25rem;
  }

  .badge-warning {
    background: rgba(245, 158, 11, 0.1);
    color: var(--warning);
  }

  .badge-info {
    background: rgba(14, 165, 233, 0.1);
    color: var(--info);
  }

  .badge-success {
    background: rgba(34, 197, 94, 0.1);
    color: var(--success);
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

  .animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .bg-muted {
    background-color: var(--muted);
  }
</style>
