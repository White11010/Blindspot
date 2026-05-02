use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRecord {
    pub id: String,

    pub username: String,

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

    pub winner: Option<String>,      // white / black / null(draw)
    pub player_color: String,        // white / black
    pub player_result: String,       // win / loss / draw

    pub opening_eco: Option<String>,
    pub opening_name: Option<String>,

    pub pgn: String,
}