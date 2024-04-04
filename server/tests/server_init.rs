use std::path::Path;

use well_i_known_server::server_init::*;
use well_i_known_server::*;
use well_i_known_server::config::server_config::*;

#[tokio::test]
async fn server_init(){
    let server_config = WIKServerConfig::new("/Users/maisytse/Documents/Workspace/rust/well-I-known/server/resources/test/wik-server-config.json");
    let _guard = init_tracing(&server_config);
    let server_path = Path::new("/Users/maisytse/Documents/Workspace/rust/well-I-known/server/data/temp/");
    let mut server_env_config = WIKServerEnvironmentConfig {
        base_dir: server_path.to_path_buf(),
        config: server_config,
        root_user: None,
    };

    // empty the server directory
    std::fs::remove_dir_all(server_path).expect("Fail to remove server directory.");
    // create a new server directory
    init_server_directory(&server_env_config);
    let conn = server_env_config.get_db_conn().await.unwrap();
    // create a new server database
    init_server_database(&conn).await;
    // create a new server root user
    init_root_user(&conn, "root", "root_password").await;

    let _ = start_server(&mut server_env_config).await;
}
