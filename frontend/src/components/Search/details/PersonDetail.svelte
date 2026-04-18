<script lang="ts">
  import {i18n} from "$lib/infrastructure/i18n/index.svelte"
  import type {SearchResult} from "$lib/infrastructure/store/SearchStore.svelte"
  import {UserIcon} from "$uikit/icons"

  let {result}: {result: SearchResult} = $props()
</script>

<article class="flex flex-col gap-8">
  <header class="text-center p-8 bg-white/5 rounded-[2rem] border border-white/10 relative overflow-hidden group">
    <div class="absolute inset-x-0 bottom-0 h-24 bg-gradient-to-t from-primary-500/10 to-transparent"></div>

    <div
      class="relative w-28 h-28 rounded-full bg-gradient-to-tr from-primary-500 to-secondary-500 p-1 mx-auto mb-6 shadow-2xl group-hover:scale-105 transition-transform duration-500"
    >
      <div class="w-full h-full rounded-full bg-[#0a0a0a] overflow-hidden flex items-center justify-center">
        {#if result.metadata?.avatar}
          <img
            src={result.metadata.avatar}
            alt={result.title}
            class="w-full h-full object-cover"
          />
        {:else}
          <UserIcon
            size={48}
            className="text-gray-600"
          />
        {/if}
      </div>
    </div>

    <h2 class="text-2xl font-bold text-white mb-1">{result.title}</h2>
    <p class="text-primary-400 font-medium tracking-wide">{result.description}</p>
    <div
      class="mt-4 text-[10px] text-gray-500 uppercase tracking-[0.2em] bg-white/5 px-4 py-1.5 rounded-full inline-block"
    >
      {result.metadata?.department || i18n.t("results.person.unknownDepartment")}
    </div>
  </header>

  <section class="p-6 bg-white/[0.02] rounded-3xl border border-white/5">
    <h3 class="text-[10px] uppercase tracking-[0.3em] text-gray-600 font-black mb-4">
      {i18n.t("results.person.skillsLabel")}
    </h3>
    <div class="flex flex-wrap gap-2">
      {#each result.metadata?.skills || ["System Architecture", "Rust", "Cloud Infrastructure"] as tag}
        <span
          class="px-4 py-2 rounded-xl bg-white/5 border border-white/10 text-[11px] text-gray-400 font-medium hover:text-white hover:border-primary-500/50 transition-colors"
        >
          {tag}
        </span>
      {/each}
    </div>
  </section>
</article>
