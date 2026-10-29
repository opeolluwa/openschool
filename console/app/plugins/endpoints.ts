export default defineNuxtPlugin(() => {
  const onboarding = {
    adminRegistration: "/onboarding/admin",
    academic: "/onboarding/institution/academics",
    contactAddress: "/onboarding/institution/address",
    generalInformation: "/onboarding/institution",
  };
  return {
    provide: { endpoints: { onboarding } },
  };
});
