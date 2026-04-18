<script lang="ts">
  import {i18n} from "$lib/infrastructure/i18n/index.svelte"
  import type {SearchResult} from "$lib/infrastructure/store/SearchStore.svelte"
  import {CloseIcon} from "$uikit/icons"
  import {fade, fly} from "svelte/transition"
  // Детальные фрагменты
  import DocumentDetail from "./details/DocumentDetail.svelte"
  import PersonDetail from "./details/PersonDetail.svelte"

  let {selectedItem, onclose}: {selectedItem: SearchResult | null; onclose: () => void} = $props()

  const detailRegistry: Record<string, any> = {
    document: DocumentDetail,
    person: PersonDetail,
  }
</script>

{#if selectedItem}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm z-[100]"
    onclick={onclose}
    transition:fade={{duration: 250}}
  ></div>

  <aside
    class="fixed right-0 top-0 h-full w-full max-w-md bg-[#0a0a0a]/95 backdrop-blur-xl border-l border-white/10 z-[101] shadow-[0_0_50px_rgba(0,0,0,0.5)] flex flex-col"
    transition:fly={{x: "100%", duration: 400, opacity: 1}}
  >
    <header class="flex items-center justify-between p-6 border-b border-white/5 bg-white/[0.02]">
      <span class="text-[10px] font-bold uppercase tracking-[0.3em] text-gray-500">{i18n.t("search.detailsTitle")}</span
      >
      <button
        onclick={onclose}
        class="w-8 h-8 rounded-full hover:bg-white/10 flex items-center justify-center text-gray-400 hover:text-white transition-all"
      >
        <CloseIcon size={18} />
      </button>
    </header>

    <div class="flex-1 overflow-y-auto p-8 custom-scrollbar">
      {#if detailRegistry[selectedItem.type]}
        {@const DetailComponent = detailRegistry[selectedItem.type]}
        <DetailComponent result={selectedItem} />
      {:else}
        <div class="h-full flex flex-col items-center justify-center text-gray-600 space-y-4">
          <div
            class="w-16 h-16 rounded-full border-2 border-dashed border-gray-800 flex items-center justify-center text-2xl font-light"
          >
            ?
          </div>
          <p class="text-sm">{i18n.t("search.noPreview")}</p>
        </div>
      {/if}
    </div>
  </aside>
{/if}
