use crate::db::connection::Database;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub version: String,
    pub database: Database,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        Self {
            app_name: "LightPanel".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database,
        }
    }
}
