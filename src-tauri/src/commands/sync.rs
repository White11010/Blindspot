use crate::db;
use crate::models::game::GameRecord;

use tauri::AppHandle;
use tauri_plugin_keyring::KeyringExt;

use tracing::{error, info, warn};

use serde_json::Value;

const KEYRING_SERVICE: &str = "my-tauri-app";
const KEYRING_USER: &str = "lichess_user";

#[tauri::command]
pub async fn sync_lichess_games(app: AppHandle, username: String) -> Result<u32, String> {
    info!("Sync started for username={}", username);

    let token = load_token(&app)?;
    let url = build_games_url(&username);

    let response_text = fetch_games_ndjson(&url, &token).await?;

    let conn = db::init_db();

    let _ = db::games::repository::delete_all_games(&conn);

    let inserted = persist_games(&conn, &username, &response_text)?;

    info!(
        "Sync finished for username={}, inserted={}",
        username, inserted
    );

    Ok(inserted)
}

fn load_token(app: &AppHandle) -> Result<String, String> {
    let token = app
        .keyring()
        .get_password(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| format!("Failed to read token from keyring: {}", e))?;

    token.ok_or_else(|| "Lichess token not found".to_string())
}

fn build_games_url(username: &str) -> String {
    format!(
        concat!(
            "https://lichess.org/api/games/user/{}",
            "?max=50",
            "&pgnInJson=true",
            "&evals=true",
            "&accuracy=true",
            "&opening=true"
        ),
        username
    )
}

async fn fetch_games_ndjson(url: &str, token: &str) -> Result<String, String> {
    info!("Requesting Lichess games: {}", url);

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("accept", "application/x-ndjson")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let status = response.status();

    if !status.is_success() {
        return Err(format!("Lichess API returned status {}", status));
    }

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    info!("Received {} bytes from Lichess", text.len());

    Ok(text)
}

fn persist_games(conn: &rusqlite::Connection, username: &str, ndjson: &str) -> Result<u32, String> {
    let mut inserted_count: u32 = 0;

    for (index, line) in ndjson.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let json = match parse_json_line(line, index) {
            Ok(value) => value,
            Err(err) => {
                warn!("{}", err);
                continue;
            }
        };

        let game = match map_json_to_game(username, &json) {
            Ok(game) => game,
            Err(err) => {
                warn!("Line {} skipped: {}", index + 1, err);
                continue;
            }
        };

        match db::games::repository::insert_game(conn, &game) {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    inserted_count += 1;
                }
            }
            Err(err) => {
                error!("Failed to insert game id={}: {}", game.id, err);
            }
        }
    }

    Ok(inserted_count)
}

fn parse_json_line(line: &str, index: usize) -> Result<Value, String> {
    serde_json::from_str::<Value>(line)
        .map_err(|e| format!("Line {} invalid JSON: {}", index + 1, e))
}

fn map_json_to_game(username: &str, json: &Value) -> Result<GameRecord, String> {
    let id = required_string(json, "id")?;
    let rated = json.get("rated").and_then(Value::as_bool).unwrap_or(false);
    let speed = json
        .get("speed")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let created_at = json
        .get("createdAt")
        .and_then(Value::as_i64)
        .unwrap_or_default();
    let pgn = json
        .get("pgn")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let time_control = build_time_control(json);
    let winner = json
        .get("winner")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let white = json
        .get("players")
        .and_then(|p| p.get("white"))
        .ok_or("Missing players.white")?;
    let black = json
        .get("players")
        .and_then(|p| p.get("black"))
        .ok_or("Missing players.black")?;
    let white_name = nested_string(white, &["user", "name"]).unwrap_or_default();
    let white_id = nested_string(white, &["user", "id"]).unwrap_or_default();
    let black_name = nested_string(black, &["user", "name"]).unwrap_or_default();
    let black_id = nested_string(black, &["user", "id"]).unwrap_or_default();
    let white_rating = white.get("rating").and_then(Value::as_i64);
    let black_rating = black.get("rating").and_then(Value::as_i64);
    let is_white_player =
        username.eq_ignore_ascii_case(&white_name) || username.eq_ignore_ascii_case(&white_id);
    let (
        player_name,
        player_id,
        opponent_name,
        opponent_id,
        player_color,
        player_rating,
        opponent_rating,
    ) = if is_white_player {
        (
            white_name.clone(),
            white_id.clone(),
            black_name.clone(),
            black_id.clone(),
            "white".to_string(),
            white_rating,
            black_rating,
        )
    } else {
        (
            black_name.clone(),
            black_id.clone(),
            white_name.clone(),
            white_id.clone(),
            "black".to_string(),
            black_rating,
            white_rating,
        )
    };
    let player_result = resolve_player_result(winner.as_deref(), &player_color);
    let opening_eco = json
        .get("opening")
        .and_then(|o| o.get("eco"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    let opening_name = json
        .get("opening")
        .and_then(|o| o.get("name"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);
    Ok(GameRecord {
        id,
        username: username.to_string(),
        rated,
        speed,
        time_control,
        created_at,
        player_name,
        player_id,
        opponent_name,
        opponent_id,
        white_name,
        white_id,
        black_name,
        black_id,
        white_rating,
        black_rating,
        player_rating,
        opponent_rating,
        winner,
        player_color,
        player_result,
        opening_eco,
        opening_name,
        pgn,
    })
}
fn build_time_control(json: &Value) -> String {
    let initial = json
        .get("clock")
        .and_then(|v| v.get("initial"))
        .and_then(Value::as_i64);
    let increment = json
        .get("clock")
        .and_then(|v| v.get("increment"))
        .and_then(Value::as_i64);
    match (initial, increment) {
        (Some(i), Some(inc)) => format!("{}+{}", i, inc),
        _ => String::new(),
    }
}
fn resolve_player_result(winner: Option<&str>, player_color: &str) -> String {
    match winner {
        Some("white") if player_color == "white" => "win".to_string(),
        Some("black") if player_color == "black" => "win".to_string(),
        Some(_) => "loss".to_string(),
        None => "draw".to_string(),
    }
}
fn required_string(json: &Value, field: &str) -> Result<String, String> {
    json.get(field)
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing field {}", field))
}
fn nested_string(value: &Value, path: &[&str]) -> Option<String> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str().map(|s| s.to_string())
}
