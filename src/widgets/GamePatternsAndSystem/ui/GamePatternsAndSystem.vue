<template>
  <v-card>
    <v-card-title>Patterns and system connection</v-card-title>
    <v-card-text class="d-flex flex-column ga-4">
      <div>
        <div class="text-subtitle-2 mb-2">Pattern tags</div>
        <div class="d-flex flex-wrap ga-2">
          <v-chip v-for="tag in analysis.pattern_tags" :key="tag" size="small" color="secondary" variant="outlined">
            {{ tag }}
          </v-chip>
          <span v-if="!analysis.pattern_tags.length" class="text-body-2 text-medium-emphasis">
            No pattern tags for this game.
          </span>
        </div>
      </div>

      <div>
        <div class="text-subtitle-2 mb-2">System connection</div>
        <v-alert type="info" variant="tonal">
          {{ analysis.system_connection.text || 'No system-level pattern found yet.' }}
          <div v-if="analysis.system_connection.secondary_text" class="mt-1 text-body-2">
            {{ analysis.system_connection.secondary_text }}
          </div>
        </v-alert>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import type { GameAnalysis } from '@/entities/game-analysis';

defineProps<{
  analysis: GameAnalysis;
}>();
</script>
