import { uniqueOpeningOptions, type Game } from '@/entities/game';
import type { MyGamesFiltersSnapshot } from '@/entities/game/model/myGamesFilters.store';

import type { Insight } from '../model/insight.types';
import { parseInsightPayload, speedLabelFromSubjectKey } from './insightPayload';

export const INSIGHT_LATE_GAME_LOSS_PATTERN = 'late_game_loss';

export type InsightMyGamesFilterPatch = Pick<
  MyGamesFiltersSnapshot,
  'searchText' | 'results' | 'speeds' | 'periods' | 'patternTag' | 'openingValue' | 'playerColors'
>;

function emptyPatch(): InsightMyGamesFilterPatch {
  return {
    searchText: '',
    results: [],
    speeds: [],
    periods: [],
    patternTag: null,
    openingValue: null,
    playerColors: [],
  };
}

export function canNavigateInsightToMyGames(insight: Insight): boolean {
  const k = insight.kind;
  if (
    k.startsWith('opening_') ||
    k === 'time_control_best' ||
    k === 'time_control_worst' ||
    k === 'time_rating_growth_30d' ||
    k === 'tactics_late_game_losses'
  ) {
    return true;
  }
  if (k === 'tactics_side_performance') {
    const { params } = parseInsightPayload(insight.payload_json);
    return typeof params.white_stronger === 'boolean';
  }
  return false;
}

function speedSlugFromLabel(label: string): string | null {
  const x = label.trim().toLowerCase();
  if (x === 'bullet' || x === 'ultrabullet') {
    return 'bullet';
  }
  if (x === 'blitz' || x === 'rapid' || x === 'classical') {
    return x;
  }
  return null;
}

function openingValueFromGames(games: readonly Game[], openingName: string): string | null {
  const opts = uniqueOpeningOptions([...games]);
  const norm = openingName.trim().toLowerCase();
  const matches = opts.filter((o) => {
    const idx = o.value.indexOf('|');
    const namePart = idx >= 0 ? o.value.slice(idx + 1) : o.value;
    return namePart.trim().toLowerCase() === norm;
  });
  if (matches.length === 1) {
    return matches[0].value;
  }
  return null;
}

export function buildMyGamesFiltersFromInsight(
  insight: Insight,
  games: readonly Game[],
): InsightMyGamesFilterPatch {
  if (!canNavigateInsightToMyGames(insight)) {
    return emptyPatch();
  }

  const { subject_key, params } = parseInsightPayload(insight.payload_json);
  const k = insight.kind;

  if (k.startsWith('opening_')) {
    const openingRaw = params.opening;
    const openingName = typeof openingRaw === 'string' ? openingRaw : '';
    if (!openingName.trim()) {
      return emptyPatch();
    }
    const ov = openingValueFromGames(games, openingName);
    if (ov != null) {
      return { ...emptyPatch(), openingValue: ov };
    }
    return { ...emptyPatch(), searchText: openingName };
  }

  if (k === 'time_control_best' || k === 'time_control_worst' || k === 'time_rating_growth_30d') {
    const labelFromParams =
      typeof params.speed_label === 'string'
        ? params.speed_label
        : typeof params.label === 'string'
          ? params.label
          : speedLabelFromSubjectKey(subject_key);
    const slug = labelFromParams ? speedSlugFromLabel(labelFromParams) : null;
    const speeds = slug != null ? [slug] : [];
    const periods = k === 'time_rating_growth_30d' ? (['30'] as MyGamesFiltersSnapshot['periods']) : [];
    return { ...emptyPatch(), speeds, periods };
  }

  if (k === 'tactics_late_game_losses') {
    return {
      ...emptyPatch(),
      results: ['loss'],
      patternTag: INSIGHT_LATE_GAME_LOSS_PATTERN,
    };
  }

  if (k === 'tactics_side_performance') {
    const ws = params.white_stronger;
    if (typeof ws !== 'boolean') {
      return emptyPatch();
    }
    return {
      ...emptyPatch(),
      playerColors: ws ? ['white'] : ['black'],
    };
  }

  return emptyPatch();
}
