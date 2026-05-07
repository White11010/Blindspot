use rusqlite::Connection;

pub fn init_game_analyses_tables(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS game_analyses (
            game_id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,

            status TEXT NOT NULL,
            depth INTEGER NOT NULL,

            accuracy REAL,
            avg_centipawn_loss REAL,
            max_advantage_cp INTEGER,
            min_advantage_cp INTEGER,
            blunders INTEGER,
            mistakes INTEGER,
            inaccuracies INTEGER,

            eval_history_json TEXT,
            key_moments_json TEXT,
            key_insight_json TEXT,
            system_connection_json TEXT,

            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            error TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_game_analyses_user_id
        ON game_analyses(user_id);

        CREATE INDEX IF NOT EXISTS idx_game_analyses_status
        ON game_analyses(status);

        CREATE TABLE IF NOT EXISTS game_pattern_tags (
            game_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            tag TEXT NOT NULL,
            weight INTEGER NOT NULL,
            PRIMARY KEY (game_id, tag)
        );

        CREATE INDEX IF NOT EXISTS idx_game_pattern_tags_user_tag
        ON game_pattern_tags(user_id, tag);

        CREATE INDEX IF NOT EXISTS idx_game_pattern_tags_tag
        ON game_pattern_tags(tag);

        CREATE TABLE IF NOT EXISTS game_key_moments (
            game_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            ply INTEGER NOT NULL,
            kind TEXT NOT NULL,
            swing_cp INTEGER NOT NULL,
            PRIMARY KEY (game_id, ply)
        );

        CREATE INDEX IF NOT EXISTS idx_game_key_moments_user_kind
        ON game_key_moments(user_id, kind);
        ",
    )
    .unwrap();
}
