pub mod apps;
pub mod dashboard;
pub mod databases;
pub mod health;
pub mod logs;
pub mod security;
pub mod settings;

use crate::core::state::AppState;
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/api/health", health::routes())
        .nest("/api/apps", apps::routes())
        .nest("/api/dashboard", dashboard::routes())
        .nest("/api/databases", databases::routes())
        .nest("/api/logs", logs::routes())
        .with_state(state)
}
