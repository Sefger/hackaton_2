<script lang="ts">
  import type {SearchResult} from "$lib/infrastructure/store/SearchStore.svelte"
  import type {Snippet} from "svelte"

  let {
    results,
    resultItem,
  }: {
    results: SearchResult[]
    resultItem: Snippet<[SearchResult]>
  } = $props()
</script>

<div class="px-2 pb-6 animate-in fade-in slide-in-from-top-2 duration-300">
  <div class="h-px bg-white/5 mx-4 mb-4"></div>
  <div class="grid gap-1">
    {#each results as result (result.id)}
      {@render resultItem(result)}
    {/each}
  </div>
</div>
