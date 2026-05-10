// Late-game loss rate vs game length using eval history length as a ply proxy when moves string is sparse.
use std::collections::HashMap;

use serde_json::json;

use crate::db::game_analyses::model::GameAnalysisRow;
use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::parsers::move_count::total_halfmoves;
use crate::services::insights::insight_common::{build_insight, CAT_TACTICS};

fn eval_history_len(a: &GameAnalysisRow) -> Option<usize> {
    a.eval_history_json.as_ref().and_then(|j| serde_json::from_str::<Vec<i32>>(j).ok().map(|v| v.len()))
}

/// Flags recurring losses in long games (`CAT_TACTICS`) when `late_game_loss` pattern density is high enough.
pub fn generate(
    user_id: &str,
    games: &[Game],
    analyses: &HashMap<String, GameAnalysisRow>,
) -> Vec<Insight> {
    let mut late_game_losses = 0i64;
    let mut total_games = 0i64;

    for game in games {
        total_games += 1;

        if is_late_game_loss(game, analyses.get(&game.id)) {
            late_game_losses += 1;
        }
    }

    if total_games < 15 || late_game_losses == 0 {
        return vec![];
    }

    let rate = (late_game_losses as f64 / total_games as f64) * 100.0;
    let rate_r = rate.round();

    vec![build_insight(
        format!("tactics_late_{user_id}"),
        user_id,
        "tactics_late_game_losses",
        CAT_TACTICS,
        "Поздние партии с поражением".to_string(),
        format!(
            "В {late_game_losses} из {total_games} партий поражение после ≥40 полуходов."
        ),
        "warning",
        confidence(total_games),
        Some("Доля партий".to_string()),
        Some(format!("{rate_r}%")),
        Some(rate_r),
        Some("Тренируй эндшпили и конвертацию.".to_string()),
        "tactics:late_long_loss",
        52,
        json!({
            "late_losses": late_game_losses,
            "total_games": total_games,
            "rate": rate_r
        }),
    )]
}

fn is_late_game_loss(game: &Game, analysis: Option<&GameAnalysisRow>) -> bool {
    let lost = game.player_result == "loss";
    if !lost {
        return false;
    }
    let eval_len = analysis
        .filter(|a| a.status == "done")
        .and_then(eval_history_len);
    let hm = total_halfmoves(game.moves.as_deref(), eval_len);
    hm >= 40
}

fn confidence(games: i64) -> i64 {
    if games >= 100 {
        95
    } else if games >= 50 {
        85
    } else {
        70
    }
}
