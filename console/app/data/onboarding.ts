import type { StepperItem } from '@nuxt/ui'

export const onboardingSteps: StepperItem[] = [
  {
    title: 'Administrator account',
    description: 'Email and password of the super admin',
    icon: 'heroicons:user'
  },
  {
    title: 'General Information',
    description: 'Your institution name, address and related information',
    icon: 'i-lucide-house'
  }, {
    title: 'Shipping',
    description: 'Set your preferred shipping method',
    icon: 'i-lucide-truck'
  }, {
    title: 'Checkout',
    description: 'Confirm your order'
  }
]
