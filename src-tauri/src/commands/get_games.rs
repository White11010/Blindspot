use crate::db;
use crate::models::game::GameRecord;
use crate::db::connection::get_conn;

#[tauri::command]
pub fn get_games(username: String) -> Result<Vec<GameRecord>, String> {
    let conn = get_conn(app)?;
    db::games::repository::get_games_by_username(&conn, &username).map_err(|e| e.to_string())
}