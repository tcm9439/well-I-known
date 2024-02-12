mod error;
mod auth;

use tracing::Level;
use axum::{routing::{get, post}, Router};
use error::AuthError;
use auth::{jwt_controller::authorize, jwt_claim::JwtClaims};

#[tokio::main]
async fn main() {
    // register tracing file appender
    let file_appender = tracing_appender::rolling::hourly("./log", "wellik-server.log");
    // _guard is needed to be in / returned to main()
    let (non_blocking_trace_file_appender, _guard) = tracing_appender::non_blocking(file_appender);

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(non_blocking_trace_file_appender)
        .with_ansi(false) // turn off ansi colors
        .init();

    // register the routes
    let app = Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize));

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // start the server
    axum::serve(listener, app).await.unwrap();
}

async fn protected(claims: JwtClaims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

