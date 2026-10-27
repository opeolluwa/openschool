<script setup lang="ts">
const values = [
  {
    label: "Excellence",
    description: "We pursue the highest standards in teaching, research, and student development.",
    icon: "ri:award-line",
  },
  {
    label: "Innovation",
    description: "We embrace new ideas, technologies, and approaches to learning and discovery.",
    icon: "ri:lightbulb-line",
  },
  {
    label: "Community",
    description: "We foster an inclusive environment where every voice is valued and respected.",
    icon: "ri:hand-heart-line",
  },
  {
    label: "Integrity",
    description: "We uphold honesty, accountability, and ethical conduct in everything we do.",
    icon: "ri:shield-check-line",
  },
];
</script>

<template>
  <section class="py-16 lg:py-20">
    <div class="mx-auto max-w-7xl px-6 lg:px-8">
      <div>
        <p
          class="text-xs font-semibold uppercase tracking-widest text-primary-600 dark:text-primary-400"
        >
          About Us
        </p>
        <h2
          class="mt-2 text-3xl font-black tracking-tight text-gray-900 sm:text-4xl dark:text-white"
        >
          Our Mission
        </h2>
        <p class="mt-2 text-sm text-gray-500 sm:text-base dark:text-gray-400 max-w-2xl">
          Model University is dedicated to nurturing well-rounded individuals
          who are equipped to lead, innovate, and make meaningful contributions
          to society through rigorous academics and transformative experiences.
        </p>
      </div>

      <div class="mt-8 grid gap-4 sm:grid-cols-2">
        <div
          v-for="value in values"
          :key="value.label"
          class="group flex items-center gap-4 rounded-xl border border-gray-200 bg-white p-5 transition-all duration-300 hover:-translate-y-0.5 hover:border-primary-300/30 hover:shadow-lg dark:border-gray-800 dark:bg-gray-800 dark:hover:border-primary-800"
        >
          <div
            class="flex size-11 shrink-0 items-center justify-center rounded-xl bg-primary-50/30 text-primary-600 dark:bg-primary-900/20 dark:text-primary-400"
          >
            <UIcon :name="value.icon" class="size-5" />
          </div>

          <div class="min-w-0 flex-1">
            <p class="text-sm font-semibold text-gray-900 dark:text-white">
              {{ value.label }}
            </p>
            <p class="mt-0.5 text-xs text-gray-500 dark:text-gray-400 sm:text-sm">
              {{ value.description }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
