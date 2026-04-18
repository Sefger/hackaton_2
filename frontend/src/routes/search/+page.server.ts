import type {PageServerLoad} from "./$types"

/**
 * Handle initial data fetching on the server.
 * This is useful for SEO, performance, and keeping API keys secure.
 */
export const load: PageServerLoad = async ({fetch, url}) => {
  const query = url.searchParams.get("q")

  if (query) {
    /**
     * SvelteKit automatically proxies these server-side fetches.
     * Use this pattern to fetch data BEFORE the page renders.
     */
    const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`)
    const results = await response.json()

    return {
      query,
      results,
    }
  }

  // Initial state when page loads without query
  return {
    query: "",
    results: [],
  }
}
