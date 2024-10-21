use crate::Cache;
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

        self.sqlite_conn.execute("create table IF NOT EXISTS cache (name TEXT PRIMARY KEY, data TEXT)", ()).expect("Failed to create table");

        self.sqlite_conn.execute("INSERT INTO cache (name, data) VALUES (?1, ?2) ON CONFLICT(name) DO UPDATE SET data = ?2", params![self.name ,&json]).expect("Failed to insert data");
    }
}