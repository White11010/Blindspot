use tauri::AppHandle;
use crate::db::users::model::User;

#[tauri::command]
pub async fn get_me(app: AppHandle) -> Result<Option<User>, String> {
    crate::services::users::get_me(&app)
}

#[tauri::command]
pub async fn sync_me(app: AppHandle) -> Result<User, String> {
    crate::services::users::sync_me(&app).await
}