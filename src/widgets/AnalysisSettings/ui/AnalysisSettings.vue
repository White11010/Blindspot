<template>
  <div class="analysis-settings">
    <v-card :title="t('settings.analysisCard')">
      <v-card-text>
        <v-switch
          v-model="enabled"
          color="primary"
          hide-details
          inset
          :label="t('settings.backgroundAnalysis')"
        />
        <p class="text-body-2 text-medium-emphasis mt-2 mb-0">
          {{ t('settings.backgroundAnalysisHint') }}
        </p>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import { computed } from 'vue';

import { useAnalysisSettingsStore } from '@/entities/analysis-settings';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const settings = useAnalysisSettingsStore();

const enabled = computed({
  get: () => settings.backgroundAnalysisEnabled,
  set: (v: boolean | null) => settings.setBackgroundAnalysisEnabled(Boolean(v)),
});
</script>
