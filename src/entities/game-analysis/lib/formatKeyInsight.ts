import type { GameAnalysis, KeyMoment } from '../model/gameAnalysis.types';

type TFn = (key: string, ...args: unknown[]) => string;
type TeFn = (key: string) => boolean;

function worstMoment(moments: KeyMoment[]): KeyMoment | null {
  let best: KeyMoment | null = null;
  for (const m of moments) {
    if (m.swing_cp >= 0) {
      continue;
    }
    if (!best || m.swing_cp < best.swing_cp) {
      best = m;
    }
  }
  return best;
}

export function getKeyInsightTitle(analysis: GameAnalysis, t: TFn, te: TeFn): string {
  const kind = analysis.key_insight.kind;
  const key = `analysis.keyInsight.kinds.${kind}.title`;
  if (te(key)) {
    return t(key);
  }
  return t('analysis.keyInsight.fallbackTitle', { kind });
}

export function getKeyInsightDescription(analysis: GameAnalysis, t: TFn, te: TeFn): string {
  const kind = analysis.key_insight.kind;
  const key = `analysis.keyInsight.kinds.${kind}.description`;
  const p: Record<string, string | number> = {};
  if (kind === 'lost_winning_position') {
    p.pawns = (analysis.max_advantage_cp / 100).toFixed(1);
  }
  if (kind === 'critical_moment') {
    const w = worstMoment(analysis.key_moments);
    if (w) {
      p.move = w.move_number;
      p.pawns = ((-w.swing_cp) / 100).toFixed(1);
    }
  }
  if (te(key)) {
    return t(key, p);
  }
  return t('analysis.keyInsight.fallbackDescription', { kind });
}
