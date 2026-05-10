// Replays the game once, asking Stockfish at each half-move boundary to build a player-centric eval timeline.
use shakmaty::fen::Fen;
use shakmaty::{Chess, EnPassantMode, Position};

use crate::services::engine::stockfish::StockfishEngine;

use super::move_token::play_token;

/// Converts engine side-to-move centipawns to good-for-tracked-player so white/black share one sign convention.
pub fn to_player_cp(white_to_move: bool, cp_side_to_move: i32, player_is_white: bool) -> i32 {
    let good_for_white = if white_to_move {
        cp_side_to_move
    } else {
        -cp_side_to_move
    };
    if player_is_white {
        good_for_white
    } else {
        -good_for_white
    }
}

/// Builds `eval_history.len() == moves.len()+1` by analyzing FEN before each ply then applying the recorded token.
pub fn analyze_eval_history(
    engine: &mut StockfishEngine,
    uci_moves: &[String],
    player_is_white: bool,
    depth: u8,
) -> Result<Vec<i32>, String> {
    let mut history = Vec::with_capacity(uci_moves.len() + 1);
    let mut pos = Chess::new();

    for i in 0..=uci_moves.len() {
        let white_to_move = pos.turn().is_white();
        let fen_str = format!("{}", Fen::from_position(pos.clone(), EnPassantMode::Legal));
        let res = engine.analyze(&fen_str, depth)?;
        let cp_stm = res.eval.unwrap_or(0);
        let player_cp = to_player_cp(white_to_move, cp_stm, player_is_white);
        history.push(player_cp);

        if i < uci_moves.len() {
            play_token(&mut pos, &uci_moves[i]).map_err(|reason| {
                format!("ply {} token `{}`: {reason}", i + 1, uci_moves[i])
            })?;
        }
    }

    Ok(history)
}
