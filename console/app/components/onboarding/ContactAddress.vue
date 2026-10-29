<script setup lang="ts">
const { schema, state } = useContactAddress()
</script>

<template>
    <UForm class="space-y-5" :schema="schema" :state="state">
        <RyderInput v-model="state.addressLineOne" name="addressLineOne" label="Address line 1" placeholder="123 Main Street" />
        <RyderInput v-model="state.addressLineTwo" name="addressLineTwo" label="Address line 2" placeholder="Suite 100" hint="Optional" />
        <div class="grid gap-5 sm:grid-cols-2">
            <RyderInput v-model="state.city" name="city" label="City" placeholder="Lagos" />
            <RyderInput v-model="state.region" name="region" label="Region / State" placeholder="Lagos" hint="Optional" />
        </div>
        <div class="grid gap-5 sm:grid-cols-2">
            <RyderInput v-model="state.country" name="country" label="Country" placeholder="Nigeria" />
            <RyderInput v-model="state.zipCode" name="zipCode" label="Zip code" placeholder="100001" />
        </div>
    </UForm>
</template>
