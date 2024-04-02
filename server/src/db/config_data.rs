use sqlx::FromRow;
use sea_query::{enum_def, ColumnDef, ForeignKey, ForeignKeyAction, SqliteQueryBuilder, Table};
use tracing::info;

use crate::db::{db_base::DbTable, db_connection::DbConnection, user::UserIden};

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
        info!("Creating table: {:?}", ConfigDataIden::Table);
        let sql = Table::create()
            .table(ConfigDataIden::Table)
            .if_not_exists()
            .col(ColumnDef::new(ConfigDataIden::AppName).string().primary_key())
            .col(ColumnDef::new(ConfigDataIden::Key).string().primary_key())
            .col(ColumnDef::new(ConfigDataIden::Value).string())
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create table config data");

        // create foreign key
        info!("Creating foreign key for {:?}", ConfigDataIden::Table);
        let foreign_key = ForeignKey::create()
            .name("fk_access_right_username")
            .from(ConfigDataIden::Table, ConfigDataIden::AppName)
            .to(UserIden::Table, UserIden::Username)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_string(SqliteQueryBuilder);

        sqlx::query(foreign_key.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create foreign key");
    }
}