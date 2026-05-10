<template>
  <span v-if="delta !== null" class="metric-delta" :class="deltaClass">
    <span class="metric-delta__arrow">{{ arrow }}</span>
    <span class="metric-delta__value">{{ formattedDelta }}</span>
    <span v-if="suffix" class="metric-delta__suffix">{{ suffix }}</span>
  </span>
</template>

<script setup lang="ts">
// Shared presentational component: minimal state, reused across pages for consistent visuals.

import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    current: number | null | undefined;
    prev: number | null | undefined;
    /** When true, delta is shown as integer percentage points (win rate style). */
    percentPoints?: boolean;
  }>(),
  { percentPoints: false },
);

const delta = computed(() => {
  if (props.current == null || props.prev == null) {
    return null;
  }
  return props.current - props.prev;
});

const arrow = computed(() => (delta.value !== null && delta.value >= 0 ? '▲' : '▼'));

const formattedDelta = computed(() => {
  if (delta.value === null) {
    return '';
  }
  const d = props.percentPoints ? Math.round(delta.value) : Math.round(delta.value * 10) / 10;
  const sign = d > 0 ? '+' : '';
  return `${sign}${d}`;
});

const deltaClass = computed(() =>
  delta.value !== null && delta.value >= 0 ? 'metric-delta--up' : 'metric-delta--down',
);

const suffix = computed(() => (props.percentPoints ? ' п.п.' : ''));
</script>

<style scoped lang="scss">
.metric-delta {
  display: inline-flex;
  align-items: baseline;
  gap: 0.15rem;
  margin-left: 0.5rem;
  font-size: 0.95rem;
  font-weight: 600;
  white-space: nowrap;

  &--up {
    color: rgb(var(--v-theme-success));
  }

  &--down {
    color: rgb(var(--v-theme-error));
  }

  &__arrow {
    font-size: 0.75rem;
    opacity: 0.9;
  }

  &__suffix {
    font-size: 0.7rem;
    font-weight: 500;
    opacity: 0.85;
  }
}
</style>
