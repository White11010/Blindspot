use rusqlite::{params, Connection};

use super::model::{GameAnalysisRow, KeyMomentRow, PatternTagRow};

#[derive(Debug, Clone)]
pub struct GameAnalysisStored {
    pub analysis: GameAnalysisRow,
    pub tags: Vec<PatternTagRow>,
    #[allow(dead_code)]
    pub moment_rows: Vec<KeyMomentRow>,
}

pub fn upsert_analysis(
    conn: &Connection,
    analysis: &GameAnalysisRow,
    tags: &[PatternTagRow],
    moments: &[KeyMomentRow],
) -> rusqlite::Result<()> {
    let tx = conn.unchecked_transaction()?;

    tx.execute(
        "DELETE FROM game_pattern_tags WHERE game_id = ?1",
        [&analysis.game_id],
    )?;
    tx.execute(
        "DELETE FROM game_key_moments WHERE game_id = ?1",
        [&analysis.game_id],
    )?;

    tx.execute(
        "
        INSERT OR REPLACE INTO game_analyses (
            game_id, user_id, status, depth,
            accuracy, avg_centipawn_loss, max_advantage_cp, min_advantage_cp,
            blunders, mistakes, inaccuracies,
            eval_history_json, key_moments_json, key_insight_json, system_connection_json,
            created_at, updated_at, error
        ) VALUES (
            ?1, ?2, ?3, ?4,
            ?5, ?6, ?7, ?8,
            ?9, ?10, ?11,
            ?12, ?13, ?14, ?15,
            ?16, ?17, ?18
        )
        ",
        params![
            analysis.game_id,
            analysis.user_id,
            analysis.status,
            analysis.depth,
            analysis.accuracy,
            analysis.avg_centipawn_loss,
            analysis.max_advantage_cp,
            analysis.min_advantage_cp,
            analysis.blunders,
            analysis.mistakes,
            analysis.inaccuracies,
            analysis.eval_history_json,
            analysis.key_moments_json,
            analysis.key_insight_json,
            analysis.system_connection_json,
            analysis.created_at,
            analysis.updated_at,
            analysis.error,
        ],
    )?;

    {
        let mut stmt = tx.prepare(
            "
            INSERT INTO game_pattern_tags (game_id, user_id, tag, weight)
            VALUES (?1, ?2, ?3, ?4)
            ",
        )?;
        for t in tags {
            stmt.execute(params![t.game_id, t.user_id, t.tag, t.weight])?;
        }
    }

    {
        let mut stmt = tx.prepare(
            "
            INSERT INTO game_key_moments (game_id, user_id, ply, kind, swing_cp)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
        )?;
        for m in moments {
            stmt.execute(params![
                m.game_id,
                m.user_id,
                m.ply,
                m.kind,
                m.swing_cp
            ])?;
        }
    }

    tx.commit()
}

pub fn update_system_connection_json(
    conn: &Connection,
    game_id: &str,
    json: &str,
    updated_at: i64,
) -> rusqlite::Result<usize> {
    conn.execute(
        "UPDATE game_analyses SET system_connection_json = ?2, updated_at = ?3 WHERE game_id = ?1",
        params![game_id, json, updated_at],
    )
}

pub fn get_analysis_stored(conn: &Connection, game_id: &str) -> rusqlite::Result<Option<GameAnalysisStored>> {
    let analysis: Option<GameAnalysisRow> = {
        let mut stmt = conn.prepare(
            "
            SELECT
                game_id, user_id, status, depth,
                accuracy, avg_centipawn_loss, max_advantage_cp, min_advantage_cp,
                blunders, mistakes, inaccuracies,
                eval_history_json, key_moments_json, key_insight_json, system_connection_json,
                created_at, updated_at, error
            FROM game_analyses
            WHERE game_id = ?1
            ",
        )?;

        let row = stmt.query_row([game_id], |row| {
            Ok(GameAnalysisRow {
                game_id: row.get(0)?,
                user_id: row.get(1)?,
                status: row.get(2)?,
                depth: row.get(3)?,
                accuracy: row.get(4)?,
                avg_centipawn_loss: row.get(5)?,
                max_advantage_cp: row.get(6)?,
                min_advantage_cp: row.get(7)?,
                blunders: row.get(8)?,
                mistakes: row.get(9)?,
                inaccuracies: row.get(10)?,
                eval_history_json: row.get(11)?,
                key_moments_json: row.get(12)?,
                key_insight_json: row.get(13)?,
                system_connection_json: row.get(14)?,
                created_at: row.get(15)?,
                updated_at: row.get(16)?,
                error: row.get(17)?,
            })
        });

        match row {
            Ok(a) => Some(a),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => return Err(e),
        }
    };

    let Some(analysis) = analysis else {
        return Ok(None);
    };

    let tags = load_tags(conn, game_id)?;
    let moment_rows = load_moment_rows(conn, game_id)?;

    Ok(Some(GameAnalysisStored {
        analysis,
        tags,
        moment_rows,
    }))
}

