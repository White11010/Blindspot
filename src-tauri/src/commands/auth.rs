// Keyring bridge invoked from Vue after Lichess OAuth; no business logic beyond `services::auth`.
use tauri::AppHandle;

/// Persists the Lichess API bearer token in the OS secure store (`services::auth::save_token`).
#[tauri::command]
pub async fn save_token(app: AppHandle, token: String) -> Result<(), String> {
    crate::services::auth::save_token(&app, &token)
}

/// Reads the saved token for gated API calls (`services::auth::load_token`).
#[tauri::command]
pub async fn load_token(app: AppHandle) -> Result<Option<String>, String> {
    crate::services::auth::load_token(&app)
}

/// Removes the token on disconnect (`services::auth::delete_token`).
#[tauri::command]
pub async fn delete_token(app: AppHandle) -> Result<(), String> {
    crate::services::auth::delete_token(&app)
}
