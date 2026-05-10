// Insights list, regeneration, and daily hero card (`services::insights::service`).
use tauri::AppHandle;

use crate::db::insights::model::Insight;
use crate::services::insights::service;

/// All insight cards for Insights page (`service::get_for_active_user`).
#[tauri::command]
pub fn get_insights(app: AppHandle) -> Result<Vec<Insight>, String> {
    service::get_for_active_user(app)
}

/// Replaces insights in DB and clears today’s daily pick (`service::regenerate_for_active_user`).
#[tauri::command]
pub fn regenerate_insights(app: AppHandle) -> Result<Vec<Insight>, String> {
    service::regenerate_for_active_user(app)
}

/// Home hero card with rotation + persistence (`service::get_daily_insight_for_active_user`).
#[tauri::command]
pub fn get_daily_insight(app: AppHandle) -> Result<Option<Insight>, String> {
    service::get_daily_insight_for_active_user(app)
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
