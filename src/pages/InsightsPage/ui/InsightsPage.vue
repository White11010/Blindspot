<template>
  <div class="insights-page d-flex flex-column flex-1 h-100 min-h-0 overflow-y-auto overflow-x-hidden pa-5">
    <insights-block />
  </div>
</template>

<script setup lang="ts">
import { InsightsBlock } from '@/widgets/InsightsBlock';
</script>

<style scoped>
.insights-page {
  box-sizing: border-box;
  width: 100%;
  border: none;
}
</style>
