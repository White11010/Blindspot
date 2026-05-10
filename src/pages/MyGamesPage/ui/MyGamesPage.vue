<template>
  <div class="my-games-page">
    <div class="my-games-page__container">
      <my-games />
    </div>
  </div>
</template>

<script setup lang="ts">
// Route-level page: composes features/widgets for this screen and wires page-local queries or navigation.

import { MyGames } from '@/widgets/MyGames';
</script>

<style scoped lang="scss">
.my-games-page {
  width: 100%;
  flex: 1;
  height: 100%;
  overflow-y: auto;

  &__container {
    padding: 2rem;
    max-width: 110rem;
    margin: 0 auto;
  }
}
</style>
