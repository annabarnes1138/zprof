# Story 0.4: Create Profile List View (First Real Screen)

**Epic:** Epic 0 - GUI Foundation
**Priority:** P0 (Blocking)
**Status:** ready-for-dev

## Dev Agent Record

**Context Reference:**
- [epic-0-story-4.context.xml](epic-0-story-4.context.xml)

## User Story

**As a** user
**I want** to see all my profiles in a clean list view
**So that** I can understand what profiles exist and which is active

## Acceptance Criteria

- [ ] Create `ProfileList.svelte` view component (replace placeholder from 0.2)
- [ ] Display profiles as cards in a responsive grid layout:
  - Grid: 3 columns on large screens (>1200px), 2 columns on medium (960-1200px)
  - Each card shows:
    - Profile name (large, bold, 20px)
    - Framework name with icon badge (oh-my-zsh, zimfw, etc.)
    - Prompt mode indicator: "Starship" or "robbyrussell theme"
    - Plugin count: "12 plugins"
    - Created date (relative: "2 days ago" using date-fns)
    - Active indicator: Green "Active" badge (if active)
- [ ] Add profile actions (visible on hover or always on mobile):
  - **"Activate" button** (primary blue) - if not active
  - **"..." menu button** (ghost) - opens dropdown with:
    - Edit (disabled with tooltip: "Coming soon")
    - Duplicate (disabled with tooltip: "Coming soon")
    - Delete (red, destructive)
- [ ] Handle empty state (no profiles):
  - Show large folder icon (from Lucide)
  - Headline: "No profiles yet"
  - Body: "Create your first profile to get started managing zsh configurations."
  - Large "Create Profile" button (navigates to `/create`)
- [ ] Add "Create New Profile" floating action button:
  - Fixed position: bottom-right corner
  - Primary blue, circular, "+" icon
  - Tooltip: "Create new profile"
  - Navigates to `/create` on click
- [ ] Implement search/filter functionality:
  - Search bar at top: "Search profiles..."
  - Filters by profile name (case-insensitive)
  - Shows "No profiles match your search" if no results
  - "Clear" button to reset search
- [ ] Add sorting options:
  - Dropdown: "Sort by..." with options:
    - Name (A-Z)
    - Name (Z-A)
    - Created (newest first) - default
    - Created (oldest first)
  - Persists sort preference to localStorage
- [ ] Integrate with IPC commands from Story 0.3:
  - Call `listProfiles()` on component mount
  - Call `activateProfile(name)` on "Activate" button click
  - Call `deleteProfile(name)` on delete confirmation
  - Refresh list after mutations (activate, delete)
- [ ] Add loading states:
  - Skeleton cards during initial load (shimmer effect)
  - Button spinners during actions (activate, delete)
  - Disable buttons during operations
- [ ] Add error state if profile loading fails:
  - Show error icon + message
  - "Retry" button to reload
  - Log error to console
- [ ] Implement delete confirmation dialog:
  - Modal dialog with:
    - Title: "Delete Profile '{name}'?"
    - Body: "This profile and its configuration will be permanently deleted. Your backed-up configs will remain safe."
    - Buttons: "Cancel" (secondary) | "Delete Profile" (destructive red)
  - Show after clicking delete in menu
  - Only proceed if user confirms
- [ ] Handle delete edge cases:
  - Can't delete active profile (button disabled with tooltip: "Deactivate this profile first")
  - Show error toast if delete fails
  - Show success toast after delete: "Profile '{name}' deleted"
- [ ] Implement toast notifications:
  - Success: Green background, checkmark icon, 3s auto-dismiss
  - Error: Red background, X icon, 5s auto-dismiss + close button
  - Position: top-right corner
  - Max 3 toasts stacked vertically

## Technical Details

### ProfileList Component

