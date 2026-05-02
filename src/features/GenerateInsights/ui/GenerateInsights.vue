<template>
  <v-btn :loading="insightsStore.isLoading" @click="onClick">Generate insights</v-btn>
</template>

<script setup lang="ts">
import { useInsightsStore } from '@/entities/insight';

const insightsStore = useInsightsStore();
function onClick(): void {
  insightsStore.regenerate();
}
</script>
