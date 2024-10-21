mod read;
mod save;
mod clear;

use std::path::PathBuf;
use std::time::Duration;

pub struct Cache {
    name: String,
    dir_name: String,
    storing_method: StoringMethod,
    sqlite_conn: rusqlite::Connection,
    revalidate_duration: Duration,
}

pub enum StoringMethod {
    JSON,
    SQLite,
}

impl Cache {
    pub fn new(name: &str) -> Cache {
        Cache {
            name: String::from(name),
            dir_name: String::from("temp"),
            storing_method: StoringMethod::SQLite,
            sqlite_conn: rusqlite::Connection::open(PathBuf::from("cache.db")).unwrap(),
            revalidate_duration: Duration::from_secs(600),
        }
    }

    pub fn set_storing_method(&mut self, method: StoringMethod) {
        self.storing_method = method;
    }

    pub fn read<T>(&self) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match self.storing_method {
            StoringMethod::JSON => {
                self.read_from_file()
            }
            StoringMethod::SQLite => {
                self.read_from_sqlite()
            }
        }
    }

    pub fn save<T>(&self, data: &T)
    where
        T: serde::Serialize,
    {
        match self.storing_method {
            StoringMethod::JSON => {
                self.save_to_file(data);
            }
            StoringMethod::SQLite => {
                self.save_to_sqlite(data);
            }
        }
    }

    pub fn clear(&self) {
        match self.storing_method {
            StoringMethod::JSON => {
                self.clear_file()
            }
            StoringMethod::SQLite => {
                self.clear_sqlite()
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
        pub(crate) name: String,
        price: i32,
    }

    impl Apple {
        fn new(price: i32) -> Apple {
            Apple {
                name: String::from("water"),
                price,
            }
        }
    }

    #[test]
    fn json() {
        save_to_file();
        read_from_file();
        clear_file();
    }

    fn save_to_file() {
        let apple = Apple::new(4);

        let mut apple_cache = Cache::new("apple2");
        apple_cache.set_storing_method(StoringMethod::JSON);

        apple_cache.save(&apple);
    }
    fn read_from_file() {
        let mut apple_cache = Cache::new("apple2");
        apple_cache.set_storing_method(StoringMethod::JSON);

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(Some(Apple::new(4)), cached_apple);
    }
    fn clear_file() {
        let apple = Apple::new(3);

        let mut apple_cache = Cache::new("apple1");
        apple_cache.set_storing_method(StoringMethod::JSON);

        apple_cache.save(&apple);

        apple_cache.clear();

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(None, cached_apple);
    }

    #[test]
    fn sqlite() {
        save_to_sqlite();
        read_from_sqlite();
        clear_sqlite();
    }

    fn save_to_sqlite() {
        let mut apple = Apple::new(5);

        apple.price = 7;

        let apple_cache = Cache::new("apple3");

        apple_cache.save(&apple);

        apple.price = 9;
    }
    fn read_from_sqlite() {
        let apple_cache = Cache::new("apple3");

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(Some(Apple::new(7)), cached_apple);
    }
    fn clear_sqlite() {
        let apple = Apple::new(3);

        let apple_cache = Cache::new("apple1");

        apple_cache.save(&apple);

        apple_cache.clear();

        let cached_apple: Option<Apple> = apple_cache.read();

        assert_eq!(None, cached_apple);
    }
}
