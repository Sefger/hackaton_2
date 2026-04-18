<script lang="ts">
  import {MediaQuery} from "$lib/reactivity/MediaQuery.svelte"
  import {cn} from "$lib/utils"
  import type {HTMLAnchorAttributes, HTMLButtonAttributes} from "svelte/elements"
  import type {ButtonProps, ButtonSize, ButtonVariant} from "./types"

  let {
    variant = "default",
    size = "medium",
    type = "button",
    children,
    isLoading = false,
    disabled = false,
    className = "",
    ...props
  }: ButtonProps = $props()

  const isDesktop = new MediaQuery("(min-width: 1024px)")
  const currentSize = $derived(props.mobileSize ? (isDesktop.current ? size : props.mobileSize) : size)

  const variantMap: Record<ButtonVariant, string> = {
    default: "bg-primary-500 hover:bg-primary-600 text-white shadow-lg shadow-primary-500/20 active:scale-[0.98]",
    outline: "border-2 border-primary-500 text-primary-500 hover:bg-primary-50/10",
    ghost: "text-primary-500 hover:bg-primary-50/5",
    accent: "bg-accent-500 hover:bg-accent-600 text-white shadow-lg shadow-accent-500/20",
    error: "bg-error-500 hover:bg-error-600 text-white",
    success: "bg-success-500 hover:bg-success-600 text-white",
  }

  const sizeMap: Record<ButtonSize, string> = {
    tiny: "px-2 py-1 text-xs rounded-xs",
    small: "px-3 py-1.5 text-sm rounded-sm",
    medium: "px-4 py-2 text-base rounded-md font-medium",
    large: "px-6 py-3 text-lg rounded-lg font-semibold",
    huge: "px-8 py-4 text-xl rounded-xl font-bold",
  }

  const baseClass =
    "inline-flex items-center justify-center transition-all duration-200 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed select-none"

  const anchorProps = $derived(props as HTMLAnchorAttributes)
  const buttonProps = $derived(props as HTMLButtonAttributes)
</script>

{#if props.href && !disabled && !isLoading}
  <a
    href={props.href}
    class={cn(baseClass, variantMap[variant], sizeMap[currentSize], className)}
    {...anchorProps}
  >
    {@render content()}
  </a>
{:else}
  <button
    type={type as "button" | "submit" | "reset"}
    {disabled}
    class={cn(baseClass, variantMap[variant], sizeMap[currentSize], className)}
    {...buttonProps}
  >
    {@render content()}
  </button>
{/if}

{#snippet content()}
  {#if isLoading}
    <div class="flex items-center gap-2">
      <svg
        class="animate-spin h-4 w-4"
        viewBox="0 0 24 24"
      >
        <circle
          class="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          stroke-width="4"
          fill="none"
        ></circle>
        <path
          class="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
        ></path>
      </svg>
      <span>Loading...</span>
    </div>
  {:else}
    {@render children?.()}
  {/if}
{/snippet}
