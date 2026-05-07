<template>
  <v-container fluid class="game-details-page">
    <v-row class="mb-2">
      <v-col cols="12">
        <div class="d-flex align-center justify-space-between flex-wrap ga-2">
          <h1 class="text-h4">Game analysis</h1>
          <div class="d-flex ga-2">
            <RunGameAnalysisButton
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
              Refresh
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

    <v-row v-if="uiState !== 'ready'">
      <v-col cols="12">
        <GameAnalysisState :state="uiState" :message="stateMessage">
          <template #actions>
            <RunGameAnalysisButton
              :game-id="gameId"
              label="Run analysis now"
              @done="onAnalysisDone"
              @failed="onRunFailed"
            />
            <v-btn variant="text" color="secondary" @click="refetch">Try refresh</v-btn>
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
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useRoute } from 'vue-router';

import { useGameAnalysisQuery, type GameAnalysis } from '@/entities/game-analysis';
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

const route = useRoute();
const externalErrorMessage = ref('');

const gameId = computed(() => String(route.params.id || ''));

const { games } = useSyncGamesQuery();
const game = computed(() => games.value.find((item) => item.id === gameId.value));

const {
  data,
  isLoading,
  isFetching,
  isError,
  error,
  refetch,
} = useGameAnalysisQuery(gameId);

const analysis = computed(() => data.value || null);

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
    return externalErrorMessage.value;
  }
  if (isError.value) {
    return error.value instanceof Error ? error.value.message : 'Unknown loading error';
  }
  return analysis.value?.error || '';
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
}
</style>