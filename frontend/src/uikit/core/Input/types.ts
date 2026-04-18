import type {HTMLAttributes} from "svelte/elements"

export type InputProps = HTMLAttributes<HTMLInputElement> & {
  value?: string
  placeholder?: string
  type?: string
  disabled?: boolean
  className?: string
  leftIcon?: boolean
  rightIcon?: boolean
}
