<template>
  <div class="d-flex flex-column ga-3">
    <v-card :color="insightColor" variant="tonal">
      <v-card-title class="d-flex align-center ga-2">
        <v-icon icon="mdi-lightbulb-on-outline" />
        {{ keyInsightTitle }}
      </v-card-title>
      <v-card-text>{{ keyInsightDescription }}</v-card-text>
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

import {
  getKeyInsightDescription,
  getKeyInsightTitle,
} from '@/entities/game-analysis';
import type { GameAnalysis } from '@/entities/game-analysis';
import { useI18n } from '@/shared/lib/i18n';

const props = defineProps<{
  analysis: GameAnalysis;
}>();

const { t, te } = useI18n();

const keyInsightTitle = computed(() => getKeyInsightTitle(props.analysis, t, te));
const keyInsightDescription = computed(() => getKeyInsightDescription(props.analysis, t, te));

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
  { label: t('analysis.overviewStats.accuracy'), value: `${Math.round(props.analysis.accuracy)}%` },
  { label: t('analysis.overviewStats.acpl'), value: Math.round(props.analysis.avg_centipawn_loss) },
  { label: t('analysis.overviewStats.blunders'), value: props.analysis.blunders },
  { label: t('analysis.overviewStats.mistakes'), value: props.analysis.mistakes },
  { label: t('analysis.overviewStats.inaccuracies'), value: props.analysis.inaccuracies },
  {
    label: t('analysis.overviewStats.maxAdvantage'),
    value: formatPawns(props.analysis.max_advantage_cp),
  },
  {
    label: t('analysis.overviewStats.minAdvantage'),
    value: formatPawns(props.analysis.min_advantage_cp),
  },
]);

function formatPawns(cp: number): string {
  const pawns = cp / 100;
  const sign = pawns > 0 ? '+' : '';
  return `${sign}${pawns.toFixed(1)}`;
}
</script>
