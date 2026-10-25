<script setup lang="ts">
import { footerSections, socialLinks, isExternalLink } from "~/data/homepage.routes";
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
          v-for="section in footerSections"
          :key="section.title"
          class="space-y-4"
        >
          <p class="text-sm font-semibold text-white">{{ section.title }}</p>
          <ul class="space-y-3">
            <li v-for="item in section.links" :key="item.label">
              <a
                v-if="isExternalLink(item)"
                :href="item.to"
                target="_blank"
                rel="noreferrer"
                class="text-sm text-gray-400 hover:text-white transition"
              >
                {{ item.label }}
              </a>
              <NuxtLink
                v-else
                :to="item.to"
                class="text-sm text-gray-400 hover:text-white transition"
              >
                {{ item.label }}
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
            v-for="social in socialLinks"
            :key="social.label"
            :href="social.to"
            rel="noreferrer"
            target="_blank"
            class="text-gray-400 hover:text-white transition"
            :aria-label="social.label"
          >
            <UIcon :name="social.icon" class="size-5" />
          </a>
        </div>
      </div>
    </div>
  </footer>
</template>
