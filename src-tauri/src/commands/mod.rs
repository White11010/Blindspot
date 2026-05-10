// Thin `#[tauri::command]` handlers registered in `lib.rs`; each delegates to `services` or `db` via one call.
pub mod app;
pub mod auth;
pub mod engine;
pub mod game_analysis;
pub mod games;
pub mod insights;
pub mod player_profile;
pub mod users;
pub mod versus;
