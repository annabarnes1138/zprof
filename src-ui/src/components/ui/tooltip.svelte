<script lang="ts">
  export let text: string;
  let showTooltip = false;
</script>

<div class="relative inline-block">
  <div
    role="button"
    tabindex="0"
    on:mouseenter={() => showTooltip = true}
    on:mouseleave={() => showTooltip = false}
    on:focus={() => showTooltip = true}
    on:blur={() => showTooltip = false}
  >
    <slot />
  </div>
  {#if showTooltip}
    <div
      class="absolute z-50 px-3 py-1.5 text-sm text-primary-foreground bg-primary rounded-md shadow-md whitespace-nowrap -top-10 left-1/2 transform -translate-x-1/2"
      role="tooltip"
    >
      {text}
      <div class="absolute w-2 h-2 bg-primary transform rotate-45 left-1/2 -translate-x-1/2 -bottom-1"></div>
    </div>
  {/if}
</div>
