// Per-phase average accuracy by re-classifying stored eval lines (no extra engine): opening/mid/end from ply index.
use std::collections::HashMap;

use serde_json::json;

use crate::db::game_analyses::model::GameAnalysisRow;
use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::parsers::move_count::is_uci_token;
use crate::services::game_analysis::classifier::{
    accuracy_from_acpl, avg_centipawn_loss, classify_player_moves_from_eval, ClassifiedMove,
};
use crate::services::insights::insight_common::{build_insight, CAT_TACTICS};

fn phase_bucket_ply(ply: i64) -> usize {
    // Three fixed bands align with UI copy (opening / middlegame / endgame) without expensive piece-count phase detection.
    if ply <= 20 {
        0
    } else if ply <= 60 {
        1
    } else {
        2
    }
}

/// Builds phase accuracy comparison `CAT_TACTICS` insight when enough games have completed analysis + eval JSON.
pub fn generate(
    user_id: &str,
    games: &[Game],
    analyses: &HashMap<String, GameAnalysisRow>,
) -> Vec<Insight> {
    let mut opening: Vec<ClassifiedMove> = Vec::new();
    let mut middlegame: Vec<ClassifiedMove> = Vec::new();
    let mut endgame: Vec<ClassifiedMove> = Vec::new();
    let mut n_games_used = 0i64;

    for g in games {
        let Some(a) = analyses.get(&g.id) else {
            continue;
        };
        if a.status != "done" {
            continue;
        }
        let Some(ref eval_json) = a.eval_history_json else {
            continue;
        };
        let Ok(eval_history) = serde_json::from_str::<Vec<i32>>(eval_json) else {
            continue;
        };
        let moves_str = g.moves.as_deref().unwrap_or("").trim();
        if moves_str.is_empty() {
            continue;
        }
        let uci_moves: Vec<String> = moves_str
            .split_whitespace()
            .filter(|t| is_uci_token(t))
            .map(|s| s.to_string())
            .collect();
        if uci_moves.len() + 1 != eval_history.len() {
            continue;
        }
        let player_is_white = g.player_color == "white";
        let Ok(classified) = classify_player_moves_from_eval(&uci_moves, &eval_history, player_is_white)
        else {
            continue;
        };
        n_games_used += 1;
        for c in classified {
            let ply = (c.half_move_index + 1) as i64;
            match phase_bucket_ply(ply) {
                0 => opening.push(c),
                1 => middlegame.push(c),
                _ => endgame.push(c),
            }
        }
    }

    if n_games_used < 12 {
        return vec![];
    }

    let acc_open = if opening.is_empty() {
        0.0
    } else {
        accuracy_from_acpl(avg_centipawn_loss(&opening))
    };
    let acc_mid = if middlegame.is_empty() {
        0.0
    } else {
        accuracy_from_acpl(avg_centipawn_loss(&middlegame))
    };
    let acc_end = if endgame.is_empty() {
        0.0
    } else {
        accuracy_from_acpl(avg_centipawn_loss(&endgame))
    };

    let o = acc_open.round() as i64;
    let m = acc_mid.round() as i64;
    let e = acc_end.round() as i64;

    let mn = acc_open.min(acc_mid).min(acc_end);
    let mx = acc_open.max(acc_mid).max(acc_end);
    let spread = mx - mn;
    if spread < 8.0 {
        return vec![];
    }

    let worst = if acc_end <= acc_mid && acc_end <= acc_open {
        "endgame"
    } else if acc_mid <= acc_open {
        "middlegame"
    } else {
        "opening"
    };
    let worst_pct = if worst == "endgame" {
        e
    } else if worst == "middlegame" {
        m
    } else {
        o
    };

    vec![build_insight(
        format!("tactics_phase_acc_{user_id}"),
        user_id,
        "tactics_accuracy_by_phase",
        CAT_TACTICS,
        "Точность по фазам".to_string(),
        format!(
            "Средняя точность по {n_games_used} проанализированным партиям: дебют (≤20 полуходов) {o}%, миттельшпиль (21–60) {m}%, эндшпиль (61+) {e}%."
        ),
        if spread >= 15.0 { "warning" } else { "info" },
        76,
        Some("Точность (худшая фаза)".to_string()),
        Some(format!("{worst_pct}%")),
        Some(worst_pct as f64),
        Some("Сфокусируй тренировку на фазе с самой низкой точностью.".to_string()),
        "tactics:accuracy_by_phase",
        77,
        json!({
            "n_games": n_games_used,
            "opening_pct": o,
            "middlegame_pct": m,
            "endgame_pct": e,
            "weakest_phase": worst
        }),
    )]
}
