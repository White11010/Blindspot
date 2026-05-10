// Heuristic tags (weights) from eval curve + blunder phase histogram; consumed by insights, similar games, and UI badges.
use crate::db::games::model::Game;

use super::classifier::ClassifiedMove;

pub type PatternTag = (String, i64);

/// Emits weighted tags like opening_blunder or lost_winning_position using fixed cp thresholds and game outcome.
pub fn detect_patterns(
    game: &Game,
    eval_history: &[i32],
    classified: &[ClassifiedMove],
    accuracy: f64,
    acpl: f64,
) -> Vec<PatternTag> {
    let mut tags: Vec<PatternTag> = Vec::new();


    let max_adv = *eval_history.iter().max().unwrap_or(&0);
    let min_adv = *eval_history.iter().min().unwrap_or(&0);

    let lost = game.player_result == "loss";
    let won = game.player_result == "win";

    if max_adv >= 200 && lost {
        tags.push(("lost_winning_position".into(), 3));
    }

    if max_adv >= 500 && !won {
        tags.push(("missed_winning_chance".into(), 3));
    }

    if min_adv <= -200 && won {
        tags.push(("comeback_win".into(), 2));
    }

    let mut opening_bl = 0;
    let mut mid_bl = 0;
    let mut end_bl = 0;

    for c in classified {
        if c.kind != "blunder" {
            continue;
        }
        let ply = c.half_move_index + 1;
        // Fixed ply bands match insight generators so opening blunders align across features without FEN phase detection.
        if ply <= 10 {
            opening_bl += 1;
        } else if ply <= 25 {
            mid_bl += 1;
        } else {
            end_bl += 1;
        }
    }

    if opening_bl > 0 {
        tags.push(("opening_blunder".into(), 2));
    }
    if mid_bl > 0 {
        tags.push(("middlegame_blunder".into(), 2));
    }
    if end_bl > 0 {
        tags.push(("endgame_blunder".into(), 2));
    }

    let blunder_count = classified.iter().filter(|c| c.kind == "blunder").count();
    if blunder_count >= 2 {
        tags.push(("multiple_blunders".into(), 2));
    }

    let has_blunder = blunder_count > 0;
    if !has_blunder && acpl > 60.0 {
        tags.push(("slow_drift".into(), 1));
    }

    if accuracy < 60.0 {
        tags.push(("low_accuracy".into(), 2));
    }

    let halfmoves = eval_history.len().saturating_sub(1);
    if lost && halfmoves >= 40 {
        tags.push(("late_game_loss".into(), 2));
    }

    tags
}

/// Chooses the tag with max weight for recurring-pattern copy; ties broken by insertion order upstream.
pub fn primary_tag(tags: &[PatternTag]) -> Option<String> {
    tags.iter()
        .max_by_key(|(_, w)| *w)
        .map(|(t, _)| t.clone())
}
