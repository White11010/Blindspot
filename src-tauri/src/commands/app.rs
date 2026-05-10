// Startup payload for Vue router guard (`services::app::bootstrap::bootstrap`).
use tauri::AppHandle;

/// Auth + user hydration used on first paint; may spawn background `sync_user` when returning stale cached user.
#[tauri::command]
pub async fn bootstrap(
    app: AppHandle,
) -> Result<crate::services::app::dto::BootstrapState, String> {
    crate::services::app::bootstrap::bootstrap(app).await
}
