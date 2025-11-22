<script lang="ts">
  export let orientation: 'horizontal' | 'vertical' = 'horizontal';
</script>

<div
  class="shrink-0 bg-border {orientation === 'horizontal' ? 'h-[1px] w-full' : 'h-full w-[1px]'}"
  role="separator"
  aria-orientation={orientation}
  {...$$restProps}
></div>
