use super::jwt_claim::JwtClaims;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};

use crate::{db::{db_connection::DbConnection, user::User}, error::AuthError};

/// Response sent to the user after authorization
#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

/// Payload sent by the user to authorize
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    username: String,
    password: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

/// Handler for the authorization user
#[instrument(skip(db_conn, payload))]
pub async fn authorize(
    State(db_conn): State<DbConnection>,
    Json(payload): Json<AuthPayload>
) -> Result<Json<AuthBody>, AuthError> {
    info!("Accept authorize user request.");
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        warn!("Missing credentials");
        return Err(AuthError::MissingCredentials);
    }

    match User::auth_user(&db_conn, &payload.username, &payload.password).await {
        Ok(false) => {
            warn!("Wrong credentials");
            return Err(AuthError::WrongCredentials);
        },
        // if the error is a AuthError, return it
        Err(e) => {
            warn!("Failed to authenticate user: {:?}", e);
            return Err(AuthError::ServerError);
        },
        _ => {}
    }

    info!("User authorized");
    // Create the authorization token
    let token = JwtClaims::new("foo").gen_token()?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
