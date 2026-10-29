<script setup lang="ts">
const { schema, state, isProcessing, onSubmit } = useAdminRegistration();
defineEmits(["onSubmit"]);
</script>

<template>
    <UForm
        class="space-y-5"
        :schema="schema"
        :state="state"
        @submit="$emit('onSubmit')"
    >
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
