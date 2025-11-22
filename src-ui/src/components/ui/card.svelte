<script lang="ts">
  export let padding = true;
</script>

<div
  class="rounded-lg border bg-card text-card-foreground shadow-sm {padding ? 'p-6' : ''}"
  {...$$restProps}
>
  <slot />
</div>
