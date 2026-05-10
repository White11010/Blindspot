// Maps Lichess games API NDJSON lines into `Game` rows (player perspective, result, opening, PGN fields).
use serde_json::Value;

use crate::db::games::model::Game;

/// Parses each non-empty line as JSON; skips malformed lines so one bad chunk does not fail the whole sync batch.
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

// API omits fields on variants; we require `id` and both players so anonymous or broken lines become None.
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

    // Match by name or id so sync works whether the API returns display name or stable Lichess id as username input.
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Минимальная строка в формате NDJSON API Lichess (одна партия на строку).
    fn sample_game_line() -> &'static str {
        r#"{"id":"abc123","rated":true,"speed":"blitz","createdAt":1700000000,"pgn":"[Event \"?\"]\n","players":{"white":{"user":{"name":"Alice","id":"alice1"},"rating":2000},"black":{"user":{"name":"Bob","id":"bob1"},"rating":1900}},"clock":{"initial":180,"increment":2},"winner":"white","opening":{"eco":"B20","name":"Sicilian Defense"}}"#
    }

    #[test]
    fn parse_ndjson_maps_game_for_logged_in_white_player() {
        let games = parse_ndjson("alice1", sample_game_line());

        assert_eq!(games.len(), 1);
        let g = &games[0];
        assert_eq!(g.id, "abc123");
        assert_eq!(g.username, "alice1");
        assert_eq!(g.platform, "Lichess");
        assert!(g.rated);
        assert_eq!(g.speed, "blitz");
        assert_eq!(g.created_at, 1_700_000_000);
        assert_eq!(g.player_color, "white");
        assert_eq!(g.player_result, "win");
        assert_eq!(g.player_name, "Alice");
        assert_eq!(g.opponent_name, "Bob");
        assert_eq!(g.opponent_id, "bob1");
        assert_eq!(g.white_rating, Some(2000));
        assert_eq!(g.black_rating, Some(1900));
        assert_eq!(g.player_rating, Some(2000));
        assert_eq!(g.opponent_rating, Some(1900));
        assert_eq!(g.time_control, "180+2");
        assert_eq!(g.opening_eco.as_deref(), Some("B20"));
        assert_eq!(g.opening_name.as_deref(), Some("Sicilian Defense"));
        assert_eq!(g.winner.as_deref(), Some("white"));
    }

    #[test]
    fn parse_ndjson_resolves_black_perspective_and_loss() {
        let games = parse_ndjson("bob1", sample_game_line());
        assert_eq!(games.len(), 1);
        let g = &games[0];
        assert_eq!(g.player_color, "black");
        assert_eq!(g.player_result, "loss");
        assert_eq!(g.player_rating, Some(1900));
        assert_eq!(g.opponent_rating, Some(2000));
    }

    #[test]
    fn parse_ndjson_skips_empty_lines_and_invalid_json() {
        let ndjson = format!(
            "\n\nnot valid json\n{}\n",
            sample_game_line()
        );
        let games = parse_ndjson("alice1", &ndjson);
        assert_eq!(games.len(), 1);
        assert_eq!(games[0].id, "abc123");
    }

    #[test]
    fn parse_ndjson_resolves_draw_when_winner_missing() {
        let line = r#"{"id":"draw1","rated":false,"speed":"rapid","createdAt":1700000001,"players":{"white":{"user":{"name":"Alice","id":"alice1"},"rating":2100},"black":{"user":{"name":"Bob","id":"bob1"},"rating":2090}}}"#;

        let white = &parse_ndjson("alice1", line)[0];
        assert_eq!(white.player_result, "draw");
        assert_eq!(white.player_color, "white");
        assert_eq!(white.winner, None);

        let black = &parse_ndjson("bob1", line)[0];
        assert_eq!(black.player_result, "draw");
        assert_eq!(black.player_color, "black");
        assert_eq!(black.winner, None);
    }
}
