use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub type Database = Arc<Mutex<Connection>>;

pub fn open_database(path: &str) -> rusqlite::Result<Database> {
    let connection = Connection::open(path)?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    Ok(Arc::new(Mutex::new(connection)))
}
