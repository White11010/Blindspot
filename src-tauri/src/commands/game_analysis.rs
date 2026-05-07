use tauri::AppHandle;

use crate::services::game_analysis::model::{GameAnalysisFull, SimilarGames};
use crate::services::game_analysis::service;

#[tauri::command]
pub async fn analyze_game(
    app: AppHandle,
    game_id: String,
    depth: Option<u8>,
) -> Result<GameAnalysisFull, String> {
    tokio::task::spawn_blocking(move || service::analyze_game(&app, &game_id, depth))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn get_game_analysis(app: AppHandle, game_id: String) -> Result<Option<GameAnalysisFull>, String> {
    service::get_analysis(&app, &game_id)
}

#[tauri::command]
pub fn analyze_pending_games(app: AppHandle, depth: Option<u8>) -> Result<(), String> {
    service::analyze_pending_games(app, depth)
}

#[tauri::command]
pub fn cancel_pending_analysis() {
    service::cancel_pending_analysis();
}

#[tauri::command]
pub fn get_similar_games(app: AppHandle, game_id: String) -> Result<SimilarGames, String> {
    service::get_similar_games(&app, &game_id)
}
