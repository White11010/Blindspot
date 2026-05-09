import { defineStore } from 'pinia';

import { setI18nLocale, type LocaleCode } from '@/shared/lib/i18n';

import { readStoredLocale, writeStoredLocale } from './localeStorage';

export const useLocaleStore = defineStore('locale', {
  state: () => ({
    locale: readStoredLocale(),
  }),

  actions: {
    setLocale(code: LocaleCode) {
      this.locale = code;
      writeStoredLocale(code);
      setI18nLocale(code);
    },
  },
});
