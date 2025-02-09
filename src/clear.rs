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
        self.sqlite_conn.execute("delete from cache where name = ?1", params![self.name]).expect("Failed to clear cache.");
    }
}