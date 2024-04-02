use crate::auth::jwt_claim::JwtClaims;
use crate::error::ApiError;
use well_i_known_core::api::data::*;
use axum::Json;
use tracing::*;

#[instrument]
pub async fn get_data_handler(
    claims: JwtClaims,
    Json(payload): Json<GetDataQuery>
) -> Result<String, ApiError> {
    // if user = root => skip auth
    // if user = admin => check if user has access
    // if user = app => username == app name
    Ok(format!(
        "Enter get_data_handler",
    ))
}

#[instrument]
pub async fn alter_data_handler(
    claims: JwtClaims,
    Json(payload): Json<UpdateDataParam>
) -> Result<String, ApiError> {
    Ok(format!(
        "Enter alter_data_handler",
    ))
}

#[instrument]
pub async fn delete_data_handler(
    claims: JwtClaims,
    Json(payload): Json<DeleteDataParam>
) -> Result<String, ApiError> {
    Ok(format!(
        "Enter delete_data_handler",
    ))
}
