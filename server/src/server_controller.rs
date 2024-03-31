use crate::{auth::jwt_claim::JwtClaims, error::AuthError};
use tracing::info;

pub async fn get_data_handler(
    claims: JwtClaims
) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Enter get_data_handler",
    ))
}

pub async fn alter_data_handler(
    claims: JwtClaims
) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Enter alter_data_handler",
    ))
}

pub async fn delete_data_handler(
    claims: JwtClaims
) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Enter delete_data_handler",
    ))
}

pub async fn alter_user_handler(
    claims: JwtClaims
) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Enter alter_user_handler",
    ))
}

pub async fn delete_user_handler(
    claims: JwtClaims
) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Enter delete_user_handler",
    ))
}

pub async fn activate_user_handler(
    claims: JwtClaims
) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Enter activate_user_handler",
    ))
}

pub async fn admin_access_handler(
    claims: JwtClaims
) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Enter admin_access_handler",
    ))
}