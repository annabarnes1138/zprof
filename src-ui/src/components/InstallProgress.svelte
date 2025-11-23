<script lang="ts">
  import { onMount } from 'svelte';
  import { installPromptEngine } from '$lib/api';

  export let engineName: string;
  export let onComplete: () => void;
  export let onError: (error: string) => void;

  let status: 'installing' | 'success' | 'error' = 'installing';
  let progress = 0;
  let currentStep = 'Downloading...';
  let error = '';

  onMount(async () => {
    try {
      // Simulate progress updates (in real implementation, this would use event streaming)
      const steps = ['Downloading...', 'Installing...', 'Configuring...'];

      // Start progress animation
      const progressInterval = setInterval(() => {
        if (progress < 90) {
          progress += 10;
        }
      }, 300);

      // Start installation
      const installPromise = installPromptEngine(engineName);

      // Update steps
      for (let i = 0; i < steps.length; i++) {
        currentStep = steps[i];
        await new Promise(resolve => setTimeout(resolve, 1000));
      }

      // Wait for installation to complete
      await installPromise;

      clearInterval(progressInterval);
      progress = 100;
      status = 'success';

      setTimeout(onComplete, 1500);
    } catch (e: any) {
      status = 'error';
      error = e.message || 'Installation failed';
      onError(error);
    }
  });
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
  <div class="bg-background rounded-lg shadow-xl p-8 max-w-md w-full">
    {#if status === 'installing'}
      <div class="text-center">
        <div class="spinner h-12 w-12 text-primary mx-auto mb-4"></div>
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
        <svg class="h-12 w-12 text-success mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <h3 class="text-xl font-semibold mb-2">Installation Complete!</h3>
        <p class="text-sm text-muted-foreground">
          {engineName} has been successfully installed and configured.
        </p>
      </div>
    {:else if status === 'error'}
      <div class="text-center">
        <svg class="h-12 w-12 text-destructive mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <h3 class="text-xl font-semibold mb-2">Installation Failed</h3>
        <p class="text-sm text-muted-foreground mb-4">{error}</p>

        <button class="btn btn-primary" on:click={() => window.location.reload()}>
          Retry
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .bg-background {
    background-color: var(--background);
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

  .text-destructive {
    color: var(--destructive);
  }

  .bg-muted {
    background-color: var(--muted);
  }

  .bg-primary {
    background-color: var(--primary);
  }

  .spinner {
    border: 3px solid var(--muted);
    border-top-color: var(--primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .btn {
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-weight: 500;
    transition: all 0.2s;
    cursor: pointer;
  }

  .btn-primary {
    background-color: var(--primary);
    color: var(--primary-foreground);
  }

  .btn-primary:hover {
    opacity: 0.9;
  }
</style>
