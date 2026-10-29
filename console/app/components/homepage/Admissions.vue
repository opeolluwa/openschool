<template>
  <section class="bg-primary-600 dark:bg-primary-700 py-16 lg:py-20">
    <div class="mx-auto max-w-7xl px-6 lg:px-8 text-center">
      <h2 class="text-3xl font-black tracking-tight text-white sm:text-4xl">
        Ready to Join Model University?
      </h2>
      <p class="mt-4 text-lg text-primary-100 max-w-2xl mx-auto">
        Take the first step toward your future. Our admissions team is ready to
        guide you through the application process.
      </p>
      <div class="mt-8 flex flex-wrap justify-center gap-4">
        <NuxtLink
          to="/admission"
          class="inline-flex items-center justify-center rounded-lg bg-white px-6 py-3 text-sm font-semibold text-primary-700 hover:bg-primary-50 transition-colors duration-200"
        >
          Start Your Application
        </NuxtLink>
        <NuxtLink
          to="/student"
          class="inline-flex items-center justify-center rounded-lg border border-white/30 px-6 py-3 text-sm font-semibold text-white hover:bg-white/10 transition-colors duration-200"
        >
          Learn More
        </NuxtLink>
      </div>
    </div>
  </section>
</template>
