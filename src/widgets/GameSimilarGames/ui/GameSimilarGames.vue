<template>
  <v-card>
    <v-card-title>{{ t('analysis.similarGamesTitle') }}</v-card-title>
    <v-card-text>
      <v-row>
        <v-col cols="12" md="6">
          <div class="text-subtitle-2 mb-2">{{ t('analysis.similarBroad') }}</div>
          <div class="d-flex flex-wrap ga-2">
            <v-chip
              v-for="id in analysis.similar_games.broad"
              :key="`b-${id}`"
              color="primary"
              variant="outlined"
              size="small"
              @click="goToGame(id)"
            >
              {{ id }}
            </v-chip>
            <span v-if="!analysis.similar_games.broad.length" class="text-body-2 text-medium-emphasis">
              {{ t('analysis.similarNoMatches') }}
            </span>
          </div>
        </v-col>

        <v-col cols="12" md="6">
          <div class="text-subtitle-2 mb-2">{{ t('analysis.similarNarrow') }}</div>
          <div class="d-flex flex-wrap ga-2">
            <v-chip
              v-for="id in analysis.similar_games.narrow"
              :key="`n-${id}`"
              color="secondary"
              variant="outlined"
              size="small"
              @click="goToGame(id)"
            >
              {{ id }}
            </v-chip>
            <span v-if="!analysis.similar_games.narrow.length" class="text-body-2 text-medium-emphasis">
              {{ t('analysis.similarNoMatches') }}
            </span>
          </div>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';

import type { GameAnalysis } from '@/entities/game-analysis';
import { useI18n } from '@/shared/lib/i18n';

defineProps<{
  analysis: GameAnalysis;
}>();

const { t } = useI18n();
const router = useRouter();

function goToGame(gameId: string) {
  router.push(`/game-details/${gameId}`);
}
</script>
