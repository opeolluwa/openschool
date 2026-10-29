// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    "@nuxt/image",
    "@nuxt/ui",
    "@nuxtjs/device",
    "@opeolluwa/ryder",
    "@nuxtjs/seo",
    "nuxt-viewport",
    "@pinia/nuxt",
    'pinia-plugin-persistedstate/nuxt',
  ],
  pinia: {
    storesDirs: ["./stores/**"],
  },
  imports: {
    dirs: [
      // Scan top-level composables
      "~/composables",
      // ... or scan composables nested one level deep with a specific name and file extension
      "~/composables/*/index.{ts,js,mjs,mts}",
      // ... or scan all composables within given directory
      "~/composables/**",
    ],
  },
  build: { transpile: ["@opeolluwa/ryder"] },
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  css: ["~/assets/main.css"],
});
