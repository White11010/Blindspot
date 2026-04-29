use serde::{Serialize, Deserialize};
use tauri::Manager;
use tauri_plugin_keyring::KeyringExt;

const KEYRING_SERVICE: &str = "my-tauri-app";
const KEYRING_USER: &str = "lichess_user";

#[tauri::command]
async fn save_token(app: tauri::AppHandle, token: String) -> Result<(), String> {
    app.keyring()
        .set_password(KEYRING_SERVICE, KEYRING_USER, &token)
        .map_err(|e| format!("Failed to save token: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn load_token(app: tauri::AppHandle) -> Result<Option<String>, String> {
    match app.keyring().get_password(KEYRING_SERVICE, KEYRING_USER) {
        Ok(Some(pwd)) => Ok(Some(pwd)),
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Failed to load token: {}", e)),
    }
}

#[tauri::command]
async fn delete_token(app: tauri::AppHandle) -> Result<(), String> {
    app.keyring()
        .delete_password(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| format!("Failed to delete token: {}", e))?;
    Ok(())
}

// Определяем структуру для части данных, которые мы ожидаем от API
#[derive(Serialize, Deserialize, Debug)]
struct LichessUser {
    id: String,
    username: String,
    perfs: serde_json::Value, // Здесь могут быть рейтинги
    // ... другие поля
}

#[tauri::command]
async fn fetch_lichess_player(
    app: tauri::AppHandle,
    username: Option<String>, // если None — запрашиваем свой профиль (/api/account)
) -> Result<LichessUser, String> {
    // 1. Получаем токен из защищённого хранилища
    let token = app
        .keyring()
        .get_password(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| format!("Ошибка доступа к хранилищу: {}", e))?;

    let token = match token {
        Some(t) => t,
        None => return Err("Токен не найден. Пожалуйста, выполните вход.".to_string()),
    };

    // 2. Формируем URL
    let url = match username {
        Some(name) => format!("https://lichess.org/api/user/{}", name),
        None => "https://lichess.org/api/account".to_string(),
    };

    // 3. Выполняем запрос с авторизацией
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Ошибка сети: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Lichess API вернул ошибку: {}",
            response.status()
        ));
    }

    let user: LichessUser = response
        .json()
        .await
        .map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;

    Ok(user)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_keyring::init())
        .invoke_handler(tauri::generate_handler![
            fetch_lichess_player,
            save_token,
            load_token,
            delete_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}