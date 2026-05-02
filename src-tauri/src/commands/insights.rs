use tauri::AppHandle;

use crate::db::insights::model::Insight;
use crate::services::insights::service;

#[tauri::command]
pub fn get_insights(app: AppHandle) -> Result<Vec<Insight>, String> {
    service::get_for_active_user(app)
}

#[tauri::command]
pub fn regenerate_insights(app: AppHandle) -> Result<Vec<Insight>, String> {
    service::regenerate_for_active_user(app)
}

// #[tauri::command]
// pub fn refresh_insights_background(app: AppHandle) -> Result<(), String> {
//     tauri::async_runtime::spawn(async move {
//         let _ = tokio::task::spawn_blocking(move || {
//             let _ = service::regenerate_for_active_user(app);
//         })
//         .await;
//     });

//     Ok(())
// }
