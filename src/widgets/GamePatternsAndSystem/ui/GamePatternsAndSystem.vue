<template>
  <v-card>
    <v-card-title>{{ t('gameDetails.patternsCardTitle') }}</v-card-title>
    <v-card-text class="d-flex flex-column ga-4">
      <div>
        <div class="text-subtitle-2 mb-2">{{ t('gameDetails.patternTagsLabel') }}</div>
        <div class="d-flex flex-wrap ga-2">
          <v-chip
            v-for="tag in analysis.pattern_tags"
            :key="tag"
            size="small"
            color="secondary"
            variant="outlined"
          >
            {{ formatPatternTagLabel(tag, t, te) }}
          </v-chip>
          <span v-if="!analysis.pattern_tags.length" class="text-body-2 text-medium-emphasis">
            {{ t('gameDetails.noPatternTags') }}
          </span>
        </div>
      </div>

      <div>
        <div class="text-subtitle-2 mb-2">{{ t('gameDetails.systemConnectionLabel') }}</div>
        <v-alert type="info" variant="tonal">
          {{ primaryText }}
          <div v-if="secondaryText" class="mt-1 text-body-2">
            {{ secondaryText }}
          </div>
        </v-alert>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import { computed } from 'vue';

import {
  getSystemConnectionPrimary,
  getSystemConnectionSecondary,
  type GameAnalysis,
} from '@/entities/game-analysis';
import { formatPatternTagLabel } from '@/shared/lib';
import { useI18n } from '@/shared/lib/i18n';

const props = defineProps<{
  analysis: GameAnalysis;
}>();

const { t, te } = useI18n();

function patternLabel(tag: string): string {
  return formatPatternTagLabel(tag, t, te);
}

const primaryText = computed(() =>
  getSystemConnectionPrimary(props.analysis.system_connection, t, te, patternLabel),
);

const secondaryText = computed(
  () => getSystemConnectionSecondary(props.analysis.system_connection, t, te, patternLabel) ?? '',
);
</script>
