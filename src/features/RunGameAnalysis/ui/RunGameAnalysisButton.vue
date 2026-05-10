<template>
  <v-btn
    color="primary"
    variant="elevated"
    prepend-icon="mdi-robot"
    :loading="isRunning"
    :disabled="isRunning || !gameId"
    :aria-busy="isRunning"
    @click="runAnalysis"
  >
    {{ isRunning ? t('gameDetails.analyzingShort') : labelShown }}
  </v-btn>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { invoke } from '@tauri-apps/api/core';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';

import type { GameAnalysis } from '@/entities/game-analysis';
import { useGameAnalysisRunStore } from '@/entities/game-analysis';
import { useI18n } from '@/shared/lib/i18n';

const props = defineProps<{
  gameId: string;
  label?: string;
  depth?: number;
}>();

const { t } = useI18n();

const labelShown = computed(() => props.label ?? t('gameDetails.runAnalysis'));

const emit = defineEmits<{
  done: [analysis: GameAnalysis];
  failed: [message: string];
}>();

const runStore = useGameAnalysisRunStore();
const { runningGameId } = storeToRefs(runStore);

const isRunning = computed(
  () => Boolean(props.gameId) && runningGameId.value === props.gameId,
);

async function runAnalysis() {
  if (!props.gameId || isRunning.value) {
    return;
  }

  runStore.setRunningGameId(props.gameId);
  try {
    const analysis = await invoke<GameAnalysis>('analyze_game', {
      gameId: props.gameId,
      depth: props.depth,
    });
    emit('done', analysis);
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Failed to run analysis';
    emit('failed', message);
  } finally {
    if (runStore.runningGameId === props.gameId) {
      runStore.setRunningGameId(null);
    }
  }
}
</script>
