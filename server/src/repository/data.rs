use well_i_known_core::crypto::cryptography::Encryption;
use crate::db::db_connection::DbConnection;
use crate::db::db_executor::db_result_handler;
use crate::repository;
use crate::dao;
use crate::error::ApiError;
use crate::WIKServerEnvironmentConfig;

use tracing::*;

/// Get the encrypted data for the given 'app, key, user' pair.
pub async fn get_config_data(db_conn: &DbConnection, app_name: &str, username: &str, config_key: &str) -> Result<String, ApiError> {
    // get the record
    let config_data = db_result_handler(
        dao::config_data::get_data_value(db_conn, app_name, username, config_key).await,
        "check app exists")?;

    // check if the record exists
    if config_data.is_none() {
        warn!("Fail to get_config_data. Did not store the '{}'-'{}' for {}.", app_name, config_key, username);
        return Err(ApiError::RecordNotFound);
    }

    Ok(config_data.unwrap())
}

/// Check if the record exists for the given 'app, key, user' pair.
pub async fn check_data_exists(db_conn: &DbConnection, app_name: &str, username: &str, config_key: &str) -> Result<bool, ApiError>{
    let exists = db_result_handler(
        dao::config_data::check_data_exists(db_conn, app_name, username, config_key).await,
        "check_data_exists")?;
    Ok(exists)
}

/// Add the data for the give 'app, key' pair.
/// The data is encrypted for the app & the admin has access to the data.
/// params:
/// - db_conn: the database connection
/// - server_config: the server configuration
/// - app_name: the app name
/// - config_key: the config key
/// - config_value: the config value in plaintext
pub async fn alter_config_data(db_conn: &DbConnection, server_config: &WIKServerEnvironmentConfig, 
    app_name: &str, config_key: &str, config_value: &str) -> Result<(), ApiError> {

    let exists = db_result_handler(
        dao::config_data::check_data_exists_for_key(db_conn, app_name, config_key).await, "check_data_exists_for_key")?;

    if exists {
        info!("The '{}'-'{}' already exists.", app_name, config_key);
        // delete all existing data records for this app-key pair
        delete_config_data(db_conn, app_name, config_key).await?;
    }

    let users_with_access_right = repository::user::get_users_with_access_to(
        db_conn, server_config, app_name).await?;

    // for each user, encrypt the data & create a new config data record
    for user in users_with_access_right {
        let encrypted_value = user.public_key.encrypt_string(config_value);
        if let Err(error) = encrypted_value {
            warn!("Fail to encrypt data for {}. Error: {}", user.username, error);
            return Err(ApiError::ServerError);
        }

        db_result_handler(
            dao::config_data::set_data_value(db_conn, app_name, &user.username, config_key, &encrypted_value.unwrap()).await,
            "add_config_data")?;
    }

    Ok(())
}

/// Remove the data for the give 'app, key' pair.
/// All the encrypted data for the app & the admin has access to the data will be deleted.
pub async fn delete_config_data(db_conn: &DbConnection, app_name: &str, config_key: &str) -> Result<(), ApiError> {
    db_result_handler(
        dao::config_data::delete_all_app_key_data(db_conn, app_name, config_key).await, 
        "delete_config_data")?;

    Ok(())
}