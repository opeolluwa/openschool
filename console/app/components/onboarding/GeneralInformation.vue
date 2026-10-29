<script setup lang="ts">
import { institutionTypes } from "~/data/onboarding";

const { schema, state, isProcessing, onSubmit } = useGeneralInformation();

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
        toast.add({ title: "Failed to save general information.", color: "error" });
        return false;
    }
}

defineExpose({ submit, isProcessing });
</script>

<template>
    <UForm ref="form" class="space-y-5" :schema="schema" :state="state">
        <RyderInput v-model="state.name" name="name" label="Institution name" placeholder="Greenfield Academy" />
        <RyderInput v-model="state.tagline" name="tagline" label="Tagline" placeholder="Inspiring the next generation" />
        <RyderInput v-model="state.logoUrl" name="logoUrl" label="Logo URL" placeholder="https://example.com/logo.png" hint="Optional" />
        <RyderSelect v-model="state.institutionType" name="institutionType" label="Institution type" :items="institutionTypes" placeholder="Select type" />
    </UForm>
</template>
