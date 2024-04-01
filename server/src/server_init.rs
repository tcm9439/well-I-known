use crate::db::{access_right::AccessRightTable, config_data::ConfigDataTable, db_connection::DbConnection, user::UserTable, db_base::DbTable};

use tracing::{debug, info};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::id;

pub fn init_server_directory(path: &PathBuf){
    debug!("Initializing server directory...");

    // // Create the server directory if it does not exist
    // if !path.exists() {
    //     std::fs::create_dir_all(path).expect("Fail to create server directory.");
    // }
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
pub fn write_server_pid(pid_file_path: &str){
    debug!("Writing pid to file...");

    // check if the file already exists
    if std::path::Path::new(pid_file_path).exists() {
        panic!("Fail to start server. Pid file already exists. Please check if the server is already running.");
    }

    let pid = id();
    let mut file = File::create(pid_file_path).expect("Fail to create pid file.");
    file.write_all(pid.to_string().as_bytes()).expect("Fail to write pid to file.");
}
