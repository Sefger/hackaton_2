<script lang="ts">
  import {cn} from "$lib/utils"
  import type {InputProps} from "./types"

  let {
    value = $bindable(""),
    placeholder = "",
    type = "text",
    disabled = false,
    className = "",
    ...props
  }: InputProps = $props()

  const baseClass =
    "w-full bg-transparent border border-white/10 rounded-xl px-4 py-2 text-white focus:outline-none focus:ring-0 focus:border-primary-500 transition-all duration-200 placeholder:text-gray-600/50 outline-none"
</script>

<input
  {type}
  {placeholder}
  {disabled}
  bind:value
  class={cn(baseClass, className)}
  {...props}
/>
