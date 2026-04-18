<script lang="ts">
  import {i18n} from "$lib/infrastructure/i18n/index.svelte"
  import type {SearchResult} from "$lib/infrastructure/store/SearchStore.svelte"
  import {UserIcon} from "$uikit/icons"

  let {result, onclick}: {result: SearchResult; onclick?: () => void} = $props()
</script>

<button
  {onclick}
  class="w-full text-left p-4 rounded-2xl hover:bg-white/5 transition-all duration-300 group/item flex items-center gap-4 border border-transparent hover:border-white/5"
>
  <div
    class="w-11 h-11 rounded-full bg-primary-500/10 overflow-hidden flex items-center justify-center text-primary-400 group-hover/item:shadow-[0_0_15px_rgba(59,130,246,0.3)] transition-all"
  >
    {#if result.metadata?.avatar}
      <img
        src={result.metadata.avatar}
        alt={result.title}
        class="w-full h-full object-cover"
      />
    {:else}
      <UserIcon size={20} />
    {/if}
  </div>

  <div class="flex-1 min-w-0">
    <h4 class="text-sm font-medium text-gray-200 group-hover/item:text-white truncate transition-colors">
      {result.title}
    </h4>
    <p class="text-xs text-gray-500 truncate mt-0.5">
      {result.description}
      {result.metadata?.department ? `• ${result.metadata.department}` : ""}
    </p>
  </div>

  <span
    class="opacity-0 group-hover/item:opacity-100 transition-opacity text-[10px] uppercase font-bold tracking-widest text-primary-500 hidden sm:block"
  >
    {i18n.t("results.person.profileAction")}
  </span>
</button>
