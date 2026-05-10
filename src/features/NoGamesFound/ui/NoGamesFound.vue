<template>
  <v-card v-if="!gamesStore.games.length && gamesStore.wasLoadedEmptyList" class="h-100">
    <v-card-text class="mt-2 d-flex justify-center">
      <p>{{ t('myGames.noGames.message') }}</p>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { useGamesStore } from '@/entities/game';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const gamesStore = useGamesStore();
</script>
