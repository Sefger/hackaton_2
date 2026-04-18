/**
 * @see https://prettier.io/docs/configuration
 * @type {import("prettier").Config}
 */
const config = {
  printWidth: 120,
  tabWidth: 2,
  semi: false,
  singleQuote: false,
  trailingComma: "es5",
  bracketSpacing: false,
  singleAttributePerLine: true,
  importOrderTypeScriptVersion: "5.5.0",
  importOrderParserPlugins: ["typescript", "decorators-legacy"],
  plugins: ["prettier-plugin-svelte", "@ianvs/prettier-plugin-sort-imports"],
  overrides: [
    {
      files: "**/*.svelte",
      options: {
        parser: "svelte",
      },
    },
    {
      files: ["**/*.md", "**/*.html"],
      options: {
        importOrder: [],
      },
    },
  ],
}

export default config
