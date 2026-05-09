import { DEFAULT_LOCALE, isLocaleCode, type LocaleCode } from '@/shared/lib/i18n';

export const LOCALE_STORAGE_KEY = 'app-locale';

export function readStoredLocale(): LocaleCode {
  try {
    const raw = localStorage.getItem(LOCALE_STORAGE_KEY);
    if (raw && isLocaleCode(raw)) {
      return raw;
    }
  } catch {
    /* ignore */
  }
  return DEFAULT_LOCALE;
}

export function writeStoredLocale(code: LocaleCode): void {
  try {
    localStorage.setItem(LOCALE_STORAGE_KEY, code);
  } catch {
    /* ignore */
  }
}
