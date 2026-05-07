<template>
  <v-card>
    <v-card-text class="py-8">
      <div v-if="state === 'loading'">
        <v-skeleton-loader type="heading, paragraph, actions" />
      </div>

      <div v-else class="d-flex flex-column ga-3">
        <v-alert :type="alertType" variant="tonal" :icon="icon">
          <div class="text-subtitle-1">{{ title }}</div>
          <div v-if="description" class="text-body-2 mt-1">{{ description }}</div>
        </v-alert>

        <div class="d-flex ga-2">
          <slot name="actions" />
        </div>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';

type AnalysisUiState = 'loading' | 'empty' | 'pending' | 'failed' | 'error';

const props = withDefaults(
  defineProps<{
    state: AnalysisUiState;
    message?: string;
  }>(),
  {
    message: '',
  },
);

const title = computed(() => {
  switch (props.state) {
    case 'empty':
      return 'No analysis found for this game';
    case 'pending':
      return 'Analysis is in progress';
    case 'failed':
      return 'Analysis failed';
    case 'error':
      return 'Could not load game analysis';
    default:
      return '';
  }
});

const description = computed(() => {
  if (props.message) {
    return props.message;
  }

  switch (props.state) {
    case 'empty':
      return 'Run analysis to unlock key moments, evaluation chart, and personal patterns.';
    case 'pending':
      return 'You can refresh or run analysis again.';
    case 'failed':
      return 'Try running analysis again.';
    case 'error':
      return 'Please retry in a moment.';
    default:
      return '';
  }
});

const alertType = computed(() => {
  if (props.state === 'failed' || props.state === 'error') {
    return 'error';
  }
  if (props.state === 'pending') {
    return 'info';
  }
  return 'warning';
});

const icon = computed(() => {
  switch (props.state) {
    case 'pending':
      return 'mdi-timer-sand';
    case 'failed':
    case 'error':
      return 'mdi-alert-circle-outline';
    case 'empty':
      return 'mdi-chart-box-outline';
    default:
      return 'mdi-information-outline';
  }
});
</script>
