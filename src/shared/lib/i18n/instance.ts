import { createI18n } from 'vue-i18n';
import { isRef } from 'vue';

import type { LocaleCode } from './config';
import { DEFAULT_LOCALE } from './config';
import en from './locales/en';
import ru from './locales/ru';

export type AppI18n = ReturnType<typeof createI18n>;

let i18nInstance: AppI18n | null = null;

export function createI18nPlugin(initialLocale: LocaleCode = DEFAULT_LOCALE): AppI18n {
  const i18n = createI18n({
    legacy: false,
    locale: initialLocale,
    fallbackLocale: DEFAULT_LOCALE,
    messages: {
      en,
      ru,
    },
  });
  i18nInstance = i18n as AppI18n;
  return i18n as AppI18n;
}

export function setI18nLocale(code: LocaleCode): void {
  if (!i18nInstance) {
    return;
  }
  const { locale } = i18nInstance.global;
  if (isRef(locale)) {
    locale.value = code;
  }
}

export function getI18nInstance(): AppI18n | null {
  return i18nInstance;
}
