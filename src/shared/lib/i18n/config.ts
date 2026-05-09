export type LocaleCode = 'en' | 'ru';

export const DEFAULT_LOCALE: LocaleCode = 'en';

export const SUPPORTED_LOCALES: LocaleCode[] = ['en', 'ru'];

export function isLocaleCode(value: string): value is LocaleCode {
  return value === 'en' || value === 'ru';
}
