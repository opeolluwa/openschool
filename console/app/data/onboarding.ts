import type { StepperItem } from '@nuxt/ui'

export const onboardingSteps = [
  {
    slot: 'admin-registration' as const,
    title: 'Administrator Account',
    description: 'Email and password of the super admin',
    icon: 'heroicons:user'
  },
  {
    slot: 'general-information' as const,
    title: 'General Information',
    description: 'Your institution name, address and related information',
    icon: 'heroicons:building-library'
  },
  {
    slot: 'contact-address' as const,
    title: 'Contact Address',
    description: 'Address and contact details for your institution',
    icon: 'heroicons:map-pin'
  },
  {
    slot: 'academics' as const,
    title: 'Academics',
    description: 'Set up your academic sessions and curriculum',
    icon: 'heroicons:academic-cap'
  }
] satisfies StepperItem[]
