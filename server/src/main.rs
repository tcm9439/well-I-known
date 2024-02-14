mod error;
mod auth;
mod config;
mod db;

use axum::{routing::{get, post}, Router};

use auth::{jwt_controller::authorize, jwt_claim::JwtClaims};
use error::AuthError;
use config::server_config::*;
use db::{db_connection::DbConnection, user::User};
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

/// Init tracing by the loaded logging config.
fn init_tracing(server_config: &WIKServerConfig) -> WorkerGuard {
    // register tracing file appender
    // _guard is needed to be in / returned to main()
    let (non_blocking_trace_file_appender, guard) = tracing_appender::non_blocking(server_config.logging.get_logging_file_appender());

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(server_config.logging.get_logging_level())
        .with_writer(non_blocking_trace_file_appender)
        .with_ansi(false) // turn off ansi colors
        .init();

    guard
}

/// Start the server with the loaded server config.
async fn start_server(server_config: &WIKServerConfig) {
    let tls_config = server_config.tls.get_rtlus_config().await;
    let db_conn = DbConnection::new(&server_config.db_path).await.unwrap();

    // register the routes
    let app = Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize))
        .with_state(db_conn.clone());

    User::create_user(&db_conn, "mt", "root", "password").await;

    // start the server
    axum_server::bind_rustls(server_config.get_server_ip(), tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    // load the server config
    let server_config = WIKServerConfig::new("./resources/test/wik-server-config.json");

    let _guard = init_tracing(&server_config);
    info!("Starting server...");
    start_server(&server_config).await;
}

async fn protected(
    claims: JwtClaims
) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

