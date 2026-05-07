export type AnalysisStatus = 'pending' | 'done' | 'failed' | string;

export interface KeyMoment {
  ply: number;
  move_number: number;
  move_san: string;
  kind: string;
  eval_before: number;
  eval_after: number;
  swing_cp: number;
  best_move_san: string | null;
  fen_before: string;
  headline: string;
  description: string;
}

export interface KeyInsight {
  title: string;
  description: string;
  severity: string;
  kind: string;
}

export interface SystemConnection {
  text: string;
  tag: string;
  count: number;
  window: number;
  secondary_text: string | null;
}

export interface SimilarGames {
  broad: string[];
  narrow: string[];
}

export interface GameAnalysis {
  game_id: string;
  status: AnalysisStatus;
  depth: number;
  key_moments: KeyMoment[];
  key_insight: KeyInsight;
  system_connection: SystemConnection;
  eval_history: number[];
  accuracy: number;
  avg_centipawn_loss: number;
  max_advantage_cp: number;
  min_advantage_cp: number;
  blunders: number;
  mistakes: number;
  inaccuracies: number;
  pattern_tags: string[];
  similar_games: SimilarGames;
  error: string | null;
}
