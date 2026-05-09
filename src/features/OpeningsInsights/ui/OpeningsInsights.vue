<template>
  <div class="opening-insights">
    <v-chip-group
      v-model="selectedCategory"
      class="opening-insights__chips mb-4"
      mandatory
      selected-class="text-primary"
    >
      <v-chip value="all" size="small" variant="tonal">{{ t('insightsPage.filterAll') }}</v-chip>
      <v-chip value="openings" size="small" variant="tonal">{{ t('insightsPage.filterOpenings') }}</v-chip>
      <v-chip value="time" size="small" variant="tonal">{{ t('insightsPage.filterTime') }}</v-chip>
      <v-chip value="tactics" size="small" variant="tonal">{{ t('insightsPage.filterTactics') }}</v-chip>
      <v-chip value="psychology" size="small" variant="tonal">{{ t('insightsPage.filterPsychology') }}</v-chip>
    </v-chip-group>

    <div class="opening-insights__grid">
      <v-card
        v-for="insight in filteredInsights"
        :key="insight.id"
        :class="['ins-card', `ins-card--${insight.category}`]"
        elevation="1"
      >
        <div class="ins-card__stripe" :data-cat="insight.category" />
        <v-card-item>
          <v-card-title class="text-body-1 d-flex align-center flex-wrap gap-2">
            <span class="ins-card__badge">{{ categoryLabel(insight.category) }}</span>
            {{ getInsightTitle(insight, t, te) }}
          </v-card-title>
        </v-card-item>
        <v-card-text class="pt-0">
          <div v-if="insight.metric_value || insight.metric_number != null" class="ins-card__metric-row">
            <span v-if="metricLabel(insight)" class="text-caption text-medium-emphasis w-100">{{
              metricLabel(insight)
            }}</span>
            <span class="text-h5 font-weight-medium">{{ insight.metric_value ?? '—' }}</span>
            <MetricDelta
              v-if="insight.metric_number != null"
              :current="insight.metric_number"
              :prev="insight.metric_prev"
              :percent-points="deltaPercentPoints(insight.kind)"
            />
          </div>
          <div v-if="deltaCaption(insight)" class="text-caption text-medium-emphasis mt-1">
            {{ deltaCaption(insight) }}
          </div>
          <p class="mt-2 mb-1">{{ getInsightSummary(insight, t, te) }}</p>
          <p v-if="recommendationText(insight)" class="text-body-2 text-medium-emphasis">
            {{ recommendationText(insight) }}
          </p>
        </v-card-text>
      </v-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';

import {
  getInsightMetricLabel,
  getInsightRecommendation,
  getInsightSummary,
  getInsightTitle,
  useInsightsStore,
} from '@/entities/insight';
import type { Insight, InsightCategory } from '@/entities/insight/model/insight.types';
import { useI18n } from '@/shared/lib/i18n';
import { MetricDelta } from '@/shared/ui';

type FilterKey = 'all' | InsightCategory;

const { t, te } = useI18n();
const insightsStore = useInsightsStore();

const selectedCategory = ref<FilterKey>('all');

const sortedAll = computed(() =>
  [...insightsStore.items].sort((a, b) => b.sort_priority - a.sort_priority),
);

const filteredInsights = computed(() => {
  if (selectedCategory.value === 'all') {
    return sortedAll.value;
  }
  return sortedAll.value.filter((i) => i.category === selectedCategory.value);
});

function categoryLabel(c: InsightCategory): string {
  switch (c) {
    case 'openings':
      return t('insightsPage.filterOpenings');
    case 'time':
      return t('insightsPage.filterTime');
    case 'tactics':
      return t('insightsPage.filterTactics');
    case 'psychology':
      return t('insightsPage.filterPsychology');
    default:
      return c;
  }
}

function deltaPercentPoints(kind: string): boolean {
  return (
    kind.startsWith('opening_') ||
    kind.startsWith('time_control_') ||
    kind === 'psychology_tilt' ||
    kind === 'psychology_comeback' ||
    kind === 'tactics_conversion_advantage' ||
    kind === 'tactics_late_game_losses' ||
    kind === 'tactics_middlegame_vs_endgame' ||
    kind === 'tactics_side_performance'
  );
}

function deltaCaption(ins: Insight): string | null {
  if (ins.metric_prev == null || ins.metric_number == null) {
    return null;
  }
  if (ins.kind === 'time_rating_growth_30d') {
    return t('insightsPage.deltaLast30');
  }
  if (ins.kind === 'time_morning_vs_evening') {
    return t('insightsPage.deltaRecentSample');
  }
  return null;
}

function metricLabel(ins: Insight): string | null {
  return getInsightMetricLabel(ins, t, te);
}

function recommendationText(ins: Insight): string | null {
  return getInsightRecommendation(ins, t, te);
}
</script>

<style lang="scss" scoped>
.opening-insights {
  &__grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;

    @media (max-width: 720px) {
      grid-template-columns: 1fr;
    }
  }
}

.ins-card {
  position: relative;
  overflow: hidden;

  &__stripe {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 4px;
    background: var(--ins-stripe, #888);
  }

  &--openings {
    --ins-stripe: #00897b;
  }

  &--time {
    --ins-stripe: #1565c0;
  }

  &--tactics {
    --ins-stripe: #ef6c00;
  }

  &--psychology {
    --ins-stripe: #6a1b9a;
  }

  &__badge {
    display: inline-block;
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    padding: 0.1rem 0.35rem;
    border-radius: 4px;
    background: rgba(var(--v-theme-surface-variant), 0.35);
  }

  &__metric-row {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    gap: 0.25rem;
  }
}
</style>
