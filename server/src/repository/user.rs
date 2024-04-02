use well_i_known_core::modal::user::UserRole;
use well_i_known_core::modal::util::id_validation::validate_id;
use crate::db::{self, db_connection::DbConnection};
use crate::error::ApiError;

use std::path::PathBuf;
use tracing::*;

pub async fn check_user_exists(db_conn: &DbConnection, username: &str) -> Result<bool, ApiError>{
    let exists = db::user::check_user_exists(db_conn, username).await;
    match exists {
        Ok(exists) => Ok(exists),
        Err(err) => {
            warn!("Fail to check_user_exists. Database error: {}", err);
            Err(ApiError::DatabaseError {
                message: err.to_string(),
            })
        },
    }
}

fn valid_to_create_role_by_role(creator: &UserRole, target_role: &UserRole) -> bool {
    match creator {
        UserRole::Root => true,
        UserRole::Admin => {
            match target_role {
                UserRole::Root => false,
                UserRole::Admin => false,
                UserRole::App => true,
            }
        },
        UserRole::App => false,
    }
}

pub async fn create_root_user(db_conn: &DbConnection, 
    creator: &UserRole,
    username: &str, role: &UserRole, password: &str) -> Result<(), ApiError> {
    debug!("Creating a root user.");
    if !valid_to_create_role_by_role(creator, role) {
        warn!("Unauthorized to create a user with role '{}'.", role);
        return Err(ApiError::Unauthorized { message: format!("Unauthorized to create a user with role '{}'.", role)});
    }

    // check if root already exists
    let root_exists = db::user::check_root_exists(db_conn).await;
    match root_exists {
        Ok(true) => {
            warn!("Try to create root user when root already exists.");
            return Err(ApiError::DuplicateRecord);
        },
        Err(err) => {
            warn!("Fail to check if root exists. Database error: {}", err);
            return Err(ApiError::DatabaseError {
                message: err.to_string(),
            });
        },
        _ => {},
    }

    if let Err(error) = db::user::create_user(db_conn, username, &role.to_string(), password).await {
        warn!("Fail to create root user. Database error: {}", error);
        return Err(ApiError::DatabaseError {
            message: error.to_string(),
        });
    }

    Ok(())
}

pub async fn create_user(db_conn: &DbConnection, 
    creator: &UserRole,
    username: &str, role: &UserRole, password: &str,
    public_key: &str, user_cert_dir: &PathBuf) -> Result<(), ApiError> {
    
    if role == &UserRole::Root {
        return Err(ApiError::InvalidArgument { 
            argument: "role".to_string(), 
            message: "Wrong call to create a root user.".to_string() 
        });
    }

    // validate the creator
    if !valid_to_create_role_by_role(creator, role) {
        return Err(ApiError::Unauthorized { message: format!("Unauthorized to create a user with role '{}'.", role)});
    }

    // check if username valid
    if let Err(error) = validate_id(username) {
        return Err(ApiError::InvalidArgument {
            argument: "username".to_string(),
            message: error,
        });
    }

    // check if user already exists
    if check_user_exists(db_conn, username).await? {
        warn!("Try to create user '{}' which is already exists.", username);
        return Err(ApiError::DuplicateRecord);
    }

    // compute the users cert path
    let public_key_path: PathBuf = user_cert_dir.join(format!("{}-cert.pem", username));
    // store the public key in the pem file
    if let Err(error) = std::fs::write(&public_key_path, public_key) {
        warn!("Fail to write public key to file. Error: {}", error);
        return Err(ApiError::ServerError);
    }

    // create the user
    if let Err(error) = db::user::create_user(db_conn, username, &role.to_string(), password).await {
        warn!("Fail to create user. Database error: {}", error);
        return Err(ApiError::DatabaseError {
            message: error.to_string(),
        });
    }

    Ok(())
}

pub async fn update_user(db_conn: &DbConnection, username: &str, password: &str) -> Result<(), ApiError>{
    // check if user already exists
    if !check_user_exists(db_conn, username).await? {
        warn!("Try to update user '{}' which does not exist.", username);
        return Err(ApiError::RecordNotFound);
    }

    // update the user
    if let Err(error) = db::user::update_user(db_conn, username, password).await {
        warn!("Fail to update user. Database error: {}", error);
        return Err(ApiError::DatabaseError {
            message: error.to_string(),
        });
    }

    Ok(())
}

pub async fn delete_user(db_conn: &DbConnection, username: &str, user_cert_dir: &PathBuf) -> Result<(), ApiError>{
    // check if user already exists
    if !check_user_exists(db_conn, username).await? {
        warn!("Try to delete user '{}' which does not exist.", username);
        return Err(ApiError::RecordNotFound);
    }

    // get the user's role
    
    // check if user is root (root cannot be deleted)

    // role == admin => delete the admin access right records

    // role == app => delete the app config records

    // delete the user
    if let Err(error) = db::user::delete_user(db_conn, username).await {
        warn!("Fail to delete user. Database error: {}", error);
        return Err(ApiError::DatabaseError {
            message: error.to_string(),
        });
    }

    // delete the user's cert file
    

    Ok(())
}