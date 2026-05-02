use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;

#[derive(Default)]
struct OpeningStats {
    games: i64,
    wins: i64,
    losses: i64,
    draws: i64,
}

pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut map: HashMap<String, OpeningStats> = HashMap::new();

    for game in games {
        let Some(name) = game.opening_name.clone() else {
            continue;
        };

        if name.trim().is_empty() {
            continue;
        }

        let stats = map.entry(name).or_default();

        stats.games += 1;

        match game.player_result.as_str() {
            "win" => stats.wins += 1,
            "loss" => stats.losses += 1,
            _ => stats.draws += 1,
        }
    }

    let mut best: Option<(String, f64, i64)> = None;
    let mut worst: Option<(String, f64, i64)> = None;

    for (name, stats) in map {
        if stats.games < 5 {
            continue;
        }

        let score = (stats.wins as f64 + stats.draws as f64 * 0.5) / stats.games as f64;

        match &best {
            None => best = Some((name.clone(), score, stats.games)),
            Some((_, current, _)) if score > *current => {
                best = Some((name.clone(), score, stats.games))
            }
            _ => {}
        }

        match &worst {
            None => worst = Some((name.clone(), score, stats.games)),
            Some((_, current, _)) if score < *current => {
                worst = Some((name.clone(), score, stats.games))
            }
            _ => {}
        }
    }

    let mut items = Vec::new();

    if let Some((name, score, games)) = best {
        items.push(make_best(user_id, &name, score, games));
    }

    if let Some((name, score, games)) = worst {
        items.push(make_worst(user_id, &name, score, games));
    }

    items
}

fn make_best(user_id: &str, opening: &str, score: f64, games: i64) -> Insight {
    let percent = (score * 100.0).round() as i64;

    Insight {
        id: format!("best-opening-{}", user_id),

        user_id: user_id.to_string(),

        kind: "best_opening".to_string(),

        title: "Лучший дебют".to_string(),

        summary: format!("{} приносит лучший результат.", opening),

        severity: "good".to_string(),

        confidence: confidence(games),

        metric_label: Some("Score".to_string()),
        metric_value: Some(format!("{}% ({} игр)", percent, games)),

        recommendation: Some("Используй чаще этот дебют.".to_string()),

        payload_json: None,

        created_at: now(),
        expires_at: None,
    }
}

fn make_worst(user_id: &str, opening: &str, score: f64, games: i64) -> Insight {
    let percent = (score * 100.0).round() as i64;

    Insight {
        id: format!("worst-opening-{}", user_id),

        user_id: user_id.to_string(),

        kind: "worst_opening".to_string(),

        title: "Проблемный дебют".to_string(),

        summary: format!("{} даёт худшие результаты.", opening),

        severity: "warning".to_string(),

        confidence: confidence(games),

        metric_label: Some("Score".to_string()),
        metric_value: Some(format!("{}% ({} игр)", percent, games)),

        recommendation: Some("Разбери типовые позиции этого дебюта.".to_string()),

        payload_json: None,

        created_at: now(),
        expires_at: None,
    }
}

fn confidence(games: i64) -> i64 {
    if games >= 30 {
        95
    } else if games >= 15 {
        85
    } else if games >= 10 {
        75
    } else {
        65
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
