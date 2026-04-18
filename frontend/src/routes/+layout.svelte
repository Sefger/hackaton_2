<script lang="ts">
  import "$tailwind/app.css"
  import {page} from "$app/state"
  import Footer from "$components/Footer/Footer.svelte"
  import Logo from "$components/Header/Logo.svelte"
  import MainLayout from "$components/Layout/MainLayout.svelte"

  let {children} = $props()
  let isLanding = $derived(page.url.pathname === "/")
</script>

<MainLayout>
  {#if !isLanding && page.status === 200}
    <Logo />
  {/if}

  {@render children()}

  {#if !isLanding}
    <Footer />
  {/if}
</MainLayout>
