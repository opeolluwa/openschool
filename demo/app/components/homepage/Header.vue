<script setup lang="ts">
import { headerNavigation } from "~/data/routes";

const navigation = headerNavigation;
const mobileMenuOpen = ref(false);
const route = useRoute();

const isHeaderHidden = ref(false);
let lastScrollY = 0;
let idleTimer: ReturnType<typeof setTimeout> | null = null;

function handleScroll() {
  if (!window.matchMedia("(min-width: 1024px)").matches) {
    isHeaderHidden.value = false;
    return;
  }
  const currentY = window.scrollY;
  if (currentY <= 80) {
    isHeaderHidden.value = false;
  } else if (currentY > lastScrollY) {
    isHeaderHidden.value = true;
  } else if (currentY < lastScrollY) {
    isHeaderHidden.value = false;
  }
  lastScrollY = currentY;

  if (idleTimer) clearTimeout(idleTimer);
  idleTimer = setTimeout(() => {
    isHeaderHidden.value = false;
  }, 300);
}

onMounted(() => {
  window.addEventListener("scroll", handleScroll, { passive: true });
});

onBeforeUnmount(() => {
  window.removeEventListener("scroll", handleScroll);
  if (idleTimer) clearTimeout(idleTimer);
});

function isActive(href: string) {
  if (href.startsWith("/#")) return route.path === "/";
  return href === "/" ? route.path === href : route.path.startsWith(href);
}
</script>

<template>
  <header
    class="fixed inset-x-0 top-0 z-50 bg-white/95 dark:bg-onyx-600/95 backdrop-blur-md shadow-sm transition-all duration-300 ease-out"
    :class="
      isHeaderHidden
        ? '-translate-y-full opacity-0'
        : 'translate-y-0 opacity-100'
    "
  >
    <nav
      class="mx-auto flex max-w-7xl items-center justify-between px-6 py-4 lg:px-8"
    >
      <div class="flex lg:flex-1">
        <RyderLogo alt="Model University" />
      </div>

      <!-- Mobile controls -->
      <div class="flex lg:hidden items-center gap-2">
        <UColorModeButton />
        <button
          type="button"
          class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700 dark:text-onyx-100"
          @click="mobileMenuOpen = true"
        >
          <span class="sr-only">Open main menu</span>
          <UIcon name="ri:menu-line" class="size-6" />
        </button>
      </div>

      <!-- Desktop nav -->
      <div class="hidden lg:flex lg:gap-x-1">
        <NuxtLink
          v-for="item in navigation"
          :key="item.label"
          :to="item.to"
          class="relative text-sm font-medium px-4 py-2 transition-all duration-200 text-gray-700 dark:text-onyx-100 hover:text-primary-500 dark:hover:text-primary-400"
          :class="
            isActive(item.to) ? 'text-primary-600 dark:text-primary-400' : ''
          "
        >
          {{ item.label }}
          <span
            class="absolute left-4 right-4 bottom-0 h-0.5 rounded-full bg-primary-500 transition-all duration-300 origin-left"
            :class="isActive(item.to) ? 'scale-x-100' : 'scale-x-0'"
          />
        </NuxtLink>
      </div>

      <!-- Desktop right -->
      <div
        class="hidden lg:flex lg:flex-1 lg:justify-end lg:items-center lg:gap-3"
      >
        <UColorModeButton />
        <NuxtLink to="/admission">
          <RyderButton size="sm" color="primary" variant="solid">
            Apply Now
          </RyderButton>
        </NuxtLink>
      </div>
    </nav>

    <!-- Mobile bottom-sheet menu -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition-opacity duration-300"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-opacity duration-300"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div
          v-if="mobileMenuOpen"
          class="fixed inset-0 z-50 bg-black/50 lg:hidden"
          @click="mobileMenuOpen = false"
        />
      </Transition>

      <Transition
        enter-active-class="transition-transform duration-300 ease-out"
        enter-from-class="translate-y-full"
        enter-to-class="translate-y-0"
        leave-active-class="transition-transform duration-300 ease-in"
        leave-from-class="translate-y-0"
        leave-to-class="translate-y-full"
      >
        <div
          v-if="mobileMenuOpen"
          class="fixed bottom-0 inset-x-0 z-50 lg:hidden bg-white dark:bg-onyx-600 rounded-t-3xl shadow-2xl"
        >
          <div class="flex justify-center pt-3 pb-1">
            <div class="w-10 h-1 rounded-lg bg-gray-300 dark:bg-onyx-400" />
          </div>

          <div class="flex items-center justify-between px-5 py-3">
            <span class="text-base font-bold text-gray-900 dark:text-onyx-50">Menu</span>
            <button
              type="button"
              class="-m-2.5 rounded-lg p-2.5 text-gray-500 dark:text-onyx-200 hover:text-gray-700 dark:hover:text-onyx-50 hover:bg-gray-100 dark:hover:bg-onyx-500 transition"
              @click="mobileMenuOpen = false"
            >
              <span class="sr-only">Close menu</span>
              <UIcon name="ri:close-line" class="size-5" />
            </button>
          </div>

          <div class="px-4 py-2">
            <NuxtLink
              v-for="item in navigation"
              :key="item.label"
              :to="item.to"
              class="flex items-center justify-between px-4 py-3.5 rounded-xl text-sm font-medium text-gray-700 dark:text-onyx-100 hover:text-white hover:bg-primary-500 transition-colors duration-150"
              :class="isActive(item.to) ? 'bg-primary-500 text-white' : ''"
              @click="mobileMenuOpen = false"
            >
              <div class="flex items-center gap-3">
                <UIcon :name="item.icon" class="size-5" />
                {{ item.label }}
              </div>
              <UIcon
                name="ri:arrow-right-s-line"
                class="size-4 text-gray-400 dark:text-onyx-300"
              />
            </NuxtLink>

            <NuxtLink
              to="/admission"
              class="flex items-center justify-between px-4 py-3.5 mt-1 rounded-xl font-medium text-white bg-primary-500 hover:bg-primary-600 transition-colors duration-150"
              @click="mobileMenuOpen = false"
            >
              <div class="flex items-center gap-3">
                <UIcon name="ri:edit-line" class="size-5" />
                Apply Now
              </div>
              <UIcon name="ri:arrow-right-s-line" class="size-4" />
            </NuxtLink>
          </div>
        </div>
      </Transition>
    </Teleport>
  </header>
</template>
