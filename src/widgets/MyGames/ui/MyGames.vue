<template>
  <div class="my-games">
    <my-games-sync />
    <my-games-section />
    <no-games-found />
  </div>
</template>

<script setup lang="ts">
import { MyGamesSection } from '@/features/MyGamesSection';
import { MyGamesSync } from '@/features/MyGamesSync';
import { NoGamesFound } from '@/features/NoGamesFound';
</script>
