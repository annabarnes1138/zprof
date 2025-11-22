<script lang="ts">
  import { Moon, Sun } from 'lucide-svelte';
  import { theme } from '../lib/stores';
  import Button from './ui/button.svelte';
  import Tooltip from './ui/tooltip.svelte';

  function toggleTheme() {
    theme.toggle();
  }
</script>

<Tooltip text={$theme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}>
  <Button variant="ghost" size="icon" on:click={toggleTheme} aria-label="Toggle theme">
    {#if $theme === 'light'}
      <Moon class="h-5 w-5" />
    {:else}
      <Sun class="h-5 w-5" />
    {/if}
  </Button>
</Tooltip>
