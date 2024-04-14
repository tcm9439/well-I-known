use sqlx::FromRow;
use sea_query::{enum_def, ColumnDef, Expr, ForeignKey, ForeignKeyAction, Query, SqliteQueryBuilder, Table};
use tracing::info;
use anyhow::Result;

use crate::db::{db_base::DbTable, db_connection::DbConnection};
use crate::dao::user::UserIden;

#[enum_def]
#[derive(Clone, FromRow, Debug)]
pub struct ConfigData {
    pub app_name: String,
    pub key: String,
    pub owner: String,
    pub value: String,
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

impl ConfigDataTable {
    /// Get the encrypted value of the given key for the given owner.
    pub async fn get_data_value(db_conn: &DbConnection, app_name: &str, owner: &str, key: &str) -> Result<Option<String>> {
        let sql = Query::select()
            .column(ConfigDataIden::Value)
            .from(ConfigDataIden::Table)
            .and_where(Expr::col(ConfigDataIden::AppName).eq(app_name))
            .and_where(Expr::col(ConfigDataIden::Key).eq(key))
            .and_where(Expr::col(ConfigDataIden::Owner).eq(owner))
            .to_string(SqliteQueryBuilder);

        let data: Option<(String, )> = sqlx::query_as(sql.as_str())
            .fetch_optional(&db_conn.pool)
            .await?;

        Ok(data.map(|(value, )| value))
    }

    /// Check if the records exists for the given 'app, key, owner' pair.
    pub async fn check_data_exists(db_conn: &DbConnection, app_name: &str, owner: &str, key: &str) -> Result<bool> {
        let sql = Query::select()
            .expr(Expr::col(ConfigDataIden::Key).count())
            .from(ConfigDataIden::Table)
            .and_where(Expr::col(ConfigDataIden::AppName).eq(app_name))
            .and_where(Expr::col(ConfigDataIden::Key).eq(key))
            .and_where(Expr::col(ConfigDataIden::Owner).eq(owner))
            .to_string(SqliteQueryBuilder);

        let count: (i32, ) = sqlx::query_as(sql.as_str())
            .fetch_one(&db_conn.pool)
            .await?;

        Ok(count.0 == 1)
    }

    /// Check if the records exists for the given 'app, key, owner' pair.
    pub async fn check_data_exists_for_key(db_conn: &DbConnection, app_name: &str, key: &str) -> Result<bool> {
        let sql = Query::select()
            .expr(Expr::col(ConfigDataIden::Key).count())
            .from(ConfigDataIden::Table)
            .and_where(Expr::col(ConfigDataIden::AppName).eq(app_name))
            .and_where(Expr::col(ConfigDataIden::Key).eq(key))
            .to_string(SqliteQueryBuilder);

        let count: (i32, ) = sqlx::query_as(sql.as_str())
            .fetch_one(&db_conn.pool)
            .await?;

        Ok(count.0 > 0)
    }

    /// Set the data value for the given 'app, key, owner' pair.
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

    /// Update the data value for the given 'app, key, owner' pair.
    pub async fn update_data_value(db_conn: &DbConnection, app_name: &str, owner: &str, key: &str, value: &str) -> Result<()> {
        let sql = Query::update()
            .table(ConfigDataIden::Table)
            .values([
                (ConfigDataIden::Value, value.into())
            ])
            .and_where(Expr::col(ConfigDataIden::AppName).eq(app_name))
            .and_where(Expr::col(ConfigDataIden::Key).eq(key))
            .and_where(Expr::col(ConfigDataIden::Owner).eq(owner))
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await?;

        Ok(())
    }

    /// Delete the data for the given 'app, key, owner' pair.
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

    /// Delete all data for the given 'app, key' pair.
    /// Useful when deleting a key for an.
    pub async fn delete_all_app_key_data(db_conn: &DbConnection, app_name: &str, key: &str) -> Result<()> {
        let sql = Query::delete()
            .from_table(ConfigDataIden::Table)
            .cond_where(Expr::col(ConfigDataIden::AppName).eq(app_name))
            .cond_where(Expr::col(ConfigDataIden::Key).eq(key))
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await?;

        Ok(())
    }

    /// Delete all data for the given 'app'.
    /// Useful when deleting an app.
    pub async fn delete_all_app_data(db_conn: &DbConnection, app_name: &str) -> Result<()> {
        let sql = Query::delete()
            .from_table(ConfigDataIden::Table)
            .cond_where(Expr::col(ConfigDataIden::AppName).eq(app_name))
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await?;

        Ok(())
    }

