// Lichess REST calls for account and NDJSON games export; bearer token comes from the OS keyring via `auth` service.
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::services::auth;

/// Error prefix when Lichess returns 429 so the UI backs off instead of treating it as a generic failure.
pub const LICHESS_RATE_LIMITED: &str = "RATE_LIMITED:";
/// Error prefix for 401 so callers distinguish expired tokens from throttling and prompt re-auth.
pub const LICHESS_UNAUTHORIZED: &str = "UNAUTHORIZED:";

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

/// Fetches the logged-in Lichess profile (`GET /api/account`) for sync and user upsert after token connect.
pub async fn fetch_me(app: &AppHandle) -> Result<LichessProfile, String> {
    let token = auth::load_token(app)?.ok_or("Token not found")?;

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

/// Streams NDJSON games (`GET /api/games/user/{username}`); optional `since_ms` and `max_games` map to Lichess query params.
pub async fn fetch_games(
    app: &AppHandle,
    username: &str,
    since_ms: Option<i64>,
    max_games: Option<u32>,
) -> Result<String, String> {
    let token = auth::load_token(app)?.ok_or("Token not found")?;

    let mut url = format!(
        concat!(
            "https://lichess.org/api/games/user/{}",
            "?moves=true",
            "&pgnInJson=true",
            "&evals=true",
            "&accuracy=true",
            "&opening=true",
            "&lastFen=true"
        ),
        username
    );

    if let Some(since) = since_ms {
        url.push_str(&format!("&since={since}"));
    }
    if let Some(max) = max_games {
        url.push_str(&format!("&max={max}"));
    }

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("Accept", "application/x-ndjson")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return Err(format!(
            "{LICHESS_RATE_LIMITED} Lichess rate limit (429)"
        ));
    }
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return Err(format!(
            "{LICHESS_UNAUTHORIZED} Invalid or expired token"
        ));
    }
    if !status.is_success() {
        return Err(format!("Lichess Games API error: {}", status));
    }

    response.text().await.map_err(|e| e.to_string())
}
