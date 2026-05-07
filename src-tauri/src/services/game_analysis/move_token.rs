//! Lichess `moves` is often SAN (`d4`, `Nf3`); APIs may still use UCI (`d2d4`). Accept both.

use shakmaty::san::San;
use shakmaty::uci::UciMove;
use shakmaty::{Chess, Move, Position};

pub fn parse_move(pos: &Chess, raw: &str) -> Result<Move, String> {
    let token = raw.trim();
    if token.is_empty() {
        return Err("empty move token".into());
    }

    if let Ok(u) = UciMove::from_ascii(token.as_bytes()) {
        return u
            .to_move(pos)
            .map_err(|e| format!("illegal UCI {token}: {e}"));
    }

    let san = San::from_ascii(token.as_bytes())
        .map_err(|e| format!("invalid SAN/UCI '{token}': {e}"))?;
    san.to_move(pos).map_err(|e| format!("illegal SAN '{token}': {e}"))
}

pub fn play_token(pos: &mut Chess, raw: &str) -> Result<(), String> {
    let m = parse_move(pos, raw)?;
    pos.play_unchecked(&m);
    Ok(())
}

/// Canonical SAN for display (played move from standard chess startpos).
pub fn token_to_canonical_san(pos: &Chess, raw: &str) -> Result<String, String> {
    let m = parse_move(pos, raw)?;
    Ok(format!("{}", San::from_move(pos, &m)))
}
