use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAnalysisRow {
    pub game_id: String,
    pub user_id: String,
    pub status: String,
    pub depth: i64,
    pub accuracy: Option<f64>,
    pub avg_centipawn_loss: Option<f64>,
    pub max_advantage_cp: Option<i64>,
    pub min_advantage_cp: Option<i64>,
    pub blunders: Option<i64>,
    pub mistakes: Option<i64>,
    pub inaccuracies: Option<i64>,
    pub eval_history_json: Option<String>,
    pub key_moments_json: Option<String>,
    pub key_insight_json: Option<String>,
    pub system_connection_json: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTagRow {
    pub game_id: String,
    pub user_id: String,
    pub tag: String,
    pub weight: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMomentRow {
    pub game_id: String,
    pub user_id: String,
    pub ply: i64,
    pub kind: String,
    pub swing_cp: i64,
}
