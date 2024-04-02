use sqlx::FromRow;
use sea_query::{enum_def, ColumnDef, Expr, ForeignKey, ForeignKeyAction, Query, SqliteQueryBuilder, Table};
use tracing::info;
use anyhow::Result;

use crate::db::{db_base::DbTable, db_connection::DbConnection, user::UserIden};

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct ConfigData {
    app_name: String,
    key: String,
    owner: String,
    value: String,
}

const CONFIG_DATA_COLUMNS: [ConfigDataIden; 4] = [
    ConfigDataIden::AppName,
    ConfigDataIden::Key,
    ConfigDataIden::Owner,
    ConfigDataIden::Value,
];

pub struct ConfigDataTable {}
impl DbTable for ConfigDataTable {
    async fn create_table(db_conn: &DbConnection) {
        info!("Creating table: {:?}", ConfigDataIden::Table);
        let sql = Table::create()
            .table(ConfigDataIden::Table)
            .if_not_exists()
            .col(ColumnDef::new(ConfigDataIden::AppName).string())
            .col(ColumnDef::new(ConfigDataIden::Key).string())
            .col(ColumnDef::new(ConfigDataIden::Owner).string())
            .col(ColumnDef::new(ConfigDataIden::Value).string())
            .primary_key(sea_query::Index::create()
                .col(ConfigDataIden::AppName)
                .col(ConfigDataIden::Key)
                .col(ConfigDataIden::Owner)
            )
            .foreign_key(ForeignKey::create()
                .from(ConfigDataIden::Table, ConfigDataIden::AppName)
                .to(UserIden::Table, UserIden::Username)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
            )
            .foreign_key(ForeignKey::create()
                .from(ConfigDataIden::Table, ConfigDataIden::Owner)
                .to(UserIden::Table, UserIden::Username)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
            )
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create table config data");
    }
}

/// Get the encrypted value of the given key for the given owner.
pub async fn get_data_value(db_conn: &DbConnection, app_name: &str, owner: &str, key: &str) -> Result<String> {
    let sql = Query::select()
        .column(ConfigDataIden::Value)
        .from(ConfigDataIden::Table)
        .and_where(Expr::col(ConfigDataIden::AppName).eq(app_name))
        .and_where(Expr::col(ConfigDataIden::Key).eq(key))
        .and_where(Expr::col(ConfigDataIden::Owner).eq(owner))
        .to_string(SqliteQueryBuilder);

    let data: (String, ) = sqlx::query_as(sql.as_str())
        .fetch_one(&db_conn.pool)
        .await?;

    Ok(data.0)
}

pub async fn set_data_value(db_conn: &DbConnection, app_name: &str, owner: &str, key: &str, value: &str) -> Result<()> {
    let sql = Query::insert()
        .into_table(ConfigDataIden::Table)
        .columns(CONFIG_DATA_COLUMNS)
        .values([
            app_name.into(),
            key.into(),
            owner.into(),
            value.into(),
        ])?
        .to_string(SqliteQueryBuilder);

    sqlx::query(sql.as_str())
        .execute(&db_conn.pool)
        .await?;

    Ok(())
}

pub async fn delete_data(db_conn: &DbConnection, app_name: &str, owner: &str, key: &str) -> Result<()> {
    let sql = Query::delete()
        .from_table(ConfigDataIden::Table)
        .cond_where(Expr::col(ConfigDataIden::AppName).eq(app_name))
        .cond_where(Expr::col(ConfigDataIden::Key).eq(key))
        .cond_where(Expr::col(ConfigDataIden::Owner).eq(owner))
        .to_string(SqliteQueryBuilder);

    sqlx::query(sql.as_str())
        .execute(&db_conn.pool)
        .await?;

    Ok(())
}