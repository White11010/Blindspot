// Active user profile sync from Lichess `/api/account` into SQLite; used on bootstrap and explicit refresh.
use tauri::AppHandle;

use crate::clients::lichess;
use crate::db::connection::get_conn;
use crate::db::users::model::User;
use crate::db::users::repository;

/// Fetches profile, upserts user, marks active when `AppHandle` is owned (e.g. spawned bootstrap task).
pub async fn sync_user(app: AppHandle) -> Result<User, String> {
    let profile = lichess::fetch_me(&app).await?;

    let user = User::from_lichess(profile);

    let conn = get_conn(&app)?;

    repository::upsert_user(&conn, &user)?;
    repository::set_active_user(&conn, &user.id)?;

    repository::get_active_user(&conn)?
        .ok_or_else(|| "Active user not found after sync".to_string())
}

/// Reads the active user from DB without hitting the network (home shell when token exists but no refresh requested).
pub fn get_me(app: &AppHandle) -> Result<Option<User>, String> {
    let conn = get_conn(app)?;
    repository::get_active_user(&conn)
}

/// Same as `sync_user` but takes `&AppHandle` for Tauri command handlers that only borrow the handle.
pub async fn sync_me(app: &AppHandle) -> Result<User, String> {
    let profile = lichess::fetch_me(app).await?;

    let user = User::from_lichess(profile);

    let conn = get_conn(app)?;
    repository::upsert_user(&conn, &user)?;
    repository::set_active_user(&conn, &user.id)?;

    repository::get_active_user(&conn)?
        .ok_or_else(|| "Active user not found after sync".to_string())
}
