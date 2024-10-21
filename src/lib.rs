use log::info;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct Cache {
    name: String,
    dir_name: String,
    revalidate_duration: Duration,
}

impl Cache {
    pub fn new(name: &str) -> Cache {
        Cache {
            name: String::from(name),
            dir_name: String::from("temp"),
            revalidate_duration: Duration::from_secs(600),
        }
    }

    pub fn read<T>(&self) -> Option<T>
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

    pub fn save<T>(&self, data: &T)
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

    pub fn clear(&self) {
        match fs::remove_file(format!("{}/{}.json", self.dir_name, self.name)) {
            Ok(_) => {
                info!("Cleared cache directory");
            }
            Err(e) => {
                info!("Failed to clean cache directory: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Apple {
        juice: String,
        pie: Vec<i32>,
    }

    impl Apple {
        fn new() -> Option<Apple> {
            Some(Apple {
                juice: String::from("water"),
                pie: Vec::new(),
            })
        }
    }

    #[test]
    fn clear() {
        let apple = Apple::new();

        let apple_cache = Cache::new("apple1");

        apple_cache.save(&apple);

        apple_cache.clear();

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(None, cached_apple);
    }

    #[test]
    fn save() {
        let apple = Apple::new();

        let apple_cache = Cache::new("apple2");

        apple_cache.save(&apple);
    }

    #[test]
    fn read(){
        let apple_cache = Cache::new("apple2");

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(Apple::new(), cached_apple);
    }
}
