use std::sync::Arc;

use crate::auth::jwt_claim::JwtClaims;
use crate::server_state::ServerState;
use crate::auth::role_validation::*;
use crate::error::ApiError;
use crate::repository;
use well_i_known_core::api::data::*;

use axum::extract::State;
use axum::Json;
use tracing::*;

/// Verify if the requester has access to that app's config.
pub async fn basic_auth_for_data_api(claims: &JwtClaims, app_name: &str) -> Result<(), ApiError>{
    throw_if_unauthorized(
        is_admin_or_self(&claims.role, &claims.sub, app_name),
        &claims.sub,
        "access data",
    )?;
    Ok(())
}

#[instrument(skip(server_state))]
pub async fn get_data_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<GetDataQuery>,
) -> Result<String, ApiError> {
    basic_auth_for_data_api(&claims, &payload.app).await?;
    let result = repository::data::get_config_data(&server_state.db_conn, &payload.app, &claims.sub, &payload.key).await?;
    Ok(result)
}

#[instrument(skip(server_state))]
pub async fn alter_data_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<UpdateDataParam>,
) -> Result<(), ApiError> {
    basic_auth_for_data_api(&claims, &payload.app).await?;
    repository::data::alter_config_data(&server_state.db_conn, &server_state.config, &payload.app, &payload.key, &payload.value).await?;
    Ok(())
}

#[instrument(skip(server_state))]
pub async fn delete_data_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<DeleteDataParam>,
) -> Result<(), ApiError> {
    basic_auth_for_data_api(&claims, &payload.app).await?;
    repository::data::delete_config_data(&server_state.db_conn, &payload.app, &payload.key).await?;
    Ok(())
}
