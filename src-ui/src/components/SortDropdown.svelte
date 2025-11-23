<script lang="ts">
  import { ArrowUpDown } from 'lucide-svelte';

  export let value: string = 'created-desc';

  const sortOptions = [
    { value: 'name-asc', label: 'Name (A-Z)' },
    { value: 'name-desc', label: 'Name (Z-A)' },
    { value: 'created-desc', label: 'Created (newest first)' },
    { value: 'created-asc', label: 'Created (oldest first)' }
  ];

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    value = target.value;
    // Persist to localStorage
    if (typeof window !== 'undefined') {
      localStorage.setItem('profileSortBy', value);
    }
  }
</script>

<div class="flex items-center gap-2">
  <ArrowUpDown class="h-4 w-4 text-muted-foreground" />
  <select
    {value}
    on:change={handleChange}
    class="rounded-md border border-input bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
  >
    {#each sortOptions as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>
