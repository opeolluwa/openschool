<script lang="ts" setup>
import type { Ref } from "vue";

definePageMeta({ layout: "onboarding" });

const { currentStep, currentStepData, hasPrev, next, prev, isLastStep } =
    useOnboardingStepper();

interface StepComponent {
    submit: () => Promise<boolean>;
    isProcessing: Ref<boolean>;
}

const adminRef = useTemplateRef<StepComponent>("adminRegistration");
const generalRef = useTemplateRef<StepComponent>("generalInformation");
const contactRef = useTemplateRef<StepComponent>("contactAddress");
const academicsRef = useTemplateRef<StepComponent>("academics");

const activeStepRef = computed(() => {
    switch (currentStep.value) {
        case 0:
            return adminRef.value;
        case 1:
            return generalRef.value;
        case 2:
            return contactRef.value;
        case 3:
            return academicsRef.value;
        default:
            return null;
    }
});

const isProcessing = computed(
    () => activeStepRef.value?.isProcessing?.value ?? false,
);

async function handleNext() {
    const stepComponent = activeStepRef.value;
    if (!stepComponent) return;

    const success = await stepComponent.submit();
    if (!success) return;

    if (isLastStep.value) {
        await navigateTo("/", { replace: true });
    } else {
        next();
    }
}
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
                <OnboardingAdminRegistration
                    v-if="currentStep === 0"
                    ref="adminRegistration"
                />
                <OnboardingGeneralInformation
                    v-else-if="currentStep === 1"
                    ref="generalInformation"
                />
                <OnboardingContactAddress
                    v-else-if="currentStep === 2"
                    ref="contactAddress"
                />
                <OnboardingAcademics
                    v-else-if="currentStep === 3"
                    ref="academics"
                />
            </div>
        </div>

        <div class="flex justify-between pt-6">
            <UButton
                leading-icon="i-lucide-arrow-left"
                variant="outline"
                size="lg"
                :disabled="!hasPrev || isProcessing"
                @click="prev()"
            >
                Prev
            </UButton>

            <UButton
                :trailing-icon="
                    isLastStep ? 'i-lucide-check' : 'i-lucide-arrow-right'
                "
                size="lg"
                :loading="isProcessing"
                :disabled="isProcessing"
                class="bg-primary"
                @click="handleNext()"
            >
                {{ isLastStep ? "Finish" : "Next" }}
            </UButton>
        </div>
    </div>
</template>
