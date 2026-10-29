<script setup lang="ts">
import { academicCalendarTypes, gradingSystemTypes } from '~/data/onboarding'

const { schema, state } = useAcademics()
</script>

<template>
    <UForm class="space-y-5" :schema="schema" :state="state">
        <div class="grid gap-5 sm:grid-cols-2">
            <RyderInput v-model="state.calendarName" name="calendarName" label="Calendar name" placeholder="2025/2026 Academic Year" />
            <RyderSelect v-model="state.academicCalendarType" name="academicCalendarType" label="Calendar type" :items="academicCalendarTypes" placeholder="Select type" />
        </div>
        <div class="grid gap-5 sm:grid-cols-2">
            <RyderInput v-model="state.gradingSystemName" name="gradingSystemName" label="Grading system name" placeholder="Standard GPA" />
            <RyderSelect v-model="state.gradingSystemType" name="gradingSystemType" label="Grading system type" :items="gradingSystemTypes" placeholder="Select type" />
        </div>
    </UForm>
</template>
