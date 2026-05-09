import type { Insight } from '../model/insight.types';

import { mergeInsightDisplayParams, parseInsightPayload } from './insightPayload';

type TFn = (key: string, ...args: unknown[]) => string;
type TeFn = (key: string) => boolean;

function bool(p: Record<string, string | number | boolean>, k: string): boolean {
  return Boolean(p[k]);
}

function localizeSpeedInParams(p: Record<string, string | number | boolean>, t: TFn): void {
  if (typeof p.speed_label === 'string') {
    const s = p.speed_label.toLowerCase();
    const k = s === 'ultrabullet' ? 'bullet' : s;
    if (k === 'bullet' || k === 'blitz' || k === 'rapid' || k === 'classical') {
      p.speed_label = t(`myGames.speed.${k}`);
    }
  }
  if (typeof p.label === 'string') {
    const s = p.label.toLowerCase();
    const k = s === 'ultrabullet' ? 'bullet' : s;
    if (k === 'bullet' || k === 'blitz' || k === 'rapid' || k === 'classical') {
      p.label = t(`myGames.speed.${k}`);
    }
  }
}

function baseParams(insight: Insight, t: TFn): Record<string, string | number | boolean> {
  const { subject_key, params } = parseInsightPayload(insight.payload_json);
  const merged = mergeInsightDisplayParams(subject_key, params);
  localizeSpeedInParams(merged, t);
  return merged;
}

function kindKey(kind: string, part: 'title' | 'summary' | 'recommendation'): string {
  return `insights.kinds.${kind}.${part}`;
}

export function getInsightTitle(insight: Insight, t: TFn, te: TeFn): string {
  const p = baseParams(insight, t);
  const k = insight.kind;
  if (k === 'tactics_side_performance') {
    const key = bool(p, 'white_stronger')
      ? 'insights.kinds.tactics_side_performance.titleWhiteStronger'
      : 'insights.kinds.tactics_side_performance.titleBlackStronger';
    if (te(key)) {
      return t(key);
    }
  }
  if (k === 'psychology_streak') {
    const key = bool(p, 'streak_win')
      ? 'insights.kinds.psychology_streak.titleWins'
      : 'insights.kinds.psychology_streak.titleLosses';
    if (te(key)) {
      return t(key);
    }
  }
  const key = kindKey(k, 'title');
  if (te(key)) {
    return t(key, p);
  }
  return t('insights.fallback.title', { kind: k });
}

export function getInsightSummary(insight: Insight, t: TFn, te: TeFn): string {
  const p = baseParams(insight, t);
  const k = insight.kind;
  if (k === 'time_morning_vs_evening') {
    const morningBetter = bool(p, 'morning_better');
    const better = morningBetter ? t('insights.slots.morning') : t('insights.slots.evening');
    const worse = morningBetter ? t('insights.slots.evening') : t('insights.slots.morning');
    p.better = better;
    p.worse = worse;
    const key = morningBetter
      ? 'insights.kinds.time_morning_vs_evening.summaryMorningBetter'
      : 'insights.kinds.time_morning_vs_evening.summaryEveningBetter';
    if (te(key)) {
      return t(key, p);
    }
  }
  const key = kindKey(k, 'summary');
  if (te(key)) {
    return t(key, p);
  }
  return t('insights.fallback.summary', { kind: k });
}

export function getInsightRecommendation(insight: Insight, t: TFn, te: TeFn): string | null {
  const p = baseParams(insight, t);
  const k = insight.kind;
  if (k === 'tactics_side_performance') {
    const key = bool(p, 'white_stronger')
      ? 'insights.kinds.tactics_side_performance.recommendationWhite'
      : 'insights.kinds.tactics_side_performance.recommendationBlack';
    if (te(key)) {
      return t(key, p);
    }
  }
  if (k === 'psychology_streak') {
    const key = bool(p, 'streak_win')
      ? 'insights.kinds.psychology_streak.recommendationWins'
      : 'insights.kinds.psychology_streak.recommendationLosses';
    if (te(key)) {
      return t(key, p);
    }
  }
  const key = kindKey(k, 'recommendation');
  if (te(key)) {
    return t(key, p);
  }
  return null;
}

export function getInsightMetricLabel(insight: Insight, t: TFn, te: TeFn): string | null {
  const k = insight.kind;
  const map: Record<string, string> = {
    opening_best: 'insights.metrics.winrate',
    opening_worst_frequent: 'insights.metrics.winrate',
    opening_rare_gem: 'insights.metrics.winrate',
    opening_dependency: 'insights.metrics.gameShare',
    time_control_best: 'insights.metrics.winrate',
    time_control_worst: 'insights.metrics.winrate',
    time_rating_growth_30d: 'insights.metrics.ratingDelta',
    time_morning_vs_evening: 'insights.metrics.winrateBestSlot',
    tactics_late_game_losses: 'insights.metrics.shareOfGames',
    tactics_side_performance: 'insights.metrics.winrateGapPp',
    tactics_middlegame_vs_endgame: 'insights.metrics.endgameErrorShare',
    tactics_blunder_streak: 'insights.metrics.gamesInRow',
    tactics_conversion_advantage: 'insights.metrics.failedToWinPct',
    psychology_tilt: 'insights.metrics.winrate',
    psychology_comeback: 'insights.metrics.winrate',
    psychology_streak: 'insights.metrics.streakLength',
  };
  const labelKey = map[k];
  if (labelKey && te(labelKey)) {
    return t(labelKey);
  }
  return null;
}
