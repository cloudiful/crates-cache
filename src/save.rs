use crate::Cache;
use chrono::Utc;
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
        let json = serde_json::to_string(data).expect("Failed to serialize cache data");

        if !PathBuf::from(&self.dir_name).is_dir() {
            fs::create_dir(&self.dir_name).expect("Failed to create cache directory");
        }

        let mut file = File::create(format!("{}/{}.json", self.dir_name, self.name)).expect("Failed to create cache file");

        file.write_all(json.as_bytes()).expect("Failed to write cache file");
    }


    pub(crate) fn save_to_sqlite<T>(&self, data: &T)
    where
        T: serde::Serialize,
    {
        let json = serde_json::to_string(data).expect("Failed to serialize cache data");

        let current_timestamp = Utc::now().timestamp_micros();

        self.sqlite_conn.execute("create table IF NOT EXISTS cache (name TEXT PRIMARY KEY, data TEXT, insert_time INTEGER NOT NULL, update_time INTEGER NOT NULL)", ()).expect("Failed to create table");

        self.sqlite_conn.execute("INSERT INTO cache (name, data, insert_time, update_time) VALUES (?1, ?2, ?3, ?3) ON CONFLICT(name) DO UPDATE SET data = ?2, update_time = ?3", params![self.name ,json, current_timestamp]).expect("Failed to insert data");
    }
}