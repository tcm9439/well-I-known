use std::sync::Arc;

use axum::{extract::State, Json, async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};

use super::jwt_claim::JwtClaims;
use crate::{db::user::User, error::AuthError, server_state::ServerState};

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

// axum extractor for decoding & verifying the JWT token
// See https://docs.rs/axum/0.7.4/axum/extract/index.html for what is an extractor
#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    Arc<ServerState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let state = Arc::from_ref(state);
        let token_data = decode::<JwtClaims>(bearer.token(), 
            &state.jwt_keys.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

/// Handler for the authorization user
#[instrument(skip(state, payload))]
pub async fn authorize_handler(
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<AuthPayload>
) -> Result<Json<AuthBody>, AuthError> {
    info!("Accept authorize user request.");
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        warn!("Missing credentials");
        return Err(AuthError::MissingCredentials);
    }

    match User::auth_user(&state.db_conn, &payload.username, &payload.password).await {
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
    let token = JwtClaims::new(&payload.username).gen_token(&state.jwt_keys)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
