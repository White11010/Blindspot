use std::collections::HashMap;

use serde_json::json;

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::insights::insight_common::{build_insight, CAT_OPENINGS};

#[derive(Default, Clone)]
struct OpeningStats {
    games: i64,
    wins: i64,
    losses: i64,
    draws: i64,
}

fn win_rate(s: &OpeningStats) -> f64 {
    if s.games == 0 {
        return 0.0;
    }
    (s.wins as f64 + s.draws as f64 * 0.5) / s.games as f64
}

fn opening_subject(name: &str) -> String {
    format!("opening:{name}")
}

fn confidence_from_n(n: i64) -> i64 {
    if n >= 30 {
        95
    } else if n >= 15 {
        85
    } else if n >= 10 {
        75
    } else if n >= 5 {
        65
    } else {
        55
    }
}

pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut map: HashMap<String, OpeningStats> = HashMap::new();
    let mut games_with_opening: i64 = 0;

    for game in games {
        let Some(name) = game.opening_name.clone() else {
            continue;
        };
        if name.trim().is_empty() {
            continue;
        }
        games_with_opening += 1;
        let stats = map.entry(name).or_default();
        stats.games += 1;
        match game.player_result.as_str() {
            "win" => stats.wins += 1,
            "loss" => stats.losses += 1,
            _ => stats.draws += 1,
        }
    }

    if games_with_opening < 10 {
        return vec![];
    }

    let mut out = Vec::new();

    // 1. Best opening (n >= 5)
    let mut best: Option<(String, f64, i64)> = None;
    for (name, st) in &map {
        if st.games < 5 {
            continue;
        }
        let wr = win_rate(st);
        match &best {
            None => best = Some((name.clone(), wr, st.games)),
            Some((_, w, _)) if wr > *w => best = Some((name.clone(), wr, st.games)),
            _ => {}
        }
    }
    if let Some((name, wr, n)) = best {
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("opening_best_{user_id}"),
            user_id,
            "opening_best",
            CAT_OPENINGS,
            "Лучший дебют".to_string(),
            format!("{name} — {pct}% побед за {n} партий."),
            "good",
            confidence_from_n(n),
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Используй этот дебют в решающих партиях.".to_string()),
            &opening_subject(&name),
            72,
            json!({ "opening": name, "pct": pct, "n": n }),
        ));
    }

    // 2. Worst among frequent (n >= 10, share >= 8% of opening games)
    let share_denom = games_with_opening.max(1) as f64;
    let mut worst_freq: Option<(String, f64, i64, f64)> = None;
    for (name, st) in &map {
        if st.games < 10 {
            continue;
        }
        let share = st.games as f64 / share_denom;
        if share < 0.08 {
            continue;
        }
        let wr = win_rate(st);
        match &worst_freq {
            None => worst_freq = Some((name.clone(), wr, st.games, share)),
            Some((_, w, _, _)) if wr < *w => {
                worst_freq = Some((name.clone(), wr, st.games, share))
            }
            _ => {}
        }
    }
    if let Some((name, wr, n, share)) = worst_freq {
        let pct = (wr * 100.0).round();
        let share_pct = (share * 100.0).round();
        out.push(build_insight(
            format!("opening_worst_frequent_{user_id}"),
            user_id,
            "opening_worst_frequent",
            CAT_OPENINGS,
            "Худший из частых дебютов".to_string(),
            format!(
                "{name} — {pct}% при {n} партиях (~{share_pct}% всех партий с дебютом)."
            ),
            "warning",
            confidence_from_n(n),
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Разбери типовые ошибки в этом дебюте.".to_string()),
            &opening_subject(&name),
            88,
            json!({
                "opening": name,
                "pct": pct,
                "n": n,
                "share_pct": share_pct
            }),
        ));
    }

    // 3. Rare gem: n < 5, wr > 70%
    let mut gem: Option<(String, f64, i64)> = None;
    for (name, st) in &map {
        if st.games >= 5 {
            continue;
        }
        let wr = win_rate(st);
        if wr <= 0.7 {
            continue;
        }
        match &gem {
            None => gem = Some((name.clone(), wr, st.games)),
            Some((_, _, gn)) if st.games > *gn => gem = Some((name.clone(), wr, st.games)),
            _ => {}
        }
    }
    if let Some((name, wr, n)) = gem {
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("opening_rare_gem_{user_id}"),
            user_id,
            "opening_rare_gem",
            CAT_OPENINGS,
            "Редкий дебют-находка".to_string(),
            format!("{name} — {pct}% за всего {n} партий (мало данных, но сигнал сильный)."),
            "good",
            60,
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Стоит сыграть ещё партии в этом дебюте.".to_string()),
            &opening_subject(&name),
            45,
            json!({ "opening": name, "pct": pct, "n": n }),
        ));
    }

    // 4. Dependency: one opening > 40% of all games (with opening tag)
    let mut top: Option<(String, i64)> = None;
    for (name, st) in &map {
        match &top {
            None => top = Some((name.clone(), st.games)),
            Some((_, g)) if st.games > *g => top = Some((name.clone(), st.games)),
            _ => {}
        }
    }
    if let Some((name, n)) = top {
        let share = n as f64 / share_denom;
        if share > 0.4 {
            let share_pct = (share * 100.0).round();
            let st = map.get(&name).unwrap();
            let wr = win_rate(st);
            let wr_pct = (wr * 100.0).round();
            let sk = "opening_dependency";
            out.push(build_insight(
                format!("opening_dependency_{user_id}"),
                user_id,
                "opening_dependency",
                CAT_OPENINGS,
                "Дебютная зависимость".to_string(),
                format!(
                    "{name} — {share_pct}% всех партий с дебютом ({n} игр), винрейт {wr_pct}%."
                ),
                "info",
                80,
                Some("Доля партий".to_string()),
                Some(format!("{share_pct}%")),
                Some(share_pct as f64),
                Some("Расширь репертуар, чтобы не быть предсказуемым.".to_string()),
                sk,
                100,
                json!({
                    "opening": name,
                    "share_pct": share_pct,
                    "n": n,
                    "wr_pct": wr_pct
                }),
            ));
        }
    }

    out
}
