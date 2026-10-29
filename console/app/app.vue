<script setup lag="ts">
import { useNuxtApp } from "#app";
const { $viewport } = useNuxtApp();

watch($viewport.breakpoint, (_newBreakPoint, _oldBreakPoint) => {});
</script>

<template>
    <template v-if="$viewport.isLessThan('tablet')">
        <div class="flex h-dvh flex-col justify-center items-center px-4">
            <RyderEmptyState
                icon="heroicons:exclamation-circle"
                title="Unsupported Device"
                description="Please use a desktop, tablet or anything larger than mobile"
            />
        </div>
    </template>

    <template v-else>
        <UApp
            :toaster="{
                position: 'top-right',
                progress: true,
                duration: 2500,
                ui: { base: 'py-2' },
            }"
        >
            <NuxtLayout> <NuxtPage /> </NuxtLayout>
        </UApp>
    </template>
</template>
