<template>
  <div class="insights-page">
    <div class="insights-page__container">
      <insights-block />
    </div>
  </div>
</template>

<script setup lang="ts">
import { InsightsBlock } from '@/widgets/InsightsBlock';
</script>

<style lang="scss" scoped>
.insights-page {
  width: 100%;
  flex: 1;
  height: 100%;
  overflow-y: auto;

  &__container {
    padding: 2rem;
    max-width: 58rem;
    margin: 0 auto;
  }
}
</style>
