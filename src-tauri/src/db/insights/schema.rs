use rusqlite::Connection;

pub fn init_insights_table(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS insights (
            id TEXT PRIMARY KEY,

            user_id TEXT NOT NULL,

            kind TEXT NOT NULL,

            title TEXT NOT NULL,
            summary TEXT NOT NULL,

            severity TEXT NOT NULL,

            confidence INTEGER NOT NULL,

            metric_label TEXT,
            metric_value TEXT,

            recommendation TEXT,

            payload_json TEXT,

            created_at INTEGER NOT NULL,
            expires_at INTEGER
        );

        CREATE INDEX IF NOT EXISTS idx_insights_user_id
        ON insights(user_id);

        CREATE INDEX IF NOT EXISTS idx_insights_kind
        ON insights(kind);

        CREATE INDEX IF NOT EXISTS idx_insights_created_at
        ON insights(created_at DESC);
        ",
    )
    .unwrap();
}
