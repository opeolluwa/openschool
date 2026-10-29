export default defineNuxtPlugin(() => {
  const onboarding = {
    adminRegistration: "",
    academic: "",
    contactAddress: "",
    generalInformation: "",
  };
  return {
    provide: { endpoints: { onboarding } },
  };
});
