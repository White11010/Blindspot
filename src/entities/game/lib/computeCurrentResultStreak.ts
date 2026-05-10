import type { Game } from '../model/games.types';

export type ResultStreak = {
  kind: 'win' | 'loss';
  length: number;
};

/**
 * Current streak of wins or losses from the most recent game (by `created_at` desc).
 */
export function computeCurrentResultStreak(games: readonly Game[]): ResultStreak | null {
  if (!games.length) {
    return null;
  }
  const sorted = [...games].sort((a, b) => b.created_at - a.created_at);
  const first = sorted[0];
  const streakWin = first.player_result === 'win';
  if (first.player_result !== 'win' && first.player_result !== 'loss') {
    return null;
  }
  let len = 0;
  for (const g of sorted) {
    const ok = streakWin ? g.player_result === 'win' : g.player_result === 'loss';
    if (ok) {
      len += 1;
    } else {
      break;
    }
  }
  if (len < 2) {
    return null;
  }
  return { kind: streakWin ? 'win' : 'loss', length: len };
}
