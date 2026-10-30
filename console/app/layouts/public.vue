<script lang="ts" setup>
import RyderDefault from "@opeolluwa/ryder/layouts/RyderDefault.vue";
</script>
<template>
    <RyderDefault>
        <template #header>
            <HomepageHeader />
        </template>

        <main class="pt-[68px] lg:pt-[72px]">
            <slot />
        </main>
        <template #footer>
             <HomepageFooter /> 
        </template>
    </RyderDefault>
</template>
