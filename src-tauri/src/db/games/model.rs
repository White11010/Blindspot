use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,

    pub username: String,
    pub platform: String,

    pub rated: bool,
    pub speed: String,
    pub time_control: String,
    pub created_at: i64,

    pub player_name: String,
    pub player_id: String,

    pub opponent_name: String,
    pub opponent_id: String,

    pub white_name: String,
    pub white_id: String,

    pub black_name: String,
    pub black_id: String,

    pub white_rating: Option<i64>,
    pub black_rating: Option<i64>,

    pub player_rating: Option<i64>,
    pub opponent_rating: Option<i64>,

    pub winner: Option<String>,
    pub player_color: String,
    pub player_result: String,

    pub opening_eco: Option<String>,
    pub opening_name: Option<String>,

    pub moves: Option<String>,
    pub last_fen: Option<String>,
    pub pgn: String,
}

/// Game row plus analysis summary for list views (e.g. My Games filters).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameListItem {
    #[serde(flatten)]
    pub base: Game,
    pub analysis_accuracy: Option<f64>,
    pub analysis_acpl: Option<f64>,
    pub pattern_tags: Vec<String>,
}
