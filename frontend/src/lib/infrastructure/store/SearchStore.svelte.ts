export type SearchResult<T = any> = {
  id: string
  title: string
  description: string
  type: string
  icon?: string
  metadata?: T
}

export class SearchStore {
  query = $state("")
  isLoading = $state(false)
  results = $state.raw<SearchResult[]>([])
  selectedItem = $state.raw<SearchResult | null>(null)

  setQuery(value: string) {
    this.query = value
  }

  setLoading(value: boolean) {
    this.isLoading = value
  }

  setResults(results: SearchResult[]) {
    this.results = results
  }

  selectItem(item: SearchResult | null) {
    this.selectedItem = item
  }
}