```svelte
<!-- src-ui/src/views/ProfileList.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { listProfiles, activateProfile, deleteProfile } from '$lib/api';
  import ProfileCard from '$lib/components/ProfileCard.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import { toast } from '$lib/stores';

  let profiles = [];
  let loading = true;
  let error = null;
  let searchQuery = '';
  let sortBy = localStorage.getItem('profileSortBy') || 'created-desc';

  onMount(async () => {
    await loadProfiles();
  });

  async function loadProfiles() {
    try {
      loading = true;
      error = null;
      profiles = await listProfiles();
    } catch (e) {
      error = e.message;
      console.error('Failed to load profiles:', e);
    } finally {
      loading = false;
    }
  }

  async function handleActivate(profileName: string) {
    try {
      await activateProfile(profileName);
      toast.success(`Activated profile '${profileName}'`);
      await loadProfiles(); // Refresh to update active badges
    } catch (e) {
      toast.error(`Failed to activate profile: ${e.message}`);
    }
  }

  async function handleDelete(profileName: string) {
    const confirmed = await showDeleteConfirmation(profileName);
    if (!confirmed) return;

    try {
      await deleteProfile(profileName);
      toast.success(`Profile '${profileName}' deleted`);
      await loadProfiles(); // Refresh list
    } catch (e) {
      toast.error(`Failed to delete profile: ${e.message}`);
    }
  }

  $: filteredProfiles = profiles
    .filter(p => p.name.toLowerCase().includes(searchQuery.toLowerCase()))
    .sort(sortComparator(sortBy));
</script>

<div class="p-6">
  <div class="mb-6 flex items-center gap-4">
    <SearchBar bind:value={searchQuery} placeholder="Search profiles..." />
    <SortDropdown bind:value={sortBy} />
  </div>

  {#if loading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each [1, 2, 3] as _}
        <SkeletonCard />
      {/each}
    </div>
  {:else if error}
    <ErrorState message={error} onRetry={loadProfiles} />
  {:else if filteredProfiles.length === 0}
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
        onAction={() => navigate('/create')}
      />
    {/if}
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each filteredProfiles as profile (profile.name)}
        <ProfileCard
          {profile}
          onActivate={() => handleActivate(profile.name)}
          onDelete={() => handleDelete(profile.name)}
        />
      {/each}
    </div>
  {/if}
</div>

<!-- Floating Action Button -->
<button
  class="fixed bottom-6 right-6 h-14 w-14 rounded-full bg-primary text-primary-foreground shadow-lg hover:shadow-xl"
  on:click={() => navigate('/create')}
  title="Create new profile"
>
  <Plus class="h-6 w-6" />
</button>
```

### ProfileCard Component

```svelte
<!-- src-ui/src/components/ProfileCard.svelte -->
<script lang="ts">
  import { formatDistanceToNow } from 'date-fns';
  import { MoreVertical, Folder } from 'lucide-svelte';
  import type { ProfileInfo } from '$lib/types';

  export let profile: ProfileInfo;
  export let onActivate: () => void;
  export let onDelete: () => void;

  $: promptDisplay = profile.prompt_mode === 'prompt_engine'
    ? profile.prompt_engine
    : `${profile.framework_theme} theme`;
  $: createdAgo = formatDistanceToNow(new Date(profile.created_at), { addSuffix: true });
</script>

<div class="group relative rounded-lg border bg-card p-4 hover:shadow-md transition-shadow">
  <!-- Active Badge -->
  {#if profile.active}
    <div class="absolute top-2 right-2">
      <span class="rounded-full bg-success px-2 py-1 text-xs text-white">Active</span>
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
      <span class="rounded bg-secondary px-2 py-0.5 text-xs">{profile.framework}</span>
      <span>{promptDisplay}</span>
    </div>
    <div>{profile.plugin_count} plugins</div>
    <div>Created {createdAgo}</div>
  </div>

  <!-- Actions -->
  <div class="mt-4 flex items-center gap-2">
    {#if !profile.active}
      <button
        class="flex-1 rounded bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
        on:click={onActivate}
      >
        Activate
      </button>
    {/if}

    <DropdownMenu>
      <button slot="trigger" class="rounded p-2 hover:bg-accent">
        <MoreVertical class="h-4 w-4" />
      </button>
      <div slot="content">
        <button disabled title="Coming soon">Edit</button>
        <button disabled title="Coming soon">Duplicate</button>
        <button
          class="text-destructive"
          disabled={profile.active}
          title={profile.active ? "Deactivate this profile first" : "Delete profile"}
          on:click={onDelete}
        >
          Delete
        </button>
      </div>
    </DropdownMenu>
  </div>
</div>
```

