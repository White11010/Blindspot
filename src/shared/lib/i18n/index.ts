export type { LocaleCode } from './config';
export { DEFAULT_LOCALE, isLocaleCode, SUPPORTED_LOCALES } from './config';
export { createI18nPlugin, getI18nInstance, setI18nLocale } from './instance';
export type { AppI18n } from './instance';

export { useI18n } from 'vue-i18n';
