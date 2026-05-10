// Middlegame vs endgame error mix and related tactical summaries from `KeyMomentRow` joined to completed analyses.
use std::collections::{HashMap, HashSet};

use serde_json::json;

use crate::db::game_analyses::model::{GameAnalysisRow, KeyMomentRow};
use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::insights::insight_common::{build_insight, CAT_TACTICS};

fn is_error_kind(k: &str) -> bool {
    matches!(k, "blunder" | "mistake" | "inaccuracy")
}

/// Emits `CAT_TACTICS` insights when error counts across phases cross tuned thresholds (see in-file magic numbers).
pub fn generate(
    user_id: &str,
    games: &[Game],
    analyses: &HashMap<String, GameAnalysisRow>,
    moments: &[KeyMomentRow],
) -> Vec<Insight> {
    let mut out = Vec::new();

    // Ply 50 split is a cheap phase proxy vs running FEN-based phase detection on every historical game.
    let mut mg_err = 0i64;
    let mut eg_err = 0i64;
    let mut games_with_moments: HashSet<String> = HashSet::new();
    for m in moments {
        if !is_error_kind(&m.kind) {
            continue;
        }
        let Some(a) = analyses.get(&m.game_id) else {
            continue;
        };
        if a.status != "done" {
            continue;
        }
        games_with_moments.insert(m.game_id.clone());
        if m.ply <= 50 {
            mg_err += 1;
        } else {
            eg_err += 1;
        }
    }
    let total_err = mg_err + eg_err;
    let n_games = games_with_moments.len() as i64;
    if total_err >= 15 && n_games >= 8 {
        let eg_share = (eg_err as f64 / total_err as f64) * 100.0;
        let mg_share = 100.0 - eg_share;
        let eg_share_r = eg_share.round();
        out.push(build_insight(
            format!("tactics_phase_{user_id}"),
            user_id,
            "tactics_middlegame_vs_endgame",
            CAT_TACTICS,
            "Миттельшпиль vs эндшпиль".to_string(),
            format!(
                "Ошибки (блиц/неточности) в ключевых моментах: миттельшпиль (≤50 полуходов) {mg_err}, эндшпиль (>50) {eg_err} из {total_err} на {n_games} партий с анализом. Доля эндшпиля: {eg_share_r}%."
            ),
            if eg_share > 52.0 { "warning" } else { "info" },
            78,
            Some("Доля ошибок в эндшпиле".to_string()),
            Some(format!("{eg_share_r}%")),
            Some(eg_share_r),
            Some("Добавь эндшпильные задачи, если хвост партии проседает.".to_string()),
            "tactics:phase_error_split",
            76,
            json!({
                "mg_err": mg_err,
                "eg_err": eg_err,
                "total_err": total_err,
                "n_games": n_games,
                "eg_share": eg_share_r
            }),
        ));
        let _ = mg_share;
    }

    // Failed conversion: had >= +2.0 advantage, did not win — with time-control breakdown
    let mut by_speed: HashMap<String, (i64, i64)> = HashMap::new();
    for g in games {
        let Some(a) = analyses.get(&g.id) else {
            continue;
        };
        if a.status != "done" {
            continue;
        }
        if a.max_advantage_cp.unwrap_or(0) < 200 {
            continue;
        }
        let label = match g.speed.as_str() {
            "bullet" => "Bullet",
            "blitz" => "Blitz",
            "rapid" => "Rapid",
            "classical" => "Classical",
            other => other,
        };
        let e = by_speed.entry(label.to_string()).or_insert((0, 0));
        e.0 += 1;
        if g.player_result != "win" {
            e.1 += 1;
        }
    }

    let mut with_adv = 0i64;
    let mut failed = 0i64;
    for (w, f) in by_speed.values() {
        with_adv += w;
        failed += f;
    }

    if with_adv >= 8 {
        let rate = (failed as f64 / with_adv as f64) * 100.0;
        let rate_r = rate.round();

        let mut parts: Vec<String> = Vec::new();
        let mut breakdown = serde_json::Map::new();
        for (label, (w, f)) in &by_speed {
            if *w < 3 {
                continue;
            }
            let r = (*f as f64 / *w as f64 * 100.0).round() as i64;
            parts.push(format!("{label}: {r}% ({f}/{w})"));
            breakdown.insert(
                label.clone(),
                json!({ "with_adv": w, "failed": f, "rate": r }),
            );
        }
        let by_line = if parts.is_empty() {
            String::new()
        } else {
            format!(" {}", parts.join(" · "))
        };
        let speed_split = parts.join(" · ");

        out.push(build_insight(
            format!("tactics_conversion_{user_id}"),
            user_id,
            "tactics_conversion_advantage",
            CAT_TACTICS,
            "Конверсия выигранных позиций".to_string(),
            format!(
                "В {with_adv} партиях был перевес ≥+2.0, но {failed} закончились не победой ({rate_r}%).{by_line}"
            ),
            if rate > 35.0 { "warning" } else { "info" },
            82,
            Some("Не победили, %".to_string()),
            Some(format!("{rate_r}%")),
            Some(rate_r),
            Some("Потренируй техническую реализацию и контроль времени.".to_string()),
            "tactics:failed_conversion_rate",
            86,
            json!({
                "with_adv": with_adv,
                "failed": failed,
                "rate": rate_r,
                "speed_split": speed_split,
                "by_speed": serde_json::Value::Object(breakdown)
            }),
        ));
    }

    out.extend(
        crate::services::insights::generators::tactics_phase_accuracy::generate(
            user_id, games, analyses,
        ),
    );

    out
}
