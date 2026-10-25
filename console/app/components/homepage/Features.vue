<script setup lang="ts">
const features = [
  {
    label: "Student Management",
    description: "Admissions, enrollment, profiles, and student records in one place.",
    icon: "ri:user-line",
  },
  {
    label: "Academics",
    description: "Curriculum planning, timetables, grading, and report cards.",
    icon: "ri:book-open-line",
  },
  {
    label: "Attendance",
    description: "Track daily attendance for students and staff with real-time reports.",
    icon: "ri:calendar-check-line",
  },
  {
    label: "Finance",
    description: "Fee collection, invoicing, payroll, and financial reporting.",
    icon: "ri:money-dollar-circle-line",
  },
  {
    label: "Communication",
    description: "Announcements, messaging, and parent-teacher communication.",
    icon: "ri:mail-send-line",
  },
  {
    label: "Administration",
    description: "Staff management, roles, permissions, and school configuration.",
    icon: "ri:settings-3-line",
  },
];
</script>

<template>
  <section id="features" class="bg-gray-50 py-16 lg:py-20 dark:bg-onyx-800/60">
    <div class="mx-auto max-w-7xl px-6 lg:px-8">
      <div>
        <p
          class="text-xs font-semibold uppercase tracking-widest text-primary-600 dark:text-primary-400"
        >
          What's Included
        </p>
        <h2
          class="mt-2 text-3xl font-black tracking-tight text-gray-900 sm:text-4xl dark:text-white"
        >
          Everything your school needs
        </h2>
        <p class="mt-2 text-sm text-gray-500 sm:text-base dark:text-gray-400">
          One platform for academics, administration, and communication.
        </p>
      </div>

      <div class="mt-8 grid gap-4 lg:grid-cols-2">
        <div
          v-for="feature in features"
          :key="feature.label"
          class="group flex items-center gap-4 rounded-xl border border-gray-200 bg-white p-5 transition-all duration-300 hover:-translate-y-0.5 hover:border-primary-300/30 hover:shadow-lg dark:border-gray-800 dark:bg-gray-800 dark:hover:border-primary-800"
        >
          <div
            class="flex size-11 shrink-0 items-center justify-center rounded-xl bg-primary-50/30 text-primary-600 dark:bg-primary-900/20 dark:text-primary-400"
          >
            <UIcon :name="feature.icon" class="size-5" />
          </div>

          <div class="min-w-0 flex-1">
            <p class="text-sm font-semibold text-gray-900 dark:text-white">
              {{ feature.label }}
            </p>
            <p class="mt-0.5 text-xs text-gray-500 dark:text-gray-400 sm:text-sm">
              {{ feature.description }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
