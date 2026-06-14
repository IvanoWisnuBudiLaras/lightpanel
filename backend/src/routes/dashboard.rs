use crate::core::{error::AppError, response, state::AppState};
use crate::models::runtime::RuntimeInfo;
use crate::services::dashboard::{resource_usage, runtime_detector, server_identity};
use axum::{routing::get, Json, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/identity", get(identity))
        .route("/resources", get(resources))
        .route("/runtimes", get(runtimes))
}

async fn identity() -> Result<Json<response::ApiResponse<server_identity::Identity>>, AppError> {
    let identity = server_identity::read_identity()?;
    Ok(response::ok(identity))
}

async fn resources() -> Result<Json<response::ApiResponse<resource_usage::ResourceUsage>>, AppError> {
    Ok(response::ok(resource_usage::read_resources()?))
}

async fn runtimes() -> Json<response::ApiResponse<Vec<RuntimeInfo>>> {
    response::ok(runtime_detector::detect_runtimes())
}
