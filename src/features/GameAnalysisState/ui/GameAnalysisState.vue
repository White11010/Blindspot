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
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

const { t } = useI18n();

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
      return t('analysis.stateEmptyTitle');
    case 'pending':
      return t('analysis.statePendingTitle');
    case 'failed':
      return t('analysis.stateFailedTitle');
    case 'error':
      return t('analysis.stateErrorTitle');
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
      return t('analysis.stateEmptyDesc');
    case 'pending':
      return t('analysis.statePendingDesc');
    case 'failed':
      return t('analysis.stateFailedDesc');
    case 'error':
      return t('analysis.stateErrorDesc');
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
