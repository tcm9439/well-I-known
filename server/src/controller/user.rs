use crate::auth::jwt_claim::JwtClaims;
use crate::error::ApiError;
use well_i_known_core::api::user::*;
use axum::Json;
use tracing::{info, instrument};

#[instrument(skip(claims, payload))]
pub async fn alter_user_handler(
    claims: JwtClaims,
    Json(payload): Json<UpdateUserParam>
) -> Result<String, ApiError> {
    // Ok(format!("Enter alter_user_handler"))
    Ok(format!(
        "Enter alter_user_handler {}",
        payload.username
    ))
}

pub async fn delete_user_handler(
    claims: JwtClaims
) -> Result<String, ApiError> {
    Ok(format!(
        "Enter delete_user_handler",
    ))
}

pub async fn validate_user_handler(
    claims: JwtClaims
) -> Result<Json<ValidateUserResponse>, ApiError> {
    let response = ValidateUserResponse {
        plaintext: "abc".to_string(),
        encrypted: "def".to_string(),
    };

    Ok(Json(response))
}
