<script lang="ts">
  import Card from '../components/ui/card.svelte';

  // TODO: Read version from package.json via Tauri command
  const version = '0.2.0';
</script>

<div class="p-6">
  <h1 class="text-3xl font-bold mb-6">About</h1>
  <Card>
    <div class="space-y-4">
      <div>
        <h2 class="text-2xl font-bold mb-2">zprof</h2>
        <p class="text-muted-foreground">Version {version}</p>
      </div>
      <div class="space-y-2">
        <p class="text-sm">A powerful zsh profile manager for developers.</p>
        <p class="text-sm text-muted-foreground">
          Manage multiple zsh configurations with ease - switch between profiles instantly,
          experiment safely, and share your setups.
        </p>
      </div>
      <div class="pt-4 border-t">
        <p class="text-sm text-muted-foreground">
          Built with Tauri, Svelte, and Rust
        </p>
      </div>
    </div>
  </Card>
</div>
