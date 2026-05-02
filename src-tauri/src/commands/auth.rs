use tauri::AppHandle;

#[tauri::command]
pub async fn save_token(app: AppHandle, token: String) -> Result<(), String> {
    crate::services::auth::save_token(&app, &token)
}

#[tauri::command]
pub async fn load_token(app: AppHandle) -> Result<Option<String>, String> {
    crate::services::auth::load_token(&app)
}

#[tauri::command]
pub async fn delete_token(app: AppHandle) -> Result<(), String> {
    crate::services::auth::delete_token(&app)
}