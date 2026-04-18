export class MediaQuery {
  #query: string
  #matches = $state(false)
  #mql: MediaQueryList | undefined

  constructor(query: string) {
    this.#query = query

    if (typeof window !== "undefined") {
      this.#mql = window.matchMedia(this.#query)
      this.#matches = this.#mql.matches

      this.#mql.addEventListener("change", this.#update)
    }
  }

  #update = (e: MediaQueryListEvent) => {
    this.#matches = e.matches
  }

  get current() {
    return this.#matches
  }

  destroy() {
    this.#mql?.removeEventListener("change", this.#update)
  }
}
