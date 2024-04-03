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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::fs;
    use crate::db::user;
    use crate::db::db_connection;

    fn get_test_path(filename: &str) -> PathBuf {
        let base_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(base_dir).join(filename).to_path_buf()
    }
    
    /// Create a new database for the test case by copying the base database
    async fn create_test_db(test_case_name: &str) -> DbConnection{
        let test_case_db_path = get_test_path(format!("output/{}.db", test_case_name).as_str());
        let test_base_path = get_test_path("resources/test/base-test.db");
        delete_test_db(test_case_name).await;
        fs::copy(&test_base_path, &test_case_db_path).unwrap();

        // create the connection
        let db_conn = db_connection::create_connection_pool(&test_case_db_path).await.unwrap();
        let db_conn = DbConnection { pool: db_conn };
        user::create_user(&db_conn, "u_root", "root", "password").await.unwrap();
        user::create_user(&db_conn, "u_admin", "admin", "password").await.unwrap();
        user::create_user(&db_conn, "u_app", "app", "password").await.unwrap();

        db_conn
    }

    async fn delete_test_db(test_case_name: &str) {
        let db_path = get_test_path(format!("output/{}.db", test_case_name).as_str());
        // delete the file if it exists
        if db_path.exists() {
            fs::remove_file(&db_path).unwrap();
        }
    }

    #[tokio::test]
    async fn test_add_and_get_access_right(){
        let db_conn = create_test_db("test_add_access_right").await;

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
        let db_conn = create_test_db("test_delete_one_access").await;

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
        let db_conn = create_test_db("test_delete_all_access").await;

        add_access_right(&db_conn, "u_admin", "test_app").await.unwrap();
        add_access_right(&db_conn, "u_admin", "test_app2").await.unwrap();
        delete_all_access_of_user(&db_conn, "u_admin").await.unwrap();
        let access_rights = get_user_access_rights(&db_conn, "u_admin").await.unwrap();
        assert_eq!(access_rights.len(), 0);
    }
}