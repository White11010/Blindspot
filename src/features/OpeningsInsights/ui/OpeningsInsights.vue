<template>
  <div class="d-flex flex-column ga-4 w-100 min-w-0">
    <div
      class="d-flex flex-column flex-md-row align-stretch align-md-center ga-4 justify-space-between"
    >
      <div class="d-flex flex-column flex-sm-row align-sm-center ga-3 min-w-0">
        <span class="text-caption text-medium-emphasis text-no-wrap flex-shrink-0">{{
          t('insightsPage.categoriesLabel')
        }}</span>
        <v-chip-group
          v-model="selectedFilter"
          class="flex-grow-1"
          mandatory
          selected-class="text-secondary"
        >
          <v-chip value="all" color="secondary" size="small" variant="tonal">{{
            t('insightsPage.filterAll')
          }}</v-chip>

          <v-chip value="openings" color="secondary" size="small" variant="tonal">{{
            t('insightsPage.filterOpenings')
          }}</v-chip>
          <v-chip value="time" color="secondary" size="small" variant="tonal">{{
            t('insightsPage.filterTime')
          }}</v-chip>
          <v-chip value="tactics" color="secondary" size="small" variant="tonal">{{
            t('insightsPage.filterTactics')
          }}</v-chip>
          <v-chip value="psychology" color="secondary" size="small" variant="tonal">{{
            t('insightsPage.filterPsychology')
          }}</v-chip>
          <v-chip value="attention" color="warning" size="small" variant="tonal">
            <v-icon icon="mdi-alert-circle-outline" start size="16" />
            {{ t('insightsPage.filterAttention') }}
          </v-chip>
        </v-chip-group>
      </div>
      <v-select
        v-model="sortOrder"
        density="compact"
        hide-details
        :items="sortItems"
        item-title="title"
        item-value="value"
        :label="t('insightsPage.sortByPriority')"
        variant="outlined"
        style="max-width: 280px"
      />
    </div>

    <v-row v-if="heroInsight && heroVisible" dense>
      <v-col cols="12">
        <v-card variant="tonal" color="secondary" class="overflow-hidden position-relative w-100">
          <v-sheet
            class="position-absolute start-0 top-0 bottom-0"
            width="4"
            :color="categoryStripeColor(heroInsight.category)"
            rounded="0"
          />
          <v-card-text class="ps-6 pb-0">
            <div class="d-flex flex-wrap align-center ga-2 mb-3">
              <v-chip color="secondary" size="small" variant="flat">
                <v-icon icon="mdi-star" start size="16" />
                {{ t('insightsPage.featuredBadge') }}
              </v-chip>
              <v-chip
                size="small"
                variant="tonal"
                :color="categoryStripeColor(heroInsight.category)"
              >
                {{ categoryLabel(heroInsight.category) }}
              </v-chip>
            </div>
            <div class="text-h6 font-weight-medium">
              {{ getInsightTitle(heroInsight, t, te) }}
            </div>
          </v-card-text>
          <v-card-text class="ps-6 d-flex flex-column flex-md-row ga-4 pt-0">
            <div class="flex-grow-1">
              <p class="text-body-1 text-medium-emphasis mb-0">
                {{ getInsightSummary(heroInsight, t, te) }}
              </p>
              <p
                v-if="recommendationText(heroInsight)"
                class="text-body-2 text-medium-emphasis mt-3 mb-0"
              >
                {{ recommendationText(heroInsight) }}
              </p>
            </div>
            <div
              v-if="heroInsight.metric_value || heroInsight.metric_number != null"
              class="d-flex flex-column align-md-end flex-shrink-0"
            >
              <span v-if="metricLabel(heroInsight)" class="text-caption text-medium-emphasis">{{
                metricLabel(heroInsight)
              }}</span>
              <span class="text-h3 font-weight-bold text-secondary">{{
                heroInsight.metric_value ?? '—'
              }}</span>
              <MetricDelta
                v-if="heroInsight.metric_number != null"
                :current="heroInsight.metric_number"
                :prev="heroInsight.metric_prev"
                :percent-points="deltaPercentPoints(heroInsight.kind)"
              />
            </div>
          </v-card-text>
          <v-card-actions v-if="canNavigateInsightToMyGames(heroInsight)" class="ps-6 pb-4">
            <v-btn
              variant="text"
              color="primary"
              class="text-none px-0"
              @click="goToMyGamesFromInsight(heroInsight)"
            >
              {{ t('insightsPage.viewGames') }}
              <v-icon icon="mdi-arrow-right" end size="18" />
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-row dense>
      <template v-for="insight in feedInsights" :key="insight.id">
        <v-col cols="12" md="6">
          <v-card class="h-100 overflow-hidden position-relative">
            <v-sheet
              class="position-absolute start-0 top-0 bottom-0"
              width="4"
              :color="categoryStripeColor(insight.category)"
              rounded="0"
            />
            <v-card-item class="ps-6">
              <v-chip
                size="small"
                variant="tonal"
                :color="categoryStripeColor(insight.category)"
                class="mb-2"
              >
                {{ categoryLabel(insight.category) }}
              </v-chip>
              <v-card-title class="text-wrap text-body-1 ps-0 pt-0">
                {{ getInsightTitle(insight, t, te) }}
              </v-card-title>
            </v-card-item>
            <v-card-text class="pt-0 ps-6">
              <div
                v-if="insight.metric_value || insight.metric_number != null"
                class="d-flex flex-wrap align-baseline ga-1"
              >
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
              <p v-if="recommendationText(insight)" class="text-body-2 text-medium-emphasis mb-0">
                {{ recommendationText(insight) }}
              </p>
              <div class="d-flex flex-wrap align-center justify-space-between ga-2 mt-3">
                <span class="text-caption text-medium-emphasis">
                  {{ t('home.insightConfidence', { pct: Math.round(insight.confidence) }) }}
                </span>
                <v-btn
                  v-if="canNavigateInsightToMyGames(insight)"
                  variant="text"
                  size="small"
                  class="text-none"
                  @click="goToMyGamesFromInsight(insight)"
                >
                  {{ t('insightsPage.viewGames') }}
                  <v-icon icon="mdi-arrow-right" end size="16" />
                </v-btn>
              </div>
            </v-card-text>
          </v-card>
        </v-col>
      </template>
    </v-row>
  </div>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { storeToRefs } from 'pinia';
