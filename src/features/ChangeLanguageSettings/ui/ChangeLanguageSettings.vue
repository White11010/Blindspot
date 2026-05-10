<template>
  <v-container fluid>
    <p class="text-title-medium">{{ t('settings.language.label') }}</p>
    <v-select
      v-model="locale"
      :items="items"
      item-title="title"
      item-value="value"
      density="comfortable"
      variant="outlined"
      hide-details
      class="mt-2"
      style="max-width: 20rem"
    />
  </v-container>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { useLocaleStore } from '@/entities/locale';
import {
  isLocaleCode,
  SUPPORTED_LOCALES,
  useI18n,
  type LocaleCode,
} from '@/shared/lib/i18n';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';

const { t } = useI18n();
const localeStore = useLocaleStore();
const { locale: storedLocale } = storeToRefs(localeStore);

const items = computed(() =>
  SUPPORTED_LOCALES.map((code) => ({
    value: code,
    title:
      code === 'en'
        ? t('settings.language.optionEn')
        : t('settings.language.optionRu'),
  })),
);

const locale = computed({
  get: () => storedLocale.value,
  set: (v: LocaleCode | string) => {
    if (isLocaleCode(v)) {
      localeStore.setLocale(v);
    }
  },
});
</script>
