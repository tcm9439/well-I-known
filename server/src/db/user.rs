// use sqlx::{FromRow, Row};
// use sea_query::{enum_def, Expr, Iden, Query, SqliteQueryBuilder, query::SelectExpr, ColumnDef, Table};
use sqlx::FromRow;
use sea_query::{enum_def, SqliteQueryBuilder, ColumnDef, Table, Query, Expr};
use anyhow::Result;
use tracing::warn;

use super::{db_base::DbTable, db_connection::DbConnection};
use well_i_known_core::password;

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct User {
    username: String,
    role: String,
    encrypted_password: String,
    password_salt: String,
    public_key: String,
    description: String,
}

// insert a user
// let query = Query::insert()
// .into_table(UserIden::Table)
// .columns([UserIden::Username])
// .values_panic(["dummy".into()])
// .to_string(SqliteQueryBuilder);
// sqlx::query(query.as_str()).execute(&pool).await.unwrap();

// let query = "select * from USER";
// let query = Query::select()
// .columns([UserIden::Id, UserIden::Username])
// .from(UserIden::Table)
// .to_string(SqliteQueryBuilder);

// let result = sqlx::query_as::<_,User>(query.as_str())
// .fetch_all(&pool)
// .await
// .unwrap();

impl User {
    pub async fn get_user(db_conn: &DbConnection, username: &str) -> Result<User>{
        let sql = Query::select()
            .columns([UserIden::Username, UserIden::Role, UserIden::EncryptedPassword, UserIden::PasswordSalt])
            .from(UserIden::Table)
            .and_where(Expr::col(UserIden::Username).eq(username))
            .to_string(SqliteQueryBuilder);

        let user = sqlx::query_as::<_, User>(sql.as_str())
            .fetch_one(&db_conn.pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(db_conn: &DbConnection, username: &str, role: &str, password: &str) -> Result<()> {
        let password = password::Password::new(password)?;

        let sql = Query::insert()
            .into_table(UserIden::Table)
            .columns([UserIden::Username, UserIden::Role, UserIden::EncryptedPassword, UserIden::PasswordSalt])
            .values_panic([username.into(), role.into(), password.hash.into(), password.salt.into()])
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await?;

        Ok(())
    }

    pub async fn auth_user(db_conn: &DbConnection, username: &str, password: &str) -> Result<bool> {
        match User::get_user(db_conn, username).await {
            Ok(user) => {
                Ok(password::verify_password(password, &user.encrypted_password, &user.password_salt))
            },
            Err(e) => {
                warn!("Failed to authenticate user: {:?}", e);
                Err(e)
            }
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
            .col(ColumnDef::new(UserIden::PublicKey).string().not_null())
            .col(ColumnDef::new(UserIden::Description).string())
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await.expect("Failed to create table");
    }
}