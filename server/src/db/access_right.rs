use crate::db::{db_base::DbTable, db_connection::DbConnection, user::UserIden};

use sqlx::FromRow;
use sea_query::{enum_def, foreign_key, ColumnDef, Expr, ForeignKey, ForeignKeyAction, Query, SqliteQueryBuilder, Table};
use tracing::info;
use anyhow::Result;

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct AccessRight {
    username: String,
    app_name: String,
}

const ACCESS_RIGHT_COLUMNS: [AccessRightIden; 2] = [
    AccessRightIden::Username,
    AccessRightIden::AppName,
];

pub struct AccessRightTable {}
impl DbTable for AccessRightTable {
    async fn create_table(db_conn: &DbConnection) {
        info!("Creating table: {:?}", AccessRightIden::Table);
        let sql = Table::create()
            .table(AccessRightIden::Table)
            .if_not_exists()
            .col(ColumnDef::new(AccessRightIden::Username).string().primary_key())
            .col(ColumnDef::new(AccessRightIden::AppName).string().not_null())
            .foreign_key(ForeignKey::create()
                // .name("fk_access_right_username")
                .from(AccessRightIden::Table, AccessRightIden::Username)
                .to(UserIden::Table, UserIden::Username)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
                )
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create table");
        // FATAL if failed to create table
    }
}

/// Get all app the given user can access to.
/// Return a list of app names.
pub async fn get_user_access_rights(db_conn: &DbConnection, username: &str) -> Result<Vec<String>> {
    let sql = Query::select()
        .column(AccessRightIden::AppName)
        .from(AccessRightIden::Table)
        .and_where(Expr::col(AccessRightIden::Username).eq(username))
        .to_string(SqliteQueryBuilder);

    let access_rights: Vec<(String, )> = sqlx::query_as(sql.as_str())
        .fetch_all(&db_conn.pool)
        .await?;

    Ok(access_rights.into_iter().map(|(app_name, )| app_name).collect())
}

pub async fn delete_all_access_of_user(db_conn: &DbConnection, username: &str) -> Result<()> {
    let sql = Query::delete()
        .from_table(AccessRightIden::Table)
        .cond_where(Expr::col(AccessRightIden::Username).eq(username))
        .to_string(SqliteQueryBuilder);

    sqlx::query(sql.as_str())
        .execute(&db_conn.pool)
        .await?;

    Ok(())
}

pub async fn add_access_right(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<()> {
    let sql = Query::insert()
        .into_table(AccessRightIden::Table)
        .columns(ACCESS_RIGHT_COLUMNS)
        .values([
            username.into(),
            app_name.into(),
        ])?
        .to_string(SqliteQueryBuilder);

    sqlx::query(sql.as_str())
        .execute(&db_conn.pool)
        .await?;

    Ok(())
}

pub async fn delete_access_right(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<()> {
    let sql = Query::delete()
        .from_table(AccessRightIden::Table)
        .cond_where(Expr::col(AccessRightIden::Username).eq(username))
        .cond_where(Expr::col(AccessRightIden::AppName).eq(app_name))
        .to_string(SqliteQueryBuilder);

    sqlx::query(sql.as_str())
        .execute(&db_conn.pool)
        .await?;

    Ok(())
}

pub async fn check_access_right_exists(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<bool> {
    let sql = Query::select()
        .expr(Expr::col(AccessRightIden::AppName).count())
        .from(AccessRightIden::Table)
        .and_where(Expr::col(AccessRightIden::Username).eq(username))
        .and_where(Expr::col(AccessRightIden::AppName).eq(app_name))
        .to_string(SqliteQueryBuilder);

    let count: (i64, ) = sqlx::query_as(sql.as_str())
        .fetch_one(&db_conn.pool)
        .await?;

    Ok(count.0 > 0)
}