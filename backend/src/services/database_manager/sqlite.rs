use crate::core::{error::AppError, state::AppState};
use rusqlite::{params, types::ValueRef};
use serde::Serialize;
use std::collections::BTreeMap;

const ALLOWED_TABLES: &[&str] = &[
    "apps",
    "app_domains",
    "app_env_vars",
    "deployments",
    "audit_logs",
    "settings",
];

#[derive(Debug, Serialize)]
pub struct TableInfo {
    pub name: String,
    pub row_count: i64,
}

#[derive(Debug, Serialize)]
pub struct TableRows {
    pub table: String,
    pub columns: Vec<String>,
    pub rows: Vec<BTreeMap<String, String>>,
}

pub fn list_tables(state: &AppState) -> Result<Vec<TableInfo>, AppError> {
    let db = state.database.lock().expect("database lock poisoned");

    ALLOWED_TABLES
        .iter()
        .map(|table| {
            let sql = format!("SELECT COUNT(*) FROM {table}");
            let row_count = db.query_row(&sql, [], |row| row.get(0))?;
            Ok(TableInfo {
                name: table.to_string(),
                row_count,
            })
        })
        .collect::<Result<Vec<_>, rusqlite::Error>>()
        .map_err(AppError::from)
}

pub fn table_rows(state: &AppState, table: &str) -> Result<TableRows, AppError> {
    validate_table(table)?;

    let db = state.database.lock().expect("database lock poisoned");
    let sql = format!("SELECT * FROM {table} LIMIT 50");
    let mut statement = db.prepare(&sql)?;
    let columns = statement
        .column_names()
        .iter()
        .map(|name| name.to_string())
        .collect::<Vec<_>>();

    let mut query = statement.query(params![])?;
    let mut rows = Vec::new();

    while let Some(row) = query.next()? {
        let mut item = BTreeMap::new();
        for (index, column) in columns.iter().enumerate() {
            item.insert(column.clone(), value_to_string(row.get_ref(index)?));
        }
        rows.push(item);
    }

    Ok(TableRows {
        table: table.to_string(),
        columns,
        rows,
    })
}

fn validate_table(table: &str) -> Result<(), AppError> {
    if ALLOWED_TABLES.contains(&table) {
        Ok(())
    } else {
        Err(AppError::BadRequest("unsupported table".to_string()))
    }
}

fn value_to_string(value: ValueRef<'_>) -> String {
    match value {
        ValueRef::Null => String::new(),
        ValueRef::Integer(value) => value.to_string(),
        ValueRef::Real(value) => value.to_string(),
        ValueRef::Text(value) => String::from_utf8_lossy(value).to_string(),
        ValueRef::Blob(_) => "[blob]".to_string(),
    }
}
