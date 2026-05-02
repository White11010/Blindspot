use tauri::AppHandle;

use crate::db::{connection::get_conn, users::repository};
use crate::services::auth;

use super::dto::BootstrapState;

pub async fn bootstrap(app: AppHandle) -> Result<BootstrapState, String> {
    match auth::load_token(&app)? {
        Some(t) => t,
        None => {
            return Ok(BootstrapState::Unauthorized);
        }
    };

    let conn = get_conn(&app)?;

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
