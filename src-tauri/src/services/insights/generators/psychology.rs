use serde_json::json;

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::insights::insight_common::{build_insight, CAT_PSYCHOLOGY};

fn is_loss(g: &Game) -> bool {
    g.player_result == "loss"
}

fn is_win(g: &Game) -> bool {
    g.player_result == "win"
}

/// Chronological: oldest → newest.
fn sorted_chrono(games: &[Game]) -> Vec<&Game> {
    let mut v: Vec<&Game> = games.iter().collect();
    v.sort_by_key(|g| g.created_at);
    v
}

/// Win rate in the 3rd and 4th game after two consecutive losses (indices i+2, i+3 after losses at i-2, i-1).
pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    if games.len() < 20 {
        return vec![];
    }

    let ch = sorted_chrono(games);
    let mut out = Vec::new();

    // Tilt
    let mut tilt_wins = 0i64;
    let mut tilt_n = 0i64;
    for i in 2..ch.len() {
        if i + 3 >= ch.len() {
            continue;
        }
        if is_loss(ch[i - 2]) && is_loss(ch[i - 1]) {
            // 3rd and 4th game after the second loss: positions i+2 and i+3 in 0-based chrono list
            // After L at i-2, L at i-1: game i = 1st after, i+1 = 2nd, i+2 = 3rd, i+3 = 4th
            for j in [i + 2, i + 3] {
                if j < ch.len() {
                    tilt_n += 1;
                    if is_win(ch[j]) {
                        tilt_wins += 1;
                    }
                }
            }
        }
    }
    if tilt_n >= 8 {
        let wr = tilt_wins as f64 / tilt_n as f64;
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("psych_tilt_{user_id}"),
            user_id,
            "psychology_tilt",
            CAT_PSYCHOLOGY,
            "Tilt-детектор".to_string(),
            format!(
                "После двух поражений подряд винрейт на 3-й и 4-й следующей партии: {pct}% ({tilt_wins}/{tilt_n})."
            ),
            if wr < 0.42 { "warning" } else { "info" },
            72,
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Сделай паузу или разминку после серии поражений.".to_string()),
            "tilt:after_ll_positions_3_4",
            98,
            json!({ "tilt_wins": tilt_wins, "tilt_n": tilt_n, "pct": pct }),
        ));
    }

    // Comeback: game right after any loss
    let mut cw = 0i64;
    let mut ct = 0i64;
    for i in 1..ch.len() {
        if is_loss(ch[i - 1]) {
            ct += 1;
            if is_win(ch[i]) {
                cw += 1;
            }
        }
    }
    if ct >= 15 {
        let wr = cw as f64 / ct as f64;
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("psych_comeback_{user_id}"),
            user_id,
            "psychology_comeback",
            CAT_PSYCHOLOGY,
            "Comeback".to_string(),
            format!("Винрейт в партии сразу после поражения: {pct}% ({cw}/{ct})."),
            if wr >= 0.48 { "good" } else { "info" },
            75,
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Сравни с общим темпом — видно, удаётся ли «отыгрываться».".to_string()),
            "comeback:after_loss",
            85,
            json!({ "wins_after_loss": cw, "trials_after_loss": ct, "pct": pct }),
        ));
    }

    // Streak (newest first)
    let mut newest: Vec<&Game> = games.iter().collect();
    newest.sort_by_key(|g| std::cmp::Reverse(g.created_at));
    if let Some(first) = newest.first() {
        let streak_win = is_win(first);
        let mut len: i64 = 0;
        for g in &newest {
            let ok = if streak_win { is_win(g) } else { is_loss(g) };
            if ok {
                len += 1;
            } else {
                break;
            }
        }
        if len >= 2 {
            let label = if streak_win { "побед" } else { "поражений" };
            let title = if streak_win {
                "Серия побед"
            } else {
                "Серия поражений"
            };
            let sev = if streak_win { "good" } else { "warning" };
            out.push(build_insight(
                format!("psych_streak_{user_id}"),
                user_id,
                "psychology_streak",
                CAT_PSYCHOLOGY,
                title.to_string(),
                format!("Текущая серия: {len} {label} подряд."),
                sev,
                80,
                Some("Длина серии".to_string()),
                Some(format!("{len}")),
                Some(len as f64),
                Some(if streak_win {
                    "Постарайся не переоценивать силу — держи дисциплину.".to_string()
                } else {
                    "Сделай перерыв или разбери последние партии.".to_string()
                }),
                "streak:current",
                92,
                json!({ "len": len, "streak_win": streak_win }),
            ));
        }
    }

    out
}
