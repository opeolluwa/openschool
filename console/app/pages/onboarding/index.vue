<script lang="ts" setup>
import { onboardingSteps } from "~/data/onboarding";

definePageMeta({ layout: "onboarding" });
const stepper = useTemplateRef("stepper");
</script>

<template>
    <div class="w-full h-full relative">
        <UStepper ref="stepper" :items="onboardingSteps" orientation="vertical">
            <template #admin-registration>
                <OnboardingAdminRegistration />
            </template>

            <template #general-information>
                <OnboardingGeneralInformation />
            </template>

            <template #contact-address>
                <OnboardingContactAddress />
            </template>

            <template #academics>
                <OnboardingAcademics />
            </template>
        </UStepper>

        <div class="flex gap-2 justify-between absolute bottom-3 w-full px-1">
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
