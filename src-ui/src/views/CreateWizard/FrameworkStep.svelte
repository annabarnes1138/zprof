<script lang="ts">
  import { onMount } from 'svelte';
  import { wizardState } from '$lib/stores';
  import { getFrameworks } from '$lib/api';
  import type { FrameworkInfo } from '$lib/types';
  import Button from '../../components/ui/button.svelte';

  let frameworks: FrameworkInfo[] = [];
  let loading = true;
  let selectedFramework = $wizardState.framework;

  onMount(async () => {
    try {
      frameworks = await getFrameworks();
      loading = false;
    } catch (error) {
      console.error('Failed to load frameworks:', error);
      loading = false;
    }
  });

  function selectFramework(frameworkName: string) {
    selectedFramework = frameworkName;
    wizardState.update(s => ({ ...s, framework: frameworkName }));
  }

  function next() {
    if (!selectedFramework) return;
    wizardState.nextStep();
  }
</script>

<div class="p-6 max-w-4xl mx-auto">
  <h2 class="text-2xl font-bold mb-2">Choose Your Framework</h2>
  <p class="text-muted-foreground mb-8">
    Select the zsh framework you want to use
  </p>

  {#if loading}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      {#each [1, 2, 3, 4] as _}
        <div class="card p-6 animate-pulse">
          <div class="h-6 w-32 bg-muted rounded mb-2"></div>
          <div class="h-4 w-full bg-muted rounded"></div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      {#each frameworks as framework (framework.name)}
        <button
          class="card p-6 text-left transition-all hover:shadow-lg {selectedFramework === framework.name ? 'ring-2 ring-primary' : ''}"
          on:click={() => selectFramework(framework.name)}
        >
          <h3 class="text-xl font-semibold mb-2">{framework.name}</h3>
          <p class="text-sm text-muted-foreground mb-3">{framework.description}</p>

          <div class="flex gap-2">
            {#if framework.supports_themes}
              <span class="badge badge-info">Themes</span>
            {/if}
            {#if framework.supports_plugins}
              <span class="badge badge-success">Plugins</span>
            {/if}
          </div>
        </button>
      {/each}
    </div>
  {/if}

  <div class="mt-8 flex justify-end">
    <Button variant="primary" disabled={!selectedFramework} on:click={next}>
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

  .badge {
    display: inline-block;
    padding: 0.125rem 0.5rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 0.25rem;
  }

  .badge-info {
    background: rgba(14, 165, 233, 0.1);
    color: var(--info);
  }

  .badge-success {
    background: rgba(34, 197, 94, 0.1);
    color: var(--success);
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
