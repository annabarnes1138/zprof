<script lang="ts">
  import { Settings } from 'lucide-svelte';
  import Button from './ui/button.svelte';
  import { link } from 'svelte-spa-router';
</script>

<header class="h-14 border-b border-border bg-background flex items-center justify-between px-6">
  <h1 class="text-xl font-bold">zprof</h1>

  <div class="flex items-center gap-2">
    <a href="/settings" use:link aria-label="Settings">
      <Button variant="ghost" size="icon">
        <Settings class="h-5 w-5" />
      </Button>
    </a>
  </div>
</header>
