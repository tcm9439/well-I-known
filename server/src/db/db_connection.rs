use tracing::{debug, info};
use std::fs;
use anyhow::{Context, Result};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Clone)]
pub struct DbConnection {
    pub pool: Pool<Sqlite>,
    pub is_new_db: bool,
}

/// Returns if a new database is created.
pub async fn create_database_if_not_exists(db_path: &str) -> Result<bool> {
    match fs::metadata(db_path) {
        Ok(_) => {
            info!("Db file exists.");
            return Ok(false);
        }
        Err(_) => debug!("File '{}' does not exist. Creating a new database file now...", db_path),
    };

    // create an empty database file
    let _ = fs::File::create(db_path)?;
    return Ok(true);
}

/// Create a new connection to the database.
/// If the connection fail, check if the database file exists.
pub async fn create_connection_pool(db_path: &str) -> Result<Pool<Sqlite>> {
    let db_path = String::from("sqlite:") + db_path;
    SqlitePoolOptions::new()
        .connect(&db_path)
        .await
        .with_context(|| format!("Failed to create connection to {}", db_path))
}

impl DbConnection {
    pub async fn new(db_path: &str) -> Result<Self> {
        let is_new_db = create_database_if_not_exists(db_path).await?;

        let pool = create_connection_pool(db_path).await?;
        let conn = DbConnection {
            pool,
            is_new_db,
        };

        Ok(conn)
    }
}
