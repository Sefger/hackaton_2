<script lang="ts">
  import {i18n} from "$lib/infrastructure/i18n/index.svelte"
  import {Input} from "$uikit/core/Input"
  import {SearchIcon} from "$uikit/icons"

  let {
    isLoading,
    query,
    onQueryChange,
    onKeyDown,
  }: {
    isLoading: boolean
    query: string
    onQueryChange: (v: string) => void
    onKeyDown: (e: KeyboardEvent) => void
  } = $props()
</script>

<div class="flex items-center px-6">
  <div class="flex-shrink-0 text-gray-500">
    {#if isLoading}
      <div class="animate-spin w-5 h-5 border-2 border-primary-500 border-t-transparent rounded-full"></div>
    {:else}
      <SearchIcon className="group-focus-within:text-primary-400 transition-colors" />
    {/if}
  </div>

  <Input
    value={query}
    oninput={(e: Event) => onQueryChange((e.currentTarget as HTMLInputElement).value)}
    onkeydown={onKeyDown}
    placeholder={i18n.t("search.placeholder")}
    className="text-lg text-white font-light py-6 px-4 !bg-transparent !border-none !ring-0 !outline-none focus:!outline-none focus:!ring-0 w-full"
  />
</div>
