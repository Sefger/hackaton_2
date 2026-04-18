<script lang="ts">
  import type {SearchResult} from "$lib/infrastructure/store/SearchStore.svelte"
  import {cn} from "$lib/utils"
  import {SearchController} from "../controllers/Search.controller.svelte"
  import DetailDrawer from "./DetailDrawer.svelte"
  import DefaultResult from "./results/DefaultResult.svelte"
  import DocumentResult from "./results/DocumentResult.svelte"
  import PersonResult from "./results/PersonResult.svelte"
  import SearchDropdown from "./SearchDropdown.svelte"
  import SearchInput from "./SearchInput.svelte"

  const controller = new SearchController()

  const resultComponents: Record<string, any> = {
    document: DocumentResult,
    person: PersonResult,
    default: DefaultResult,
  }
</script>

{#snippet resultItem(result: SearchResult)}
  {@const Component = resultComponents[result.type] || resultComponents.default}
  <Component
    {result}
    onclick={() => controller.selectItem(result)}
  />
{/snippet}

<div class="relative w-full max-w-2xl group">
  <div
    class="absolute -inset-0.5 bg-gradient-to-r from-primary-500/50 to-secondary-500/50 rounded-[2.5rem] blur-xl opacity-0 group-focus-within:opacity-100 transition-opacity duration-700"
  ></div>

  <section
    class={cn(
      "relative flex flex-col bg-[#0a0a0a]/90 backdrop-blur-2xl border border-white/10 transition-all duration-300",
      controller.hasResults ? "rounded-[2rem]" : "rounded-full"
    )}
  >
    <SearchInput
      isLoading={controller.isLoading}
      query={controller.query}
      onQueryChange={controller.setQuery}
      onKeyDown={controller.handleKeydown}
    />

    {#if controller.hasResults}
      <SearchDropdown
        results={controller.results}
        {resultItem}
      />
    {/if}
  </section>
</div>

<DetailDrawer
  selectedItem={controller.selectedItem}
  onclose={() => controller.selectItem(null)}
/>
