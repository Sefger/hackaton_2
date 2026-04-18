import adapter from "@sveltejs/adapter-auto"
import {vitePreprocess} from "@sveltejs/vite-plugin-svelte"

/** @type {Record<string, string>} */
export const svelteKitAliases = {
  $lib: "src/lib",
  $core: "src/core",
  $uikit: "src/uikit",
  $components: "src/components",
  $tailwind: "src/tailwind",
  $config: "src/config",
  $test: "tests",
}

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
    alias: svelteKitAliases,
  },
}

export default config
