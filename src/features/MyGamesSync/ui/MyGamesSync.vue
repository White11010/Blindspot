<template>
  <v-card v-if="!gamesStore.games.length" :title="t('myGames.sync.title')" class="h-100">
    <v-card-text class="mt-2 d-flex justify-center">
      <v-btn :loading="gamesStore.loading" @click="loadPlayerData">{{
        t('myGames.sync.loadButton')
      }}</v-btn>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { useGamesStore } from '@/entities/game';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();

const gamesStore = useGamesStore();

async function loadPlayerData(): Promise<void> {
  await gamesStore.sync();
}
</script>
