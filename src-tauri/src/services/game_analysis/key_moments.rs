// Selects a few worst player moments and enriches them with FEN/SAN for the game detail timeline.
use shakmaty::fen::Fen;
use shakmaty::{Chess, EnPassantMode};

use super::classifier::ClassifiedMove;
use super::model::KeyMoment;
use super::move_token::{play_token, token_to_canonical_san};

fn position_at_prefix(uci_moves: &[String], up_to: usize) -> Result<Chess, String> {
    let mut pos = Chess::new();
    for (i, tok) in uci_moves.iter().take(up_to).enumerate() {
        play_token(&mut pos, tok).map_err(|reason| format!("prefix ply {} `{}`: {reason}", i + 1, tok))?;
    }
    Ok(pos)
}

/// Takes top `top_n` bad/brilliant swings by absolute swing, then rebuilds board prefix for SAN and headlines.
pub fn pick_key_moments(
    uci_moves: &[String],
    classified: &[ClassifiedMove],
    top_n: usize,
) -> Result<Vec<KeyMoment>, String> {
    let mut candidates: Vec<&ClassifiedMove> = classified
        .iter()
        .filter(|c| {
            matches!(
                c.kind,
                "blunder" | "mistake" | "inaccuracy" | "brilliant"
            )
        })
        .collect();

    candidates.sort_by_key(|c| std::cmp::Reverse(c.swing_cp.abs()));

    let mut moments = Vec::new();
    for c in candidates.into_iter().take(top_n) {
        let k = c.half_move_index;
        let pos_before = position_at_prefix(uci_moves, k)?;
        let fen_before = format!(
            "{}",
            Fen::from_position(pos_before.clone(), EnPassantMode::Legal)
        );

        let move_san = token_to_canonical_san(&pos_before, &c.uci).unwrap_or_else(|_| c.uci.clone());

        let best_move_san = c
            .best_move_uci
            .as_ref()
            .and_then(|b| token_to_canonical_san(&pos_before, b).ok());

        let fullmove = (k / 2) + 1;
        let ply = (k + 1) as i64;

        let lost_pawns = (-c.swing_cp).max(0) as f64 / 100.0;
        let headline = format!("Move {} — {}", fullmove, title_case(c.kind));
        let description = if c.swing_cp < -30 {
            format!("You lost ~{:.1} pawns of advantage here.", lost_pawns)
        } else if c.kind == "brilliant" {
            "You found a strong continuation in a difficult position.".to_string()
        } else {
            format!("Eval swing {} centipawns.", c.swing_cp)
        };

        moments.push(KeyMoment {
            ply,
            move_number: fullmove as i64,
            move_san,
            kind: c.kind.to_string(),
            eval_before: c.eval_before,
            eval_after: c.eval_after,
            swing_cp: c.swing_cp,
            best_move_san,
            fen_before,
            headline,
            description,
        });
    }

    Ok(moments)
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
