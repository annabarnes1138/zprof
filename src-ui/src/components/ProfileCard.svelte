<script lang="ts">
  import { formatDistanceToNow } from 'date-fns';
  import { Folder, MoreVertical, Loader2 } from 'lucide-svelte';
  import type { ProfileInfo } from '$lib/types';

  export let profile: ProfileInfo;
  export let onActivate: () => void;
  export let onDelete: () => void;
  export let activating: boolean = false;

  // Compute prompt display
  $: promptDisplay = profile.prompt_mode === 'prompt_engine'
    ? profile.prompt_engine || 'Unknown'
    : `${profile.framework_theme || 'default'} theme`;

  // Compute created date
  $: createdAgo = formatDistanceToNow(new Date(profile.created_at), { addSuffix: true });

  let showMenu = false;

  function toggleMenu() {
    showMenu = !showMenu;
  }

  function handleDelete() {
    showMenu = false;
    onDelete();
  }
</script>

<div class="group relative rounded-lg border border-border bg-card p-4 hover:shadow-md transition-shadow">
  <!-- Active Badge -->
  {#if profile.active}
    <div class="absolute top-2 right-2">
      <span class="rounded-full bg-green-600 px-2 py-1 text-xs text-white font-medium">Active</span>
    </div>
  {/if}

  <!-- Profile Icon & Name -->
  <div class="mb-3 flex items-center gap-3">
    <Folder class="h-8 w-8 text-primary" />
    <h3 class="text-xl font-semibold">{profile.name}</h3>
  </div>

  <!-- Metadata -->
  <div class="space-y-1 text-sm text-muted-foreground">
    <div class="flex items-center gap-2">
      <span class="rounded bg-secondary px-2 py-0.5 text-xs font-medium">{profile.framework}</span>
      <span>{promptDisplay}</span>
    </div>
    <div>{profile.plugin_count} plugin{profile.plugin_count !== 1 ? 's' : ''}</div>
    <div>Created {createdAgo}</div>
  </div>

  <!-- Actions -->
  <div class="mt-4 flex items-center gap-2">
    {#if !profile.active}
      <button
        class="flex-1 rounded bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
        on:click={onActivate}
        disabled={activating}
      >
        {#if activating}
          <Loader2 class="h-4 w-4 animate-spin" />
          Activating...
        {:else}
          Activate
        {/if}
      </button>
    {/if}

    <!-- Dropdown Menu -->
    <div class="relative">
      <button
        class="rounded p-2 hover:bg-accent transition-colors"
        on:click={toggleMenu}
        aria-label="Profile actions"
      >
        <MoreVertical class="h-4 w-4" />
      </button>

      {#if showMenu}
        <div class="absolute right-0 mt-1 w-48 rounded-md border border-border bg-popover shadow-lg z-10">
          <div class="py-1">
            <button
              class="w-full px-4 py-2 text-left text-sm hover:bg-accent disabled:opacity-50 disabled:cursor-not-allowed"
              disabled
              title="Coming soon"
            >
              Edit
            </button>
            <button
              class="w-full px-4 py-2 text-left text-sm hover:bg-accent disabled:opacity-50 disabled:cursor-not-allowed"
              disabled
              title="Coming soon"
            >
              Duplicate
            </button>
            <button
              class="w-full px-4 py-2 text-left text-sm text-destructive hover:bg-accent disabled:opacity-50 disabled:cursor-not-allowed"
              disabled={profile.active}
              title={profile.active ? "Deactivate this profile first" : "Delete profile"}
              on:click={handleDelete}
            >
              Delete
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<!-- Click outside to close menu -->
{#if showMenu}
  <button
    class="fixed inset-0 z-0"
    on:click={() => showMenu = false}
    aria-label="Close menu"
  ></button>
{/if}
