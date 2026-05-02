use serde::{Serialize};

use crate::db::users::model::User;

#[derive(Debug, Serialize)]
#[serde(tag = "state", content = "data")]
pub enum BootstrapState {
    Unauthorized,
    Authorized {
        user: User,
        is_stale: bool,
    }
}