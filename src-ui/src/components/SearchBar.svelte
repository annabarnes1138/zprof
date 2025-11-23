<script lang="ts">
  import { Search, X } from 'lucide-svelte';

  export let value: string = '';
  export let placeholder: string = 'Search...';

  function handleClear() {
    value = '';
  }
</script>

<div class="relative flex-1">
  <Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
  <input
    type="text"
    bind:value
    {placeholder}
    class="w-full rounded-md border border-input bg-background pl-10 pr-10 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
  />
  {#if value}
    <button
      on:click={handleClear}
      class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
      aria-label="Clear search"
    >
      <X class="h-4 w-4" />
    </button>
  {/if}
</div>
