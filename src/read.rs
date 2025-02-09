use crate::Cache;
use log::info;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;

impl Cache{
    pub(crate) fn read_from_file<T>(&self) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let cache_file = PathBuf::from(format!("{}/{}.json", self.dir_name, self.name));

        if cache_file.exists() {
            info!("Cache file found at {:?}", cache_file);

            let file_metadata = cache_file.metadata().expect("Failed to get file metadata");

            let now = SystemTime::now();

            let duration = now.duration_since(file_metadata.modified().expect("Failed to get file modified time")).expect("Failed to calculate cache duration");

            info!("Last modified: {} minutes ago", duration.as_secs() / 60);

            if duration > self.revalidate_duration {
                None
            } else {
                let mut cache_str = String::new();
                File::open(cache_file).expect("Failed to open cache file").read_to_string(&mut cache_str).expect("Failed to read cache file");
                let result = serde_json::from_str(&cache_str).expect("Failed to parse cache file");
                Some(result)
            }
        } else { None }
    }

    pub(crate) fn read_from_sqlite<T>(&self) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut stmt = self.sqlite_conn.prepare("SELECT * FROM cache where name = ?1").expect("Failed to prepare query");

        let results = stmt.query_map([&self.name], |row| {
            let result_str: Result<String, rusqlite::Error> = row.get(1);
            result_str
        }).expect("Failed to query query");

        let mut strings = Vec::new();

        for result in results {
            strings.push(result.unwrap());
        }

        let result:T = serde_json::from_str(&strings.first().unwrap()).expect("Failed to parse cache file");

        Some(result)
    }
}