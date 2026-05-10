// Game detail analysis: heavy CPU work off async runtime via `spawn_blocking` (`services::game_analysis::service`).
use tauri::AppHandle;

use crate::services::game_analysis::model::{GameAnalysisFull, SimilarGames};
use crate::services::game_analysis::service;

/// Runs full persist+tags analysis on the Rust thread pool so the WebView thread is not blocked by Stockfish.
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

/// Reads stored analysis JSON for the game detail page (`service::get_analysis`).
#[tauri::command]
pub fn get_game_analysis(app: AppHandle, game_id: String) -> Result<Option<GameAnalysisFull>, String> {
    service::get_analysis(&app, &game_id)
}

/// Starts background batch worker (`service::analyze_pending_games`); returns immediately with Ok or busy error.
#[tauri::command]
pub fn analyze_pending_games(app: AppHandle, depth: Option<u8>) -> Result<(), String> {
    service::analyze_pending_games(app, depth)
}

/// Sets cancel flag for the batch worker (`service::cancel_pending_analysis`).
#[tauri::command]
pub fn cancel_pending_analysis() {
    service::cancel_pending_analysis();
}

/// Tag/moment based similar game ids for the analysis tab (`service::get_similar_games`).
#[tauri::command]
pub fn get_similar_games(app: AppHandle, game_id: String) -> Result<SimilarGames, String> {
    service::get_similar_games(&app, &game_id)
}
