use sqlx::FromRow;
use sea_query::{enum_def, SqliteQueryBuilder, ColumnDef, Table, Query, Expr};
use anyhow::Result;
use tracing::warn;

use super::{db_base::DbTable, db_connection::DbConnection};

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct AccessRight {
    username: String,
    app_name: String,
}

impl AccessRight {
}

pub struct AccessRightTable {}
impl DbTable for AccessRightTable {
    async fn create_table(&self, db_conn: &DbConnection) {
        let sql = Table::create()
            .table(AccessRightIden::Table)
            .if_not_exists()
            .col(ColumnDef::new(AccessRightIden::Username).string().primary_key())
            .col(ColumnDef::new(AccessRightIden::AppName).string().not_null())
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create table");
    }
}