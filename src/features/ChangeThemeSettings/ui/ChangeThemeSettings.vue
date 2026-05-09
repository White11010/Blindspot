<template>
  <v-container fluid>
    <p class="text-title-medium">{{ t('settings.themeSection') }}</p>
    <v-radio-group v-model="themeMode" inline hide-details>
      <v-radio :label="t('settings.themeLight')" value="light" />
      <v-radio class="ml-2" :label="t('settings.themeDark')" value="dark" />
    </v-radio-group>
  </v-container>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useTheme } from 'vuetify';

import { useI18n } from '@/shared/lib/i18n';

const THEME_KEY = 'app-theme';

const { t } = useI18n();
const theme = useTheme();

const themeMode = ref<'light' | 'dark'>(theme.global.name.value as 'light' | 'dark');

watch(themeMode, (val) => {
  theme.change(val);
  localStorage.setItem(THEME_KEY, val);
});
</script>
