use serde_json::Value;

use crate::db::games::model::Game;

pub fn parse_ndjson(username: &str, ndjson: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in ndjson.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let json = match serde_json::from_str::<Value>(line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some(game) = map_game(username, &json) {
            games.push(game);
        }
    }

    games
}

fn map_game(username: &str, json: &Value) -> Option<Game> {
    let id = get_string(json, "id")?;

    let rated = json.get("rated").and_then(Value::as_bool).unwrap_or(false);

    let speed = get_string_default(json, "speed");

    let created_at = json
        .get("createdAt")
        .and_then(Value::as_i64)
        .unwrap_or_default();

    let pgn = get_string_default(json, "pgn");

    let moves = json
        .get("moves")
        .and_then(Value::as_str)
        .map(str::to_string);

    let last_fen = json
        .get("lastFen")
        .and_then(Value::as_str)
        .map(str::to_string);

    let winner = json
        .get("winner")
        .and_then(Value::as_str)
        .map(str::to_string);

    let time_control = build_time_control(json);

    let white = json.get("players")?.get("white")?;
    let black = json.get("players")?.get("black")?;

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

    let player_result = resolve_result(winner.as_deref(), &player_color);

    let opening_eco = json
        .get("opening")
        .and_then(|v| v.get("eco"))
        .and_then(Value::as_str)
        .map(str::to_string);

    let opening_name = json
        .get("opening")
        .and_then(|v| v.get("name"))
        .and_then(Value::as_str)
        .map(str::to_string);

    Some(Game {
        id,

        username: username.to_string(),
        platform: "Lichess".to_string(),

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

        moves,
        last_fen,
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

fn resolve_result(winner: Option<&str>, color: &str) -> String {
    match winner {
        Some("white") if color == "white" => "win".to_string(),
        Some("black") if color == "black" => "win".to_string(),
        Some(_) => "loss".to_string(),
        None => "draw".to_string(),
    }
}

fn get_string(json: &Value, key: &str) -> Option<String> {
    json.get(key).and_then(Value::as_str).map(str::to_string)
}

fn get_string_default(json: &Value, key: &str) -> String {
    get_string(json, key).unwrap_or_default()
}

fn nested_string(value: &Value, path: &[&str]) -> Option<String> {
    let mut current = value;

    for key in path {
        current = current.get(*key)?;
    }

    current.as_str().map(str::to_string)
}
