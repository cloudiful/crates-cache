use crate::Cache;
use chrono::Utc;
use log::info;
use rusqlite::params;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

impl Cache {
    pub(crate) fn save_to_file<T>(&self, data: &T)
    where
        T: serde::Serialize,
    {
        info!("Saving data to file {}.json", self.name);
        let json = serde_json::to_string(data).expect("Failed to serialize cache data");

        if !&self.dir.is_dir() {
            fs::create_dir(&self.dir).expect("Failed to create cache directory");
        }

        let mut file = File::create(self.dir.join(PathBuf::from(format!("{}.json", self.name))))
            .expect("Failed to create cache file");

        file.write_all(json.as_bytes())
            .expect("Failed to write cache file");
    }

    pub(crate) fn save_to_sqlite<T>(&self, data: &T)
    where
        T: serde::Serialize,
    {
        info!("Saving data {} to sqlite table {}", self.name, self.table);
        let json = serde_json::to_string(data).expect("Failed to serialize cache data");

        let current_timestamp = Utc::now().timestamp_micros();

        let sql = format!(
            "INSERT INTO {} (name, data, insert_time, update_time) VALUES (?1, ?2, ?3, ?3) ON CONFLICT(name) DO UPDATE SET data = ?2, update_time = ?3",
            self.table
        );

        self.sqlite_conn
            .execute(&sql, params![self.name, json, current_timestamp])
            .expect("Failed to insert data");
    }
}
