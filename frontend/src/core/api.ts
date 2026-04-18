/// <reference types="vite/client" />

/**
 * This service handles communication with your corporate search backend.
 */
export class SearchService {
  private static BASE_URL = import.meta.env.VITE_API_URL || "https://api.your-workspace.com/v1"

  /**
   * Performs a global search across documents, people, and tools.
   */
  static async query(text: string, options: {filter?: string; limit?: number} = {}) {
    try {
      const response = await fetch(`${this.BASE_URL}/search?q=${encodeURIComponent(text)}`, {
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
      })

      if (!response.ok) {
        throw new Error(`Search failed with status: ${response.status}`)
      }

      return await response.json()
    } catch (error) {
      console.error("Search API Error:", error)
      return {results: [], error: true}
    }
  }

  /**
   * Fetches real-time suggestions based on current partial query.
   */
  static async getSuggestions(partial: string) {
    if (partial.length < 2) return []

    const res = await fetch(`${this.BASE_URL}/suggestions?q=${encodeURIComponent(partial)}`)
    return await res.json()
  }
}
