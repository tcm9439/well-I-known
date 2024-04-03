use crate::auth::jwt_claim::JwtClaims;
use crate::error::ApiError;
use crate::repository;
use crate::server_state::ServerState;
use well_i_known_core::api::user::*;
use well_i_known_core::modal::user::UserRole;

use axum::Json;
use axum::extract::State;
use tracing::*;
use std::{str::FromStr, sync::Arc};

#[instrument(skip(server_state))]
pub async fn alter_user_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<UpdateUserParam>
) -> Result<(), ApiError> {
    // check if user already exists
    let exists = repository::user::check_user_exists(&server_state.db_conn, &payload.username).await?;

    if exists {
        // Case: update user
        info!("User {} exists, updating user.", &payload.username);

        // valid no extra params provided
        if !payload.role.is_none() {
            warn!("User role cannot be updated.");
            return Err(ApiError::InvalidArgument {
                argument: "role".to_string(),
                message: "Role cannot be updated after created.".to_string(),
            });
        }

        if !payload.public_key.is_none() {
            warn!("User public key cannot be updated.");
            return Err(ApiError::InvalidArgument {
                argument: "public_key".to_string(),
                // TODO see if we can update public key (by decrypting the old config with root key)
                message: "Public key cannot be updated after created.".to_string(),
            });
        }

        repository::user::update_user(
            &server_state.db_conn,
            &payload.username,
            &payload.password
        ).await?;
    } else {
        // Case: create user 
        info!("User {} does not exist, creating user.", &payload.username);

        // Validate params exists
        if payload.role.is_none() {
            warn!("User role is required when creating user.");
            return Err(ApiError::InvalidArgument {
                argument: "role".to_string(),
                message: "Role is required".to_string(),
            });
        }

        // Validate role enum
        let role = UserRole::from_str(payload.role.as_ref().unwrap());
        if let Err(_e) = role {
            warn!("Invalid role '{}' provided.", payload.role.as_ref().unwrap());
            return Err(ApiError::InvalidArgument {
                argument: "role".to_string(),
                message: "Invalid role".to_string(),
            });
        }

        if role.as_ref().unwrap() == &UserRole::Root {
            info!("Creating root user...");
            repository::user::create_root_user(
                &server_state.db_conn,
                &claims.get_role(),
                &payload.username,
                &role.unwrap(),
                &payload.password
            ).await?;
        } else {
            if payload.public_key.is_none() {
                warn!("User's public key is required when creating non-root user.");
                return Err(ApiError::InvalidArgument {
                    argument: "public_key".to_string(),
                    message: "Public key is required".to_string(),
                });
            }

            let user_cert_dir = server_state.config.get_users_certs_dir_path();
            repository::user::create_user(
                &server_state.db_conn,
                &claims.get_role(),
                &payload.username,
                &role.unwrap(),
                &payload.password,
                &payload.public_key.unwrap(),
                &user_cert_dir
            ).await?;
        }
    }

    Ok(())
}

#[instrument(skip(server_state))]
pub async fn delete_user_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<DeleteUserParam>
) -> Result<(), ApiError> {
    repository::user::delete_user(
        &server_state.db_conn,
        &payload.username,
        &server_state.config.get_users_certs_path(&payload.username)
    ).await?;

    Ok(())
}

#[instrument(skip(server_state))]
pub async fn validate_user_handler(
    claims: JwtClaims,
    State(server_state): State<Arc<ServerState>>,
    Json(payload): Json<ValidateUserParam>
) -> Result<Json<ValidateUserResponse>, ApiError> {
    // check if user exists
    let exists = repository::user::check_user_exists(&server_state.db_conn, &payload.username).await?;

    if !exists {
        warn!("User {} does not exist.", &payload.username);
        return Err(ApiError::RecordNotFound);
    }

    let user = repository::user::get_user(
        &server_state.db_conn,
        &payload.username,
        &server_state.config.get_users_certs_path(&payload.username)
    ).await?;

    let key = user.get_public_key();
    if let Err(error) = key {
        warn!("Fail to get user's public key. Error: {}", error);
        return Err(ApiError::ServerError);
    }
    let (plaintext, encrypted) = key.unwrap().generate_validate_string();

    // pack the response
    let response = ValidateUserResponse {
        plaintext,
        encrypted,
    };

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_alter_user_handler() {
        // TODO
    }
}