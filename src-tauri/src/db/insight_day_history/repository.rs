use rusqlite::{params, Connection, OptionalExtension};

/// `date` is calendar day `YYYY-MM-DD` (local).
pub fn get_pick_for_date(
    conn: &Connection,
    user_id: &str,
    date: &str,
) -> rusqlite::Result<Option<(String, String)>> {
    let row: Option<(String, String)> = conn
        .query_row(
            "
            SELECT insight_id, category
            FROM insight_day_history
            WHERE user_id = ?1 AND date = ?2
            ",
            params![user_id, date],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;
    Ok(row)
}

pub fn insert_pick(
    conn: &Connection,
    user_id: &str,
    date: &str,
    insight_id: &str,
    category: &str,
) -> rusqlite::Result<usize> {
    conn.execute(
        "
        INSERT OR REPLACE INTO insight_day_history (user_id, date, insight_id, category)
        VALUES (?1, ?2, ?3, ?4)
        ",
        params![user_id, date, insight_id, category],
    )
}

pub fn delete_pick_for_date(conn: &Connection, user_id: &str, date: &str) -> rusqlite::Result<usize> {
    conn.execute(
        "DELETE FROM insight_day_history WHERE user_id = ?1 AND date = ?2",
        params![user_id, date],
    )
}

/// Recent picks from `from_date` inclusive (typically today minus 7 days).
pub fn list_history_since(
    conn: &Connection,
    user_id: &str,
    from_date: &str,
) -> rusqlite::Result<Vec<(String, String, String)>> {
    let mut stmt = conn.prepare(
        "
        SELECT date, insight_id, category
        FROM insight_day_history
        WHERE user_id = ?1 AND date >= ?2
        ORDER BY date DESC
        ",
    )?;
    let rows = stmt.query_map(params![user_id, from_date], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;
    let mut v = Vec::new();
    for r in rows {
        v.push(r?);
    }
    Ok(v)
}
