pub mod migrations;
pub mod repo;

use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Shared database handle for concurrent access.
pub type DbPool = Arc<Mutex<Connection>>;

/// Initialize the database at the given path with WAL mode and migrations.
pub fn init_db(path: &Path) -> Result<DbPool> {
    let conn = Connection::open(path)?;

    // Enable WAL mode for concurrent reads.
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    // Run migrations.
    migrations::run(&conn)?;

    Ok(Arc::new(Mutex::new(conn)))
}

/// Initialize an in-memory database for testing.
pub fn init_memory_db() -> Result<DbPool> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;
    migrations::run(&conn)?;
    Ok(Arc::new(Mutex::new(conn)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_memory_db() {
        let db = init_memory_db().unwrap();
        let conn = db.lock().unwrap();
        let count: i64 = conn
            .query_row("SELECT count(*) FROM content", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
}
