use tracing::*;
use std::{fs, path::PathBuf};
use anyhow::{Context, Result};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Clone)]
pub struct DbConnection {
    pub pool: Pool<Sqlite>,
}

/// Check if the database file exists.
pub fn check_if_database_exists(db_path: &PathBuf) -> bool {
    match fs::metadata(db_path) {
        Ok(_) => {
            info!("Db file exists.");
            return true;
        }
        Err(_) => {
            debug!("File '{:?}' does not exist.", db_path);
            return false
        }
    };
}

/// Create a new database at the given path.
pub fn create_database(db_path: &PathBuf) -> Result<()> {
    // create an empty database file
    let _ = fs::File::create(db_path)?;
    return Ok(());
}

pub async fn enable_sqlite_foreign_key_support(db_conn: &DbConnection) -> Result<()> {
    let sql = "PRAGMA foreign_keys = ON;";
    sqlx::query(sql)
        .execute(&db_conn.pool)
        .await?;
    Ok(())
}

/// Create a new connection to the database.
/// If the connection fail, check if the database file exists.
pub async fn create_connection_pool(db_path: &PathBuf) -> Result<Pool<Sqlite>> {
    let db_path = db_path.to_str();
    match db_path {
        Some(db_path) => {
            let db_connection_str = String::from("sqlite:") + db_path;
            SqlitePoolOptions::new()
                .connect(&db_connection_str)
                .await
                .with_context(|| format!("Failed to create connection to {:?}", db_path))
        }
        None => {
            Err(anyhow::anyhow!("Failed to convert db_path to string."))
        }
    }
}

impl DbConnection {
    pub async fn new(db_path: &PathBuf) -> Result<Self> {
        let db_exists = check_if_database_exists(db_path);
        if !db_exists {
            return Err(anyhow::anyhow!("Database file does not exist."));
        }

        let pool = create_connection_pool(db_path).await?;
        let conn = DbConnection {
            pool,
        };

        Ok(conn)
    }
}
