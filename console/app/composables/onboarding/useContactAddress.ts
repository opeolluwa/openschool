import { reactive } from 'vue'
import * as v from 'valibot'

const schema = v.object({
  addressLineOne: v.pipe(v.string(), v.trim(), v.minLength(1, 'Address is required.')),
  addressLineTwo: v.optional(v.pipe(v.string(), v.trim())),
  city: v.pipe(v.string(), v.trim(), v.minLength(1, 'City is required.')),
  region: v.optional(v.pipe(v.string(), v.trim())),
  country: v.pipe(v.string(), v.trim(), v.minLength(1, 'Country is required.')),
  zipCode: v.pipe(v.string(), v.trim(), v.minLength(1, 'Zip code is required.'))
})

type Schema = v.InferOutput<typeof schema>

export function useContactAddress() {
  const state = reactive<Schema>({
    addressLineOne: '',
    addressLineTwo: '',
    city: '',
    region: '',
    country: '',
    zipCode: ''
  })

  const isProcessing = ref(false)
  const { $api, $endpoints } = useNuxtApp()

  const onSubmit = async (req: Schema) => {
    try {
      isProcessing.value = true
      await $api.post($endpoints.onboarding.contactAddress, req)
    } catch (error) {
      throw error
    } finally {
      isProcessing.value = false
    }
  }

  return { schema, state, isProcessing, onSubmit }
}
