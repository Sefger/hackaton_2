import ru from "./ru.json"

// We can add other languages later
const dictionaries = {
  ru,
}

export type Locale = keyof typeof dictionaries
// Nested key extraction (e.g. "search.placeholder")
type Dictionary = typeof ru

// Get deep paths types from the dictionary to allow autocompletion and type checking
type PathsToStringProps<T> = T extends string
  ? []
  : {
      [K in Extract<keyof T, string>]: [K, ...PathsToStringProps<T[K]>]
    }[Extract<keyof T, string>]

type Join<T extends string[], D extends string> = T extends []
  ? never
  : T extends [infer F]
    ? F
    : T extends [infer F, ...infer R]
      ? F extends string
        ? `${F}${D}${Join<Extract<R, string[]>, D>}`
        : never
      : string

export type TranslationKey = Join<PathsToStringProps<Dictionary>, ".">

class I18nStore {
  locale = $state<Locale>("ru")

  // Get deep value function
  private getNestedValue(obj: any, path: string) {
    return path.split(".").reduce((acc, part) => acc && acc[part], obj)
  }

  // The 't' translation mechanism that reacts to locale changes
  get t() {
    return (key: TranslationKey) => {
      const dict = dictionaries[this.locale]
      const val = this.getNestedValue(dict, key as string)
      return val || key
    }
  }

  setLocale(l: Locale) {
    this.locale = l
  }
}

// Singleton global store for translations
export const i18n = new I18nStore()
