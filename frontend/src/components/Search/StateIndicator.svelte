<script lang="ts">
  import {DotLottieSvelte} from "@lottiefiles/dotlottie-svelte"

  interface Props {
    isLoading: boolean
  }

  let {isLoading}: Props = $props()
</script>

{#if isLoading}
  <div class="mt-8 flex flex-col items-center gap-4">
    <div class="w-48 h-48">
      <DotLottieSvelte
        src="/loader.lottie"
        loop
        autoplay
      />
    </div>
    <p class="text-[10px] text-primary-400 uppercase tracking-[0.3em] font-bold">Scanning neural networks...</p>
  </div>
{/if}
