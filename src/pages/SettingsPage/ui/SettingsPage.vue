<template>
  <div class="settings-page">
    <appearance-settings />
    <analysis-settings class="mt-6" />
  </div>
</template>

<script setup lang="ts">
// Route-level page: composes features/widgets for this screen and wires page-local queries or navigation.

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
