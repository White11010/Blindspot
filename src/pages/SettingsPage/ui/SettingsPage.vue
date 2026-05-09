<template>
  <div class="settings-page">
    <appearance-settings />
    <analysis-settings class="mt-6" />
  </div>
</template>

<script setup lang="ts">
import { AnalysisSettings } from '@/widgets/AnalysisSettings';
import { AppearanceSettings } from '@/widgets/AppearanceSettings';
</script>

<style scoped>
.settings-page {
  padding: 2rem;
  max-width: 58rem;
  flex: 1;
  height: 100%;
}
</style>