fn load_tags(conn: &Connection, game_id: &str) -> rusqlite::Result<Vec<PatternTagRow>> {
    let mut stmt = conn.prepare(
        "SELECT game_id, user_id, tag, weight FROM game_pattern_tags WHERE game_id = ?1",
    )?;
    let rows = stmt.query_map([game_id], |row| {
        Ok(PatternTagRow {
            game_id: row.get(0)?,
            user_id: row.get(1)?,
            tag: row.get(2)?,
            weight: row.get(3)?,
        })
    })?;
    let mut v = Vec::new();
    for r in rows {
        v.push(r?);
    }
    Ok(v)
}

fn load_moment_rows(conn: &Connection, game_id: &str) -> rusqlite::Result<Vec<KeyMomentRow>> {
    let mut stmt = conn.prepare(
        "SELECT game_id, user_id, ply, kind, swing_cp FROM game_key_moments WHERE game_id = ?1 ORDER BY ply",
    )?;
    let rows = stmt.query_map([game_id], |row| {
        Ok(KeyMomentRow {
            game_id: row.get(0)?,
            user_id: row.get(1)?,
            ply: row.get(2)?,
            kind: row.get(3)?,
            swing_cp: row.get(4)?,
        })
    })?;
    let mut v = Vec::new();
    for r in rows {
        v.push(r?);
    }
    Ok(v)
}

/// Games needing analysis: no row yet, or status is not `done` (e.g. failed run).
pub fn count_pending_games(conn: &Connection, username: &str) -> rusqlite::Result<i64> {
    let n: i64 = conn.query_row(
        "
        SELECT COUNT(*)
        FROM games g
        LEFT JOIN game_analyses ga ON g.id = ga.game_id
        WHERE g.username = ?1 AND (ga.game_id IS NULL OR ga.status != 'done')
        ",
        [username],
        |row| row.get(0),
    )?;
    Ok(n)
}

pub fn get_pending_game_ids(
    conn: &Connection,
    username: &str,
    limit: u32,
) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "
        SELECT g.id
        FROM games g
        LEFT JOIN game_analyses ga ON g.id = ga.game_id
        WHERE g.username = ?1 AND (ga.game_id IS NULL OR ga.status != 'done')
        ORDER BY g.created_at ASC
        LIMIT ?2
        ",
    )?;
    let rows = stmt.query_map(params![username, limit], |row| row.get::<_, String>(0))?;
    let mut v = Vec::new();
    for r in rows {
        v.push(r?);
    }
    Ok(v)
}

/// Count games in the last `n_recent` by date (for username) that have `tag` for `user_id`.
pub fn count_similar_in_recent(
    conn: &Connection,
    username: &str,
    user_id: &str,
    tag: &str,
    n_recent: u32,
) -> rusqlite::Result<i64> {
    let n = i64::from(n_recent);
    let count: i64 = conn.query_row(
        "
        WITH recent AS (
            SELECT id FROM games WHERE username = ?1 ORDER BY created_at DESC LIMIT ?2
        )
        SELECT COUNT(*) FROM recent r
        INNER JOIN game_pattern_tags t ON t.game_id = r.id AND t.user_id = ?3 AND t.tag = ?4
        ",
        params![username, n, user_id, tag],
        |row| row.get(0),
    )?;
    Ok(count)
}

/// Other games with overlapping pattern tags, ranked by sum of min(weights) per tag.
pub fn find_similar_by_tags(
    conn: &Connection,
    user_id: &str,
    game_id: &str,
    limit: u32,
) -> rusqlite::Result<Vec<String>> {
    let lim = i64::from(limit);
    let mut stmt = conn.prepare(
        "
        SELECT t2.game_id AS gid,
               SUM(CASE WHEN t1.weight < t2.weight THEN t1.weight ELSE t2.weight END) AS score
        FROM game_pattern_tags t1
        INNER JOIN game_pattern_tags t2
            ON t1.tag = t2.tag AND t1.user_id = t2.user_id AND t2.game_id != t1.game_id
        INNER JOIN games g ON g.id = t2.game_id
        WHERE t1.game_id = ?1 AND t1.user_id = ?2
        GROUP BY t2.game_id
        ORDER BY score DESC, g.created_at DESC
        LIMIT ?3
        ",
    )?;
    let rows = stmt.query_map(params![game_id, user_id, lim], |row| row.get::<_, String>(0))?;
    let mut v = Vec::new();
    for r in rows {
        v.push(r?);
    }
    Ok(v)
}

/// Games where the same moment `kind` appears (from game_key_moments), excluding `game_id`.
pub fn find_similar_by_moment_kind(
    conn: &Connection,
    user_id: &str,
    game_id: &str,
    kind: &str,
    limit: u32,
) -> rusqlite::Result<Vec<String>> {
    let lim = i64::from(limit);
    let mut stmt = conn.prepare(
        "
        SELECT m2.game_id
        FROM game_key_moments m2
        INNER JOIN games g ON g.id = m2.game_id
        WHERE m2.user_id = ?1 AND m2.kind = ?2 AND m2.game_id != ?3
        GROUP BY m2.game_id
        ORDER BY MAX(g.created_at) DESC
        LIMIT ?4
        ",
    )?;
    let rows = stmt.query_map(params![user_id, kind, game_id, lim], |row| row.get::<_, String>(0))?;
    let mut v = Vec::new();
    for r in rows {
        v.push(r?);
    }
    Ok(v)
}
