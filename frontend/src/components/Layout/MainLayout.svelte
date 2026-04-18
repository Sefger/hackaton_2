<script lang="ts">
  import ShootingStars from "$uikit/core/ShootingStars.svelte"

  interface Props {
    children: any
  }

  let {children}: Props = $props()
</script>

<div
  class="min-h-screen w-full flex flex-col items-center relative bg-[#05050a] px-4 overflow-x-hidden selection:bg-primary-500 selection:bg-opacity-30"
>
  <div class="absolute inset-0 pointer-events-none overflow-hidden">
    <ShootingStars />
    <div class="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-blue-600/10 blur-[120px] rounded-full"></div>
    <div class="absolute bottom-[10%] right-[-5%] w-[35%] h-[35%] bg-indigo-600/10 blur-[100px] rounded-full"></div>
    <div class="absolute top-[30%] right-[10%] w-[25%] h-[25%] bg-purple-600/5 blur-[80px] rounded-full"></div>
    <div
      class="absolute inset-0 opacity-[0.03] pointer-events-none bg-[url('https://grainy-gradients.vercel.app/noise.svg')]"
    ></div>
  </div>

  <div class="z-10 w-full max-w-4xl flex flex-col items-center gap-12 pt-20 pb-20">
    {@render children()}
  </div>
</div>
