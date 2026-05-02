use tauri::AppHandle;

use crate::clients::lichess;
use crate::db::connection::get_conn;
use crate::db::games::model::Game;
use crate::db::games::repository;
use crate::db::users::repository as users_repository;
use crate::parsers::lichess_games;

#[derive(Debug, serde::Serialize)]
pub struct SyncGamesResult {
    pub inserted: u32,
    pub username: String,
}

pub async fn sync_games(
    app: AppHandle,
) -> Result<SyncGamesResult, String> {
    let conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?
        .ok_or("Active user not found")?;

    let ndjson = lichess::fetch_games(
        &app,
        &user.username,
    )
    .await?;

    let games = lichess_games::parse_ndjson(
        &user.username,
        &ndjson,
    );

    let inserted = persist_games(
        &conn,
        games,
    )?;

    Ok(SyncGamesResult {
        inserted,
        username: user.username,
    })
}

pub fn get_my_games(
    app: AppHandle,
    limit: u32,
) -> Result<Vec<Game>, String> {
    let conn = get_conn(&app)?;

    let user = users_repository::get_active_user(&conn)?
        .ok_or("Active user not found")?;

    repository::get_games_by_username(
        &conn,
        &user.username,
        limit,
    )
    .map_err(|e| e.to_string())
}

fn persist_games(
    conn: &rusqlite::Connection,
    games: Vec<Game>,
) -> Result<u32, String> {
    let mut inserted = 0;

    for game in games {
        let rows = repository::upsert_game(
            conn,
            &game,
        )
        .map_err(|e| e.to_string())?;

        if rows > 0 {
            inserted += 1;
        }
    }

    Ok(inserted)
}