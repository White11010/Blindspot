use tauri::AppHandle;

use crate::services::engine::stockfish::{
    ensure_engine_started, get_engine, EngineResult, StockfishEngine,
};

#[tauri::command]
pub fn init_engine(app: AppHandle) -> Result<(), String> {
    let engine = StockfishEngine::new(&app)?;

    let global = get_engine();
    let mut guard = global.lock().map_err(|_| "Failed to lock engine mutex")?;

    *guard = Some(engine);

    Ok(())
}

#[tauri::command]
pub fn analyze_position(app: AppHandle, fen: String, depth: Option<u8>) -> Result<EngineResult, String> {
    ensure_engine_started(&app)?;

    let global = get_engine();

    let mut guard = global.lock().map_err(|_| "Failed to lock engine mutex")?;

    let engine = guard.as_mut().expect("engine present after ensure_engine_started");

    engine.analyze(&fen, depth.unwrap_or(12))
}
