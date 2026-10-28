// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@nuxt/image", "@nuxt/ui", "@nuxtjs/device", "@opeolluwa/ryder", "@nuxtjs/seo"],
  build: { transpile: ["@opeolluwa/ryder"] },
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  css: ["~/assets/main.css"],
});
