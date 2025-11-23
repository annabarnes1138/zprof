<script lang="ts">
  import { FolderOpen } from 'lucide-svelte';

  export let title: string;
  export let message: string;
  export let actionLabel: string = '';
  export let onAction: (() => void) | null = null;
</script>

<div class="flex flex-col items-center justify-center py-12 px-4">
  <FolderOpen class="h-16 w-16 text-muted-foreground mb-4" />
  <h2 class="text-2xl font-bold mb-2">{title}</h2>
  <p class="text-muted-foreground text-center mb-6 max-w-md">{message}</p>

  {#if actionLabel && onAction}
    <button
      class="rounded bg-primary px-6 py-3 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors"
      on:click={onAction}
    >
      {actionLabel}
    </button>
  {/if}
</div>
