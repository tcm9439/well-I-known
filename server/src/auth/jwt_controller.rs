use crate::auth::jwt_claim::JwtClaims;
use crate::repository::user::UserRepository;
use crate::{error::ApiError, server_state::ServerState};

use std::sync::Arc;
use std::fmt::Debug;
use axum::{extract::State, Json, async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use tracing::*;

/// Response sent to the user after authorization
#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

/// Payload sent by the user to authorize
#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

impl Debug for AuthPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // don't print the password
        write!(f, "AuthPayload {{ username: {} }}", self.username)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

/// axum extractor for decoding & verifying the JWT token
/// See https://docs.rs/axum/0.7.4/axum/extract/index.html for what is an extractor
/// This extractor try to extract the JWT token from the Authorization header 
/// and pass it to the API controllers.
/// If failed, return an ApiError
#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
// FromRequestParts => will not consume the request (can carry on to the other extractors)
where
    Arc<ServerState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::InvalidToken)?;
        // Decode the user data
        let state = Arc::from_ref(state);
        let token_data = decode::<JwtClaims>(
            bearer.token(), 
            &state.jwt_keys.decoding, &Validation::default())
            .map_err(|_| ApiError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

/// Handler for the authorization user
/// i.e. the API controller for login
/// Returns the JWT token if the user is authorized
#[instrument(skip(state))]
pub async fn authorize_handler(
    State(state): State<Arc<ServerState>>,
    Json(payload): Json<AuthPayload>
) -> Result<Json<AuthBody>, ApiError> {
    info!("Accept authorize user request.");
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        warn!("Missing credentials");
        return Err(ApiError::MissingCredentials);
    }

    let user_role = UserRepository::auth_user(&state.db_conn, &payload.username, &payload.password).await?;
    info!("User authorized");

    // Create the authorization token
    let token = JwtClaims::new(&payload.username, &user_role.to_string()).gen_token(&state.jwt_keys)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
