<script lang="ts">
  import { toast, type Toast } from '$lib/stores';
  import { CheckCircle, XCircle, Info, X } from 'lucide-svelte';
  import { fade, fly } from 'svelte/transition';

  let toasts: Toast[] = [];
  toast.subscribe(value => toasts = value);

  function getIcon(type: Toast['type']) {
    switch (type) {
      case 'success': return CheckCircle;
      case 'error': return XCircle;
      case 'info': return Info;
    }
  }

  function getColorClasses(type: Toast['type']) {
    switch (type) {
      case 'success': return 'bg-green-600 text-white';
      case 'error': return 'bg-red-600 text-white';
      case 'info': return 'bg-blue-600 text-white';
    }
  }
</script>

<div class="fixed top-4 right-4 z-50 flex flex-col gap-2 max-w-sm">
  {#each toasts.slice(-3) as t (t.id)}
    <div
      transition:fly={{ x: 300, duration: 200 }}
      class="rounded-lg shadow-lg p-4 flex items-start gap-3 {getColorClasses(t.type)}"
    >
      <svelte:component this={getIcon(t.type)} class="h-5 w-5 flex-shrink-0 mt-0.5" />
      <p class="flex-1 text-sm font-medium">{t.message}</p>
      <button
        on:click={() => toast.remove(t.id)}
        class="flex-shrink-0 hover:opacity-80 transition-opacity"
        aria-label="Dismiss"
      >
        <X class="h-4 w-4" />
      </button>
    </div>
  {/each}
</div>
