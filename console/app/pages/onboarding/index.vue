<script lang="ts" setup>
definePageMeta({ layout: "onboarding" });

const { currentStep, currentStepData, hasNext, hasPrev, next, prev } =
    useOnboardingStepper();
</script>

<template>
    <div class="flex flex-col h-full px-24">
        <div class="mb-8">
            <RyderPageHeader
                :title="currentStepData?.title"
                :subtitle="currentStepData?.description"
            />
        </div>

        <div class="flex-1 overflow-y-auto">
            <div>
                <OnboardingAdminRegistration v-if="currentStep === 0" />
                <OnboardingGeneralInformation v-else-if="currentStep === 1" />
                <OnboardingContactAddress v-else-if="currentStep === 2" />
                <OnboardingAcademics v-else-if="currentStep === 3" />
            </div>
        </div>

        <div class="flex justify-between pt-6">
            <UButton
                leading-icon="i-lucide-arrow-left"
                variant="outline"
                size="lg"
                :disabled="!hasPrev"
                @click="prev()"
            >
                Prev
            </UButton>

            <UButton
                trailing-icon="i-lucide-arrow-right"
                size="lg"
                :disabled="!hasNext"
                @click="next()"
            >
                Next
            </UButton>
        </div>
    </div>
</template>
