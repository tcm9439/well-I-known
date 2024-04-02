use crate::auth::jwt_claim::JwtClaims;
use crate::error::ApiError;
use well_i_known_core::api::admin::*;
use axum::Json;
use tracing::*;

#[instrument]
pub async fn admin_access_handler(
    claims: JwtClaims,
    Json(payload): Json<AdminAccessParam>
) -> Result<String, ApiError> {
    // check if given user is admin

    // valid param: operator is delete / create
    
    // check if access right exists

    // operation: delete

    // operation: create
    // check if app exists
    
    Ok(format!(
        "Enter admin_access_handler",
    ))
}
