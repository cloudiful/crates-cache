use crate::Cache;
use log::info;
use rusqlite::params;
use std::fs;
use std::path::PathBuf;

impl Cache {
    pub(crate) fn clear_file(&self) {
        let cache_file = PathBuf::from(format!("{}/{}.json", self.dir_name, self.name));
        if cache_file.exists() {
            match fs::remove_file(cache_file.as_path()) {
                Ok(_) => {
                    info!("Cleared cache directory");
                }
                Err(e) => {
                    info!("Failed to clean cache directory: {}", e);
                }
            }
        }
    }

    pub(crate) fn clear_sqlite(&self) {
        let sql= format!("delete from {} where name = ?1", self.table);
        self.sqlite_conn.execute(&sql, params![self.name]).expect("Failed to clear cache.");
    }
}