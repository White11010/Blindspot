<template>
  <h3>{{ t('insightsPage.title') }}</h3>
  <generate-insights v-if="insightsStore.items.length === 0" />
  <openings-insights v-else />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';

import { useInsightsStore } from '@/entities/insight';
import { GenerateInsights } from '@/features/GenerateInsights';
import { OpeningsInsights } from '@/features/OpeningsInsights';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const insightsStore = useInsightsStore();

onMounted(() => {
  void insightsStore.load();
});
</script>
