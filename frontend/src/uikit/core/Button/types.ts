import type {Snippet} from "svelte"
import type {HTMLAnchorAttributes, HTMLButtonAttributes} from "svelte/elements"

export type ButtonVariant = "default" | "outline" | "ghost" | "accent" | "error" | "success"
export type ButtonSize = "small" | "medium" | "large" | "huge" | "tiny"

export type ButtonProps = (HTMLButtonAttributes | HTMLAnchorAttributes) & {
  variant?: ButtonVariant
  size?: ButtonSize
  mobileSize?: ButtonSize
  isLoading?: boolean
  disabled?: boolean
  children?: Snippet
  className?: string
  href?: string
}
