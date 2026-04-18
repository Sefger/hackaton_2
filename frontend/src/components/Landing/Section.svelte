<script lang="ts">
  import {cn} from "$lib/utils"

  interface Props {
    children: any
    className?: string
    align?: "left" | "right" | "center"
  }

  let {children, className, align = "left"}: Props = $props()
</script>

<section class="min-h-screen flex items-center justify-center p-6 pointer-events-none">
  <div
    class={cn(
      "w-full max-w-[700px] bg-[#0a0a12]/70 backdrop-blur-[25px] p-8 md:p-14 rounded-[1.5rem] border border-white/10 pointer-events-auto shadow-[0_20px_50px_rgba(0,0,0,0.7),0_0_30px_rgba(0,102,255,0.1)] animate-float",
      align === "right" && "md:ml-auto",
      align === "center" && "text-center mx-auto",
      className
    )}
  >
    {@render children()}
  </div>
</section>
