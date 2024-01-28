mod error;
mod auth;

use axum::{routing::{get, post}, Router};
use error::AuthError;
use auth::{jwt_controller::authorize, jwt_claim::JwtClaims};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize));

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn protected(claims: JwtClaims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

