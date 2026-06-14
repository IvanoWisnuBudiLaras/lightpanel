use crate::db::connection::Database;

const INITIAL_MIGRATION: &str = include_str!("../../../migrations/001_init.sql");

pub fn run_migrations(database: &Database) -> rusqlite::Result<()> {
    let connection = database.lock().expect("database lock poisoned");
    connection.execute_batch(INITIAL_MIGRATION)
}
