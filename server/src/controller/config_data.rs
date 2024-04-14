use std::sync::Arc;

use crate::auth::jwt_claim::JwtClaims;
use crate::server_state::ServerState;
use crate::auth::role_validation::RoleValidationUtil;
use crate::error::ApiError;
use crate::repository::config_data::ConfigDataRepository;
use well_i_known_core::api::data::*;

use axum::extract::State;
use axum::Json;
use tracing::*;

/// Verify if the requester has access to that app's config.
pub async fn basic_auth_for_data_api(claims: &JwtClaims, app_name: &str) -> Result<(), ApiError>{
    RoleValidationUtil::throw_if_unauthorized(
        RoleValidationUtil::is_admin_or_self(&claims.role, &claims.sub, app_name),
        &claims.sub,
        "access data",
    )?;
    Ok(())
}

#[instrument(skip(server_state))] // tracing of function start and end
pub async fn get_data_handler(
    // provided by axum extractors jwt::controller::JwtClaims
    claims: JwtClaims,
    // provided by axum with_state()
    // Arc is used to share the state between threads
    State(server_state): State<Arc<ServerState>>,
    // provided by axum extractors, which converts the request body to a json object of the specified struct type
    Json(payload): Json<GetDataQuery>
) -> Result<String, ApiError> { // the return is converted to a Response by axum
    basic_auth_for_data_api(&claims, &payload.app).await?;
    let result = ConfigDataRepository::get_config_data(&server_state.db_conn, &payload.app, &claims.sub, &payload.key).await?;
    Ok(result)
}

#[instrument(skip(server_state))]
pub async fn alter_data_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<UpdateDataParam>,
) -> Result<(), ApiError> {
    basic_auth_for_data_api(&claims, &payload.app).await?;
    ConfigDataRepository::alter_config_data(&server_state.db_conn, &server_state.config, &payload.app, &payload.key, &payload.value).await?;
    Ok(())
}

#[instrument(skip(server_state))]
pub async fn delete_data_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<DeleteDataParam>,
) -> Result<(), ApiError> {
    basic_auth_for_data_api(&claims, &payload.app).await?;
    ConfigDataRepository::delete_config_data(&server_state.db_conn, &payload.app, &payload.key).await?;
    Ok(())
}
