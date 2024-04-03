use well_i_known_core::modal::user::{self, SeverUserModal, UserRole};
use well_i_known_core::modal::util::id_validation::validate_id;
use crate::db::db_connection::DbConnection;
use crate::db::db_executor::db_result_handler;
use crate::dao;
use crate::auth::role_validation::*;
use crate::error::ApiError;

use std::path::PathBuf;
use tracing::*;

pub async fn check_user_exists(db_conn: &DbConnection, username: &str) -> Result<bool, ApiError>{
    let exists = db_result_handler(
        dao::user::check_user_exists(db_conn, username).await,
        "check_user_exists")?;
    Ok(exists)
}

pub async fn create_root_user(db_conn: &DbConnection, 
    creator: &str, creator_role: &UserRole,
    username: &str, role: &UserRole, password: &str) -> Result<(), ApiError> {
    debug!("Creating a root user.");

    throw_if_unauthorized(
        can_create_account(creator_role, role),
        creator, "create root user");

    // check if root already exists
    let root_exists = db_result_handler(
        dao::user::check_root_exists(db_conn).await,
        "check_root_exists")?;

    if root_exists {
        warn!("Try to create root user when root already exists.");
        return Err(ApiError::DuplicateRecord);
    }

    db_result_handler(
        dao::user::create_user(db_conn, username, &role.to_string(), password).await,
        "create_root_user")?;

    Ok(())
}

pub async fn create_user(db_conn: &DbConnection, 
    creator: &str, creator_role: &UserRole,
    username: &str, role: &UserRole, password: &str,
    public_key: &str, user_cert_path: &PathBuf) -> Result<(), ApiError> {
    
    if role == &UserRole::Root {
        return Err(ApiError::InvalidArgument { 
            argument: "role".to_string(), 
            message: "Wrong call to create a root user.".to_string() 
        });
    }

    // validate the creator
    throw_if_unauthorized(
        can_create_account(creator_role, role),
        creator, "create app/admin user");

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
    // store the public key in the pem file
    if let Err(error) = std::fs::write(&user_cert_path, public_key) {
        warn!("Fail to write public key to file. Error: {}", error);
        return Err(ApiError::ServerError);
    }

    // create the user
    db_result_handler(
        dao::user::create_user(db_conn, username, &role.to_string(), password).await,
        "create user")?;

    Ok(())
}

pub async fn update_user(db_conn: &DbConnection, username: &str, password: &str) -> Result<(), ApiError>{
    // check if user already exists
    if !check_user_exists(db_conn, username).await? {
        warn!("Try to update user '{}' which does not exist.", username);
        return Err(ApiError::RecordNotFound);
    }

    // update the user
    db_result_handler(
        dao::user::update_user(db_conn, username, password).await,
        "update user")?;

    Ok(())
}

pub async fn delete_user(db_conn: &DbConnection, username: &str, user_cert_path: &PathBuf) -> Result<(), ApiError>{
    // check if user already exists
    if !check_user_exists(db_conn, username).await? {
        warn!("Try to delete user '{}' which does not exist.", username);
        return Err(ApiError::RecordNotFound);
    }

    // get the user's role
    let user = get_user(db_conn, username, user_cert_path).await?;
    
    match user.role {
        user::UserRole::Root => {
            // root cannot be deleted
            warn!("Try to delete root user, which is not allowed.");
            return Err(ApiError::InvalidArgument { 
                argument: "username".to_string(), 
                message: "Root user cannot be deleted.".to_string() 
            });
        }

        user::UserRole::Admin => {
            // delete the admin access right records
            db_result_handler(
                dao::access_right::delete_all_access_of_user(db_conn, username).await,
                "delete_all_access_of_user")?;
            }
            
        user::UserRole::App => {
            // delete the app config records
            db_result_handler(
                dao::access_right::delete_all_access_of_app(db_conn, username).await,
                "delete_all_access_of_app")?;
        }
    }

    // delete the user
    db_result_handler(
        dao::user::delete_user(db_conn, username).await,
        "delete user")?;

    // delete the user's cert file
    if let Err(error) = std::fs::remove_file(&user_cert_path) {
        warn!("Fail to delete user's cert file. Error: {}", error);
    }

    Ok(())
}

pub async fn get_user(db_conn: &DbConnection, username: &str, user_cert_file: &PathBuf) -> Result<SeverUserModal, ApiError> {
    let user = db_result_handler(
        dao::user::get_user(db_conn, username).await,
        "get_user")?;


    let user = SeverUserModal::new(username, &user.role, &user_cert_file);
    match user {
        Ok(user) => return Ok(user),
        Err(error) => {
            warn!("Fail to parse user. Error: {}", error);
            return Err(ApiError::ServerError);
        },
    }
}

pub async fn is_valid_user_of_role(db_conn: &DbConnection, username: &str, role: &UserRole) -> Result<bool, ApiError> {
    db_result_handler(
        dao::user::check_user_with_role_exists(db_conn, username, &role.to_string()).await,
        "check_user_with_role_exists")
}
