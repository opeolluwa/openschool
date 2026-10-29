<template>
  <section class="py-16 lg:py-20">
    <div class="mx-auto max-w-7xl px-6 lg:px-8">
      <div>
        <p
          class="text-xs font-semibold uppercase tracking-widest text-primary-600 dark:text-primary-400"
        >
          Campus Life
        </p>
        <h2
          class="mt-2 text-3xl font-black tracking-tight text-gray-900 sm:text-4xl dark:text-white"
        >
          Life at Model University
        </h2>
        <p class="mt-2 text-sm text-gray-500 sm:text-base dark:text-gray-400 max-w-2xl">
          From state-of-the-art facilities to a thriving student community,
          experience a campus designed to inspire growth, creativity, and
          lifelong friendships.
        </p>
      </div>

      <div class="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <NuxtImg
          src="/images/homepage/hero.jpg"
          alt="Model University campus"
          width="400"
          height="300"
          class="w-full h-56 object-cover rounded-xl"
          loading="lazy"
        />
        <NuxtImg
          src="/images/homepage/hero-2.jpg"
          alt="Students on campus"
          width="400"
          height="300"
          class="w-full h-56 object-cover rounded-xl"
          loading="lazy"
        />
        <NuxtImg
          src="/images/homepage/hero-3.jpg"
          alt="University facilities"
          width="400"
          height="300"
          class="w-full h-56 object-cover rounded-xl sm:col-span-2 lg:col-span-1"
          loading="lazy"
        />
      </div>
    </div>
  </section>
</template>
