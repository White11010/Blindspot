<template>
  <div class="game-details-page">
    <div class="game-details-page__container">
      <v-container fluid>
        <v-row class="mb-2">
          <v-col cols="12">
            <div class="d-flex align-center justify-space-between flex-wrap ga-2">
              <h1 class="text-h4">{{ t('gameDetails.pageTitle') }}</h1>
              <div class="d-flex ga-2">
                <RunGameAnalysisButton
                  v-if="!showAnalysisProgressUi"
                  :game-id="gameId"
                  @done="onAnalysisDone"
                  @failed="onRunFailed"
                />
                <v-btn
                  variant="tonal"
                  color="secondary"
                  prepend-icon="mdi-refresh"
                  :loading="isFetching"
                  @click="refetch"
                >
                  {{ t('gameDetails.refresh') }}
                </v-btn>
              </div>
            </div>
          </v-col>
        </v-row>

        <v-row v-if="game">
          <v-col cols="12">
            <GameDetailsHero :game="game" />
          </v-col>
        </v-row>

        <v-row v-if="showAnalysisProgressUi">
          <v-col cols="12">
            <v-card>
              <v-card-text class="py-10 d-flex flex-column align-center text-center ga-4">
                <v-progress-circular indeterminate color="primary" size="48" width="4" />
                <div>
                  <p class="text-h6 mb-1">{{ analysisProgressTitle }}</p>
                  <p class="text-body-2 text-medium-emphasis mb-0">
                    {{ analysisProgressSubtitle }}
                  </p>
                </div>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>

        <v-row v-else-if="uiState !== 'ready'">
          <v-col cols="12">
            <GameAnalysisState :state="uiState" :message="stateMessage">
              <template #actions>
                <RunGameAnalysisButton
                  v-if="!showAnalysisProgressUi"
                  :game-id="gameId"
                  :label="t('gameDetails.runAnalysisNow')"
                  @done="onAnalysisDone"
                  @failed="onRunFailed"
                />
                <v-btn variant="text" color="secondary" @click="refetch">{{
                  t('gameDetails.tryRefresh')
                }}</v-btn>
              </template>
            </GameAnalysisState>
          </v-col>
        </v-row>

        <template v-else-if="analysis">
          <v-row>
            <v-col cols="12">
              <GameAnalysisOverview :analysis="analysis" />
            </v-col>
          </v-row>
          <v-row>
            <v-col cols="12" md="8">
              <GameEvalHistory :analysis="analysis" />
            </v-col>
            <v-col cols="12" md="4">
              <GamePatternsAndSystem :analysis="analysis" />
            </v-col>
          </v-row>
          <v-row>
            <v-col cols="12" md="8">
              <GameKeyMoments :analysis="analysis" />
            </v-col>
            <v-col cols="12" md="4">
              <GameSimilarGames :analysis="analysis" />
            </v-col>
          </v-row>
        </template>
      </v-container>
    </div>
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from '@/shared/lib/i18n';
import { useRoute } from 'vue-router';

import { useAnalysisSettingsStore } from '@/entities/analysis-settings';
import {
  useGameAnalysisQuery,
  useGameAnalysisRunStore,
  type GameAnalysis,
} from '@/entities/game-analysis';
import { useSyncGamesQuery } from '@/entities/game';
import { GameAnalysisState } from '@/features/GameAnalysisState';
import { RunGameAnalysisButton } from '@/features/RunGameAnalysis';
import { GameAnalysisOverview } from '@/widgets/GameAnalysisOverview';
import { GameDetailsHero } from '@/widgets/GameDetailsHero';
import { GameEvalHistory } from '@/widgets/GameEvalHistory';
import { GameKeyMoments } from '@/widgets/GameKeyMoments';
import { GamePatternsAndSystem } from '@/widgets/GamePatternsAndSystem';
import { GameSimilarGames } from '@/widgets/GameSimilarGames';

type UiState = 'loading' | 'empty' | 'pending' | 'failed' | 'error' | 'ready';

const { t } = useI18n();
const route = useRoute();
const externalErrorMessage = ref('');

const gameId = computed(() => String(route.params.id || ''));

const { games } = useSyncGamesQuery();
const game = computed(() => games.value.find((item) => item.id === gameId.value));

const { data, isLoading, isFetching, isError, refetch } = useGameAnalysisQuery(gameId);

const analysis = computed(() => data.value || null);

const { backgroundAnalysisEnabled } = storeToRefs(useAnalysisSettingsStore());
const { runningGameId } = storeToRefs(useGameAnalysisRunStore());

/** Same idea as MyGames list loaders: bg analysis on and no finished analysis row yet (or still pending). */
const lacksCompletedAnalysis = computed(() => {
  const a = analysis.value;
  if (a?.status === 'done') {
    return false;
  }
  if (a?.status === 'failed') {
    return false;
  }
  return true;
});

const showAnalysisProgressUi = computed(
  () =>
    Boolean(gameId.value) &&
    backgroundAnalysisEnabled.value &&
    !isLoading.value &&
    lacksCompletedAnalysis.value,
);

const isEngineWorkingOnThisGame = computed(
  () => Boolean(gameId.value) && runningGameId.value === gameId.value,
);

const analysisProgressTitle = computed(() =>
  isEngineWorkingOnThisGame.value
    ? t('gameDetails.progressInProgress')
    : t('gameDetails.progressQueued'),
);

const analysisProgressSubtitle = computed(() =>
  isEngineWorkingOnThisGame.value
    ? t('gameDetails.progressInProgressHint')
    : t('gameDetails.progressQueuedHint'),
);

const uiState = computed<UiState>(() => {
  if (isLoading.value && !analysis.value) {
    return 'loading';
  }
  if (isError.value) {
    return 'error';
  }
  if (!analysis.value) {
    return 'empty';
  }
  if (analysis.value.status === 'pending') {
    return 'pending';
  }
  if (analysis.value.status === 'failed') {
    return 'failed';
  }
  return 'ready';
});

const stateMessage = computed(() => {
  if (externalErrorMessage.value) {
    return t('errors.generic');
  }
  if (isError.value) {
    return t('errors.unknownLoad');
  }
  return analysis.value?.error ? t('errors.generic') : '';
});

function onAnalysisDone(_nextAnalysis: GameAnalysis) {
  externalErrorMessage.value = '';
  refetch();
}

function onRunFailed(message: string) {
  externalErrorMessage.value = message;
}
</script>

<style scoped>
.game-details-page {
  padding: 1.25rem;

  width: 100%;
  flex: 1;
  height: 100%;
  overflow-y: auto;

  &__container {
    padding: 2rem;
    max-width: 100rem;
    margin: 0 auto;
  }
}
</style>
