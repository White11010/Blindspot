<template>
  <v-card>
    <v-card-title>Key moments</v-card-title>
    <v-card-text>
      <v-alert v-if="!analysis.key_moments.length" type="info" variant="tonal">
        No key moments detected for this game.
      </v-alert>
      <v-timeline v-else density="comfortable" side="end" align="start">
        <v-timeline-item
          v-for="moment in analysis.key_moments"
          :key="`${moment.ply}-${moment.kind}`"
          dot-color="secondary"
          size="small"
        >
          <template #opposite>Move {{ moment.move_number }}</template>
          <div class="text-subtitle-2">{{ moment.headline }}</div>
          <div class="text-body-2">{{ moment.description }}</div>
          <div class="text-caption mt-1">
            Played: <strong>{{ moment.move_san }}</strong>
            <span v-if="moment.best_move_san"> • Best: {{ moment.best_move_san }}</span>
          </div>
          <v-chip class="mt-2" size="small" variant="tonal" color="warning">
            Swing {{ formatCp(moment.swing_cp) }}
          </v-chip>
        </v-timeline-item>
      </v-timeline>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import type { GameAnalysis } from '@/entities/game-analysis';

defineProps<{
  analysis: GameAnalysis;
}>();

function formatCp(value: number): string {
  const sign = value > 0 ? '+' : '';
  return `${sign}${value} cp`;
}
</script>
