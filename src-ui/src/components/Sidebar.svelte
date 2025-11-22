<script lang="ts">
  import { link, location } from 'svelte-spa-router';
  import { Home, Plus, Settings, Info, ChevronLeft } from 'lucide-svelte';
  import { sidebarCollapsed } from '../lib/stores';
  import Button from './ui/button.svelte';
  import Separator from './ui/separator.svelte';
  import ThemeToggle from './ThemeToggle.svelte';
  import Tooltip from './ui/tooltip.svelte';

  const navItems = [
    { path: '/', icon: Home, label: 'Profiles' },
    { path: '/create', icon: Plus, label: 'Create Profile' },
    { path: '/settings', icon: Settings, label: 'Settings' },
    { path: '/about', icon: Info, label: 'About' },
  ];

  function toggleSidebar() {
    sidebarCollapsed.toggle();
  }

  function isActive(path: string): boolean {
    if (path === '/') {
      return $location === '/' || $location === '/profiles';
    }
    return $location === path;
  }
</script>

<aside
  class="h-screen bg-background border-r border-border flex flex-col transition-all duration-300 {$sidebarCollapsed ? 'w-16' : 'w-60'}"
>
  <!-- Navigation items -->
  <nav class="flex-1 py-4 space-y-1">
    {#each navItems as item}
      <Tooltip text={$sidebarCollapsed ? item.label : ''}>
        <a
          href={item.path}
          use:link
          class="flex items-center gap-3 px-3 mx-2 py-2 rounded-md transition-colors {isActive(item.path)
            ? 'bg-accent text-accent-foreground border-l-4 border-primary'
            : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'}"
          aria-label={item.label}
          aria-current={isActive(item.path) ? 'page' : undefined}
        >
          <svelte:component this={item.icon} class="h-5 w-5 flex-shrink-0" />
          {#if !$sidebarCollapsed}
            <span class="text-sm font-medium">{item.label}</span>
          {/if}
        </a>
      </Tooltip>
    {/each}
  </nav>

  <!-- Bottom section: Theme toggle and collapse button -->
  <div class="p-4 space-y-2">
    <Separator />
    <div class="flex {$sidebarCollapsed ? 'justify-center' : 'justify-between items-center'} pt-2">
      {#if !$sidebarCollapsed}
        <div class="flex items-center gap-2">
          <ThemeToggle />
        </div>
      {:else}
        <div class="flex flex-col gap-2">
          <ThemeToggle />
        </div>
      {/if}

      {#if !$sidebarCollapsed}
        <Tooltip text="Collapse sidebar">
          <Button variant="ghost" size="icon" on:click={toggleSidebar} aria-label="Collapse sidebar">
            <ChevronLeft class="h-5 w-5" />
          </Button>
        </Tooltip>
      {:else}
        <Tooltip text="Expand sidebar">
          <Button variant="ghost" size="icon" on:click={toggleSidebar} aria-label="Expand sidebar">
            <ChevronLeft class="h-5 w-5 transform rotate-180" />
          </Button>
        </Tooltip>
      {/if}
    </div>
  </div>
</aside>
