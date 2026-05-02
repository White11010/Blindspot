use tauri::AppHandle;

#[tauri::command]
pub async fn bootstrap(app: AppHandle) -> Result<crate::services::app::dto::BootstrapState, String> {
    crate::services::app::bootstrap::bootstrap(app).await
}