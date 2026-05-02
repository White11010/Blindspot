use rusqlite::{params, Connection};

use crate::models::game::GameRecord;

pub fn insert_game(
    conn: &Connection,
    game: &GameRecord,
) -> rusqlite::Result<usize> {
    conn.execute(
        "
        INSERT OR IGNORE INTO games (
            id,

            username,

            rated,
            speed,
            time_control,
            created_at,

            player_name,
            player_id,

            opponent_name,
            opponent_id,

            white_name,
            white_id,

            black_name,
            black_id,

            white_rating,
            black_rating,

            player_rating,
            opponent_rating,

            winner,
            player_color,
            player_result,

            opening_eco,
            opening_name,

            pgn
        )
        VALUES (
            ?1,  ?2,  ?3,  ?4,  ?5,  ?6,
            ?7,  ?8,  ?9,  ?10,
            ?11, ?12, ?13, ?14,
            ?15, ?16, ?17, ?18,
            ?19, ?20, ?21,
            ?22, ?23,
            ?24
        )
        ",
        params![
            game.id,

            game.username,

            game.rated,
            game.speed,
            game.time_control,
            game.created_at,

            game.player_name,
            game.player_id,

            game.opponent_name,
            game.opponent_id,

            game.white_name,
            game.white_id,

            game.black_name,
            game.black_id,

            game.white_rating,
            game.black_rating,

            game.player_rating,
            game.opponent_rating,

            game.winner,
            game.player_color,
            game.player_result,

            game.opening_eco,
            game.opening_name,

            game.pgn
        ],
    )
}

pub fn get_games_by_username(
    conn: &Connection,
    username: &str,
) -> rusqlite::Result<Vec<GameRecord>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id,

            username,

            rated,
            speed,
            time_control,
            created_at,

            player_name,
            player_id,

            opponent_name,
            opponent_id,

            white_name,
            white_id,

            black_name,
            black_id,

            white_rating,
            black_rating,

            player_rating,
            opponent_rating,

            winner,
            player_color,
            player_result,

            opening_eco,
            opening_name,

            pgn
        FROM games
        WHERE username = ?1
        ORDER BY created_at DESC
        LIMIT 100
        "
    )?;

    let rows = stmt.query_map([username], |row| {
        Ok(GameRecord {
            id: row.get(0)?,

            username: row.get(1)?,

            rated: row.get(2)?,
            speed: row.get(3)?,
            time_control: row.get(4)?,
            created_at: row.get(5)?,

            player_name: row.get(6)?,
            player_id: row.get(7)?,

            opponent_name: row.get(8)?,
            opponent_id: row.get(9)?,

            white_name: row.get(10)?,
            white_id: row.get(11)?,

            black_name: row.get(12)?,
            black_id: row.get(13)?,

            white_rating: row.get(14)?,
            black_rating: row.get(15)?,

            player_rating: row.get(16)?,
            opponent_rating: row.get(17)?,

            winner: row.get(18)?,
            player_color: row.get(19)?,
            player_result: row.get(20)?,

            opening_eco: row.get(21)?,
            opening_name: row.get(22)?,

            pgn: row.get(23)?,
        })
    })?;

    let mut games = Vec::new();

    for row in rows {
        games.push(row?);
    }

    Ok(games)
}

pub fn delete_all_games(
    conn: &Connection,
) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM games", [])
}