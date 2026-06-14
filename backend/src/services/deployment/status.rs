use crate::core::{error::AppError, state::AppState};
use crate::models::{app::AppRecord, deployment::Deployment};
use rusqlite::params;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn record_success(
    state: &AppState,
    app: &AppRecord,
    release_path: &str,
    log_path: &str,
) -> Result<Deployment, AppError> {
    let db = state.database.lock().expect("database lock poisoned");
    let id = Uuid::new_v4().to_string();
    let now = timestamp_id();

    db.execute(
        "INSERT INTO deployments (id, app_id, status, release_path, log_path, created_at)
         VALUES (?1, ?2, 'succeeded', ?3, ?4, ?5)",
        params![&id, &app.id, release_path, log_path, &now],
    )?;

    db.execute(
        "UPDATE apps SET status = 'deployed', updated_at = ?1 WHERE id = ?2",
        params![&now, &app.id],
    )?;

    Ok(Deployment {
        id,
        app_id: app.id.clone(),
        status: "succeeded".to_string(),
        release_path: Some(release_path.to_string()),
        log_path: Some(log_path.to_string()),
        created_at: now,
    })
}

pub fn timestamp_id() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}
