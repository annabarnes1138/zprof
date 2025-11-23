<script lang="ts">
  import { AlertCircle, RefreshCw } from 'lucide-svelte';

  export let message: string;
  export let onRetry: () => void;
</script>

<div class="flex flex-col items-center justify-center py-12 px-4">
  <AlertCircle class="h-16 w-16 text-destructive mb-4" />
  <h2 class="text-2xl font-bold mb-2">Error Loading Profiles</h2>
  <p class="text-muted-foreground text-center mb-6 max-w-md">{message}</p>

  <button
    class="rounded bg-primary px-6 py-3 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors flex items-center gap-2"
    on:click={onRetry}
  >
    <RefreshCw class="h-4 w-4" />
    Retry
  </button>
</div>
