<script lang="ts" setup>
import { onboardingSteps } from "~/data/onboarding";

definePageMeta({ layout: "onboarding" });
const stepper = useTemplateRef("stepper");
</script>

<template>
    <div class="w-full">
        <UStepper ref="stepper" :items="onboardingSteps">
            <template #content="{ item }">
                <Placeholder class="aspect-video">
                    {{ item.title }}
                </Placeholder>
            </template>
        </UStepper>

        <div class="flex gap-2 justify-between mt-4 bg-red-500">
            <UButton
                leading-icon="i-lucide-arrow-left"
                :disabled="!stepper?.hasPrev"
                @click="stepper?.prev()"
            >
                Prev
            </UButton>

            <UButton
                trailing-icon="i-lucide-arrow-right"
                :disabled="!stepper?.hasNext"
                @click="stepper?.next()"
            >
                Next
            </UButton>
        </div>
    </div>
</template>
