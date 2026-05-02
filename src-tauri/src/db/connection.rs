use rusqlite::Connection;
use tauri::{AppHandle, Manager};

pub fn init(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let path = app.path().app_data_dir()?.join("app.db");
    println!("{:?}", path);

    std::fs::create_dir_all(path.parent().unwrap())?;

    let conn = Connection::open(path)?;

    crate::db::users::schema::init_users_table(&conn);
    crate::db::games::schema::init_games_table(&conn);
    crate::db::insights::schema::init_insights_table(&conn);

    Ok(())
}

pub fn get_conn(app: &AppHandle) -> Result<Connection, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("app.db");

    Connection::open(path).map_err(|e| e.to_string())
}