import { computed, watch } from 'vue';
import { useRouter } from 'vue-router';

import { useGamesStore, useMyGamesFiltersStore } from '@/entities/game';
import {
  buildMyGamesFiltersFromInsight,
  canNavigateInsightToMyGames,
  getInsightMetricLabel,
  getInsightRecommendation,
  getInsightSummary,
  getInsightTitle,
  useInsightsFiltersStore,
  useInsightsStore,
} from '@/entities/insight';
import type { Insight, InsightCategory } from '@/entities/insight/model/insight.types';
import { useI18n } from '@/shared/lib/i18n';
import { MetricDelta } from '@/shared/ui';

const { t, te } = useI18n();
const router = useRouter();
const insightsStore = useInsightsStore();
const gamesStore = useGamesStore();
const filtersStore = useMyGamesFiltersStore();
const insightsFiltersStore = useInsightsFiltersStore();
const { heroInsight } = storeToRefs(insightsStore);
// Two-way binding via storeToRefs keeps the toolbar UI and persisted state in sync without manual writes.
const { selectedFilter, sortOrder } = storeToRefs(insightsFiltersStore);

// Mirror state changes into sessionStorage so navigating to another page and back restores the filter.
watch(
  () => insightsFiltersStore.$state,
  () => {
    insightsFiltersStore.persist();
  },
  { deep: true },
);

function goToMyGamesFromInsight(insight: Insight): void {
  filtersStore.reset();
  const patch = buildMyGamesFiltersFromInsight(insight, gamesStore.games);
  filtersStore.$patch(patch);
  filtersStore.persist();
  void router.push({ name: 'MyGames' });
}

const sortItems = computed(() => [
  { value: 'highFirst' as const, title: t('insightsPage.sortPriorityHighFirst') },
  { value: 'lowFirst' as const, title: t('insightsPage.sortPriorityLowFirst') },
]);

const baseList = computed(() => {
  const list = [...insightsStore.items];
  const mult = sortOrder.value === 'highFirst' ? -1 : 1;
  return list.sort((a, b) => mult * (a.sort_priority - b.sort_priority));
});

const categoryFiltered = computed(() => {
  const filter = selectedFilter.value;
  if (filter === 'all') {
    return baseList.value;
  }
  if (filter === 'attention') {
    // "Needs attention" lives on the same axis as categories: restrict to warning/critical severities.
    return baseList.value.filter((i) => i.severity === 'warning' || i.severity === 'critical');
  }
  return baseList.value.filter((i) => i.category === filter);
});

const heroVisible = computed(() => {
  if (!heroInsight.value) {
    return false;
  }
  return categoryFiltered.value.some((i) => i.id === heroInsight.value!.id);
});

const feedInsights = computed(() => {
  const heroId = heroVisible.value && heroInsight.value ? heroInsight.value.id : null;
  return categoryFiltered.value.filter((i) => i.id !== heroId);
});

function categoryStripeColor(c: InsightCategory): string {
  switch (c) {
    case 'openings':
      return 'teal';
    case 'time':
      return 'info';
    case 'tactics':
      return 'warning';
    case 'psychology':
      return 'purple';
    default:
      return 'surface-variant';
  }
}

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
    kind === 'psychology_rest_effect' ||
    kind === 'tactics_conversion_advantage' ||
    kind === 'tactics_late_game_losses' ||
    kind === 'tactics_middlegame_vs_endgame' ||
    kind === 'tactics_side_performance' ||
    kind === 'tactics_accuracy_by_phase' ||
    kind === 'opponent_rating_performance' ||
    kind === 'time_games_per_day_pattern'
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
