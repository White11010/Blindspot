<template>
  <v-btn
    color="primary"
    variant="elevated"
    prepend-icon="mdi-robot"
    :loading="isRunning"
    :disabled="isRunning || !gameId"
    @click="runAnalysis"
  >
    {{ isRunning ? 'Analyzing...' : label }}
  </v-btn>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

import type { GameAnalysis } from '@/entities/game-analysis';

const props = withDefaults(
  defineProps<{
    gameId: string;
    label?: string;
    depth?: number;
  }>(),
  {
    label: 'Run analysis',
    depth: undefined,
  },
);

const emit = defineEmits<{
  done: [analysis: GameAnalysis];
  failed: [message: string];
}>();

const isRunning = ref(false);

async function runAnalysis() {
  if (!props.gameId || isRunning.value) {
    return;
  }

  isRunning.value = true;
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
    isRunning.value = false;
  }
}
</script>
