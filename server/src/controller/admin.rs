use std::sync::Arc;

use crate::auth::jwt_claim::JwtClaims;
use crate::server_state::ServerState;
use crate::auth::role_validation::*;
use crate::error::ApiError;
use crate::repository::{user, access_right};
use well_i_known_core::api::admin::*;
use well_i_known_core::modal::user::UserRole;

use axum::extract::State;
use axum::Json;
use tracing::*;

/// Authorization for the requester calling the admin access right API.
async fn basic_auth_for_admin_api(server_state: &Arc<ServerState>, claims: &JwtClaims, payload: &AdminAccessParam) -> Result<(), ApiError> {
    // check if user is admin
    let authorized = is_admin(&claims.role);
    throw_if_unauthorized(authorized, &claims.sub, "create admin access right")?;

    // check if the target user is admin (& valid)
    let is_admin = user::is_valid_user_of_role(
        &server_state.db_conn, 
        &payload.admin,
        &UserRole::Admin,
    ).await?;

    if !is_admin {
        warn!("User {} is not admin / does not exists.", &payload.admin);
        return Err(ApiError::InvalidArgument { 
            argument: "admin".to_string(), 
            message: "Given user is not admin.".to_string() 
        });
    }

    Ok(())
}

#[instrument(skip(server_state))]
pub async fn create_admin_access_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<AdminAccessParam>,
) -> Result<(), ApiError> {
    basic_auth_for_admin_api(&server_state, &claims, &payload).await?;

    // check if access right exists
    let exists = access_right::check_access_right_exists(
        &server_state.db_conn, 
        &payload.admin,
        &payload.app, 
    ).await?;

    if exists {
        warn!("Access right already exists for user {} and app {} but try to create one.", &payload.admin, &payload.app);
        return Err(ApiError::DuplicateRecord);
    } 

    access_right::add_access_right(
        &server_state.db_conn, 
        &payload.admin,
        &payload.app, 
    ).await?;

    Ok(())
}

#[instrument(skip(server_state))]
pub async fn delete_admin_access_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<AdminAccessParam>,
) -> Result<(), ApiError> {
    basic_auth_for_admin_api(&server_state, &claims, &payload).await?;
    
    // check if access right exists
    let exists = access_right::check_access_right_exists(
        &server_state.db_conn, 
        &payload.admin,
        &payload.app, 
    ).await?;

    if !exists {
        warn!("Access right does not exist for user {} and app {} but try to delete one.", &payload.admin, &payload.app);
        return Err(ApiError::RecordNotFound);
    }

    access_right::delete_access_right(
        &server_state.db_conn, 
        &payload.admin,
        &payload.app,
    ).await?;

    Ok(())
}
