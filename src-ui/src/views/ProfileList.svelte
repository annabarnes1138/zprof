<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus } from 'lucide-svelte';
  import { push } from 'svelte-spa-router';
  import { listProfiles, activateProfile, deleteProfile } from '$lib/api';
  import { toast } from '$lib/stores';
  import type { ProfileInfo } from '$lib/types';

  import ProfileCard from '../components/ProfileCard.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import SearchBar from '../components/SearchBar.svelte';
  import SortDropdown from '../components/SortDropdown.svelte';
  import SkeletonCard from '../components/SkeletonCard.svelte';
  import ErrorState from '../components/ErrorState.svelte';
  import ConfirmDialog from '../components/ConfirmDialog.svelte';

  let profiles: ProfileInfo[] = [];
  let loading = true;
  let error: string | null = null;
  let searchQuery = '';
  let sortBy = '';

  // Action loading states
  let activatingProfile: string | null = null;

  // Confirmation dialog state
  let showDeleteDialog = false;
  let profileToDelete: string | null = null;

  // Initialize sortBy from localStorage
  if (typeof window !== 'undefined') {
    sortBy = localStorage.getItem('profileSortBy') || 'created-desc';
  }

  onMount(async () => {
    await loadProfiles();
  });

  async function loadProfiles() {
    try {
      loading = true;
      error = null;
      profiles = await listProfiles();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load profiles';
      console.error('Failed to load profiles:', e);
    } finally {
      loading = false;
    }
  }

  async function handleActivate(profileName: string) {
    try {
      activatingProfile = profileName;
      await activateProfile(profileName);
      toast.success(`Activated profile '${profileName}'`);
      await loadProfiles(); // Refresh to update active badges
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to activate profile';
      toast.error(message);
    } finally {
      activatingProfile = null;
    }
  }

  function requestDelete(profileName: string) {
    profileToDelete = profileName;
    showDeleteDialog = true;
  }

  async function confirmDelete() {
    if (!profileToDelete) return;

    const name = profileToDelete;
    showDeleteDialog = false;
    profileToDelete = null;

    try {
      await deleteProfile(name);
      toast.success(`Profile '${name}' deleted`);
      await loadProfiles(); // Refresh list
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to delete profile';
      toast.error(message);
    }
  }

  function cancelDelete() {
    showDeleteDialog = false;
    profileToDelete = null;
  }

  function sortComparator(a: ProfileInfo, b: ProfileInfo): number {
    switch (sortBy) {
      case 'name-asc':
        return a.name.localeCompare(b.name);
      case 'name-desc':
        return b.name.localeCompare(a.name);
      case 'created-asc':
        return new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
      case 'created-desc':
      default:
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    }
  }

  $: filteredProfiles = profiles
    .filter(p => p.name.toLowerCase().includes(searchQuery.toLowerCase()))
    .sort(sortComparator);

  function navigateToCreate() {
    push('/create');
  }
</script>

<div class="p-6">
  <div class="mb-6 flex items-center justify-between">
    <h1 class="text-3xl font-bold">Profiles</h1>
  </div>

  {#if loading}
    <div class="mb-6 flex items-center gap-4">
      <div class="flex-1 h-10 bg-muted rounded animate-pulse"></div>
      <div class="w-48 h-10 bg-muted rounded animate-pulse"></div>
    </div>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each [1, 2, 3] as _}
        <SkeletonCard />
      {/each}
    </div>
  {:else if error}
    <ErrorState message={error} onRetry={loadProfiles} />
  {:else}
    <!-- Search and Sort -->
    <div class="mb-6 flex items-center gap-4">
      <SearchBar bind:value={searchQuery} placeholder="Search profiles..." />
      <SortDropdown bind:value={sortBy} />
    </div>

    <!-- Profile Grid or Empty States -->
    {#if filteredProfiles.length === 0}
      {#if searchQuery}
        <EmptyState
          title="No profiles found"
          message="No profiles match your search. Try different keywords or clear the search."
          actionLabel="Clear Search"
          onAction={() => searchQuery = ''}
        />
      {:else}
        <EmptyState
          title="No profiles yet"
          message="Create your first profile to get started managing zsh configurations."
          actionLabel="Create Profile"
          onAction={navigateToCreate}
        />
      {/if}
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each filteredProfiles as profile (profile.name)}
          <ProfileCard
            {profile}
            onActivate={() => handleActivate(profile.name)}
            onDelete={() => requestDelete(profile.name)}
            activating={activatingProfile === profile.name}
          />
        {/each}
      </div>
    {/if}
  {/if}
</div>

<!-- Floating Action Button -->
<button
  class="fixed bottom-6 right-6 h-14 w-14 rounded-full bg-primary text-primary-foreground shadow-lg hover:shadow-xl transition-shadow flex items-center justify-center"
  on:click={navigateToCreate}
  title="Create new profile"
  aria-label="Create new profile"
>
  <Plus class="h-6 w-6" />
</button>

<!-- Delete Confirmation Dialog -->
<ConfirmDialog
  open={showDeleteDialog}
  title="Delete Profile '{profileToDelete}'?"
  message="This profile and its configuration will be permanently deleted. Your backed-up configs will remain safe."
  confirmLabel="Delete Profile"
  cancelLabel="Cancel"
  onConfirm={confirmDelete}
  onCancel={cancelDelete}
/>
