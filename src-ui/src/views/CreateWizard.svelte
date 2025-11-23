<script lang="ts">
  import { wizardState } from '$lib/stores';
  import { onDestroy } from 'svelte';
  import FrameworkStep from './CreateWizard/FrameworkStep.svelte';
  import PromptModeStep from './CreateWizard/PromptModeStep.svelte';
  import EngineSelectStep from './CreateWizard/EngineSelectStep.svelte';
  import ThemeSelectStep from './CreateWizard/ThemeSelectStep.svelte';
  import ReviewStep from './CreateWizard/ReviewStep.svelte';

  // Computed: Which steps to show based on prompt mode
  $: steps = [
    { id: 'framework', title: 'Framework', component: FrameworkStep },
    { id: 'prompt-mode', title: 'Prompt Mode', component: PromptModeStep },
    // Conditional steps based on prompt mode
    ...($wizardState.promptMode === 'engine'
      ? [{ id: 'engine', title: 'Prompt Engine', component: EngineSelectStep }]
      : $wizardState.promptMode === 'theme'
      ? [{ id: 'theme', title: 'Theme', component: ThemeSelectStep }]
      : []),
    { id: 'review', title: 'Review', component: ReviewStep },
  ];

  $: currentStep = steps[$wizardState.step] || steps[0];

  function handleComplete() {
    // Navigate back to profile list
    window.location.hash = '#/';
  }

  // Reset wizard state when leaving
  onDestroy(() => {
    // Don't reset if user completed the wizard
    // They might want to create another profile
  });
</script>

<div class="min-h-screen bg-background">
  <!-- Progress Indicator -->
  <div class="border-b border-border bg-card">
    <div class="max-w-5xl mx-auto px-6 py-4">
      <div class="flex items-center justify-between mb-2">
        <h1 class="text-xl font-semibold">Create Profile</h1>
        <span class="text-sm text-muted-foreground">
          Step {$wizardState.step + 1} of {steps.length}
        </span>
      </div>

      <!-- Step Indicator -->
      <div class="flex items-center gap-2">
        {#each steps as step, index}
          <div class="flex items-center flex-1">
            <div
              class="flex items-center justify-center w-8 h-8 rounded-full {
                index < $wizardState.step
                  ? 'bg-success text-white'
                  : index === $wizardState.step
                  ? 'bg-primary text-white'
                  : 'bg-muted text-muted-foreground'
              }"
            >
              {#if index < $wizardState.step}
                <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
              {:else}
                {index + 1}
              {/if}
            </div>
            <span class="ml-2 text-sm {index === $wizardState.step ? 'font-medium' : 'text-muted-foreground'}">
              {step.title}
            </span>
          </div>

          {#if index < steps.length - 1}
            <div class="flex-shrink-0 w-12 h-0.5 {index < $wizardState.step ? 'bg-success' : 'bg-muted'}"></div>
          {/if}
        {/each}
      </div>
    </div>
  </div>

  <!-- Current Step Content -->
  <div class="py-8">
    {#if currentStep.id === 'framework'}
      <FrameworkStep />
    {:else if currentStep.id === 'prompt-mode'}
      <PromptModeStep />
    {:else if currentStep.id === 'engine'}
      <EngineSelectStep />
    {:else if currentStep.id === 'theme'}
      <ThemeSelectStep />
    {:else if currentStep.id === 'review'}
      <ReviewStep onComplete={handleComplete} />
    {/if}
  </div>
</div>

<style>
  .bg-background {
    background-color: var(--background);
  }

  .bg-card {
    background-color: var(--card);
  }

  .border-b {
    border-bottom-width: 1px;
  }

  .border-border {
    border-color: var(--border);
  }

  .text-muted-foreground {
    color: var(--muted-foreground);
  }

  .bg-primary {
    background-color: var(--primary);
  }

  .bg-success {
    background-color: var(--success);
  }

  .bg-muted {
    background-color: var(--muted);
  }
</style>
