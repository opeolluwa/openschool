<script setup lang="ts">
const links = {
  Product: [
    { name: "Features", href: "/#features" },
    { name: "Pricing", href: "/pricing" },
    { name: "Self-Hosting", href: "/docs/self-hosting" },
    { name: "Demo", href: "/demo" },
    { name: "Changelog", href: "/changelog" },
  ],
  Resources: [
    { name: "Documentation", href: "/docs" },
    { name: "API Reference", href: "/docs/api" },
    { name: "Tutorials", href: "/docs/tutorials" },
    { name: "Blog", href: "/blog" },
  ],
  Community: [
    { name: "GitHub", href: "https://github.com/openschool" },
    { name: "Discussions", href: "https://github.com/openschool/discussions" },
    { name: "Contributing", href: "/docs/contributing" },
  ],
  Legal: [
    { name: "Privacy Policy", href: "/privacy-policy" },
    { name: "Terms of Service", href: "/terms" },
    { name: "License", href: "/license" },
  ],
};

const socials = [
  { name: "GitHub", href: "https://github.com/openschool", icon: "ri:github-line" },
  { name: "Discord", href: "#", icon: "ri:discord-line" },
  { name: "Twitter / X", href: "#", icon: "ri:twitter-x-fill" },
  { name: "YouTube", href: "#", icon: "ri:youtube-line" },
];
</script>

<template>
  <footer class="bg-gray-950 text-white">
    <div class="mx-auto max-w-7xl px-6 py-16 lg:px-8">
      <div class="grid lg:grid-cols-6 gap-12">
        <!-- Brand -->
        <div class="lg:col-span-2">
          <NuxtLink to="/">
            <img
              src="#"
              alt="OpenSchool"
              width="271"
              height="70"
              class="h-10 w-auto"
            />
          </NuxtLink>
          <p class="mt-4 text-sm text-gray-400 leading-relaxed max-w-xs">
            A free, open-source school management platform. Self-host on your
            own infrastructure for complete control over student data,
            academics, and administration.
          </p>
        </div>

        <!-- Link columns -->
        <div
          v-for="(items, section) in links"
          :key="section"
          class="space-y-4"
        >
          <p class="text-sm font-semibold text-white">{{ section }}</p>
          <ul class="space-y-3">
            <li v-for="item in items" :key="item.name">
              <a
                v-if="item.href.startsWith('http')"
                :href="item.href"
                target="_blank"
                rel="noreferrer"
                class="text-sm text-gray-400 hover:text-white transition"
              >
                {{ item.name }}
              </a>
              <NuxtLink
                v-else
                :to="item.href"
                class="text-sm text-gray-400 hover:text-white transition"
              >
                {{ item.name }}
              </NuxtLink>
            </li>
          </ul>
        </div>
      </div>

      <!-- Bottom bar -->
      <div
        class="mt-12 border-t border-white/10 pt-8 flex flex-col sm:flex-row items-center justify-between gap-4"
      >
        <p class="text-xs text-gray-500">
          &copy; {{ new Date().getFullYear() }} OpenSchool. Released under the
          MIT License.
        </p>

        <div class="flex items-center gap-5">
          <a
            v-for="social in socials"
            :key="social.name"
            :href="social.href"
            rel="noreferrer"
            target="_blank"
            class="text-gray-400 hover:text-white transition"
            :aria-label="social.name"
          >
            <UIcon :name="social.icon" class="size-5" />
          </a>
        </div>
      </div>
    </div>
  </footer>
</template>
