use std::{path::Path, sync::Mutex};

use miette::{IntoDiagnostic, Result};
use once_cell::sync::Lazy;
use rusqlite::Connection;

use crate::env::get_app_data_dir;

static CACHE_DB_CONNECT: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let connect = Connection::open(get_app_data_dir().unwrap().join("cache.db")).unwrap();
    connect
        .execute(
            "CREATE TABLE IF NOT EXISTS sha256 (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    path  TEXT NOT NULL,
                    value TEXT NOT NULL,
                )",
            (),
        )
        .unwrap();

    Mutex::new(connect)
});

pub fn query_file_sha256<P: AsRef<Path>>(path: P) -> Result<String> {
    let path_buf = dunce::canonicalize(path).into_diagnostic()?;
    let path = path_buf.to_string_lossy();
    CACHE_DB_CONNECT
        .lock()
        .unwrap()
        .query_row(
            "SELECT value FROM sha256 WHERE path = ?1",
            [path.as_ref()],
            |row| row.get(0),
        )
        .into_diagnostic()
}

pub fn update_file_sha256<P: AsRef<Path>>(path: P, value: &str) -> Result<()> {
    let path_buf = dunce::canonicalize(path).into_diagnostic()?;
    let path = path_buf.to_string_lossy();

    CACHE_DB_CONNECT
        .lock()
        .unwrap()
        .execute(
            "INSERT OR REPLACE INTO sha256 (id, path, value) VALUES(
                    (SELECT id FROM sha256 WHERE path = ?1),
                    ?1,
                    ?2
                )",
            (path, value),
        )
        .into_diagnostic()?;
    Ok(())
}
