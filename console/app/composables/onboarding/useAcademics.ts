import { reactive } from 'vue'
import * as v from 'valibot'

const schema = v.object({
  calendarName: v.pipe(v.string(), v.trim(), v.minLength(1, 'Calendar name is required.')),
  academicCalendarType: v.pipe(v.string(), v.minLength(1, 'Select a calendar type.')),
  gradingSystemName: v.pipe(v.string(), v.trim(), v.minLength(1, 'Grading system name is required.')),
  gradingSystemType: v.pipe(v.string(), v.minLength(1, 'Select a grading system type.'))
})

type Schema = v.InferOutput<typeof schema>

export function useAcademics() {
  const state = reactive<Schema>({
    calendarName: '',
    academicCalendarType: '',
    gradingSystemName: '',
    gradingSystemType: ''
  })

  const isProcessing = ref(false)
  const { $api, $endpoints } = useNuxtApp()

  const onSubmit = async (req: Schema) => {
    try {
      isProcessing.value = true
      await $api.post($endpoints.onboarding.academic, req)
    } catch (error) {
      throw error
    } finally {
      isProcessing.value = false
    }
  }

  return { schema, state, isProcessing, onSubmit }
}
