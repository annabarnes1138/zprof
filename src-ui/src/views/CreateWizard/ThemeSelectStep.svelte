<script lang="ts">
  import { onMount } from 'svelte';
  import { wizardState } from '$lib/stores';
  import { getThemes } from '$lib/api';
  import type { ThemeInfo } from '$lib/types';
  import Button from '../../components/ui/button.svelte';

  let themes: ThemeInfo[] = [];
  let loading = true;
  let searchQuery = '';
  let selectedTheme = $wizardState.frameworkTheme;

  onMount(async () => {
    if (!$wizardState.framework) {
      console.error('No framework selected');
      return;
    }

    try {
      themes = await getThemes($wizardState.framework);
      loading = false;
    } catch (error) {
      console.error('Failed to load themes:', error);
      loading = false;
    }
  });

  $: filteredThemes = themes.filter(t =>
    t.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    t.description.toLowerCase().includes(searchQuery.toLowerCase())
  );

  function selectTheme(themeName: string) {
    selectedTheme = themeName;
    wizardState.update(s => ({ ...s, frameworkTheme: themeName }));
  }

  function next() {
    if (!selectedTheme) return;
    wizardState.nextStep();
  }

  function back() {
    wizardState.prevStep();
  }
</script>

<div class="p-6 max-w-5xl mx-auto">
  <h2 class="text-2xl font-bold mb-2">Select a Theme</h2>
  <p class="text-muted-foreground mb-6">
    Choose a theme for {$wizardState.framework}
  </p>

  <!-- Search -->
  <div class="mb-6">
    <div class="relative">
      <svg class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
      <input
        type="text"
        placeholder="Search themes..."
        class="w-full pl-10 pr-4 py-2 border rounded-lg"
        bind:value={searchQuery}
      />
    </div>
  </div>

  <!-- Theme Grid -->
  {#if loading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each [1, 2, 3, 4, 5, 6] as _}
        <div class="card p-4 animate-pulse">
          <div class="h-24 bg-muted rounded mb-3"></div>
          <div class="h-6 w-32 bg-muted rounded mb-2"></div>
          <div class="h-4 w-full bg-muted rounded"></div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filteredThemes as theme (theme.name)}
        <button
          class="card p-4 text-left transition-all hover:shadow-lg {selectedTheme === theme.name ? 'ring-2 ring-primary' : ''}"
          on:click={() => selectTheme(theme.name)}
        >
          <!-- Theme Preview Placeholder -->
          <div class="preview-box mb-3">
            <div class="preview-content">
              <div class="prompt-line">$ git status</div>
              <div class="prompt-line">On branch main</div>
            </div>
          </div>

          <!-- Theme Name & Description -->
          <h3 class="text-lg font-semibold mb-1">{theme.name}</h3>
          <p class="text-sm text-muted-foreground">{theme.description}</p>
        </button>
      {/each}
    </div>

    {#if filteredThemes.length === 0}
      <div class="text-center py-12">
        <p class="text-muted-foreground">No themes match your search</p>
        <Button variant="secondary" on:click={() => searchQuery = ''} class="mt-4">
          Clear Search
        </Button>
      </div>
    {/if}
  {/if}

  <!-- Navigation -->
  <div class="mt-8 flex justify-between">
    <Button variant="secondary" on:click={back}>
      ← Back
    </Button>
    <Button variant="primary" disabled={!selectedTheme} on:click={next}>
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

  .ring-2 {
    box-shadow: 0 0 0 2px var(--primary);
  }

  .ring-primary {
    --tw-ring-color: var(--primary);
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

  .preview-box {
    height: 6rem;
    background: #1a1b26;
    border-radius: 0.375rem;
    padding: 0.75rem;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.75rem;
    overflow: hidden;
  }

  .preview-content {
    color: #a9b1d6;
  }

  .prompt-line {
    margin-bottom: 0.25rem;
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
