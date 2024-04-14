use std::path::Path;

use well_i_known_server::server_init::ServerInit;
use well_i_known_server::WIKServer;
use well_i_known_server::config::server_config::*;

/// integration test for server initialization
/// can only use well_i_known_server pub methods that expose outside the crate
#[tokio::test]
async fn server_init(){
    // set config
    let server_config = WIKServerConfig::new("/Users/maisytse/Documents/Workspace/rust/well-I-known/server/resources/test/wik-server-config.json");
    let server_path = Path::new("/Users/maisytse/Documents/Workspace/rust/well-I-known/server/data/temp/");

    // init tracing
    let _guard = WIKServer::init_tracing(&server_config);
    
    // init server environment
    let mut server_env_config = WIKServerEnvironmentConfig {
        base_dir: server_path.to_path_buf(),
        config: server_config,
        root_user: None,
    };

    // empty the server directory for testing
    std::fs::remove_dir_all(server_path).expect("Fail to remove server directory.");
    // create a new server directory
    ServerInit::init_server_directory(&server_env_config);

    // create a db connection
    let conn = server_env_config.get_db_conn().await.unwrap();
    // create a new server database
    ServerInit::init_server_database(&conn).await;
    // create a new server root user
    ServerInit::init_root_user(&conn, "root", "root_password").await;

    // start the server
    let _ = WIKServer::start_server(&mut server_env_config).await;
}
