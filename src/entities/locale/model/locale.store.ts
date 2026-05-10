// i18n driver store: persists locale code and calls `setI18nLocale` so Vuetify + vue-i18n stay aligned.
import { defineStore } from 'pinia';

import { setI18nLocale, type LocaleCode } from '@/shared/lib/i18n';

import { readStoredLocale, writeStoredLocale } from './localeStorage';

export const useLocaleStore = defineStore('locale', {
  // Single `locale` field restored from localStorage on boot; settings UI and `setLocale` action write through here.
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
