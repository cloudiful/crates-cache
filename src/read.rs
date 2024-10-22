use crate::Cache;
use chrono::{DateTime, Local, TimeDelta};
use log::info;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;

impl Cache {
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

            if TimeDelta::from_std(duration).unwrap() > self.valid_period {
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
            let result_str: String = row.get(1).unwrap();
            // let insert_time: i64 = row.get(2).unwrap();
            let update_time: i64 = row.get(3).unwrap();
            let datetime = DateTime::from_timestamp_micros(update_time).unwrap();
            let duration = Local::now().signed_duration_since(datetime);
            Ok((result_str, duration))
        }).expect("Failed to query query");

        let mut strings = Vec::new();

        for result_set in results {
            match result_set {
                Ok(result) => {
                    if result.1 < self.valid_period {
                        strings.push(result.0);
                    }
                }
                Err(_) => {
                    return None
                }
            }
        }

        match strings.first() {
            None => { None }
            Some(string) => {
                let result: T = serde_json::from_str(string).expect("Failed to parse cache file");
                Some(result)
            }
        }
    }
}