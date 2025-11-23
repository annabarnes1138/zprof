<script lang="ts">
  import { X } from 'lucide-svelte';

  export let open: boolean = false;
  export let title: string;
  export let message: string;
  export let confirmLabel: string = 'Confirm';
  export let cancelLabel: string = 'Cancel';
  export let onConfirm: () => void;
  export let onCancel: () => void;

  function handleBackdropClick() {
    onCancel();
  }

  function handleEscape(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) {
      onCancel();
    }
  }
</script>

<svelte:window on:keydown={handleEscape} />

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
    on:click={handleBackdropClick}
    role="presentation"
  >
    <!-- Dialog -->
    <div
      class="bg-card rounded-lg shadow-lg max-w-md w-full p-6"
      on:click|stopPropagation
      role="dialog"
      aria-modal="true"
      aria-labelledby="dialog-title"
    >
      <!-- Header -->
      <div class="flex items-start justify-between mb-4">
        <h2 id="dialog-title" class="text-xl font-semibold">{title}</h2>
        <button
          on:click={onCancel}
          class="text-muted-foreground hover:text-foreground"
          aria-label="Close"
        >
          <X class="h-5 w-5" />
        </button>
      </div>

      <!-- Body -->
      <p class="text-muted-foreground mb-6">{message}</p>

      <!-- Actions -->
      <div class="flex justify-end gap-3">
        <button
          on:click={onCancel}
          class="rounded px-4 py-2 text-sm font-medium bg-secondary text-secondary-foreground hover:bg-secondary/80 transition-colors"
        >
          {cancelLabel}
        </button>
        <button
          on:click={onConfirm}
          class="rounded px-4 py-2 text-sm font-medium bg-destructive text-destructive-foreground hover:bg-destructive/90 transition-colors"
        >
          {confirmLabel}
        </button>
      </div>
    </div>
  </div>
{/if}
