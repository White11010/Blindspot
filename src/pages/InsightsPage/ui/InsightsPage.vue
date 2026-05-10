<template>
  <div class="insights-page d-flex flex-column flex-1 h-100 min-h-0 overflow-y-auto overflow-x-hidden pa-5">
    <insights-block />
  </div>
</template>

<script setup lang="ts">
// Route-level page: composes features/widgets for this screen and wires page-local queries or navigation.

import { InsightsBlock } from '@/widgets/InsightsBlock';
</script>

<style scoped>
.insights-page {
  box-sizing: border-box;
  width: 100%;
  border: none;
}
</style>
