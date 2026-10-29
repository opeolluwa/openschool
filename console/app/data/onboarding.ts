export interface OnboardingStep {
  key: string
  title: string
  description: string
  icon: string
}

export const institutionTypes = [
  { label: 'Higher Education', value: 'higher_education' },
  { label: 'Primary & Secondary Education', value: 'primary_and_secondary_education' },
  { label: 'Vocational Education', value: 'vocational_education' }
]

export const academicCalendarTypes = [
  { label: 'Semester', value: 'semester' },
  { label: 'Terms', value: 'terms' },
  { label: 'Quarters', value: 'quarters' },
  { label: 'Self-paced', value: 'self_paced' }
]

export const gradingSystemTypes = [
  { label: 'GPA', value: 'gpa' },
  { label: 'Grades', value: 'grades' },
  { label: 'Competency', value: 'competency' },
  { label: 'Custom', value: 'custom' }
]

export const onboardingSteps: OnboardingStep[] = [
  {
    key: 'admin-registration',
    title: 'Administrator Account',
    description: 'Email and password of the super admin',
    icon: 'heroicons:user'
  },
  {
    key: 'general-information',
    title: 'General Information',
    description: 'Your institution name, address and related information',
    icon: 'heroicons:building-library'
  },
  {
    key: 'contact-address',
    title: 'Contact Address',
    description: 'Address and contact details for your institution',
    icon: 'heroicons:map-pin'
  },
  {
    key: 'academics',
    title: 'Academics',
    description: 'Set up your academic sessions and curriculum',
    icon: 'heroicons:academic-cap'
  }
]
