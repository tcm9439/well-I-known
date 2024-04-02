mod auth;
mod config;
mod controller;
mod db;
mod error;
mod repository;
mod server_state;
mod server_init;

use auth::jwt_controller::authorize_handler;
use db::db_connection::DbConnection;
use controller::user::*;
use controller::admin::*;
use controller::data::*;
use config::server_config::*;
use server_state::ServerState;

use axum::{routing::{delete, get, post}, Router};
use anyhow::Result;
use tracing::*;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use std::path::Path;

/// Init tracing by the loaded logging config.
fn init_tracing(server_config: &WIKServerConfig) -> WorkerGuard {
    // TODO change timestamp to local time instead of UTC
    
    // register tracing file appender
    // _guard is needed to be in / returned to main()
    let (non_blocking_trace_file_appender, guard) = tracing_appender::non_blocking(server_config.logging.get_logging_file_appender());

    // logging to stdout seems to be enabled by default for fmt::Subscriber
    let subscriber = fmt::Subscriber::builder()
        .with_max_level(server_config.logging.get_logging_level())
        .finish()
        .with(fmt::Layer::default()
            .with_ansi(false)
            .with_writer(non_blocking_trace_file_appender));

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber.");

    guard
}

/// Start the server with the loaded server config.
/// server_base_dir: The base directory of the server data.
/// server_config: The server config.
async fn start_server(server_config: &WIKServerEnvironmentConfig) -> Result<()> {
    debug!("Starting server...");
    debug!("Init TLS...");
    let tls_config = server_config.config.tls.get_rustls_config().await;
    debug!("Init database connection...");
    let db_path = server_config.to_full_path(&server_config.config.db_path);
    let db_conn = DbConnection::new(&db_path).await?;

    let server_state = ServerState {
        db_conn,
        config: server_config.clone(),
        jwt_keys: auth::jwt_key::JwtKeys::new(server_config.config.jwt_secret.as_bytes()),
    };

    // register the routes
    trace!("Registering routes...");
    let app = Router::new()
        .route("/login", post(authorize_handler))
        .route("/data", get(get_data_handler))
        .route("/data", post(alter_data_handler))
        .route("/data", delete(delete_data_handler))
        .route("/users/validate", post(validate_user_handler))
        .route("/users", post(alter_user_handler))
        .route("/users", delete(delete_user_handler))
        .route("/admin/access", post(admin_access_handler))
        .with_state(server_state.into());
    
    info!("Server started at: {}", server_config.config.get_server_ip());
    // start the server
    axum_server::bind_rustls(server_config.config.get_server_ip(), tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}


#[tokio::main]
async fn main() {
    // load the server config
    let server_config = WIKServerConfig::new("/Users/maisytse/Documents/Workspace/rust/well-I-known/server/resources/test/wik-server-config.json");

    let _guard = init_tracing(&server_config);
    let server_path = Path::new("./data/temp/");
    let server_env_config = WIKServerEnvironmentConfig {
        base_dir: server_path.to_path_buf(),
        config: server_config,
        root_key: None,
    };
    let _ = start_server(&server_env_config).await;
}
