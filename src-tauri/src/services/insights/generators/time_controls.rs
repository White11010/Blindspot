use std::collections::HashMap;

use chrono::{Local, TimeZone, Timelike};
use serde_json::json;

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::insights::insight_common::{build_insight, now_ms, CAT_TIME};
#[derive(Default)]
struct SpeedStats {
    games: i64,
    wins: i64,
    losses: i64,
    draws: i64,
}

fn win_rate(s: &SpeedStats) -> f64 {
    if s.games == 0 {
        return 0.0;
    }
    (s.wins as f64 + s.draws as f64 * 0.5) / s.games as f64
}

fn normalize_speed(speed: &str) -> String {
    match speed {
        "bullet" => "Bullet".to_string(),
        "blitz" => "Blitz".to_string(),
        "rapid" => "Rapid".to_string(),
        "classical" => "Classical".to_string(),
        other => other.to_string(),
    }
}

fn speed_subject(label: &str) -> String {
    format!("speed:{label}")
}

fn conf_speed(n: i64) -> i64 {
    if n >= 50 {
        95
    } else if n >= 25 {
        85
    } else if n >= 10 {
        70
    } else {
        50
    }
}

/// Strong / weak time control by win rate.
fn acc_bucket(b: &mut SpeedStats, g: &Game) {
    b.games += 1;
    match g.player_result.as_str() {
        "win" => b.wins += 1,
        "loss" => b.losses += 1,
        _ => b.draws += 1,
    }
}

pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut map: HashMap<String, SpeedStats> = HashMap::new();

    for game in games {
        let label = normalize_speed(&game.speed);
        let s = map.entry(label).or_default();
        s.games += 1;
        match game.player_result.as_str() {
            "win" => s.wins += 1,
            "loss" => s.losses += 1,
            _ => s.draws += 1,
        }
    }

    let mut best: Option<(String, f64, i64)> = None;
    let mut worst: Option<(String, f64, i64)> = None;

    for (label, s) in map {
        if s.games < 10 {
            continue;
        }
        let wr = win_rate(&s);
        match &best {
            None => best = Some((label.clone(), wr, s.games)),
            Some((_, w, _)) if wr > *w => best = Some((label.clone(), wr, s.games)),
            _ => {}
        }
        match &worst {
            None => worst = Some((label.clone(), wr, s.games)),
            Some((_, w, _)) if wr < *w => worst = Some((label.clone(), wr, s.games)),
            _ => {}
        }
    }

    let mut out = Vec::new();
    if let Some((label, wr, n)) = best {
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("time_best_{user_id}"),
            user_id,
            "time_control_best",
            CAT_TIME,
            "Сильный контроль времени".to_string(),
            format!("Лучший винрейт в {label}: {pct}% за {n} партий."),
            "good",
            conf_speed(n),
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Используй этот формат для рейтинговых целей.".to_string()),
            &speed_subject(&label),
            70,
            json!({ "speed_label": label, "pct": pct, "n": n }),
        ));
    }
    if let Some((label, wr, n)) = worst {
        let pct = (wr * 100.0).round();
        out.push(build_insight(
            format!("time_worst_{user_id}"),
            user_id,
            "time_control_worst",
            CAT_TIME,
            "Слабый контроль времени".to_string(),
            format!("Худший винрейт в {label}: {pct}% за {n} партий."),
            "warning",
            conf_speed(n),
            Some("Винрейт".to_string()),
            Some(format!("{pct}%")),
            Some(pct as f64),
            Some("Добавь тренировок в этом темпе.".to_string()),
            &speed_subject(&label),
            68,
            json!({ "speed_label": label, "pct": pct, "n": n }),
        ));
    }

    out.extend(rating_growth_30d(user_id, games));
    out.extend(morning_vs_evening(user_id, games));

    out
}

