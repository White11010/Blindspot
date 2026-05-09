use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMoment {
    pub ply: i64,
    pub move_number: i64,
    pub move_san: String,
    pub kind: String,
    pub eval_before: i32,
    pub eval_after: i32,
    pub swing_cp: i32,
    pub best_move_san: Option<String>,
    pub fen_before: String,
    pub headline: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInsight {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConnection {
    /// Legacy; leave empty for new analyses — UI uses `primary_variant` + i18n.
    #[serde(default)]
    pub text: String,
    pub tag: String,
    pub count: i64,
    pub window: i32,
    #[serde(default)]
    pub secondary_text: Option<String>,
    /// `no_tags` | `similar_low` | `similar_high`
    #[serde(default)]
    pub primary_variant: String,
    /// `none` | `revisit` | `win_rate`
    #[serde(default)]
    pub secondary_variant: String,
    #[serde(default)]
    pub secondary_total: i64,
    #[serde(default)]
    pub secondary_wr_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarGames {
    pub broad: Vec<String>,
    pub narrow: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAnalysisFull {
    pub game_id: String,
    pub status: String,
    pub depth: u8,
    pub key_moments: Vec<KeyMoment>,
    pub key_insight: KeyInsight,
    pub system_connection: SystemConnection,
    pub eval_history: Vec<i32>,
    pub accuracy: f64,
    pub avg_centipawn_loss: f64,
    pub max_advantage_cp: i32,
    pub min_advantage_cp: i32,
    pub blunders: i32,
    pub mistakes: i32,
    pub inaccuracies: i32,
    pub pattern_tags: Vec<String>,
    pub similar_games: SimilarGames,
    pub error: Option<String>,
}
