use super::{db_base::DbTable, db_connection::DbConnection};
use well_i_known_core::crypto::password;

use sqlx::FromRow;
use sea_query::{enum_def, SqliteQueryBuilder, ColumnDef, Asterisk, Table, Query, Expr};
use anyhow::Result;
use tracing::warn;

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub username: String,
    pub role: String,
    pub encrypted_password: String,
    pub password_salt: String,
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
                Ok((valid_user, user.role))
            } else {
                warn!("Failed to authenticate user {} due to wrong password", username);
                Err(anyhow::anyhow!("Failed to authenticate user"))
            }
        },
        Err(e) => {
            warn!("Failed to authenticate user: {:?}", e);
            Err(e)
        }
    }
}

pub struct UserTable {}
impl DbTable for UserTable {
    async fn create_table(&self, db_conn: &DbConnection) {
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