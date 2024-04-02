// async fn main() {
//     // load the server config
//     let server_config = WIKServerConfig::new("/Users/maisytse/Documents/Workspace/rust/well-I-known/server/resources/test/wik-server-config.json");

//     let _guard = init_tracing(&server_config);
//     let server_path = Path::new("./data/temp/");
//     let server_env_config = WIKServerEnvironmentConfig {
//         base_dir: server_path.to_path_buf(),
//         config: server_config,
//         root_key: None,
//     };
//     let _ = start_server(&server_env_config).await;
// }

#[test]
fn server_init(){

}
