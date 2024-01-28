use super::jwt_claim::JwtClaims;

use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::AuthError;

/// Response sent to the user after authorization
#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

/// Payload sent by the user to authorize
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
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
pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // TODO replace the dummy check 
    // Here you can check the user credentials from a database
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    // Create the authorization token
    let token = JwtClaims::new("foo").gen_token()?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
