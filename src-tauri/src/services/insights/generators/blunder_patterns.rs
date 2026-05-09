use serde_json::json;

use crate::db::games::model::Game;
use crate::db::insights::model::Insight;
use crate::services::insights::insight_common::{build_insight, CAT_TACTICS};

#[derive(Default)]
struct SideStats {
    games: i64,
    wins: i64,
    losses: i64,
    draws: i64,
}

pub fn generate(user_id: &str, games: &[Game]) -> Vec<Insight> {
    let mut white = SideStats::default();
    let mut black = SideStats::default();

    for game in games {
        match game.player_color.as_str() {
            "white" => accumulate(&mut white, &game.player_result),
            "black" => accumulate(&mut black, &game.player_result),
            _ => {}
        }
    }

    let mut insights = Vec::new();

    if white.games > 10 && black.games > 10 {
        let white_wr = winrate(&white);
        let black_wr = winrate(&black);

        if (white_wr - black_wr).abs() >= 0.08 {
            let gap = (white_wr - black_wr) * 100.0;
            let gap_r = gap.round();
            if white_wr > black_wr {
                insights.push(build_insight(
                    format!("tactics_side_{user_id}"),
                    user_id,
                    "tactics_side_performance",
                    CAT_TACTICS,
                    "Ты сильнее играешь белыми".to_string(),
                    format!(
                        "Белые: {:.0}% ({} игр), чёрные: {:.0}% ({} игр).",
                        white_wr * 100.0,
                        white.games,
                        black_wr * 100.0,
                        black.games
                    ),
                    "info",
                    confidence(white.games + black.games),
                    Some("Разница винрейта, п.п.".to_string()),
                    Some(format!("+{gap_r}")),
                    Some(gap_r),
                    Some("Работай над дебютами за чёрных и защитой.".to_string()),
                    "side:white_vs_black",
                    58,
                    json!({
                        "white_wr": (white_wr * 100.0).round(),
                        "black_wr": (black_wr * 100.0).round(),
                        "white_games": white.games,
                        "black_games": black.games,
                        "gap_pp": gap_r,
                        "white_stronger": true
                    }),
                ));
            } else {
                let gap_r = (-gap).round();
                insights.push(build_insight(
                    format!("tactics_side_{user_id}"),
                    user_id,
                    "tactics_side_performance",
                    CAT_TACTICS,
                    "Проблема с игрой чёрными".to_string(),
                    format!(
                        "Белые: {:.0}%, чёрные: {:.0}% ({} / {} игр).",
                        white_wr * 100.0,
                        black_wr * 100.0,
                        white.games,
                        black.games
                    ),
                    "warning",
                    confidence(white.games + black.games),
                    Some("Разница винрейта, п.п.".to_string()),
                    Some(format!("-{gap_r}")),
                    Some(gap_r as f64),
                    Some("Чёрные требуют отдельной тренировочной фокусировки.".to_string()),
                    "side:white_vs_black",
                    58,
                    json!({
                        "white_wr": (white_wr * 100.0).round(),
                        "black_wr": (black_wr * 100.0).round(),
                        "white_games": white.games,
                        "black_games": black.games,
                        "gap_pp": gap_r,
                        "white_stronger": false
                    }),
                ));
            }
        }
    }

    insights
}

fn accumulate(stats: &mut SideStats, result: &str) {
    stats.games += 1;

    match result {
        "win" => stats.wins += 1,
        "loss" => stats.losses += 1,
        _ => stats.draws += 1,
    }
}

fn winrate(s: &SideStats) -> f64 {
    if s.games == 0 {
        return 0.0;
    }

    (s.wins as f64 + s.draws as f64 * 0.5) / s.games as f64
}

fn confidence(games: i64) -> i64 {
    if games >= 80 {
        95
    } else if games >= 40 {
        85
    } else if games >= 20 {
        75
    } else {
        60
    }
}
