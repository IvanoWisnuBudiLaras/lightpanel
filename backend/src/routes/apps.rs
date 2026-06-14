use crate::core::{error::AppError, response, state::AppState};
use crate::models::app::{AppRecord, CreateAppRequest, UpdateAppRequest};
use crate::services::{
    apps,
    deployment::adapters::static_site::{
        DeployStaticRequest, StaticDeployResult, StaticSiteAdapter,
    },
    deployment::adapters::node::{DeployNodeRequest, NodeAdapter, NodeDeployResult},
    systemd::unit_generator,
};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_apps).post(create_app))
        .route("/{id}", get(get_app).put(update_app).delete(delete_app))
        .route("/{id}/deploy/static", post(deploy_static))
        .route("/{id}/deploy/node", post(deploy_node))
        .route("/{id}/systemd/preview", get(systemd_preview))
}

async fn list_apps(
    State(state): State<AppState>,
) -> Result<Json<response::ApiResponse<Vec<AppRecord>>>, AppError> {
    Ok(response::ok(apps::list_apps(&state)?))
}

async fn create_app(
    State(state): State<AppState>,
    Json(payload): Json<CreateAppRequest>,
) -> Result<Json<response::ApiResponse<AppRecord>>, AppError> {
    Ok(response::ok(apps::create_app(&state, payload)?))
}

async fn get_app(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<response::ApiResponse<AppRecord>>, AppError> {
    Ok(response::ok(apps::get_app(&state, &id)?))
}

async fn update_app(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAppRequest>,
) -> Result<Json<response::ApiResponse<AppRecord>>, AppError> {
    Ok(response::ok(apps::update_app(&state, &id, payload)?))
}

async fn delete_app(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<response::ApiResponse<AppRecord>>, AppError> {
    Ok(response::ok(apps::soft_delete_app(&state, &id)?))
}

async fn systemd_preview(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<response::ApiResponse<unit_generator::SystemdPreview>>, AppError> {
    let app = apps::get_app(&state, &id)?;
    Ok(response::ok(unit_generator::preview_for_app(&app)?))
}

async fn deploy_static(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<DeployStaticRequest>,
) -> Result<Json<response::ApiResponse<StaticDeployResult>>, AppError> {
    let app = apps::get_app(&state, &id)?;
    Ok(response::ok(StaticSiteAdapter::deploy(&state, &app, payload)?))
}

async fn deploy_node(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<DeployNodeRequest>,
) -> Result<Json<response::ApiResponse<NodeDeployResult>>, AppError> {
    let app = apps::get_app(&state, &id)?;
    Ok(response::ok(NodeAdapter::deploy(&state, &app, payload)?))
}
