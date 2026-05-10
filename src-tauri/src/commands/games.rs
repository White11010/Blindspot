// Games sync and list commands used by Pinia `gamesSync` / `games` stores (`invoke('sync_games' | 'get_games' | …)`).
use tauri::AppHandle;

use crate::db::games::model::GameListItem;
use crate::services::games;

/// Incremental Lichess import for the active user (`services::games::sync_games`).
#[tauri::command]
pub async fn sync_games(app: AppHandle) -> Result<games::SyncGamesResult, String> {
    games::sync_games(app).await
}

/// My Games grid; default limit is effectively unbounded so local-first UX does not truncate libraries without an explicit cap.
#[tauri::command]
pub fn get_games(app: AppHandle, limit: Option<u32>) -> Result<Vec<GameListItem>, String> {
    games::get_my_games(app, limit.unwrap_or(100000))
}

/// Fire-and-forget sync on app resume (`services::games::sync_games` in async_runtime) so the command returns immediately.
#[tauri::command]
pub fn refresh_games_background(app: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        let _ = games::sync_games(app).await;
    });

    Ok(())
}
