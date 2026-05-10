// DDL for daily insight pick history (dedupe by user+date primary key).
use rusqlite::Connection;

/// Creates `insight_day_history` and an index for listing recent picks by user and date descending.
pub fn init_insight_day_history_table(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS insight_day_history (
            user_id TEXT NOT NULL,
            date TEXT NOT NULL,
            insight_id TEXT NOT NULL,
            category TEXT NOT NULL,
            PRIMARY KEY (user_id, date)
        );

        CREATE INDEX IF NOT EXISTS idx_insight_day_history_user_date
        ON insight_day_history(user_id, date DESC);
        ",
    )
    .unwrap();
}
