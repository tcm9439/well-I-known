use crate::dao::user::UserIden;
use crate::db::{db_base::DbTable, db_connection::DbConnection};

use sqlx::FromRow;
use sea_query::{enum_def, ColumnDef, Expr, ForeignKey, ForeignKeyAction, Query, SqliteQueryBuilder, Table};
use tracing::info;
use anyhow::Result;

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct AccessRight {
    pub username: String,
    pub app_name: String,
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
            .col(ColumnDef::new(AccessRightIden::Username).string())
            .col(ColumnDef::new(AccessRightIden::AppName).string())
            .primary_key(sea_query::Index::create()
                .col(AccessRightIden::Username)
                .col(AccessRightIden::AppName)
            )
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

/// Delete all access rights of the given user.
/// Used when deleting an admin which has access rights to some apps.
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

// Delete all access rights of the given app.
pub async fn delete_all_access_of_app(db_conn: &DbConnection, app_name: &str) -> Result<()> {
    let sql = Query::delete()
        .from_table(AccessRightIden::Table)
        .cond_where(Expr::col(AccessRightIden::AppName).eq(app_name))
        .to_string(SqliteQueryBuilder);

    sqlx::query(sql.as_str())
        .execute(&db_conn.pool)
        .await?;

    Ok(())
}

/// Add access right of all config of a given app to the given user.
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

/// Delete the access right of the given user to the given app.
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

/// Check if the given user has access right to the given app.
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

#[cfg(test)]
mod tests {
    use super::*;
    use well_i_known_core::modal::user::UserRole;
    use crate::db::db_test_util::create_test_db;
    use crate::dao::user;
    
    async fn create_access_right_test_db(test_case_name: &str) -> DbConnection{
        let db_conn = create_test_db(test_case_name).await;
        user::create_user(&db_conn, "u_root", &UserRole::Root, "password").await.unwrap();
        user::create_user(&db_conn, "u_admin", &UserRole::Admin, "password").await.unwrap();
        user::create_user(&db_conn, "u_app", &UserRole::App, "password").await.unwrap();
        db_conn
    }

    #[tokio::test]
    async fn test_add_and_get_access_right(){
        let db_conn = create_access_right_test_db("test_add_access_right").await;

        let has_access = check_access_right_exists(&db_conn, "u_admin", "test_app").await.unwrap();
        assert_eq!(has_access, false);
        
        // grant right
        add_access_right(&db_conn, "u_admin", "test_app").await.unwrap();

        let has_access = check_access_right_exists(&db_conn, "u_admin", "test_app").await.unwrap();
        assert_eq!(has_access, true);

        let access_rights = get_user_access_rights(&db_conn, "u_admin").await.unwrap();
        assert_eq!(access_rights.len(), 1);
        assert_eq!(access_rights[0], "test_app");

        add_access_right(&db_conn, "u_admin", "test_app2").await.unwrap();
        let access_rights = get_user_access_rights(&db_conn, "u_admin").await.unwrap();
        assert_eq!(access_rights.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_one_access(){
        let db_conn = create_access_right_test_db("test_delete_one_access").await;

        add_access_right(&db_conn, "u_admin", "test_app").await.unwrap();
        add_access_right(&db_conn, "u_admin", "test_app2").await.unwrap();
        delete_access_right(&db_conn, "u_admin", "test_app").await.unwrap();
        let has_access = check_access_right_exists(&db_conn, "u_admin", "test_app").await.unwrap();
        assert_eq!(has_access, false);
        let access_rights = get_user_access_rights(&db_conn, "u_admin").await.unwrap();
        assert_eq!(access_rights.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_all_access(){
        let db_conn = create_access_right_test_db("test_delete_all_access").await;

        add_access_right(&db_conn, "u_admin", "test_app").await.unwrap();
        add_access_right(&db_conn, "u_admin", "test_app2").await.unwrap();
        delete_all_access_of_user(&db_conn, "u_admin").await.unwrap();
        let access_rights = get_user_access_rights(&db_conn, "u_admin").await.unwrap();
        assert_eq!(access_rights.len(), 0);
    }
}