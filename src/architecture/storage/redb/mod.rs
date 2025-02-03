// Global database object using redb that allows tables to be created and accessed, for structured data storage

use crate::fl;
use once_cell::sync::Lazy;
use redb::Database;
use std::path::{Path, PathBuf};
use std::process::exit;
use tracing::{debug, error};

pub static DATABASE: Lazy<Database> = Lazy::new(|| {
    let db_path = std::env::var("XDG_DATA_HOME")
        .map_or_else(
            |_| {
                PathBuf::from(
                    std::env::var("HOME")
                        .map(|home| format!("{}/.local/share", home))
                        .unwrap()
                        .as_str(),
                )
            },
            PathBuf::from,
        )
        .join("vshell.db");
    let database: Database;

    if Path::exists(&db_path) {
        match Database::open(&db_path) {
            Ok(db) => {
                database = db;
                debug!(
                    "{}",
                    fl!(
                        "architecture-storage-redb_debug_database-opened",
                        path = format!("{:?}", &db_path.display())
                    )
                );
            }
            Err(e) => {
                error!(
                    "{}",
                    fl!(
                        "architecture-storage-redb_error_database-open-fail",
                        error = format!("{:?}", e)
                    )
                );
                exit(1);
            }
        }
    } else {
        match Database::create(&db_path) {
            Ok(db) => {
                database = db;
                debug!(
                    "{}",
                    fl!(
                        "architecture-storage-redb_debug_database-created",
                        path = format!("{:?}", &db_path.display())
                    )
                );
            }
            Err(e) => {
                error!(
                    "{}",
                    fl!(
                        "architecture-storage-redb_error_database-create-fail",
                        error = format!("{:?}", e)
                    )
                );
                exit(1);
            }
        }
    }

    database
});
