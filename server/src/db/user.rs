use super::{db_base::DbTable, db_connection::DbConnection};
use well_i_known_core::crypto::password;

use sqlx::FromRow;
use sea_query::{enum_def, SqliteQueryBuilder, ColumnDef, Asterisk, Table, Query, Expr};
use anyhow::Result;
use tracing::{info, warn};

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub username: String,
    pub role: String,
    pub encrypted_password: String,
    pub password_salt: String,
}

const USER_COLUMNS: [UserIden; 4] = [
    UserIden::Username,
    UserIden::Role,
    UserIden::EncryptedPassword,
    UserIden::PasswordSalt,
];

pub struct UserTable {}
impl DbTable for UserTable {
    async fn create_table(db_conn: &DbConnection) {
        let sql = Table::create()
            .table(UserIden::Table)
            .if_not_exists()
            .col(ColumnDef::new(UserIden::Username).string().primary_key())
            .col(ColumnDef::new(UserIden::EncryptedPassword).string().not_null())
            .col(ColumnDef::new(UserIden::PasswordSalt).string().not_null())
            .col(ColumnDef::new(UserIden::Role).string().not_null())
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create table");
    }
}

pub async fn get_user(db_conn: &DbConnection, username: &str) -> Result<User>{
    let sql = Query::select()
        .column(Asterisk)
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Username).eq(username))
        .to_string(SqliteQueryBuilder);

    let user = sqlx::query_as::<_, User>(sql.as_str())
        .fetch_one(&db_conn.pool)
        .await?;

    Ok(user)
}

/// Check if the given username exists in the database.
/// Return true if the username exists, otherwise false.
pub async fn check_user_exists(db_conn: &DbConnection, username: &str) -> Result<bool> {
    let sql = Query::select()
        .expr(Expr::col(UserIden::Username).count())
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Username).eq(username))
        .to_string(SqliteQueryBuilder);

    let count: (i32, ) = sqlx::query_as(sql.as_str())
        .fetch_one(&db_conn.pool)
        .await?;

    Ok(count.0 == 1)
}

/// Check if the root user already exists in the database.
pub async fn check_root_exists(db_conn: &DbConnection) -> Result<bool> {
    let sql = Query::select()
        .expr(Expr::col(UserIden::Username).count())
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Role).eq("root"))
        .to_string(SqliteQueryBuilder);

    let count: (i32, ) = sqlx::query_as(sql.as_str())
        .fetch_one(&db_conn.pool)
        .await?;

    Ok(count.0 == 1)
}

/// Create a new user, which involve:
/// - generating a new salt and hash the password.
/// - create a pem file to store the public key.
pub async fn create_user(db_conn: &DbConnection, 
    username: &str, role: &str, password: &str
) -> Result<()> {
    let password = password::Password::new(password)?;

    let sql = Query::insert()
        .into_table(UserIden::Table)
        .columns(USER_COLUMNS)
        .values([
            username.into(), 
            role.into(), 
            password.hash.into(), 
            password.salt.into(), 
        ])?
        .to_string(SqliteQueryBuilder);

    sqlx::query(sql.as_str())
        .execute(&db_conn.pool)
        .await?;

    Ok(())
}

/// Update the user's info. 
/// Attributes allowed to update:
///  - password (& salt)
pub async fn update_user(db_conn: &DbConnection, 
    username: &str, password: &str,
) -> Result<()> {
    let password = password::Password::new(password)?;

        let sql = Query::update()
            .table(UserIden::Table)
            .values([
                (UserIden::EncryptedPassword, password.hash.into()), 
                (UserIden::PasswordSalt, password.salt.into()), 
            ])
            .and_where(Expr::col(UserIden::Username).eq(username))
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await?;

        Ok(())
}

/// Delete the user from the database.
pub async fn delete_user(db_conn: &DbConnection, username: &str) -> Result<()> {
    let sql = Query::delete()
        .from_table(UserIden::Table)
        .cond_where(Expr::col(UserIden::Username).eq(username))
        .to_string(SqliteQueryBuilder);

    sqlx::query(sql.as_str())
        .execute(&db_conn.pool)
        .await?;

    Ok(())
}

/// Check if the given username and password is valid.
/// Return 
///     - true if the password is valid, otherwise false.
///     - role of the user.
pub async fn auth_user(db_conn: &DbConnection, username: &str, password: &str) -> Result<(bool, String)> {
    match get_user(db_conn, username).await {
        Ok(user) => {
            let valid_user = password::verify_password(password, &user.encrypted_password, &user.password_salt);
            if valid_user{
                Ok((true, user.role))
            } else {
                warn!("Failed to authenticate user {} due to wrong password", username);
                Ok((false, "".to_string()))
            }
        },
        Err(e) => {
            warn!("Failed to authenticate user: {:?}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::fs;
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
        DbConnection { pool: db_conn }
    }

    async fn delete_test_db(test_case_name: &str) {
        let db_path = get_test_path(format!("output/{}.db", test_case_name).as_str());
        // delete the file if it exists
        if db_path.exists() {
            fs::remove_file(&db_path).unwrap();
        }
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let db_conn = create_test_db("test_create_and_get_user").await;
        let _ = create_user(&db_conn, "test_user", "root", "password").await.unwrap();
        let user = get_user(&db_conn, "test_user").await.unwrap();
        assert_eq!(user.username, "test_user");
        assert_eq!(user.role, "root");
    }

    #[tokio::test]
    async fn test_user_exists() {
        let db_conn = create_test_db("test_user_exists").await;
        let exists = check_user_exists(&db_conn, "test_user").await.unwrap();
        assert_eq!(exists, false);
        let exists = check_root_exists(&db_conn).await.unwrap();
        assert_eq!(exists, false);

        let _ = create_user(&db_conn, "test_user", "root", "password").await.unwrap();

        let exists = check_user_exists(&db_conn, "test_user").await.unwrap();
        assert_eq!(exists, true);
        let exists = check_root_exists(&db_conn).await.unwrap();
        assert_eq!(exists, true);
    }

    #[tokio::test]
    async fn test_auth_user() {
        let db_conn = create_test_db("test_auth_user").await;
        let _ = create_user(&db_conn, "test_user", "root", "password").await.unwrap();
        let (valid, role) = auth_user(&db_conn, "test_user", "password").await.unwrap();
        assert_eq!(valid, true);
        assert_eq!(role, "root");

        let (valid, _) = auth_user(&db_conn, "test_user", "wrong_password").await.unwrap();
        assert_eq!(valid, false);
    }

    #[tokio::test]
    async fn test_update_user() {
        let db_conn = create_test_db("test_update_user").await;
        let _ = create_user(&db_conn, "test_user", "root", "password").await.unwrap();
        let _ = update_user(&db_conn, "test_user", "new_password").await.unwrap();
        let user = get_user(&db_conn, "test_user").await.unwrap();
        assert_eq!(password::verify_password("new_password", &user.encrypted_password, &user.password_salt), true);
    }
}