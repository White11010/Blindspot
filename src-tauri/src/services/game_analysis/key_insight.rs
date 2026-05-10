// Picks a single headline insight from pattern tags and worst swing when persisting analysis for the game hero card.
use crate::db::games::model::Game;

use super::classifier::ClassifiedMove;
use super::model::KeyInsight;
use super::pattern_detector::PatternTag;

/// Priority-ordered narrative: decisive tags first, else worst blunder swing, else generic neutral copy.
pub fn build_key_insight(
    game: &Game,
    tags: &[PatternTag],
    classified: &[ClassifiedMove],
    max_adv: i32,
) -> KeyInsight {
    let worst = classified
        .iter()
        .filter(|c| c.swing_cp < 0)
        .min_by_key(|c| c.swing_cp);

    let tag_set: std::collections::HashSet<&str> =
        tags.iter().map(|(t, _)| t.as_str()).collect();

    if tag_set.contains("lost_winning_position") && game.player_result == "loss" {
        let pawns = max_adv as f64 / 100.0;
        return KeyInsight {
            title: "You lost a winning position".into(),
            description: format!(
                "You had about +{:.1} advantage but lost the game.",
                pawns
            ),
            severity: "high".into(),
            kind: "lost_winning_position".into(),
        };
    }

    if tag_set.contains("missed_winning_chance") {
        return KeyInsight {
            title: "Missed a decisive chance".into(),
            description: "You reached a winning eval but did not convert.".into(),
            severity: "high".into(),
            kind: "missed_winning_chance".into(),
        };
    }

    if tag_set.contains("comeback_win") {
        return KeyInsight {
            title: "Great comeback".into(),
            description: "You were worse on the clock/eval but won.".into(),
            severity: "good".into(),
            kind: "comeback_win".into(),
        };
    }

    if tag_set.contains("multiple_blunders") {
        return KeyInsight {
            title: "Several critical slips".into(),
            description: "Multiple blunders shaped this game.".into(),
            severity: "warning".into(),
            kind: "multiple_blunders".into(),
        };
    }

    if tag_set.contains("middlegame_blunder") {
        return KeyInsight {
            title: "Middlegame turning point".into(),
            description: "A blunder in the middlegame changed the evaluation.".into(),
            severity: "warning".into(),
            kind: "middlegame_blunder".into(),
        };
    }

    if tag_set.contains("opening_blunder") {
        return KeyInsight {
            title: "Early inaccuracy".into(),
            description: "The opening phase contained a serious mistake.".into(),
            severity: "warning".into(),
            kind: "opening_blunder".into(),
        };
    }

    if tag_set.contains("endgame_blunder") {
        return KeyInsight {
            title: "Endgame slip".into(),
            description: "A late blunder affected the result.".into(),
            severity: "warning".into(),
            kind: "endgame_blunder".into(),
        };
    }

    if tag_set.contains("slow_drift") {
        return KeyInsight {
            title: "Quiet deterioration".into(),
            description: "No single huge blunder, but small errors added up.".into(),
            severity: "info".into(),
            kind: "slow_drift".into(),
        };
    }

    if let Some(w) = worst {
        let pawns = (-w.swing_cp) as f64 / 100.0;
        return KeyInsight {
            title: "Key moment decided the game".into(),
            description: format!(
                "Largest slip around move {} (~{:.1} pawns).",
                w.display_move_number, pawns
            ),
            severity: "info".into(),
            kind: "critical_moment".into(),
        };
    }

    KeyInsight {
        title: "Solid game".into(),
        description: "No major tactical swings detected.".into(),
        severity: "info".into(),
        kind: "neutral".into(),
    }
}
