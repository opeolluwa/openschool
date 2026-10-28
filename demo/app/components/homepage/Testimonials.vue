<script setup lang="ts">
const testimonials = [
  {
    name: "Amina Ibrahim",
    role: "Computer Science, Class of 2025",
    initials: "AI",
    quote:
      "Model University gave me the foundation and confidence to pursue my dream career in tech. The faculty and resources are world-class.",
  },
  {
    name: "James Okafor",
    role: "Business Administration, Class of 2024",
    initials: "JO",
    quote:
      "The hands-on learning approach and internship opportunities prepared me for the real world long before graduation.",
  },
  {
    name: "Fatima Al-Hassan",
    role: "Medicine, Class of 2026",
    initials: "FA",
    quote:
      "From the modern laboratories to the supportive community, everything about this university inspires me to keep pushing forward.",
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
          Testimonials
        </p>
        <h2
          class="mt-2 text-3xl font-black tracking-tight text-gray-900 sm:text-4xl dark:text-white"
        >
          Hear from Our Students
        </h2>
        <p class="mt-2 text-sm text-gray-500 sm:text-base dark:text-gray-400">
          Discover what makes Model University a place where students thrive.
        </p>
      </div>

      <div class="mt-8 grid gap-6 lg:grid-cols-3">
        <div
          v-for="testimonial in testimonials"
          :key="testimonial.name"
          class="rounded-xl border border-gray-200 bg-white p-6 dark:border-gray-800 dark:bg-gray-800"
        >
          <UIcon
            name="ri:double-quotes-l"
            class="size-8 text-primary-500/40"
          />
          <p
            class="mt-3 text-sm italic leading-relaxed text-gray-600 dark:text-gray-300"
          >
            "{{ testimonial.quote }}"
          </p>

          <div
            class="mt-5 border-t border-gray-100 pt-4 dark:border-gray-700"
          >
            <div class="flex items-center gap-3">
              <div
                class="flex size-9 shrink-0 items-center justify-center rounded-full bg-primary-100 text-xs font-semibold text-primary-700 dark:bg-primary-900/30 dark:text-primary-400"
              >
                {{ testimonial.initials }}
              </div>
              <div>
                <p
                  class="text-sm font-semibold text-gray-900 dark:text-white"
                >
                  {{ testimonial.name }}
                </p>
                <p class="text-xs text-gray-500 dark:text-gray-400">
                  {{ testimonial.role }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