fn rating_growth_30d(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let now = now_ms();
    let window: i64 = 30 * 24 * 60 * 60 * 1000;
    let cutoff = now.saturating_sub(window);

    let mut by_speed: HashMap<String, Vec<&Game>> = HashMap::new();
    for g in games {
        if g.created_at < cutoff {
            continue;
        }
        let label = normalize_speed(&g.speed);
        by_speed.entry(label).or_default().push(g);
    }

    let mut best: Option<(String, i64, i64)> = None; // label, delta, n_games

    for (label, mut list) in by_speed {
        list.sort_by_key(|g| g.created_at);
        let with_rating: Vec<i64> = list.iter().filter_map(|g| g.player_rating).collect();
        if with_rating.len() < 5 {
            continue;
        }
        let first = *with_rating.first().unwrap();
        let last = *with_rating.last().unwrap();
        let delta = last - first;
        match &best {
            None => best = Some((label, delta, list.len() as i64)),
            Some((_, d, _)) if delta > *d => best = Some((label, delta, list.len() as i64)),
            _ => {}
        }
    }

    let mut v = Vec::new();
    if let Some((label, delta, n)) = best {
        if delta > 0 {
            v.push(build_insight(
                format!("time_rating_growth_{user_id}"),
                user_id,
                "time_rating_growth_30d",
                CAT_TIME,
                "Рост рейтинга за 30 дней".to_string(),
                format!("Сильнее всего в {label}: +{delta} за 30 дней ({n} партий в выборке)."),
                "good",
                75,
                Some("Δ рейтинга".to_string()),
                Some(format!("+{delta}")),
                Some(delta as f64),
                Some("Продолжай закреплять прогресс в этом контроле.".to_string()),
                &speed_subject(&label),
                82,
                json!({ "label": label, "delta": delta, "n": n }),
            ));
        }
    }
    v
}

fn morning_vs_evening(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut morning = SpeedStats::default();
    let mut evening = SpeedStats::default();

    for g in games {
        let hour = Local
            .timestamp_millis_opt(g.created_at)
            .single()
            .map(|dt| dt.hour())
            .unwrap_or(12);

        if (6..12).contains(&hour) {
            acc_bucket(&mut morning, g);
        } else if (18..24).contains(&hour) {
            acc_bucket(&mut evening, g);
        }
    }

    if morning.games < 10 || evening.games < 10 {
        return vec![];
    }

    let wr_m = win_rate(&morning);
    let wr_e = win_rate(&evening);
    let gap = (wr_m - wr_e) * 100.0;
    if gap.abs() < 5.0 {
        return vec![];
    }

    let (better, wr_b, n_b, worse, wr_w, n_w) = if wr_m > wr_e {
        (
            "Утро (6–12, локальное время)",
            wr_m,
            morning.games,
            "Вечер (18–24)",
            wr_e,
            evening.games,
        )
    } else {
        (
            "Вечер (18–24, локальное время)",
            wr_e,
            evening.games,
            "Утро (6–12)",
            wr_m,
            morning.games,
        )
    };

    let pct_b = (wr_b * 100.0).round();
    let pct_w = (wr_w * 100.0).round();

    vec![build_insight(
        format!("time_dayparts_{user_id}"),
        user_id,
        "time_morning_vs_evening",
        CAT_TIME,
        "Время суток".to_string(),
        format!(
            "{better}: {pct_b}% ({n_b} игр) vs {worse}: {pct_w}% ({n_w} игр), разница {:.0} п.п.",
            gap.abs()
        ),
        "info",
        70,
        Some("Винрейт (лучший слот)".to_string()),
        Some(format!("{pct_b}%")),
        Some(pct_b as f64),
        Some("Учитывай усталость и концентрацию при планировании партий.".to_string()),
        "time_of_day:local",
        55,
        json!({
            "morning_better": wr_m > wr_e,
            "pct_b": pct_b,
            "n_b": n_b,
            "pct_w": pct_w,
            "n_w": n_w,
            "gap_pp": gap.abs().round()
        }),
    )]
}
