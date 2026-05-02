<template>
  <div class="home-page">
    <lichess-account-connection />
  </div>
</template>

<script setup lang="ts">
import { LichessAccountConnection } from '@/widgets/LichessAccountConnection';
</script>

<style scoped>
.home-page {
  padding: 2rem;
  max-width: 940px;
  flex: 1;
  height: 100%;
}
</style>
