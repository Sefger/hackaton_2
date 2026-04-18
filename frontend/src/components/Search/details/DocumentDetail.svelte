<script lang="ts">
  import {i18n} from "$lib/infrastructure/i18n/index.svelte"
  import type {SearchResult} from "$lib/infrastructure/store/SearchStore.svelte"
  import {DocIcon} from "$uikit/icons"

  let {result}: {result: SearchResult} = $props()
</script>

<article class="flex flex-col gap-8">
  <div class="flex items-start gap-5 p-6 bg-white/5 rounded-3xl border border-white/10 relative overflow-hidden group">
    <div class="absolute inset-0 bg-primary-500/5 opacity-0 group-hover:opacity-100 transition-opacity"></div>
    <div class="relative w-14 h-14 rounded-2xl bg-primary-500/20 flex items-center justify-center text-primary-400">
      <DocIcon size={28} />
    </div>
    <div class="relative">
      <h2 class="text-lg font-semibold text-white leading-tight">{result.title}</h2>
      <p class="text-xs text-gray-500 mt-1 uppercase tracking-wider">
        {i18n.t("results.document.type")} • {result.metadata?.size || i18n.t("results.document.unknownSize")}
      </p>
    </div>
  </div>

  <section class="grid grid-cols-2 gap-3">
    <div class="p-5 rounded-2xl bg-white/[0.03] border border-white/5 flex flex-col gap-1">
      <span class="text-[10px] uppercase font-bold text-gray-600 tracking-widest"
        >{i18n.t("results.document.authorLabel")}</span
      >
      <span class="text-sm text-gray-200 font-medium"
        >{result.metadata?.author || i18n.t("results.document.unknownAuthor")}</span
      >
    </div>
    <div class="p-5 rounded-2xl bg-white/[0.03] border border-white/5 flex flex-col gap-1">
      <span class="text-[10px] uppercase font-bold text-gray-600 tracking-widest"
        >{i18n.t("results.document.modifiedLabel")}</span
      >
      <span class="text-sm text-gray-200 font-medium">{i18n.t("results.document.modifiedRecently")}</span>
    </div>
  </section>

  <section class="p-7 rounded-3xl bg-gradient-to-b from-white/[0.04] to-transparent border border-white/10">
    <h3 class="text-xs uppercase font-extrabold text-primary-500 tracking-widest mb-3">
      {i18n.t("results.document.descriptionLabel")}
    </h3>
    <p class="text-sm text-gray-400 leading-relaxed font-light italic">
      «{result.description}»
    </p>
  </section>

  <button
    class="w-full py-5 bg-primary-500 hover:bg-primary-400 text-white rounded-2xl font-bold tracking-wide transition-all hover:scale-[1.02] active:scale-95 shadow-lg shadow-primary-500/20"
  >
    {i18n.t("results.document.downloadAction")}
  </button>
</article>
