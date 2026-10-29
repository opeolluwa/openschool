import { reactive } from 'vue'
import * as v from 'valibot'

const schema = v.object({
  name: v.pipe(v.string(), v.trim(), v.minLength(1, 'Institution name is required.')),
  tagline: v.pipe(v.string(), v.trim(), v.minLength(1, 'Tagline is required.')),
  logoUrl: v.optional(v.pipe(v.string(), v.trim())),
  institutionType: v.pipe(v.string(), v.minLength(1, 'Select an institution type.'))
})

type Schema = v.InferOutput<typeof schema>

export function useGeneralInformation() {
  const state = reactive<Schema>({
    name: '',
    tagline: '',
    logoUrl: '',
    institutionType: ''
  })

  const isProcessing = ref(false)
  const { $api, $endpoints } = useNuxtApp()

  const onSubmit = async (req: Schema) => {
    try {
      isProcessing.value = true
      await $api.post($endpoints.onboarding.generalInformation, req)
    } catch (error) {
      throw error
    } finally {
      isProcessing.value = false
    }
  }

  return { schema, state, isProcessing, onSubmit }
}
