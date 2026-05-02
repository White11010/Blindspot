use rusqlite::Connection;

pub fn init_games_table(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS games (
            id TEXT PRIMARY KEY,

            -- чей локальный кабинет / чей sync
            username TEXT NOT NULL,

            -- базовые свойства партии
            rated INTEGER NOT NULL,
            speed TEXT NOT NULL,
            time_control TEXT NOT NULL,
            created_at INTEGER NOT NULL,

            -- игрок аккаунта
            player_name TEXT NOT NULL,
            player_id TEXT NOT NULL,

            -- оппонент
            opponent_name TEXT NOT NULL,
            opponent_id TEXT NOT NULL,

            -- стороны
            white_name TEXT NOT NULL,
            white_id TEXT NOT NULL,
            black_name TEXT NOT NULL,
            black_id TEXT NOT NULL,

            -- рейтинги
            white_rating INTEGER,
            black_rating INTEGER,
            player_rating INTEGER,
            opponent_rating INTEGER,

            -- результат
            winner TEXT,              -- white / black / null(draw)
            player_color TEXT NOT NULL, -- white / black
            player_result TEXT NOT NULL, -- win / loss / draw

            -- дебют
            opening_eco TEXT,
            opening_name TEXT,

            -- сырье
            pgn TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_games_username
        ON games(username);

        CREATE INDEX IF NOT EXISTS idx_games_created_at
        ON games(created_at DESC);

        CREATE INDEX IF NOT EXISTS idx_games_player_id
        ON games(player_id);

        CREATE INDEX IF NOT EXISTS idx_games_opponent_id
        ON games(opponent_id);

        CREATE INDEX IF NOT EXISTS idx_games_opening_eco
        ON games(opening_eco);

        CREATE INDEX IF NOT EXISTS idx_games_speed
        ON games(speed);
        ",
    )
    .unwrap();
}