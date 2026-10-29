<script setup lang="ts">
const { schema, state, isProcessing, onSubmit } = useAdminRegistration();

const formRef = useTemplateRef<{ validate: () => Promise<any> }>("form");
const toast = useToast();

async function submit(): Promise<boolean> {
    try {
        await formRef.value!.validate();
    } catch {
        return false;
    }

    try {
        await onSubmit(state);
        return true;
    } catch {
        toast.add({ title: "Failed to save admin registration.", color: "error" });
        return false;
    }
}

defineExpose({ submit, isProcessing });
</script>

<template>
    <UForm ref="form" class="space-y-5" :schema="schema" :state="state">
        <div class="grid gap-5 sm:grid-cols-2">
            <RyderInput
                v-model="state.firstName"
                name="firstName"
                label="First name"
                placeholder="John"
            />
            <RyderInput
                v-model="state.lastName"
                name="lastName"
                label="Last name"
                placeholder="Doe"
            />
        </div>
        <RyderInput
            v-model="state.email"
            name="email"
            label="Email"
            type="email"
            placeholder="admin@school.edu"
        />
        <RyderPasswordInput
            v-model="state.password"
            name="password"
            label="Password"
        />
    </UForm>
</template>
