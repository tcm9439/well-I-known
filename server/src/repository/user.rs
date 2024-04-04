use well_i_known_core::modal::user::{self, ServerUserKeyModal, SeverUserModal, UserRole};
use well_i_known_core::modal::util::id_validation::validate_id;
use crate::config::server_config::WIKServerEnvironmentConfig;
use crate::db::db_connection::DbConnection;
use crate::db::db_executor::db_result_handler;
use crate::dao;
use crate::auth::role_validation::*;
use crate::error::ApiError;

use std::path::PathBuf;
use tracing::*;

/// Check if the given username is a valid user.
pub async fn check_user_exists(db_conn: &DbConnection, username: &str) -> Result<bool, ApiError>{
    let exists = db_result_handler(
        dao::user::check_user_exists(db_conn, username).await,
        "check_user_exists")?;
    Ok(exists)
}

/// Create a root user. Can only be called once.
pub async fn create_root_user(db_conn: &DbConnection, username: &str, password: &str) -> Result<(), ApiError> {
    debug!("Creating a root user.");

    // check if root already exists
    let root_exists = db_result_handler(
        dao::user::check_root_exists(db_conn).await,
        "check_root_exists")?;

    if root_exists {
        warn!("Try to create root user when root already exists.");
        return Err(ApiError::DuplicateRecord);
    }

    db_result_handler(
        dao::user::create_user(db_conn, username, &UserRole::Root, password).await,
        "create_root_user")?;

    Ok(())
}

/// Create a user with role 'app' or 'admin'.
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
        creator, "create app/admin user")?;

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
        dao::user::create_user(db_conn, username, &role, password).await,
        "create user")?;

    Ok(())
}

/// Update the user data for the user id by the username.
/// The updatable attributes are:
/// - password
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

/// Delete a user and his related records.
/// Root: Cannot be removed.
/// Admin: Remove access right.
/// App: Remove config data.
pub async fn delete_user(db_conn: &DbConnection, username: &str, user_cert_path: &PathBuf) -> Result<(), ApiError>{
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

/// Get one user by username.
/// Throe record not found error if the user does not exist.
pub async fn get_user(db_conn: &DbConnection, username: &str, user_cert_file: &PathBuf) -> Result<SeverUserModal, ApiError> {
    let user = db_result_handler(
        dao::user::get_user(db_conn, username).await,
        "get_user")?;

    if user.is_none() {
        warn!("User '{}' not found.", username);
        return Err(ApiError::RecordNotFound);
    }
    let user = user.unwrap();

    let user = SeverUserModal::new(username, &user.role, &user_cert_file);
    match user {
        Ok(user) => return Ok(user),
        Err(error) => {
            warn!("Fail to parse user. Error: {}", error);
            return Err(ApiError::ServerError);
        },
    }
}

/// Check if the given username is a exiting user with the given role.
pub async fn is_valid_user_of_role(db_conn: &DbConnection, username: &str, role: &UserRole) -> Result<bool, ApiError> {
    db_result_handler(
        dao::user::check_user_with_role_exists(db_conn, username, role).await,
        "check_user_with_role_exists")
}

/// Get all the user that can access to the given app's config, include
/// - Root
/// - The app
/// - All admin with access right
pub async fn get_users_with_access_to(db_conn: &DbConnection, server_config: &WIKServerEnvironmentConfig,
        app_name: &str) -> Result<Vec<ServerUserKeyModal>, ApiError> {

    let app_users_exists = db_result_handler(
        dao::user::check_user_with_role_exists(db_conn, app_name, &UserRole::App).await,
        "get_user")?;

    if !app_users_exists {
        warn!("App '{}' not found.", app_name);
        return Err(ApiError::RecordNotFound);
    }

    let admin_users = db_result_handler(
        dao::user::get_admin_with_access(db_conn, app_name).await,
        "get_admin_with_access")?;

    // map the users to ServerUserKeyModal
    let mut users: Vec<ServerUserKeyModal> = Vec::new();

    // map root
    let root = server_config.root_user.as_ref().unwrap();
    users.push(ServerUserKeyModal::new_from_key(&root.username, &root.public_key));

    // map app
    let app = ServerUserKeyModal::new(&app_name, &server_config.get_users_certs_path(app_name));

    if let Err(error) = app {
        warn!("Fail to create app user. Error: {}", error);
        return Err(ApiError::ServerError);
    }
    users.push(app.unwrap());

    // map admin
    for admin in admin_users {
        let admin = ServerUserKeyModal::new(&admin.username, &server_config.get_users_certs_path(&admin.username));
        if let Err(error) = admin {
            warn!("Fail to create admin user. Error: {}", error);
            return Err(ApiError::ServerError);
        }
        users.push(admin.unwrap());
    }

    Ok(users)
}
