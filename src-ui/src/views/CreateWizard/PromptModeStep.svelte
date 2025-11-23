<script lang="ts">
  import { wizardState } from '$lib/stores';
  import Button from '../../components/ui/button.svelte';
  import Card from '../../components/ui/card.svelte';

  let selectedMode: 'engine' | 'theme' | null = $wizardState.promptMode;

  function selectMode(mode: 'engine' | 'theme') {
    selectedMode = mode;
    wizardState.update(s => ({ ...s, promptMode: mode }));
  }

  function next() {
    if (!selectedMode) return;
    wizardState.nextStep();
  }

  function back() {
    wizardState.prevStep();
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
        <svg class="h-8 w-8 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
        </svg>
        <div>
          <h3 class="text-xl font-semibold">Standalone Prompt Engine</h3>
          <p class="text-sm text-muted-foreground">Advanced, customizable prompts</p>
        </div>
      </div>

      <ul class="space-y-2 text-sm mb-4">
        <li class="flex items-start gap-2">
          <svg class="h-4 w-4 mt-0.5 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span>Highly customizable (Starship, P10k, etc.)</span>
        </li>
        <li class="flex items-start gap-2">
          <svg class="h-4 w-4 mt-0.5 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span>Cross-shell compatible (some engines)</span>
        </li>
        <li class="flex items-start gap-2">
          <svg class="h-4 w-4 mt-0.5 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span>Asynchronous rendering (faster)</span>
        </li>
        <li class="flex items-start gap-2">
          <svg class="h-4 w-4 mt-0.5 text-warning" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
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
        <svg class="h-8 w-8 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
        </svg>
        <div>
          <h3 class="text-xl font-semibold">Framework Theme</h3>
          <p class="text-sm text-muted-foreground">Simple, built-in themes</p>
        </div>
      </div>

      <ul class="space-y-2 text-sm mb-4">
        <li class="flex items-start gap-2">
          <svg class="h-4 w-4 mt-0.5 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span>Zero installation required</span>
        </li>
        <li class="flex items-start gap-2">
          <svg class="h-4 w-4 mt-0.5 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span>Curated by framework maintainers</span>
        </li>
        <li class="flex items-start gap-2">
          <svg class="h-4 w-4 mt-0.5 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span>Simple, reliable, fast</span>
        </li>
        <li class="flex items-start gap-2">
          <svg class="h-4 w-4 mt-0.5 text-warning" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
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

  <div class="mt-8 flex justify-between">
    <Button variant="secondary" on:click={back}>
      Back
    </Button>
    <Button variant="primary" disabled={!selectedMode} on:click={next}>
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

  .text-warning {
    color: var(--warning);
  }

  .ring-2 {
    box-shadow: 0 0 0 2px var(--primary);
  }

  .ring-primary {
    --tw-ring-color: var(--primary);
  }
</style>
