use crate::core::{error::AppError, response, state::AppState};
use crate::services::database_manager::sqlite;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/internal/tables", get(list_tables))
        .route("/internal/tables/{name}/rows", get(table_rows))
}

async fn list_tables(
    State(state): State<AppState>,
) -> Result<Json<response::ApiResponse<Vec<sqlite::TableInfo>>>, AppError> {
    Ok(response::ok(sqlite::list_tables(&state)?))
}

async fn table_rows(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<response::ApiResponse<sqlite::TableRows>>, AppError> {
    Ok(response::ok(sqlite::table_rows(&state, &name)?))
}
