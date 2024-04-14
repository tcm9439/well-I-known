use crate::dao::{access_right::AccessRightTable, config_data::ConfigDataTable, user::UserTable};
use crate::repository::user::UserRepository;
use crate::db::db_base::DbTable;
use crate::db::db_connection::DbConnection;
use crate::config::server_config::{self, WIKServerEnvironmentConfig};
use well_i_known_core::crypto::cryptography::WikRsaKeyPair;

use tracing::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::id;

pub struct ServerInit {}

fn create_dir_if_not_exists(path: &PathBuf){
    if !path.exists() {
        // panic if fail to create the directory
        std::fs::create_dir_all(path).expect("Fail to create server directory.");
    }
}

/// Server initialization functions.
/// Should only be called once when the server start for the first time.
impl ServerInit {
    /// Create all server directories.
    /// Including config, tls, root_certs, users_certs, data, log.
    pub fn init_server_directory(config: &WIKServerEnvironmentConfig){
        debug!("Initializing server directory...");

        trace!("Creating server directory...");
        create_dir_if_not_exists(&config.get_config_dir_path());
        create_dir_if_not_exists(&config.get_tls_certs_dir_path());
        let root_certs_dir = config.get_root_certs_dir_path();
        create_dir_if_not_exists(&root_certs_dir);
        create_dir_if_not_exists(&config.get_users_certs_dir_path());
        create_dir_if_not_exists(&config.get_data_dir_path());
        create_dir_if_not_exists(&config.get_log_dir_path());

        trace!("Creating database...");
        let db_path = config.get_db_path();
        if !DbConnection::check_if_database_exists(&db_path) {
            DbConnection::create_database(&db_path).expect("Fail to create database.");
        }

        trace!("Generating root keys...");
        let root_key_pair = WikRsaKeyPair::new().expect("Fail to generate root key pair.");
        root_key_pair.save(
            &root_certs_dir, 
            server_config::ROOT_KEY_PEM_FILENAME, 
            server_config::ROOT_CERT_PEM_FILENAME)
            .expect("Fail to save root key pair.");

        debug!("Server directory initialized.");
    }

    /// Initialize the server database.
    /// Create database tables.
    pub async fn init_server_database(db_conn: &DbConnection) {
        info!("Enabling sqlite foreign key support...");
        DbConnection::enable_sqlite_foreign_key_support(db_conn).await.expect("Fail to enable sqlite foreign key support.");
        info!("Creating database tables...");
        UserTable::create_table(db_conn).await;
        AccessRightTable::create_table(db_conn).await;
        ConfigDataTable::create_table(db_conn).await;
        info!("Tables created.");
    }

    /// Create the root user. Should only be called once.
    pub async fn init_root_user(db_conn: &DbConnection, username: &str, password: &str) {
        UserRepository::create_root_user(db_conn, username, password).await.expect("Fail to create root user.");
    }

    /// Write the server pid to the pid file.
    /// If fail to write, the program exit and the server will not start.
    pub fn write_server_pid(base_path: &PathBuf){
        debug!("Writing pid to file...");

        // check if the file already exists
        let pid_file_path = base_path.join("data").join("wellik-server.pid");
        if pid_file_path.exists() {
            panic!("Fail to start server. Pid file already exists. Please check if the server is already running.");
        }

        let pid = id();
        let mut file = File::create(pid_file_path).expect("Fail to create pid file.");
        file.write_all(pid.to_string().as_bytes()).expect("Fail to write pid to file.");
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    use crate::db::db_connection::DbConnection;

    fn get_test_path(filename: &str) -> PathBuf {
        let base_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(base_dir).join(filename).to_path_buf()
    }
    
    #[tokio::test]
    async fn test_create_dir_if_not_exists() {
        let test_dir = get_test_path("output/test_create_dir_if_not_exists");
        assert_eq!(test_dir.exists(), false);
        create_dir_if_not_exists(&test_dir);
        assert!(test_dir.exists());
        std::fs::remove_dir(&test_dir).unwrap();
    }
    
    #[tokio::test]
    async fn test_init_server_database(){
        let test_dir = get_test_path("output/db_init");
        create_dir_if_not_exists(&test_dir);

        // create database
        let db_path = test_dir.join("test.db");
        DbConnection::create_database(&db_path).expect("Fail to create database.");

        // create connection pool
        let db_conn = DbConnection::new(&db_path).await.unwrap();

        // init database
        ServerInit::init_server_database(&db_conn).await;
        std::fs::remove_file(&db_path).unwrap();
    }
}