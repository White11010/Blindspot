import type { LocaleCode } from '@/shared/lib/i18n/config';

function toLocaleCode(locale: LocaleCode | string | undefined): LocaleCode {
  return locale === 'ru' ? 'ru' : 'en';
}

type FormatOptions = {
  withRelativeDays?: boolean;
  /** App locale (`en` / `ru`); drives `Intl` and relative phrases. */
  locale?: LocaleCode | string;
};

const DAY_MS = 86_400_000;

function intlTag(code: LocaleCode): string {
  return code === 'ru' ? 'ru-RU' : 'en-GB';
}

function startOfLocalDayMs(timestampMs: number): number {
  const d = new Date(timestampMs);
  return new Date(d.getFullYear(), d.getMonth(), d.getDate()).getTime();
}

/**
 * My Games table: localized; no time; relative for the last 7 calendar days;
 * year after month if not current year.
 */
export function formatMyGamesTableDate(
  createdAtMs: number,
  locale: LocaleCode | string = 'en',
): string {
  const code = toLocaleCode(locale);
  const gameDay = startOfLocalDayMs(createdAtMs);
  const nowDay = startOfLocalDayMs(Date.now());
  const diffDays = Math.round((nowDay - gameDay) / DAY_MS);

  if (diffDays >= 0 && diffDays < 7) {
    return new Intl.RelativeTimeFormat(intlTag(code), { numeric: 'auto' }).format(
      -diffDays,
      'day',
    );
  }

  const d = new Date(createdAtMs);
  const sameCalendarYear = d.getFullYear() === new Date().getFullYear();

  return new Intl.DateTimeFormat(intlTag(code), {
    day: 'numeric',
    month: 'short',
    ...(sameCalendarYear ? {} : { year: 'numeric' as const }),
  }).format(d);
}

function isSameDay(d1: Date, d2: Date): boolean {
  return (
    d1.getFullYear() === d2.getFullYear() &&
    d1.getMonth() === d2.getMonth() &&
    d1.getDate() === d2.getDate()
  );
}

function formatTimePortion(date: Date, locale: LocaleCode): string {
  if (locale === 'ru') {
    return date.toLocaleTimeString('ru-RU', {
      hour: 'numeric',
      minute: '2-digit',
      hour12: false,
    });
  }
  return date.toLocaleTimeString('en-US', {
    hour: 'numeric',
    minute: '2-digit',
    hour12: true,
  });
}

export function formatTimestamp(
  timestamp: number | string | Date,
  options: FormatOptions = {},
): string {
  const { withRelativeDays = false, locale: localeOpt = 'en' } = options;
  const locale = toLocaleCode(localeOpt);

  const date = new Date(timestamp);
  const now = new Date();

  const yesterday = new Date(now);
  yesterday.setDate(now.getDate() - 1);

  const time = formatTimePortion(date, locale);

  if (withRelativeDays) {
    if (isSameDay(date, now)) {
      return time;
    }
    if (isSameDay(date, yesterday)) {
      const relative = new Intl.RelativeTimeFormat(intlTag(locale), { numeric: 'auto' }).format(
        -1,
        'day',
      );
      return `${relative} ${time}`;
    }
  }

  const datePart = new Intl.DateTimeFormat(intlTag(locale), {
    day: 'numeric',
    month: 'short',
  }).format(date);

  return `${datePart} ${time}`;
}
