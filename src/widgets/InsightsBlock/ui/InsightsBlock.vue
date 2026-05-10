<template>
  <div class="d-flex flex-column ga-6">
    <header>
      <h1 class="text-h5 font-weight-medium mb-1">{{ t('insightsPage.title') }}</h1>
      <p class="text-body-2 text-medium-emphasis mb-0">{{ pageSubtitle }}</p>
    </header>

    <template v-if="showKpiSkeleton">
      <v-row dense>
        <v-col v-for="i in 4" :key="i" cols="12" sm="6" md="3">
          <v-skeleton-loader type="card" class="rounded" />
        </v-col>
      </v-row>
    </template>

    <v-row v-else dense>
      <v-col cols="12" sm="6" md="3">
        <v-card variant="tonal" class="h-100 pa-4 d-flex flex-column">
          <div class="text-overline text-medium-emphasis">{{ t('insightsPage.kpiTotalLabel') }}</div>
          <div class="text-h4 font-weight-bold mt-1">{{ insightsStore.items.length }}</div>
          <div class="text-caption text-medium-emphasis mt-2">
            {{ t('insightsPage.kpiTotalHint', { n: insightsStore.warningInsights.length }) }}
          </div>
        </v-card>
      </v-col>
      <v-col cols="12" sm="6" md="3">
        <v-card variant="tonal" class="h-100 pa-4 d-flex flex-column">
          <div class="text-overline text-medium-emphasis">{{ t('insightsPage.kpiAccuracyLabel') }}</div>
          <div class="text-h4 font-weight-bold mt-1">
            {{ avgAccuracyDisplay }}
          </div>
          <div
            v-if="accuracyDeltaFromBench != null"
            class="text-caption mt-2"
            :class="accuracyDeltaClass"
          >
            {{ accuracyDeltaArrow }}
            {{ t('insightsPage.kpiAccuracyDeltaFromAvg', { n: accuracyDeltaAbs }) }}
          </div>
          <div v-else class="text-caption text-medium-emphasis mt-2">
            {{ avgAccuracySubtext }}
          </div>
        </v-card>
      </v-col>
      <v-col cols="12" sm="6" md="3">
        <v-card variant="tonal" class="h-100 pa-4 d-flex flex-column">
          <div class="text-overline text-medium-emphasis">{{ t('insightsPage.kpiRatingLabel') }}</div>
          <div
            class="text-h4 font-weight-bold mt-1"
            :class="ratingGrowthInsight ? 'text-success' : undefined"
          >
            {{ ratingGrowthDisplay }}
          </div>
          <div class="text-caption text-medium-emphasis mt-2">
            {{ ratingGrowthSubtext }}
          </div>
        </v-card>
      </v-col>
      <v-col cols="12" sm="6" md="3">
        <v-card variant="tonal" class="h-100 pa-4 d-flex flex-column">
          <div class="text-overline text-medium-emphasis">{{ t('insightsPage.kpiBestLabel') }}</div>
          <div class="text-h6 font-weight-bold mt-1">{{ bestAreaLabel }}</div>
          <div class="text-caption text-medium-emphasis mt-2">{{ bestAreaHint }}</div>
        </v-card>
      </v-col>
    </v-row>

    <v-card v-if="insightsStore.items.length === 0" variant="tonal" class="pa-6">
      <p class="text-body-1 text-medium-emphasis mb-4">{{ t('insightsPage.emptyHint') }}</p>
      <generate-insights />
    </v-card>
    <openings-insights v-else />
  </div>
</template>

<script setup lang="ts">
import { keepPreviousData, useQuery } from '@tanstack/vue-query';
import { invoke } from '@tauri-apps/api/core';
import { computed, onMounted, onUnmounted, ref } from 'vue';

import { useGamesStore, useSyncGamesQuery } from '@/entities/game';
import { useInsightsStore } from '@/entities/insight';
import { formatLastGamesSyncLabel } from '@/entities/games-sync/lib/formatLastGamesSync';
import { GenerateInsights } from '@/features/GenerateInsights';
import type { PentagonDto, PlayerProfileChartResponse } from '@/features/HomeProfileChartBlock/model/types';
import { OpeningsInsights } from '@/features/OpeningsInsights';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();

const insightsStore = useInsightsStore();
const gamesStore = useGamesStore();
const gamesQuery = useSyncGamesQuery();

const PROFILE_SPEED = 'rapid' as const;

const profileQuery = useQuery({
  queryKey: ['playerProfileChart', 'insightsKpi', PROFILE_SPEED],
  queryFn: async () =>
    invoke<PlayerProfileChartResponse>('get_player_profile_chart', { speed: PROFILE_SPEED }),
  staleTime: 1000 * 60 * 30,
  placeholderData: keepPreviousData,
});

const nowTick = ref(Date.now());
let tickId: ReturnType<typeof setInterval> | undefined;

onMounted(() => {
  void insightsStore.load();
  tickId = setInterval(() => {
    nowTick.value = Date.now();
  }, 30_000);
});

onUnmounted(() => {
  if (tickId != null) {
    clearInterval(tickId);
  }
});

