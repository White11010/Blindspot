// Cold-start flow: if token exists, return cached user quickly and refresh in background; else require full sync.
use tauri::AppHandle;

use crate::db::{connection::get_conn, users::repository};
use crate::services::auth;

use super::dto::BootstrapState;

/// Returns unauthorized without token; with token returns active user and flags stale data when background sync spawns.
pub async fn bootstrap(app: AppHandle) -> Result<BootstrapState, String> {
    match auth::load_token(&app)? {
        Some(t) => t,
        None => {
            return Ok(BootstrapState::Unauthorized);
        }
    };

    let conn = get_conn(&app)?;

    // When a row already exists we still want fresh Lichess ratings but avoid blocking first paint on network latency.
    if let Some(user) = repository::get_active_user(&conn)? {
        tauri::async_runtime::spawn({
            let app = app.clone();
            async move {
                let _ = crate::services::users::sync_user(app).await;
            }
        });

        return Ok(BootstrapState::Authorized {
            user,
            is_stale: true,
        });
    }

    let user = crate::services::users::sync_user(app).await?;

    Ok(BootstrapState::Authorized {
        user,
        is_stale: false,
    })
}
