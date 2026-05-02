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
  width: 820px;
}
</style>
