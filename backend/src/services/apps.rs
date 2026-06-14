use crate::core::{error::AppError, state::AppState};
use crate::models::app::{AppRecord, CreateAppRequest, UpdateAppRequest};
use super::apps_validation::{now_string, validate_app_input, validate_domain};
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

pub fn list_apps(state: &AppState) -> Result<Vec<AppRecord>, AppError> {
    let db = state.database.lock().expect("database lock poisoned");
    let mut statement = db.prepare(&app_select_sql("WHERE apps.deleted_at IS NULL"))?;
    let rows = statement.query_map([], row_to_app)?;

    rows.collect::<Result<Vec<_>, _>>().map_err(AppError::from)
}

pub fn get_app(state: &AppState, id: &str) -> Result<AppRecord, AppError> {
    find_app(state, id)?.ok_or_else(|| AppError::NotFound("app not found".to_string()))
}

pub fn create_app(state: &AppState, payload: CreateAppRequest) -> Result<AppRecord, AppError> {
    validate_app_input(&payload.name, &payload.runtime, payload.internal_port)?;
    validate_domain(payload.primary_domain.as_deref())?;

    let db = state.database.lock().expect("database lock poisoned");
    let id = Uuid::new_v4().to_string();
    let now = now_string();

    db.execute(
        "INSERT INTO apps (id, name, runtime, internal_port, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, 'created', ?5, ?5)",
        params![
            &id,
            &payload.name,
            &payload.runtime,
            i64::from(payload.internal_port),
            &now
        ],
    )
    .map_err(map_write_error)?;

    replace_primary_domain(&db, &id, payload.primary_domain.as_deref(), &now)?;
    get_app_by_connection(&db, &id)
}

pub fn update_app(
    state: &AppState,
    id: &str,
    payload: UpdateAppRequest,
) -> Result<AppRecord, AppError> {
    validate_app_input(&payload.name, &payload.runtime, payload.internal_port)?;
    validate_domain(payload.primary_domain.as_deref())?;

    let db = state.database.lock().expect("database lock poisoned");
    let now = now_string();
    let changed = db.execute(
        "UPDATE apps
         SET name = ?1, runtime = ?2, internal_port = ?3, updated_at = ?4
         WHERE id = ?5 AND deleted_at IS NULL",
        params![
            &payload.name,
            &payload.runtime,
            i64::from(payload.internal_port),
            &now,
            id
        ],
    )
    .map_err(map_write_error)?;

    if changed == 0 {
        return Err(AppError::NotFound("app not found".to_string()));
    }

    replace_primary_domain(&db, id, payload.primary_domain.as_deref(), &now)?;
    get_app_by_connection(&db, id)
}

pub fn soft_delete_app(state: &AppState, id: &str) -> Result<AppRecord, AppError> {
    let db = state.database.lock().expect("database lock poisoned");
    let now = now_string();
    let changed = db.execute(
        "UPDATE apps SET deleted_at = ?1, updated_at = ?1
         WHERE id = ?2 AND deleted_at IS NULL",
        params![&now, id],
    )?;

    if changed == 0 {
        return Err(AppError::NotFound("app not found".to_string()));
    }

    insert_audit_log(&db, "delete_app", id, &now)?;
    get_app_by_connection(&db, id)
}

fn find_app(state: &AppState, id: &str) -> Result<Option<AppRecord>, AppError> {
    let db = state.database.lock().expect("database lock poisoned");
    let sql = app_select_sql("WHERE apps.id = ?1 AND apps.deleted_at IS NULL");

    db.query_row(&sql, params![id], row_to_app)
        .optional()
        .map_err(AppError::from)
}

fn get_app_by_connection(db: &Connection, id: &str) -> Result<AppRecord, AppError> {
    let sql = app_select_sql("WHERE apps.id = ?1");

    db.query_row(&sql, params![id], row_to_app)
        .map_err(AppError::from)
}

fn app_select_sql(where_clause: &str) -> String {
    format!(
        "SELECT apps.id, apps.name, apps.runtime, apps.internal_port, apps.status,
            domains.domain, apps.created_at, apps.updated_at, apps.deleted_at
         FROM apps
         LEFT JOIN app_domains domains
            ON domains.app_id = apps.id AND domains.is_primary = 1
         {where_clause}
         ORDER BY apps.created_at DESC"
    )
}

fn row_to_app(row: &rusqlite::Row<'_>) -> rusqlite::Result<AppRecord> {
    let internal_port: i64 = row.get(3)?;

    Ok(AppRecord {
        id: row.get(0)?,
        name: row.get(1)?,
        runtime: row.get(2)?,
        internal_port: internal_port as u16,
        status: row.get(4)?,
        primary_domain: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
        deleted_at: row.get(8)?,
    })
}

fn replace_primary_domain(
    db: &Connection,
    app_id: &str,
    domain: Option<&str>,
    now: &str,
) -> Result<(), AppError> {
    db.execute(
        "DELETE FROM app_domains WHERE app_id = ?1 AND is_primary = 1",
        params![app_id],
    )?;

    let Some(domain) = domain.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(());
    };

    db.execute(
        "INSERT INTO app_domains (id, app_id, domain, is_primary, created_at)
         VALUES (?1, ?2, ?3, 1, ?4)",
        params![Uuid::new_v4().to_string(), app_id, domain, now],
    )
    .map_err(map_write_error)?;

    Ok(())
}

fn insert_audit_log(db: &Connection, action: &str, target: &str, now: &str) -> Result<(), AppError> {
    db.execute(
        "INSERT INTO audit_logs (id, actor, action, target, details, created_at)
         VALUES (?1, 'system', ?2, ?3, NULL, ?4)",
        params![Uuid::new_v4().to_string(), action, target, now],
    )?;

    Ok(())
}

fn map_write_error(error: rusqlite::Error) -> AppError {
    match error {
        rusqlite::Error::SqliteFailure(_, _) => {
            AppError::Conflict("app name or domain already exists".to_string())
        }
        other => AppError::from(other),
    }
}
