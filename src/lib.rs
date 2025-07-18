mod read;
mod save;
mod clear;
mod remove;
#[cfg(test)]
mod test;

use chrono::TimeDelta;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Cache {
    name: String,
    dir: PathBuf,
    storing_method: StoringMethod,
    sqlite_conn: rusqlite::Connection,
    table: String,
    valid_period: TimeDelta,
}

#[derive(Debug)]
pub enum StoringMethod {
    JSON,
    SQLite,
}

impl Cache {
    pub fn new(name: &str) -> Cache {
        let cache = Cache {
            name: String::from(name),
            dir: PathBuf::from("temp"),
            storing_method: StoringMethod::SQLite,
            sqlite_conn: rusqlite::Connection::open(PathBuf::from("cache.db")).unwrap(),
            table: "cache".to_string(),
            valid_period: TimeDelta::minutes(10),
        };

        cache.create_table(&cache.table);

        cache
    }

    pub fn set_storing_method(&mut self, method: StoringMethod) {
        self.storing_method = method;
    }

    pub fn set_valid_period(&mut self, valid_period: TimeDelta) {
        self.valid_period = valid_period;
    }

    pub fn set_table(&mut self, table_name: &str) {
        self.table = table_name.to_string();
        self.create_table(&self.table);
    }

    fn create_table(&self, name: &str) {
        let sql = format!("create table IF NOT EXISTS {} (name TEXT PRIMARY KEY, data TEXT, insert_time INTEGER NOT NULL, update_time INTEGER NOT NULL)", name);
        self.sqlite_conn
            .execute(&sql, ())
            .expect("Failed to create table");
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

    /**
    Remove item from cache
    **/
    pub fn remove(&self) {
        match self.storing_method {
            StoringMethod::JSON => {
                self.remove_file()
            }
            StoringMethod::SQLite => {
                self.remove_sqlite()
            }
        }
    }

    /**
    Clear all cache
    **/
    pub fn clear(&self) {
        match self.storing_method {
            StoringMethod::JSON => {
                self.remove_all_files()
            }
            StoringMethod::SQLite => {
                self.clear_table()
            }
        }
    }
}