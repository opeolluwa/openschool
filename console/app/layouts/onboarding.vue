<script lang="ts" setup>
const { currentStep, steps } = useOnboardingStepper();
</script>

<template>
    <div class="grid grid-cols-12 h-dvh">
        <div class="col-span-3 bg-primary-50 flex flex-col overflow-hidden">
            <div class="p-12">
                <h2 class="text-xl font-bold text-primary-900 mb-1">Welcome!</h2>
                <p class="text-sm text-primary-700 mb-8">
                    Let's set up your institution. Complete the steps to get started.
                </p>

                <nav class="space-y-6">
                    <div
                        v-for="(step, index) in steps"
                        :key="step.key"
                        class="flex items-start gap-3"
                    >
                        <div
                            class="w-10 h-10 rounded-full flex items-center justify-center shrink-0"
                            :class="[
                                index < currentStep
                                    ? 'bg-primary-600 text-white'
                                    : index === currentStep
                                      ? 'bg-primary-600 text-white'
                                      : 'bg-primary-100 text-primary-400',
                            ]"
                        >
                            <UIcon
                                v-if="index < currentStep"
                                name="heroicons:check"
                                class="text-lg"
                            />
                            <UIcon v-else :name="step.icon" class="text-lg" />
                        </div>
                        <div>
                            <p
                                class="text-sm font-semibold"
                                :class="
                                    index <= currentStep
                                        ? 'text-primary-900'
                                        : 'text-primary-400'
                                "
                            >
                                {{ step.title }}
                            </p>
                            <p
                                class="text-xs mt-0.5"
                                :class="
                                    index <= currentStep
                                        ? 'text-primary-700'
                                        : 'text-primary-300'
                                "
                            >
                                {{ step.description }}
                            </p>
                        </div>
                    </div>
                </nav>
            </div>

            <div class="mt-auto" />
        </div>

        <div class="col-span-9 pt-12 pb-8 px-24 overflow-hidden">
            <slot />
        </div>
    </div>
</template>
