<script lang="ts">
  interface Props {
    text: string
  }
  let {text}: Props = $props()
</script>

<span
  class="inline-block px-3 py-1 bg-primary-500/15 border border-primary-500/50 rounded-full text-[10px] md:text-xs font-bold text-primary-400 uppercase tracking-[0.1em] shadow-[0_0_15px_rgba(0,102,255,0.1)] transition-all duration-300 hover:bg-primary-500/30 hover:shadow-[0_0_20px_rgba(0,102,255,0.3)] mr-2 mb-2"
>
  {text}
</span>
