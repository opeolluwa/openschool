<script lang="ts" setup>
import RyderDefault from "@opeolluwa/ryder/layouts/RyderDefault.vue";
</script>
<template>
    <RyderDefault>
        <template #header>
            <HomepageHeader />
        </template>

        <slot />
        <template #footer>
             <HomepageFooter /> 
        </template>
    </RyderDefault>
</template>
