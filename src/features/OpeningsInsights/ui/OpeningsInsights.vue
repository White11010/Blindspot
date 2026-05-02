<template>
  <div class="opening-insights__grid">
    <v-card v-for="insight in insightsStore.items" :key="insight.id" :title="insight.title">
      <v-card-text>
        {{ insight.summary }}
      </v-card-text>
      <v-card-text>
        {{ insight.recommendation }}
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { useInsightsStore } from '@/entities/insight';

const insightsStore = useInsightsStore();
</script>

<style lang="scss" scoped>
.opening-insights {
  &__grid {
    margin-top: 1rem;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }
}
</style>
