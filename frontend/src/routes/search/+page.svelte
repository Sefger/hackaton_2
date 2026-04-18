<script lang="ts">
  import {PageController} from "$components/controllers/Page.controller.svelte"
  import QuickFilters from "$components/QuickFilters/QuickFilters.svelte"
  import Search from "$components/Search/Search.svelte"
  import StateIndicator from "$components/Search/StateIndicator.svelte"

  interface Props {
    data: {
      query: string
      results: any[]
    }
  }

  let {data}: Props = $props()
  const controller = new PageController(data)
</script>

<Search />

<QuickFilters filters={controller.quickFilters} />

<StateIndicator isLoading={controller.isLoading} />
