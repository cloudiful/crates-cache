use crate::Cache;

impl Cache {
    pub(crate) fn clear_table(&self) {
        let sql = format!("delete from {}", self.table);
        self.sqlite_conn.execute(&sql, ()).expect("Failed to clear cache.");
    }
}