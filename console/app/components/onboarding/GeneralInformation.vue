<script setup lang="ts">
import { institutionTypes } from '~/data/onboarding'

const { schema, state } = useGeneralInformation()
</script>

<template>
    <UForm class="space-y-5" :schema="schema" :state="state">
        <RyderInput v-model="state.name" name="name" label="Institution name" placeholder="Greenfield Academy" />
        <RyderInput v-model="state.tagline" name="tagline" label="Tagline" placeholder="Inspiring the next generation" />
        <RyderInput v-model="state.logoUrl" name="logoUrl" label="Logo URL" placeholder="https://example.com/logo.png" hint="Optional" />
        <RyderSelect v-model="state.institutionType" name="institutionType" label="Institution type" :items="institutionTypes" placeholder="Select type" />
    </UForm>
</template>
