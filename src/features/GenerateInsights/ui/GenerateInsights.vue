<template>
  <v-btn :loading="insightsStore.isRefreshing" @click="onClick">{{ t('home.generateInsights') }}</v-btn>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { useQueryClient } from '@tanstack/vue-query';

import { useInsightsStore } from '@/entities/insight';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const insightsStore = useInsightsStore();
const queryClient = useQueryClient();

async function onClick(): Promise<void> {
  await insightsStore.regenerate();
  await queryClient.invalidateQueries({ queryKey: ['insights', 'load'] });
}
</script>
