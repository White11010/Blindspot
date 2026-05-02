mod commands;
mod services;
mod clients;
mod db;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_keyring::init())
        .setup(|app| {
            db::connection::init(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::save_token,
            commands::auth::load_token,
            commands::auth::delete_token,

            commands::users::get_me,
            commands::users::sync_me,

            commands::app::bootstrap
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}