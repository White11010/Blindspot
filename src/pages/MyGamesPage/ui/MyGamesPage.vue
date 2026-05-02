<template>
  <div class="my-games-page">
    <v-card v-if="!gamesStore.games.length" title="Expore your games" class="h-100">
      <v-card-text class="mt-2 d-flex justify-center">
        <v-btn :loading="gamesStore.loading" @click="loadPlayerData">Load your games</v-btn>
      </v-card-text>
    </v-card>
    <div v-if="!gamesStore.loading && gamesStore.games.length" class="my-games-page__table-wrapper">
      <my-games />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useGamesStore } from '@/entities/game';
import { MyGames } from '@/widgets/MyGames';

const gamesStore = useGamesStore();

async function loadPlayerData(): Promise<void> {
  await gamesStore.sync();
}
</script>

<style scoped lang="scss">
.my-games-page {
  padding: 2rem;
  max-width: 940px;
  flex: 1;
  height: 100%;
}
</style>
