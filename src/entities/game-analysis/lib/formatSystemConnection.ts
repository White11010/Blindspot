import type { SystemConnection } from '../model/gameAnalysis.types';

type TFn = (key: string, ...args: unknown[]) => string;
type TeFn = (key: string) => boolean;

export function getSystemConnectionPrimary(
  sc: SystemConnection,
  t: TFn,
  te: TeFn,
  patternLabel: (tag: string) => string,
): string {
  const variant = (sc.primary_variant ?? '').trim();
  if (!variant && sc.text) {
    return t('analysis.systemConnection.legacyHint');
  }
  if (variant === 'no_tags') {
    const k = 'analysis.systemConnection.noTags';
    return te(k) ? t(k, { window: sc.window }) : '';
  }
  if (variant === 'similar_low') {
    const k = 'analysis.systemConnection.similarLow';
    return te(k) ? t(k, { pattern: patternLabel(sc.tag), window: sc.window }) : '';
  }
  if (variant === 'similar_high') {
    const k = 'analysis.systemConnection.similarHigh';
    return te(k) ? t(k, { count: sc.count, window: sc.window }) : '';
  }
  return t('analysis.systemConnection.generic');
}

export function getSystemConnectionSecondary(
  sc: SystemConnection,
  t: TFn,
  te: TeFn,
  patternLabel: (tag: string) => string,
): string | null {
  const v = (sc.secondary_variant ?? '').trim();
  if (v === 'revisit') {
    const k = 'analysis.systemConnection.secondaryRevisit';
    return te(k) ? t(k, { pattern: patternLabel(sc.tag), total: sc.secondary_total }) : null;
  }
  if (v === 'win_rate') {
    const k = 'analysis.systemConnection.secondaryWinRate';
    return te(k)
      ? t(k, { total: sc.secondary_total, pct: Math.round(sc.secondary_wr_pct ?? 0) })
      : null;
  }
  if (!v && sc.secondary_text) {
    return null;
  }
  return null;
}
