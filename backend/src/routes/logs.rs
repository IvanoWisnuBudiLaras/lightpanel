use crate::{
    core::{error::AppError, response, state::AppState},
    services::logs::journal,
};
use axum::{routing::get, Json, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/errors", get(error_logs))
        .route("/deploy", get(deploy_logs))
}

async fn error_logs() -> Result<Json<response::ApiResponse<journal::LogSnapshot>>, AppError> {
    Ok(response::ok(journal::read_nginx_error_log(200)?))
}

async fn deploy_logs() -> Result<Json<response::ApiResponse<journal::LogSnapshot>>, AppError> {
    Ok(response::ok(journal::read_panel_deploy_log(200)?))
}
