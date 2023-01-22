use std::{fs, path::Path, sync::Mutex};

use once_cell::sync::Lazy;
use rusqlite::Connection;

use crate::{env::get_depository_dir, error::Error};

static CACHE_DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let path = get_depository_dir().join(".cache");
    if !path.exists() {
        fs::create_dir_all(&path);
    }
    let db_file = path.join("cache.db");
    let db = Connection::open(db_file).unwrap();
    db.execute(
        "CREATE TABLE IF NOT EXISTS sha256(
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            archive INTEGER,
            path    TEXT NOT NULL,
            hash    TEXT NOT NULL
        )",
        (),
    )
    .unwrap();
    Mutex::new(db)
});

pub fn query_file_sha256<T: AsRef<Path>>(path: T) -> Option<String> {
    let path = path.as_ref().to_string_lossy();
    CACHE_DB
        .lock()
        .unwrap()
        .query_row(
            "SELECT hash FROM sha256 WHERE path = ?1",
            [path.as_ref()],
            |row| row.get(0),
        )
        .ok()
}

pub fn insert_or_update_file_sha256<T: AsRef<Path>>(path: T, sha256: &str) -> Result<(), Error> {
    let path = path.as_ref().to_string_lossy();
    CACHE_DB
        .lock()
        .unwrap()
        .execute(
            "INSERT OR REPLACE INTO sha256 (id, archive, path, hash)
        VALUES(
            (SELECT id FROM sha256 WHERE path = ?1),
            0,
            ?1,
            ?2
        )
    ",
            (path, sha256),
        )
        .map_err(|e| Error::err(e.to_string()))?;

    Ok(())
}
