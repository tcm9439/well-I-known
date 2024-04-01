use crate::db::db_base;
use crate::db::{access_right::AccessRightTable, config_data::ConfigDataTable, db_connection::DbConnection, user::UserTable, db_base::DbTable};
use crate::db::db_connection::{check_if_database_exists, create_database};
use well_i_known_core::crypto::cryptography::RsaKeyPair;

use tracing::{debug, info, trace};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::id;

pub fn create_dir_if_not_exists(path: &PathBuf){
    if !path.exists() {
        // panic if fail to create the directory
        std::fs::create_dir_all(path).expect("Fail to create server directory.");
    }
}

pub fn init_server_directory(config: &WIKServerEnvironmentConfig){
    debug!("Initializing server directory...");

    trace!("Creating server directory...");
    create_dir_if_not_exists(config.get_config_dir_path());
    create_dir_if_not_exists(config.get_tls_certs_dir_path());
    let root_certs_dir = config.get_root_certs_dir_path();
    create_dir_if_not_exists(root_certs_dir);
    create_dir_if_not_exists(config.get_users_certs_dir_path());
    create_dir_if_not_exists(config.get_data_dir_path());
    create_dir_if_not_exists(config.get_log_dir_path());

    trace!("Creating database...");
    let db_path = config.get_db_path();
    if !check_if_database_exists(db_path) {
        create_database(db_path);
    }

    trace!("Generating root keys...");
    let root_key_pair = RsaKeyPair::new();
    root_key_pair.save_to_pem_file(root_certs_dir, "wellik-root-key.pem", "wellik-root-cert.pem");

    debug!("Server directory initialized.");
}

pub async fn create_tables(db_conn: &DbConnection) {
    info!("Creating database tables...");
    UserTable{}.create_table(db_conn).await;
    AccessRightTable{}.create_table(db_conn).await;
    ConfigDataTable{}.create_table(db_conn).await;
    info!("Tables created.");
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
