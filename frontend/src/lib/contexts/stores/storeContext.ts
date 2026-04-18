import {getContext, setContext} from "svelte"
import {SearchStore} from "../../infrastructure/store/SearchStore.svelte"

const SEARCH_STORE_KEY = Symbol("SEARCH_STORE")

export function initStores() {
  const searchStore = new SearchStore()

  setContext(SEARCH_STORE_KEY, {searchStore})

  return {searchStore}
}

export function getStore() {
  return getContext<{searchStore: SearchStore}>(SEARCH_STORE_KEY)
}
