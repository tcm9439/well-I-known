use super::db_connection::DbConnection;

pub trait DbTable {
    async fn create_table(&self, db_conn: &DbConnection);
}
