// OS keyring wrapper for the Lichess OAuth token; commands call this instead of persisting secrets in SQLite.
use tauri::AppHandle;
use tauri_plugin_keyring::KeyringExt;

const SERVICE: &str = "chessinsight"; // Keyring namespace so this app’s secrets do not collide with other tools.
const ACCOUNT: &str = "lichess_token"; // Single-account slot; multi-account would need a different key per user.

/// Stores or overwrites the Lichess API token used by `clients::lichess` after the user connects in settings.
pub fn save_token(app: &AppHandle, token: &str) -> Result<(), String> {
    app.keyring()
        .set_password(SERVICE, ACCOUNT, token)
        .map_err(|e| e.to_string())
}

/// Returns None if the user never saved a token; callers treat that as logged-out / unauthorized.
pub fn load_token(app: &AppHandle) -> Result<Option<String>, String> {
    app.keyring()
        .get_password(SERVICE, ACCOUNT)
        .map_err(|e| e.to_string())
}

/// Removes the token on logout so subsequent Lichess calls fail until the user reconnects.
pub fn delete_token(app: &AppHandle) -> Result<(), String> {
    app.keyring()
        .delete_password(SERVICE, ACCOUNT)
        .map_err(|e| e.to_string())
}
