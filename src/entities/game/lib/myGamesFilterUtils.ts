import { normalizePatternTagId } from '@/shared/lib/patternTags';

import type { Game, MyGamesPeriod } from '../model/games.types';

export { normalizePatternTagId };

const DAY_MS = 86_400_000;

/** `null` = no cutoff (all time). */
export function periodCutoffMs(periods: MyGamesPeriod[]): number | null {
  if (!periods.length || periods.includes('all')) {
    return null;
  }
  const days = periods
    .filter((p): p is Exclude<MyGamesPeriod, 'all'> => p !== 'all')
    .map((p) => Number(p));
  const maxDays = Math.max(...days);
  return Date.now() - maxDays * DAY_MS;
}

function speedMatchesFilter(gameSpeed: string, selected: string[]): boolean {
  if (!selected.length) {
    return true;
  }
  const g = gameSpeed.toLowerCase();
  return selected.some((s) => {
    const x = s.toLowerCase();
    if (x === 'bullet') {
      return g === 'bullet' || g === 'ultrabullet';
    }
    return g === x;
  });
}

function openingKey(game: Game): string {
  return `${game.opening_eco ?? ''}|${game.opening_name ?? ''}`;
}

export function filterMyGames(
  games: Game[],
  opts: {
    searchText: string;
    results: Array<'win' | 'loss' | 'draw'>;
    speeds: string[];
    periods: MyGamesPeriod[];
    patternTag: string | null;
    openingValue: string | null;
  },
): Game[] {
  const q = opts.searchText.trim().toLowerCase();
  const cutoff = periodCutoffMs(opts.periods);

  return games.filter((g) => {
    if (cutoff !== null && g.created_at < cutoff) {
      return false;
    }
    if (opts.results.length && !opts.results.includes(g.player_result)) {
      return false;
    }
    if (!speedMatchesFilter(g.speed, opts.speeds)) {
      return false;
    }
    if (opts.patternTag) {
      const want = normalizePatternTagId(opts.patternTag);
      const tags = (g.pattern_tags ?? []).map((t) => normalizePatternTagId(t));
      if (!tags.includes(want)) {
        return false;
      }
    }
    if (opts.openingValue && openingKey(g) !== opts.openingValue) {
      return false;
    }
    if (q) {
      const opp = (g.opponent_name ?? '').toLowerCase();
      const opName = (g.opening_name ?? '').toLowerCase();
      const opEco = (g.opening_eco ?? '').toLowerCase();
      if (!opp.includes(q) && !opName.includes(q) && !opEco.includes(q)) {
        return false;
      }
    }
    return true;
  });
}

export function uniquePatternTags(games: Game[]): string[] {
  const set = new Set<string>();
  for (const g of games) {
    for (const t of g.pattern_tags ?? []) {
      set.add(t);
    }
  }
  return [...set].sort((a, b) => a.localeCompare(b));
}

export type OpeningOption = { value: string; title: string };

export type PatternOption = { value: string; title: string };

export function patternTagOptions(games: Game[]): PatternOption[] {
  return uniquePatternTags(games).map((tag) => ({
    value: tag,
    title: tag.replace(/_/g, ' '),
  }));
}

export function uniqueOpeningOptions(games: Game[]): OpeningOption[] {
  const map = new Map<string, OpeningOption>();
  for (const g of games) {
    if (!g.opening_name && !g.opening_eco) {
      continue;
    }
    const value = openingKey(g);
    if (map.has(value)) {
      continue;
    }
    const eco = g.opening_eco ?? '';
    const name = g.opening_name ?? '';
    const title = [eco, name].filter(Boolean).join(' — ') || name || eco;
    map.set(value, { value, title });
  }
  return [...map.values()].sort((a, b) => a.title.localeCompare(b.title));
}