    /// Delete all data for the given 'owner'.
    /// Useful when deleting a user.
    pub async fn delete_all_data_for_owner(db_conn: &DbConnection, owner: &str) -> Result<()> {
        let sql = Query::delete()
            .from_table(ConfigDataIden::Table)
            .cond_where(Expr::col(ConfigDataIden::Owner).eq(owner))
            .to_string(SqliteQueryBuilder);

        sqlx::query(sql.as_str())
            .execute(&db_conn.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use well_i_known_core::modal::user::UserRole;

    use super::*;
    use crate::dao::user::UserTable;
    use crate::db::db_test_util::*;

    async fn create_config_data_test_db(test_case_name: &str) -> DbConnection{
        // create the connection
        let db_conn = create_test_db(test_case_name).await;
        // insert base data
        UserTable::create_user(&db_conn, "u_root", &UserRole::Root, "password").await.unwrap();
        UserTable::create_user(&db_conn, "u_admin", &UserRole::Admin, "password").await.unwrap();
        UserTable::create_user(&db_conn, "u_app", &UserRole::App, "password").await.unwrap();
        db_conn
    }

    #[tokio::test]
    async fn test_add_and_get_data(){
        let db_conn = create_config_data_test_db("test_add_and_get_data").await;

        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(exists, false);
        
        ConfigDataTable::set_data_value(&db_conn, "u_app", "u_root", "test_key", "test_value").await.unwrap();

        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(exists, true);
        let value = ConfigDataTable::get_data_value(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[tokio::test]
    async fn test_update_data(){
        let db_conn = create_config_data_test_db("test_update_data").await;

        ConfigDataTable::set_data_value(&db_conn, "u_app", "u_root", "test_key", "test_value").await.unwrap();

        let value = ConfigDataTable::get_data_value(&db_conn, "u_app", "u_root", "test_key").await.unwrap().unwrap();
        assert_eq!(value, "test_value");

        ConfigDataTable::update_data_value(&db_conn, "u_app", "u_root", "test_key", "new_value").await.unwrap();

        let value = ConfigDataTable::get_data_value(&db_conn, "u_app", "u_root", "test_key").await.unwrap().unwrap();
        assert_eq!(value, "new_value");
    }

    #[tokio::test]
    async fn test_delete_data(){
        let db_conn = create_config_data_test_db("test_delete_data").await;

        ConfigDataTable::set_data_value(&db_conn, "u_app", "u_root", "test_key", "test_value").await.unwrap();

        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(exists, true);

        ConfigDataTable::delete_data(&db_conn, "u_app", "u_root", "test_key").await.unwrap();

        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(exists, false);
    }

    #[tokio::test]
    async fn test_delete_all_app_data(){
        let db_conn = create_config_data_test_db("test_delete_all_app_data").await;

        ConfigDataTable::set_data_value(&db_conn, "u_app", "u_root", "test_key", "test_value").await.unwrap();
        ConfigDataTable::set_data_value(&db_conn, "u_app", "u_root", "test_key2", "test_value2").await.unwrap();

        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(exists, true);
        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key2").await.unwrap();
        assert_eq!(exists, true);

        ConfigDataTable::delete_all_app_data(&db_conn, "u_app").await.unwrap();

        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(exists, false);
        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key2").await.unwrap();
        assert_eq!(exists, false);
    }

    #[tokio::test]
    async fn test_delete_all_data_for_owner(){
        let db_conn = create_config_data_test_db("test_delete_all_data_for_owner").await;

        ConfigDataTable::set_data_value(&db_conn, "u_app", "u_root", "test_key", "test_value").await.unwrap();
        ConfigDataTable::set_data_value(&db_conn, "u_app", "u_root", "test_key2", "test_value2").await.unwrap();

        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(exists, true);
        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key2").await.unwrap();
        assert_eq!(exists, true);

        ConfigDataTable::delete_all_data_for_owner(&db_conn, "u_root").await.unwrap();

        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key").await.unwrap();
        assert_eq!(exists, false);
        let exists = ConfigDataTable::check_data_exists(&db_conn, "u_app", "u_root", "test_key2").await.unwrap();
        assert_eq!(exists, false);
    }
}