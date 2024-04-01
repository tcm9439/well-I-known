mod error;
mod auth;
mod config;
mod db;
mod server_state;
mod server_init;
mod server_controller;

use std::{io, path::Path};

use auth::jwt_controller::authorize_handler;
use db::db_connection::DbConnection;
use server_controller::*;
use config::server_config::*;

use axum::{routing::{delete, get, post}, Router};
use server_state::ServerState;
use anyhow::Result;
use tracing::{debug, info, trace};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::writer::MakeWriterExt;

/// Init tracing by the loaded logging config.
fn init_tracing(server_config: &WIKServerConfig) -> WorkerGuard {
    // register tracing file appender
    // _guard is needed to be in / returned to main()
    let (non_blocking_trace_file_appender, guard) = tracing_appender::non_blocking(server_config.logging.get_logging_file_appender());

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(server_config.logging.get_logging_level())
        .with_writer(non_blocking_trace_file_appender)
        .with_writer(io::stdout.with_max_level(tracing::Level::INFO))
        .with_ansi(false) // turn off ansi colors
        .init();

    guard
}

/// Start the server with the loaded server config.
/// server_base_dir: The base directory of the server data.
/// server_config: The server config.
async fn start_server(server_base_dir: &Path, server_config: &WIKServerConfig) -> Result<()> {
    debug!("Starting server...");
    debug!("Init TLS...");
    let tls_config = server_config.tls.get_rustls_config().await;
    debug!("Init database connection...");
    let db_conn = DbConnection::new(&server_config.db_path).await?;

    let server_state = ServerState {
        db_conn,
        config: server_config.clone(),
        jwt_keys: auth::jwt_key::JwtKeys::new(server_config.jwt_secret.as_bytes()),
    };

    // register the routes
    trace!("Registering routes...");
    let app = Router::new()
        .route("/login", post(authorize_handler))
        .route("/data", get(get_data_handler))
        .route("/data", post(alter_data_handler))
        .route("/data", delete(delete_data_handler))
        .route("/users/activate", post(activate_user_handler))
        .route("/users", post(alter_user_handler))
        .route("/users", delete(delete_user_handler))
        .route("/admin/access", post(admin_access_handler))
        .with_state(server_state.into());
    
    info!("Server started at: {}", server_config.get_server_ip());
    // start the server
    axum_server::bind_rustls(server_config.get_server_ip(), tls_config)
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
    info!("Starting server...");
    let server_path = Path::new("./data/temp/");
    let _ = start_server(server_path, &server_config).await;
}
