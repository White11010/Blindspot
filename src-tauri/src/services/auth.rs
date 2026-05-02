use tauri::AppHandle;
use tauri_plugin_keyring::KeyringExt;

const SERVICE: &str = "chessinsight";
const ACCOUNT: &str = "lichess_token";

pub fn save_token(app: &AppHandle, token: &str) -> Result<(), String> {
    app.keyring()
        .set_password(SERVICE, ACCOUNT, token)
        .map_err(|e| e.to_string())
}

pub fn load_token(app: &AppHandle) -> Result<Option<String>, String> {
    app.keyring()
        .get_password(SERVICE, ACCOUNT)
        .map_err(|e| e.to_string())
}

pub fn delete_token(app: &AppHandle) -> Result<(), String> {
    app.keyring()
        .delete_password(SERVICE, ACCOUNT)
        .map_err(|e| e.to_string())
}