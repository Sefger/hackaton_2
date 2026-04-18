import {initStores} from "$lib/contexts/stores/storeContext"
import {i18n} from "$lib/infrastructure/i18n/index.svelte"
import {DocIcon, UserIcon} from "$uikit/icons"

export class PageController {
  private searchStore = initStores().searchStore

  isLoading = $derived(this.searchStore.isLoading)

  quickFilters = $derived([
    {name: i18n.t("filters.documents"), Icon: DocIcon},
    {name: i18n.t("filters.people"), Icon: UserIcon},
  ])

  constructor(initialData?: {query: string; results: any[]}) {
    if (initialData) {
      if (initialData.query) this.searchStore.setQuery(initialData.query)
      if (initialData.results) this.searchStore.setResults(initialData.results)
    }
  }
}
