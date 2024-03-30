use sqlx::FromRow;
use sea_query::{enum_def, SqliteQueryBuilder, ColumnDef, Table, Query, Expr};
use anyhow::Result;
use tracing::{debug, warn};

use super::{db_base::DbTable, db_connection::DbConnection};

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct ConfigData {
    app_name: String,
    key: String,
    value: String,
}

impl ConfigData {
}

pub struct ConfigDataTable {}
impl DbTable for ConfigDataTable {
    async fn create_table(&self, db_conn: &DbConnection) {
        let sql = Table::create()
            .table(ConfigDataIden::Table)
            .if_not_exists()
            .col(ColumnDef::new(ConfigDataIden::AppName).string().primary_key())
            .col(ColumnDef::new(ConfigDataIden::Key).string().primary_key())
            .col(ColumnDef::new(ConfigDataIden::Value).string())
            .to_string(SqliteQueryBuilder);

        debug!("Creating table config data. sql: {}", sql);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create table config data");
    }
}