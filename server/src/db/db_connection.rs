use tracing::{debug, info};
use std::fs;
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
            debug!("File '{}' does not exist. Creating a new database file now...", db_path);
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

/// Create a new connection to the database.
/// If the connection fail, check if the database file exists.
pub async fn create_connection_pool(db_path: &PathBuf) -> Result<Pool<Sqlite>> {
    let db_path = String::from("sqlite:") + db_path;
    SqlitePoolOptions::new()
        .connect(&db_path)
        .await
        .with_context(|| format!("Failed to create connection to {}", db_path))
}

impl DbConnection {
    pub async fn new(db_path: &str) -> Result<Self> {
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
