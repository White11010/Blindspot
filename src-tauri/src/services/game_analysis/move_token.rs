//! Normalizes Lichess `moves` strings (SAN or UCI) into legal `shakmaty` moves for replay and SAN display in analysis.

use shakmaty::san::San;
use shakmaty::uci::UciMove;
use shakmaty::{Chess, Move, Position};

/// Parses a single token as UCI first, then SAN, so NDJSON exports work regardless of Lichess move format.
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

/// Applies one parsed move with `play_unchecked` after legality was validated in `parse_move`.
pub fn play_token(pos: &mut Chess, raw: &str) -> Result<(), String> {
    let m = parse_move(pos, raw)?;
    pos.play_unchecked(&m);
    Ok(())
}

/// Canonical SAN string for UI labels (key moments) from the position before the move was played.
pub fn token_to_canonical_san(pos: &Chess, raw: &str) -> Result<String, String> {
    let m = parse_move(pos, raw)?;
    Ok(format!("{}", San::from_move(pos, &m)))
}
