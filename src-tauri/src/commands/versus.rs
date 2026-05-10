// Versus compare page: long-running fetch+analyze pipeline (`services::versus`).
use tauri::AppHandle;

use crate::services::versus;

/// Full head-to-head payload for three speeds (`versus::versus_compare`); UI listens on `versus://progress` events.
#[tauri::command]
pub async fn versus_compare(
    app: AppHandle,
    opponent_username: String,
) -> Result<versus::VersusCompareResponse, String> {
    versus::versus_compare(app, opponent_username).await
}

/// Cooperative cancel for in-flight `versus_compare` (`versus::cancel_versus_compare`).
#[tauri::command]
pub fn versus_cancel_compare() {
    versus::cancel_versus_compare();
}
