use rusqlite::Connection;

pub fn init_users_table(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            username TEXT NOT NULL,

            bullet_rating INTEGER,
            bullet_games INTEGER,

            blitz_rating INTEGER,
            blitz_games INTEGER,

            rapid_rating INTEGER,
            rapid_games INTEGER,

            classical_rating INTEGER,
            classical_games INTEGER,

            is_active INTEGER NOT NULL DEFAULT 0,

            UNIQUE(platform, username)
        );

        CREATE INDEX IF NOT EXISTS idx_users_active
        ON users(is_active);
        ",
    )
    .unwrap();
}