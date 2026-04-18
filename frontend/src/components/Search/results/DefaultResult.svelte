<script lang="ts">
  import type {SearchResult} from "$lib/infrastructure/store/SearchStore.svelte"
  import {SearchIcon} from "$uikit/icons"

  let {result}: {result: SearchResult} = $props()
</script>

<button class="w-full text-left p-4 rounded-2xl hover:bg-white/5 transition-colors group/item flex items-center gap-4">
  <div
    class="w-10 h-10 rounded-xl bg-white/5 flex items-center justify-center text-gray-400 group-hover/item:text-primary-400 transition-colors"
  >
    <SearchIcon size={20} />
  </div>
  <div class="flex-1">
    <h4 class="text-sm font-medium text-gray-200 group-hover/item:text-white">{result.title}</h4>
    <p class="text-xs text-gray-500">{result.description}</p>
  </div>
</button>