const showKpiSkeleton = computed(() => gamesQuery.isPending.value);

const gamesCount = computed(() => gamesStore.games.length);

const pageSubtitle = computed(() => {
  const gamesPart = t('insightsPage.basedOnGames', { n: gamesCount.value });
  if (!insightsStore.lastLoadedAt) {
    return `${gamesPart} · ${t('insightsPage.insightsNotLoadedYet')}`;
  }
  const relative = formatLastGamesSyncLabel(t, insightsStore.lastLoadedAt, nowTick.value);
  return `${gamesPart} · ${t('insightsPage.updatedRelative', { relative })}`;
});

const avgAccuracyRounded = computed(() => {
  const vals = gamesStore.games
    .map((g) => g.analysis_accuracy)
    .filter((x): x is number => x != null && !Number.isNaN(x));
  if (vals.length === 0) {
    return null;
  }
  return Math.round(vals.reduce((a, b) => a + b, 0) / vals.length);
});

const benchAccuracyRounded = computed(() => {
  const b = profileQuery.data.value?.benchmark.accuracy;
  if (b == null || Number.isNaN(b)) {
    return null;
  }
  return Math.round(Number(b));
});

const avgAccuracyDisplay = computed(() => {
  if (avgAccuracyRounded.value == null) {
    return String(t('common.emDash'));
  }
  return `${avgAccuracyRounded.value}%`;
});

const accuracyDeltaFromBench = computed(() => {
  if (avgAccuracyRounded.value == null || benchAccuracyRounded.value == null) {
    return null;
  }
  return avgAccuracyRounded.value - benchAccuracyRounded.value;
});

const accuracyDeltaAbs = computed(() => {
  const d = accuracyDeltaFromBench.value;
  if (d == null) {
    return '';
  }
  return String(Math.abs(d));
});

const accuracyDeltaArrow = computed(() => {
  const d = accuracyDeltaFromBench.value;
  if (d == null || d === 0) {
    return '';
  }
  return d > 0 ? '▲' : '▼';
});

const accuracyDeltaClass = computed(() => {
  const d = accuracyDeltaFromBench.value;
  if (d == null || d === 0) {
    return 'text-medium-emphasis';
  }
  return d > 0 ? 'text-success' : 'text-error';
});

const avgAccuracySubtext = computed(() => {
  if (avgAccuracyRounded.value == null) {
    return t('insightsPage.kpiAccuracyNoData');
  }
  if (benchAccuracyRounded.value == null) {
    return String(t('common.emDash'));
  }
  return '';
});

type MetricKey = keyof PentagonDto;

const METRIC_KEYS: MetricKey[] = ['accuracy', 'stability', 'conversion', 'openings', 'endgame'];

const bestAreaFromProfile = computed(() => {
  const payload = profileQuery.data.value;
  const player = payload?.player;
  if (!player) {
    return null;
  }
  let bestKey: MetricKey = 'accuracy';
  let bestVal = -1;
  for (const key of METRIC_KEYS) {
    const raw = key === 'conversion' ? player.conversion : (player[key] as number);
    if (raw == null || Number.isNaN(Number(raw))) {
      continue;
    }
    const v = Number(raw);
    if (v > bestVal) {
      bestVal = v;
      bestKey = key;
    }
  }
  if (bestVal < 0) {
    return null;
  }
  const bench = payload.benchmark;
  const benchRaw = bestKey === 'conversion' ? bench.conversion : (bench[bestKey] as number);
  const benchVal =
    benchRaw != null && !Number.isNaN(Number(benchRaw)) ? Math.round(Number(benchRaw)) : null;
  return {
    key: bestKey,
    player: Math.round(bestVal),
    bench: benchVal,
  };
});

const bestAreaLabel = computed(() => {
  const b = bestAreaFromProfile.value;
  if (!b) {
    return String(t('common.emDash'));
  }
  const key = b.key;
  const label =
    key === 'accuracy'
      ? t('home.profileMetric.accuracy')
      : key === 'stability'
        ? t('home.profileMetric.stability')
        : key === 'conversion'
          ? t('home.profileMetric.conversion')
          : key === 'openings'
            ? t('home.profileMetric.openings')
            : t('home.profileMetric.endgame');
  return label;
});

const bestAreaHint = computed(() => {
  const b = bestAreaFromProfile.value;
  if (!b) {
    return t('insightsPage.kpiBestNoData');
  }
  const benchStr = b.bench != null ? String(b.bench) : String(t('common.emDash'));
  return t('insightsPage.kpiBestHint', { player: b.player, bench: benchStr });
});

const ratingGrowthInsight = computed(() =>
  insightsStore.items.find((i) => i.kind === 'time_rating_growth_30d'),
);

const ratingGrowthDisplay = computed(() => {
  const ins = ratingGrowthInsight.value;
  if (ins?.metric_value) {
    return ins.metric_value;
  }
  return String(t('common.emDash'));
});

const ratingGrowthSubtext = computed(() => {
  if (ratingGrowthInsight.value) {
    return t('insightsPage.kpiRatingHint');
  }
  return t('insightsPage.kpiRatingNoData');
});
</script>
