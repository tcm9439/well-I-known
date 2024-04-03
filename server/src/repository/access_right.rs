use crate::db::{self, db_connection::DbConnection};
use crate::error::ApiError;

use tracing::*;

pub async fn check_access_right_exists(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<bool, ApiError>{
    let exists = db::access_right::check_access_right_exists(db_conn, username, app_name).await;
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

pub async fn add_access_right(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<(), ApiError> {
    // check if the app exists
    let app_exists = db::user::check_user_exists(db_conn, app_name).await;

    let add = db::access_right::add_access_right(db_conn, username, app_name).await;
    match add {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Fail to add_access_right. Database error: {}", err);
            Err(ApiError::DatabaseError {
                message: err.to_string(),
            })
        },
    }
}

pub async fn delete_access_right(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<(), ApiError> {
    let delete = db::access_right::delete_access_right(db_conn, username, app_name).await;
    match delete {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("Fail to delete_access_right. Database error: {}", err);
            Err(ApiError::DatabaseError {
                message: err.to_string(),
            })
        },
    }
}