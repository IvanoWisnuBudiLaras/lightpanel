use crate::core::{response, state::AppState};
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(health))
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: String,
    version: String,
}

async fn health(State(state): State<AppState>) -> Json<response::ApiResponse<HealthResponse>> {
    response::ok(HealthResponse {
        status: "ok",
        service: state.app_name,
        version: state.version,
    })
}