## Files Created/Modified

**New Files:**
- `src-ui/src/components/ProfileCard.svelte`
- `src-ui/src/components/EmptyState.svelte`
- `src-ui/src/components/SearchBar.svelte`
- `src-ui/src/components/SortDropdown.svelte`
- `src-ui/src/components/SkeletonCard.svelte`
- `src-ui/src/components/ErrorState.svelte`
- `src-ui/src/components/ConfirmDialog.svelte`
- `src-ui/src/components/ui/dropdown-menu.svelte`
- `src-ui/src/lib/stores/toast.ts`

**Modified Files:**
- `src-ui/src/views/ProfileList.svelte` (replace placeholder)
- `src-ui/package.json` (add date-fns dependency)

## Dependencies

- **Blocks:** Stories 0.2 (UI foundation), 0.3 (IPC layer)
- **npm packages:**
  - `date-fns` - Relative date formatting

## Testing

**Manual Verification:**

1. **Empty State:**
   - Delete all profiles (or use fresh installation)
   - Verify empty state shows with CTA button
   - Click "Create Profile" → navigates to `/create`

2. **Profile List:**
   - Create 3-5 test profiles
   - Verify all cards display correctly
   - Verify grid layout (3 cols on large, 2 on medium)

3. **Active Profile:**
   - Activate a profile
   - Verify green "Active" badge appears
   - Verify "Activate" button hidden on active profile

4. **Activate Action:**
   - Click "Activate" on different profile
   - Verify button shows spinner
   - Verify success toast appears
   - Verify badges update (old active → inactive, new active)

5. **Delete Action:**
   - Click "..." menu → "Delete"
   - Verify confirmation dialog appears
   - Click "Cancel" → dialog closes, profile remains
   - Click "Delete" → profile removed, success toast

6. **Delete Active Profile:**
   - Try deleting active profile
   - Verify "Delete" button is disabled
   - Verify tooltip: "Deactivate this profile first"

7. **Search:**
   - Type in search bar
   - Verify list filters in real-time
   - Search for non-existent → "No profiles found" message
   - Clear search → all profiles return

8. **Sorting:**
   - Test all sort options (Name A-Z, Z-A, Created newest/oldest)
   - Verify order changes
   - Reload page → verify sort preference persists

9. **Loading States:**
   - Throttle network to see skeleton cards
   - Verify smooth transition to real cards

10. **Error States:**
    - Simulate API error (disconnect backend)
    - Verify error state with "Retry" button
    - Click "Retry" → attempts reload

**Visual Testing:**
- Card spacing: 16px gap (Tailwind `gap-4`)
- Card padding: 16px (Tailwind `p-4`)
- Active badge: green background, white text
- Hover shadow: subtle elevation increase
- FAB: 56x56px, bottom-right corner, 24px margin

## Notes

- Use relative dates: "2 days ago" vs "2023-11-21"
- Disable "Edit" and "Duplicate" buttons (Coming in Epic 1 or later)
- Keep loading states subtle (skeleton cards better than spinners)
- Toast auto-dismiss: Success 3s, Error 5s
- Profile list should poll for changes (future: use Tauri events)

## References

- UX Design Spec: [docs/ux-design-specification.md](../../../ux-design-specification.md) (Section 5.1: User Journeys - Profile List)
- Architecture Doc: [docs/developer/architecture.md](../../../developer/architecture.md) (Section: Data Flow Patterns)
- Epic 0: [docs/planning/v0.2.0/epic-0-gui-foundation.md](../epic-0-gui-foundation.md) (Story 0.4)
- date-fns: https://date-fns.org/docs/formatDistanceToNow
