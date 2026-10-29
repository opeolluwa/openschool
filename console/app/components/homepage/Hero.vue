<template>
  <section class="relative overflow-hidden pt-36 pb-20">
    <div class="mx-auto max-w-7xl px-6 lg:px-8">
      <div class="grid gap-12 lg:grid-cols-2 lg:items-center">
        <div>

          <h1
            class="mt-3 text-5xl font-black sm:text-6xl leading-tight text-gray-900 dark:text-white"
          >
            Welcome to
            <span class="text-primary-500">Model University</span>
          </h1>
          <p class="mt-4 text-lg text-gray-600 dark:text-gray-300 max-w-xl">
            Shaping future leaders through academic excellence, innovation, and
            a vibrant campus community. Discover programs designed to prepare
            you for a world of possibilities.
          </p>
          <div class="mt-8 flex flex-wrap gap-4">
            <NuxtLink
              to="/admission"
              class="inline-flex items-center justify-center rounded-lg bg-primary px-6 py-3 text-sm font-semibold text-white hover:bg-primary-600 transition-colors duration-200"
            >
              Apply Now
            </NuxtLink>
            <NuxtLink
              to="/programs"
              class="inline-flex items-center justify-center rounded-lg border border-gray-300 dark:border-onyx-500 px-6 py-3 text-sm font-semibold text-gray-700 dark:text-onyx-100 hover:border-primary-500 hover:text-primary-600 dark:hover:text-primary-400 transition-colors duration-200"
            >
              Explore Programs
            </NuxtLink>
          </div>
        </div>

        <div class="lg:order-2">
          <NuxtImg
            src="/images/homepage/hero-6.jpg"
            alt="Model University Campus"
            width="768"
            height="576"
            sizes="sm:100vw lg:45vw"
            loading="lazy"
            class="w-full h-[420px] object-cover rounded-2xl"
          />
        </div>
      </div>
    </div>
  </section>
</template>
