// Second-pass JSON blob: counts how often the primary pattern tag appears in recent games for contextual copy variants.
use rusqlite::{params, Connection};

use crate::db::game_analyses::repository as ga_repo;

use super::model::SystemConnection;
use super::pattern_detector::primary_tag;

/// Computes similar-tag count in a `window`-game lookback plus optional win-rate note for i18n templates on the client.
pub fn build_system_connection(
    conn: &Connection,
    username: &str,
    user_id: &str,
    tags: &[(String, i64)],
) -> Result<SystemConnection, String> {
    let window: i32 = 10; // Small window keeps COUNT fast and matches recent-habit wording without stale data.
    if tags.is_empty() {
        return Ok(SystemConnection {
            text: String::new(),
            tag: "general".into(),
            count: 0,
            window,
            secondary_text: None,
            primary_variant: "no_tags".into(),
            secondary_variant: "none".into(),
            secondary_total: 0,
            secondary_wr_pct: 0.0,
        });
    }

    let tag = primary_tag(tags).unwrap_or_else(|| tags[0].0.clone());

    let count = ga_repo::count_similar_in_recent(conn, username, user_id, &tag, window as u32)
        .map_err(|e| e.to_string())?;

    let primary_variant = if count <= 1 {
        "similar_low"
    } else {
        "similar_high"
    };

    let (secondary_variant, secondary_total, secondary_wr_pct) =
        match winrate_note_data(conn, username, user_id, &tag) {
            Ok(WinrateNote::LowSample { total }) => ("revisit", total, 0.0),
            Ok(WinrateNote::WinRate { total, wr_pct }) => ("win_rate", total, wr_pct),
            Err(_) => ("none", 0, 0.0),
        };

    Ok(SystemConnection {
        text: String::new(),
        tag,
        count,
        window,
        secondary_text: None,
        primary_variant: primary_variant.into(),
        secondary_variant: secondary_variant.into(),
        secondary_total,
        secondary_wr_pct,
    })
}

enum WinrateNote {
    LowSample { total: i64 },
    WinRate { total: i64, wr_pct: f64 },
}

fn winrate_note_data(
    conn: &Connection,
    username: &str,
    user_id: &str,
    tag: &str,
) -> Result<WinrateNote, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "
        SELECT g.player_result, COUNT(*) as c
        FROM games g
        INNER JOIN game_pattern_tags t ON t.game_id = g.id AND t.user_id = ?1 AND t.tag = ?2
        WHERE g.username = ?3
        GROUP BY g.player_result
        ",
    )?;

    let rows = stmt.query_map(params![user_id, tag, username], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;

    let mut wins = 0i64;
    let mut losses = 0i64;
    let mut draws = 0i64;
    for r in rows {
        let (res, c) = r?;
        match res.as_str() {
            "win" => wins += c,
            "loss" => losses += c,
            _ => draws += c,
        }
    }

    let total = wins + losses + draws;
    if total < 5 {
        return Ok(WinrateNote::LowSample { total });
    }

    let wr = (wins as f64 + draws as f64 * 0.5) / total as f64 * 100.0;
    Ok(WinrateNote::WinRate {
        total,
        wr_pct: wr,
    })
}
