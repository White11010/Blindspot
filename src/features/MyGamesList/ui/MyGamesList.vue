<template>
    <v-data-table :headers="headers" :items="gamesStore.games" />
</template>

<script setup lang="ts">
import { useGamesStore } from '@/entities/game';

const gamesStore = useGamesStore()
const headers = [
    {
        key: 'speed',
        title: '',
        width: '96px'
    },
    {
        key: 'opponent_name',
        title: 'Opponent'
    },
    {
        key: 'player_result',
        title: 'Result',
        width: '48px'
    },
    {
        key: 'created_at',
        title: 'Date',
        width: '144px'
    },
]
</script>