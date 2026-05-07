<template>
  <v-data-table v-if="games.length" :headers="headers" :items="games" class="my-games-list">
    <template #item.watchDetails="{ item }">
      <v-btn variant="plain" color="secondary" icon="mdi-chart-timeline-variant-shimmer" @click="onDetailsButtonClick(item.id)"></v-btn>
    </template>
    <template #item.analize="{ item }">
      <v-btn variant="plain" color="primary" icon="mdi-open-in-new"></v-btn>
    </template>
  </v-data-table>
</template>

<script setup lang="ts">
import { useSyncGamesQuery } from '@/entities/game';
import { useRouter } from 'vuetify/lib/composables/router.mjs';

const router = useRouter()

const { games } = useSyncGamesQuery();

const headers = [
  {
    key: 'speed',
    title: '',
    width: '96px',
  },
  {
    key: 'opponent_name',
    title: 'Opponent',
  },
  {
    key: 'player_result',
    title: 'Result',
    width: '48px',
  },
  {
    key: 'created_at',
    title: 'Date',
    width: '144px',
  },
  {
    key: 'watchDetails',
    title: '',
    width: '48px'
  },
  {
    key: 'analize',
    title: '',
    width: '48px'
  }
];

function onDetailsButtonClick (id: string) {
  router?.push(`/game-details/${id}`)
}
</script>

<style lang="scss" scoped>
.my-games-list {
  :deep(td) {
    height: 64px;
  }
}
</style>