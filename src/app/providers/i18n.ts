import type { LocaleCode } from '@/shared/lib/i18n';
import { createI18nPlugin } from '@/shared/lib/i18n';

export function createAppI18n(initialLocale: LocaleCode) {
  return createI18nPlugin(initialLocale);
}
