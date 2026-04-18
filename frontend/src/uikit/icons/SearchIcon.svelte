<script lang="ts">
  import {cn} from "$lib/utils"
  import type {IconProps} from "./types"

  let {size = 20, className = "", color = "currentColor"}: IconProps = $props()
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke={color}
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  class={cn("icon", className)}
>
  <circle
    cx="11"
    cy="11"
    r="8"
  ></circle>
  <path d="m21 21-4.3-4.3"></path>
</svg>
