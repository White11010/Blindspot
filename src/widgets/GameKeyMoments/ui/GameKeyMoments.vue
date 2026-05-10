<template>
  <v-card>
    <v-card-title>{{ t('analysis.keyMomentsTitle') }}</v-card-title>
    <v-card-text>
      <v-alert v-if="!analysis.key_moments.length" type="info" variant="tonal">
        {{ t('analysis.keyMomentsEmpty') }}
      </v-alert>
      <v-timeline v-else density="comfortable" side="end" align="start">
        <v-timeline-item
          v-for="moment in analysis.key_moments"
          :key="`${moment.ply}-${moment.kind}`"
          dot-color="secondary"
          size="small"
        >
          <template #opposite>{{ t('analysis.keyMomentOpposite', { move: moment.move_number }) }}</template>
          <div class="text-subtitle-2">{{ headline(moment) }}</div>
          <div class="text-body-2">{{ description(moment) }}</div>
          <div class="text-caption mt-1">
            <span>{{ t('analysis.keyMomentPlayed') }} <strong>{{ moment.move_san }}</strong></span>
            <span v-if="moment.best_move_san" class="ml-1">
              · {{ t('analysis.keyMomentBest') }} {{ moment.best_move_san }}
            </span>
          </div>
          <v-chip class="mt-2" size="small" variant="tonal" color="warning">
            {{ t('analysis.keyMomentSwing', { cp: formatCp(moment.swing_cp) }) }}
          </v-chip>
        </v-timeline-item>
      </v-timeline>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import { getKeyMomentDescription, getKeyMomentHeadline } from '@/entities/game-analysis';
import type { GameAnalysis, KeyMoment } from '@/entities/game-analysis';
import { useI18n } from '@/shared/lib/i18n';

defineProps<{
  analysis: GameAnalysis;
}>();

const { t, te } = useI18n();

function headline(moment: KeyMoment): string {
  return getKeyMomentHeadline(moment, t, te);
}

function description(moment: KeyMoment): string {
  return getKeyMomentDescription(moment, t, te);
}

function formatCp(value: number): string {
  const sign = value > 0 ? '+' : '';
  return `${sign}${value}`;
}
</script>
