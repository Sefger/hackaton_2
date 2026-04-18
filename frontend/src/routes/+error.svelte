<script lang="ts">
  import {DotLottieSvelte} from "@lottiefiles/dotlottie-svelte"
  import {page} from "$app/state"
</script>

<div class="flex flex-col items-center justify-center min-h-[60vh] text-center px-4">
  <div class="w-64 h-64 md:w-80 md:h-80 mb-8">
    <DotLottieSvelte
      src="/loader.lottie"
      loop
      autoplay
    />
  </div>

  <h1 class="text-4xl md:text-6xl font-black text-white mb-4 tracking-tighter">
    {page.status === 404 ? "LOST IN SPACE" : "NEURAL GLITCH"}
  </h1>

  <p class="text-primary-400 uppercase tracking-[0.3em] font-bold text-sm mb-8">
    {page.status === 404
      ? "The planet you're looking for doesn't exist in our records."
      : "An unexpected error occurred in the neural network."}
  </p>

  <a
    href="/"
    class="px-8 py-3 bg-primary-500 hover:bg-primary-400 text-black font-bold rounded-full transition-all hover:scale-105 active:scale-95 uppercase tracking-widest text-xs"
  >
    Return to Base
  </a>
</div>
