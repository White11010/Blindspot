//! Ply counting from Lichess `moves` strings vs eval history length; used by insights that split opening/mid/end by move count.

#[inline]
fn is_file(b: u8) -> bool {
    (b'a'..=b'h').contains(&b)
}

#[inline]
fn is_rank(b: u8) -> bool {
    (b'1'..=b'8').contains(&b)
}

/// Single UCI token (`e2e4`, `e7e8q`), not PGN comments or clock tags.
pub fn is_uci_token(token: &str) -> bool {
    let b = token.as_bytes();
    match b.len() {
        4 => is_file(b[0]) && is_rank(b[1]) && is_file(b[2]) && is_rank(b[3]),
        5 => {
            is_file(b[0])
                && is_rank(b[1])
                && is_file(b[2])
                && is_rank(b[3])
                && matches!(b[4], b'q' | b'r' | b'b' | b'n')
        }
        _ => false,
    }
}

/// Counts half-moves by scanning only UCI-like tokens.
pub fn count_uci_halfmoves_from_moves_str(moves: Option<&str>) -> i64 {
    let s = moves.map(str::trim).unwrap_or("");
    if s.is_empty() {
        return 0;
    }
    s.split_whitespace().filter(|t| is_uci_token(t)).count() as i64
}

/// Total plies: prefer `eval_history.len() - 1` when analysis is present, else UCI token count.
pub fn total_halfmoves(moves: Option<&str>, eval_history_len: Option<usize>) -> i64 {
    if let Some(n) = eval_history_len {
        if n >= 2 {
            return (n - 1) as i64;
        }
    }
    count_uci_halfmoves_from_moves_str(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_only_uci_tokens() {
        let s = "e2e4 c7c5 g1f3 {%clk 0:01:00} d7d6";
        assert_eq!(count_uci_halfmoves_from_moves_str(Some(s)), 4);
    }

    #[test]
    fn prefers_eval_history_length() {
        assert_eq!(total_halfmoves(Some("e2e4 e7e5"), Some(6)), 5);
    }

    #[test]
    fn empty_moves_no_eval() {
        assert_eq!(total_halfmoves(None, None), 0);
    }
}
