use rusqlite::{params, Connection};

use super::model::Insight;

pub fn replace_user_insights(
    conn: &Connection,
    user_id: &str,
    items: &[Insight],
) -> rusqlite::Result<()> {
    let tx = conn.unchecked_transaction()?;

    tx.execute("DELETE FROM insights WHERE user_id = ?1", [user_id])?;

    {
        let mut stmt = tx.prepare(
            "
            INSERT INTO insights (
                id,
                user_id,
                kind,

                title,
                summary,

                severity,

                confidence,

                metric_label,
                metric_value,

                recommendation,

                payload_json,

                category,
                metric_number,
                metric_prev,

                sort_priority,

                created_at,
                expires_at
            )
            VALUES (
                ?1, ?2, ?3,
                ?4, ?5,
                ?6,
                ?7,
                ?8, ?9,
                ?10,
                ?11,
                ?12, ?13, ?14,
                ?15,
                ?16, ?17
            )
            ",
        )?;

        for item in items {
            stmt.execute(params![
                item.id,
                item.user_id,
                item.kind,
                item.title,
                item.summary,
                item.severity,
                item.confidence,
                item.metric_label,
                item.metric_value,
                item.recommendation,
                item.payload_json,
                item.category,
                item.metric_number,
                item.metric_prev,
                item.sort_priority,
                item.created_at,
                item.expires_at,
            ])?;
        }
    }

    tx.commit()
}

pub fn get_user_insights(conn: &Connection, user_id: &str) -> rusqlite::Result<Vec<Insight>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            user_id,
            kind,

            title,
            summary,

            severity,

            confidence,

            metric_label,
            metric_value,

            recommendation,

            payload_json,

            category,
            metric_number,
            metric_prev,

            sort_priority,

            created_at,
            expires_at
        FROM insights
        WHERE user_id = ?1
        ORDER BY sort_priority DESC, confidence DESC, created_at DESC
        ",
    )?;

    let rows = stmt.query_map([user_id], |row| {
        Ok(Insight {
            id: row.get(0)?,
            user_id: row.get(1)?,
            kind: row.get(2)?,

            title: row.get(3)?,
            summary: row.get(4)?,

            severity: row.get(5)?,

            confidence: row.get(6)?,

            metric_label: row.get(7)?,
            metric_value: row.get(8)?,

            recommendation: row.get(9)?,

            payload_json: row.get(10)?,

            category: row.get(11)?,
            metric_number: row.get(12)?,
            metric_prev: row.get(13)?,

            sort_priority: row.get(14)?,

            created_at: row.get(15)?,
            expires_at: row.get(16)?,
        })
    })?;

    let mut items = Vec::new();

    for row in rows {
        items.push(row?);
    }

    Ok(items)
}
