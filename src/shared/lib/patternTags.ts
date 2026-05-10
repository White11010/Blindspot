/**
 * Canonical pattern tag ids from backend `pattern_detector.rs` (`detect_patterns`).
 * Keep in sync with Rust when new tags are added.
 */
export const BACKEND_PATTERN_TAG_IDS = [
  'lost_winning_position',
  'missed_winning_chance',
  'comeback_win',
  'opening_blunder',
  'middlegame_blunder',
  'endgame_blunder',
  'multiple_blunders',
  'slow_drift',
  'low_accuracy',
  'late_game_loss',
] as const;

export type BackendPatternTagId = (typeof BACKEND_PATTERN_TAG_IDS)[number];

/** Virtual My Games pattern filter (not stored in `game_pattern_tags`); matches the late-loss insight cohort. */
export const MY_GAMES_LONG_LOSS_FILTER_TAG = 'long_loss_min_halfmoves';

const KNOWN = new Set<string>(BACKEND_PATTERN_TAG_IDS);

/** Canonical tag id (snake_case); tolerates display strings with spaces. */
export function normalizePatternTagId(raw: string): string {
  return raw.trim().toLowerCase().replace(/\s+/g, '_');
}

export function isKnownBackendPatternTag(raw: string): boolean {
  return KNOWN.has(normalizePatternTagId(raw));
}

/** vue-i18n message path, e.g. `myGames.patterns.opening_blunder`. */
export function patternTagMessageKey(raw: string): string {
  return `myGames.patterns.${normalizePatternTagId(raw)}`;
}

/** Resolve a backend tag to a translated label; unknown tags stay humanized. */
export function formatPatternTagLabel(
  tag: string,
  t: (key: string) => string,
  te: (key: string) => boolean,
): string {
  const key = patternTagMessageKey(tag);
  if (te(key)) {
    return t(key);
  }
  return tag.replace(/_/g, ' ');
}
