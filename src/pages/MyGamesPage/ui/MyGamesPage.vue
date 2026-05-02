<template>
  <div class="my-games-page">
    <v-card v-if="!gamesStore.games.length" title="Expore your games">
      <v-card-text>
        <v-btn @click="loadPlayerData">Load your games</v-btn>
      </v-card-text>
    </v-card>
    <div v-if="gamesStore.games.length" class="my-games-page__table-wrapper">
      <my-games />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useGamesStore } from '@/entities/game';
import { useUserStore } from '@/entities/user';
import { MyGames } from '@/widgets/MyGames';

const gamesStore = useGamesStore();
const userStore = useUserStore();

async function loadPlayerData(): Promise<void> {
  await userStore.getCurrentUser();
  await gamesStore.sync(userStore.user!.username);
}
</script>

<style scoped lang="scss">
.my-games-page {
  width: 820px;
}
</style>
