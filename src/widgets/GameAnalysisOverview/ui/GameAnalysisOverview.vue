<template>
  <div class="d-flex flex-column ga-3">
    <v-card :color="insightColor" variant="tonal">
      <v-card-title class="d-flex align-center ga-2">
        <v-icon icon="mdi-lightbulb-on-outline" />
        {{ analysis.key_insight.title }}
      </v-card-title>
      <v-card-text>{{ analysis.key_insight.description }}</v-card-text>
    </v-card>

    <v-row>
      <v-col cols="6" md="3" v-for="item in stats" :key="item.label">
        <v-card>
          <v-card-text>
            <div class="text-caption text-medium-emphasis">{{ item.label }}</div>
            <div class="text-h5 font-weight-bold">{{ item.value }}</div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

import type { GameAnalysis } from '@/entities/game-analysis';

const props = defineProps<{
  analysis: GameAnalysis;
}>();

const insightColor = computed(() => {
  switch (props.analysis.key_insight.severity) {
    case 'high':
      return 'error';
    case 'warning':
      return 'warning';
    case 'good':
      return 'success';
    default:
      return 'info';
  }
});

const stats = computed(() => [
  { label: 'Accuracy', value: `${Math.round(props.analysis.accuracy)}%` },
  { label: 'ACPL', value: Math.round(props.analysis.avg_centipawn_loss) },
  { label: 'Blunders', value: props.analysis.blunders },
  { label: 'Mistakes', value: props.analysis.mistakes },
  { label: 'Inaccuracies', value: props.analysis.inaccuracies },
  { label: 'Max advantage', value: formatPawns(props.analysis.max_advantage_cp) },
  { label: 'Min advantage', value: formatPawns(props.analysis.min_advantage_cp) },
]);

function formatPawns(cp: number): string {
  const pawns = cp / 100;
  const sign = pawns > 0 ? '+' : '';
  return `${sign}${pawns.toFixed(1)}`;
}
</script>
