use crate::db::{db_base::DbTable, db_connection::DbConnection, user::UserIden};

use sqlx::FromRow;
use sea_query::{enum_def, ColumnDef, ForeignKey, ForeignKeyAction, SqliteQueryBuilder, Table};
use tracing::info;

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
        info!("Creating table: {:?}", AccessRightIden::Table);
        let sql = Table::create()
            .table(AccessRightIden::Table)
            .if_not_exists()
            .col(ColumnDef::new(AccessRightIden::Username).string().primary_key())
            .col(ColumnDef::new(AccessRightIden::AppName).string().not_null())
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create table");
        // FATAL if failed to create table

        // create foreign key
        info!("Creating foreign key for {:?}", AccessRightIden::Table);
        let foreign_key = ForeignKey::create()
            .name("fk_access_right_username")
            .from(AccessRightIden::Table, AccessRightIden::Username)
            .to(UserIden::Table, UserIden::Username)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_string(SqliteQueryBuilder);

        sqlx::query(foreign_key.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create foreign key");
    }
}