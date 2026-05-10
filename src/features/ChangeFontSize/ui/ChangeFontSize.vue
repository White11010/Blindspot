<template>
  <v-container fluid>
    <p class="text-title-medium">{{ t('settings.fontSizeSection') }}</p>
    <v-radio-group v-model="fontMode" inline hide-details>
      <v-radio :label="t('settings.fontNormal')" value="normal" />
      <v-radio class="ml-2" :label="t('settings.fontLarge')" value="large" />
    </v-radio-group>
  </v-container>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { onMounted, ref, watch } from 'vue';

import { useI18n } from '@/shared/lib/i18n';

const FONT_KEY = 'app-font-size';

const { t } = useI18n();

type FontMode = 'normal' | 'large';

const fontMode = ref<FontMode>('normal');

function applyFont(mode: FontMode): void {
  const scale = mode === 'large' ? 1.25 : 1;
  document.documentElement.style.setProperty('--app-font-scale', String(scale));
}

onMounted(() => {
  const saved = localStorage.getItem(FONT_KEY) as FontMode | null;

  fontMode.value = saved === 'large' ? 'large' : 'normal';

  applyFont(fontMode.value);
});

watch(fontMode, (val) => {
  applyFont(val);
  localStorage.setItem(FONT_KEY, val);
});
</script>
