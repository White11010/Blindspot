// Active Lichess profile read/refresh from SQLite + `/api/account` (`services::users`).
use crate::db::users::model::User;
use tauri::AppHandle;

/// Cached active user row without network (`services::users::get_me`).
#[tauri::command]
pub async fn get_me(app: AppHandle) -> Result<Option<User>, String> {
    crate::services::users::get_me(&app)
}

/// Forces Lichess profile fetch and upserts DB (`services::users::sync_me`).
#[tauri::command]
pub async fn sync_me(app: AppHandle) -> Result<User, String> {
    crate::services::users::sync_me(&app).await
}
