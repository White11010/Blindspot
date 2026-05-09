import { openUrl } from '@tauri-apps/plugin-opener';

import type { Game } from '../model/games.types';

export function shortOpeningDisplay(name: string | null): string {
  if (!name?.trim()) {
    return '—';
  }
  const n = name.trim();
  const colon = n.indexOf(':');
  if (colon !== -1) {
    return n.slice(0, colon).trim() || '—';
  }
  const comma = n.indexOf(',');
  if (comma !== -1) {
    return n.slice(0, comma).trim() || '—';
  }
  return n;
}

export function openingTooltipText(name: string | null, eco: string | null): string {
  const parts: string[] = [];
  if (eco?.trim()) {
    parts.push(eco.trim());
  }
  if (name?.trim()) {
    parts.push(name.trim());
  }
  return parts.join(' — ') || '';
}

export function speedChipLabel(speed: string): string {
  const s = speed.toLowerCase();
  if (s === 'ultrabullet' || s === 'bullet') {
    return 'Bullet';
  }
  if (s === 'blitz') {
    return 'Blitz';
  }
  if (s === 'rapid') {
    return 'Rapid';
  }
  if (s === 'classical') {
    return 'Classical';
  }
  return speed ? speed.charAt(0).toUpperCase() + speed.slice(1).toLowerCase() : '—';
}

export function accuracyPercentRounded(acc: number | null | undefined): number | null {
  if (acc == null || Number.isNaN(acc)) {
    return null;
  }
  return Math.round(acc);
}

export type AccuracyTone = 'high' | 'mid' | 'low';

/** Uses rounded percent: >85 high, 70–85 mid, <70 low. */
export function accuracyToneFromRounded(rounded: number | null): AccuracyTone | null {
  if (rounded == null) {
    return null;
  }
  if (rounded > 85) {
    return 'high';
  }
  if (rounded >= 70) {
    return 'mid';
  }
  return 'low';
}

export function resultLetter(result: Game['player_result']): 'W' | 'D' | 'L' {
  switch (result) {
    case 'win':
      return 'W';
    case 'loss':
      return 'L';
    case 'draw':
      return 'D';
  }
}

export function getExternalGameUrl(game: Game): string | null {
  if (game.platform === 'Lichess') {
    return `https://lichess.org/${encodeURIComponent(game.id)}`;
  }
  if (game.platform === 'Chess.com') {
    return null;
  }
  return null;
}

export async function openExternalGame(game: Game): Promise<void> {
  const href = getExternalGameUrl(game);
  if (!href) {
    return;
  }
  await openUrl(href);
}
