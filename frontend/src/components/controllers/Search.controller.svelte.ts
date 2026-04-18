import {initStores} from "$lib/contexts/stores/storeContext"
import type {SearchResult, SearchStore} from "$lib/infrastructure/store/SearchStore.svelte"

export class SearchController {
  private searchStore: SearchStore = initStores().searchStore

  // Реактивные производные свойства (Runes)
  query = $derived(this.searchStore.query)
  results = $derived(this.searchStore.results)
  isLoading = $derived(this.searchStore.isLoading)
  selectedItem = $derived(this.searchStore.selectedItem)
  hasResults = $derived(this.searchStore.results.length > 0)

  constructor() {}

  // Метод для обновления запроса (Action) - только обновляет строку, не триггерит поиск!
  setQuery = (value: string) => {
    this.searchStore.setQuery(value)
  }

  // Выбор элемента (Action)
  selectItem = (item: SearchResult | null) => {
    this.searchStore.selectItem(item)
  }

  // Основная логика поиска (вызывается только по нажатию Enter)
  handleSearch = async () => {
    const q = this.searchStore.query.trim()

    if (!q) {
      this.searchStore.setResults([])
      return
    }

    this.searchStore.setLoading(true)

    try {
      const response = await fetch(`/api/search?q=${encodeURIComponent(q)}`)
      if (!response.ok) throw new Error("Search failed")

      const data: SearchResult[] = await response.json()
      this.searchStore.setResults(data)
    } catch (error) {
      console.error("Search fetch error:", error)
      this.searchStore.setResults([])
    } finally {
      this.searchStore.setLoading(false)
    }
  }

  handleKeydown = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      this.handleSearch()
    }
  }

  clear = () => {
    this.searchStore.setQuery("")
    this.searchStore.setResults([])
  }
}
