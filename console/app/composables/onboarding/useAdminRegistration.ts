import { reactive } from "vue";
import * as v from "valibot";

const schema = v.object({
  firstName: v.pipe(v.string(), v.trim(), v.minLength(1, "First name is required.")),
  lastName: v.pipe(v.string(), v.trim(), v.minLength(1, "Last name is required.")),
  email: v.pipe(
    v.string(),
    v.trim(),
    v.minLength(1, "Email is required."),
    v.email("Enter a valid email address."),
  ),
  password: v.pipe(
    v.string(),
    v.minLength(8, "Password must be at least 8 characters."),
  ),
});

type Schema = v.InferOutput<typeof schema>;

export function useAdminRegistration() {
  const state = reactive<Schema>({
    firstName: "",
    lastName: "",
    email: "",
    password: "",
  });

  const isProcessing = ref(false);
  const { $api, $endpoints } = useNuxtApp();

  const onSubmit = async (req: Schema) => {
    try {
      isProcessing.value = true;
      // $api.post($endpoints.onboarding.adminRegistration);
      console.log(req);
    } catch {
    } finally {
      isProcessing.value = false;
    }
  };
  return { schema, state, isProcessing, onSubmit };
}
