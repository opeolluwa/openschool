<script setup lang="ts">
const programs = [
  {
    label: "Sciences",
    description: "Biology, Chemistry, Physics, Mathematics, and Environmental Science.",
    icon: "ri:flask-line",
  },
  {
    label: "Engineering",
    description: "Civil, Mechanical, Electrical, and Computer Engineering programs.",
    icon: "ri:tools-line",
  },
  {
    label: "Arts & Humanities",
    description: "Literature, History, Philosophy, Languages, and Fine Arts.",
    icon: "ri:palette-line",
  },
  {
    label: "Business",
    description: "Accounting, Finance, Management, Marketing, and Entrepreneurship.",
    icon: "ri:briefcase-line",
  },
  {
    label: "Medicine & Health",
    description: "Medicine, Nursing, Pharmacy, and Public Health studies.",
    icon: "ri:heart-pulse-line",
  },
  {
    label: "Education",
    description: "Teacher training, Curriculum Design, and Educational Leadership.",
    icon: "ri:graduation-cap-line",
  },
];
</script>

<template>
  <section id="programs" class="bg-gray-50 py-16 lg:py-20 dark:bg-onyx-800/60">
    <div class="mx-auto max-w-7xl px-6 lg:px-8">
      <div>
        <p
          class="text-xs font-semibold uppercase tracking-widest text-primary-600 dark:text-primary-400"
        >
          Academic Programs
        </p>
        <h2
          class="mt-2 text-3xl font-black tracking-tight text-gray-900 sm:text-4xl dark:text-white"
        >
          Faculties & Departments
        </h2>
        <p class="mt-2 text-sm text-gray-500 sm:text-base dark:text-gray-400">
          Explore our diverse range of undergraduate and postgraduate programs.
        </p>
      </div>

      <div class="mt-8 grid gap-4 lg:grid-cols-2">
        <div
          v-for="program in programs"
          :key="program.label"
          class="group flex items-center gap-4 rounded-xl border border-gray-200 bg-white p-5 transition-all duration-300 hover:-translate-y-0.5 hover:border-primary-300/30 hover:shadow-lg dark:border-gray-800 dark:bg-gray-800 dark:hover:border-primary-800"
        >
          <div
            class="flex size-11 shrink-0 items-center justify-center rounded-xl bg-primary-50/30 text-primary-600 dark:bg-primary-900/20 dark:text-primary-400"
          >
            <UIcon :name="program.icon" class="size-5" />
          </div>

          <div class="min-w-0 flex-1">
            <p class="text-sm font-semibold text-gray-900 dark:text-white">
              {{ program.label }}
            </p>
            <p class="mt-0.5 text-xs text-gray-500 dark:text-gray-400 sm:text-sm">
              {{ program.description }}
            </p>
          </div>
        </div>
      </div>

      <div class="mt-8 text-center">
        <NuxtLink
          to="/programs"
          class="inline-flex items-center gap-1 text-sm font-semibold text-primary-600 hover:text-primary-700 dark:text-primary-400 dark:hover:text-primary-300 transition-colors"
        >
          View All Programs
          <UIcon name="ri:arrow-right-line" class="size-4" />
        </NuxtLink>
      </div>
    </div>
  </section>
</template>
