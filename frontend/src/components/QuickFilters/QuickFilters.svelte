<script lang="ts">
  import {Button} from "$uikit/core/Button"

  interface Props {
    filters: Array<{name: string; Icon: any}>
  }

  let {filters}: Props = $props()
</script>

<div class="flex flex-wrap justify-center gap-3 animate-in fade-in slide-in-from-bottom-8 duration-1000 delay-200">
  {#each filters as filter}
    <Button
      variant="ghost"
      size="small"
      className="rounded-full border border-white/5 bg-white/5 text-gray-400 hover:text-primary-400 hover:bg-primary-500/10 hover:border-primary-500/20 transition-all duration-300 backdrop-blur-md group py-2"
    >
      <filter.Icon
        size={16}
        className="mr-2 group-hover:scale-110 transition-transform"
      />
      {filter.name}
    </Button>
  {/each}
</div>
