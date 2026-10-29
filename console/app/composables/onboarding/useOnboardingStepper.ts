import { onboardingSteps } from '~/data/onboarding'

export function useOnboardingStepper() {
  const currentStep = useState('onboarding-step', () => 0)
  const hasNext = computed(() => currentStep.value < onboardingSteps.length - 1)
  const hasPrev = computed(() => currentStep.value > 0)

  function next() {
    if (hasNext.value) currentStep.value++
  }

  function prev() {
    if (hasPrev.value) currentStep.value--
  }

  const currentStepData = computed(() => onboardingSteps[currentStep.value])

  return { currentStep, hasNext, hasPrev, next, prev, steps: onboardingSteps, currentStepData }
}
