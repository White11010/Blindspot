use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::services::auth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perf {
    pub games: Option<i64>,
    pub rating: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perfs {
    pub bullet: Option<Perf>,
    pub blitz: Option<Perf>,
    pub rapid: Option<Perf>,
    pub classical: Option<Perf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LichessProfile {
    pub id: String,
    pub username: String,
    pub perfs: Option<Perfs>,
}

pub async fn fetch_me(app: &AppHandle) -> Result<LichessProfile, String> {
    let token = auth::load_token(app)?
        .ok_or("Token not found")?;

    let client = reqwest::Client::new();

    let response = client
        .get("https://lichess.org/api/account")
        .header("Accept", "application/json")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Lichess API error: {}", response.status()));
    }

    response
        .json::<LichessProfile>()
        .await
        .map_err(|e| e.to_string())
}