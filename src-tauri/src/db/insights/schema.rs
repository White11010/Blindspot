// DDL for `insights` (one row per card) with indexes for list ordering by user, kind, category, and priority.
use rusqlite::Connection;

/// Creates `insights` and indexes used when listing and sorting cards for the Insights UI.
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

            category TEXT NOT NULL,

            metric_number REAL,
            metric_prev REAL,

            sort_priority INTEGER NOT NULL DEFAULT 0,

            created_at INTEGER NOT NULL,
            expires_at INTEGER
        );

        CREATE INDEX IF NOT EXISTS idx_insights_user_id
        ON insights(user_id);

        CREATE INDEX IF NOT EXISTS idx_insights_kind
        ON insights(kind);

        CREATE INDEX IF NOT EXISTS idx_insights_category
        ON insights(category);

        CREATE INDEX IF NOT EXISTS idx_insights_sort_priority
        ON insights(sort_priority DESC);

        CREATE INDEX IF NOT EXISTS idx_insights_created_at
        ON insights(created_at DESC);
        ",
    )
    .unwrap();
}
