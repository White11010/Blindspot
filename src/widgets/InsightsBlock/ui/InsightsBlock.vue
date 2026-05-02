<template>
  <h3>Explore your personal insights</h3>
  <!-- <v-card :title="'Explore your personal insights'"> -->
  <generate-insights v-if="insightsStore.items.length === 0" />
  <openings-insights v-else />
  <!-- </v-card> -->
</template>

<script setup lang="ts">
import { useInsightsStore } from '@/entities/insight';
import { GenerateInsights } from '@/features/GenerateInsights';
import { OpeningsInsights } from '@/features/OpeningsInsights';

const insightsStore = useInsightsStore();
</script>
