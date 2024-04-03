use crate::db::db_connection::DbConnection;
use crate::db::db_executor::db_result_handler;
use crate::dao;
use crate::error::ApiError;

use tracing::*;

pub async fn check_access_right_exists(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<bool, ApiError>{
    let exists = db_result_handler(
        dao::access_right::check_access_right_exists(db_conn, username, app_name).await,
        "check_access_right_exists")?;
    Ok(exists)
}

pub async fn add_access_right(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<(), ApiError> {
    // check if the app exists
    let app_exists = db_result_handler(
        dao::user::check_user_exists(db_conn, app_name).await,
        "check app exists")?;

    if !app_exists {
        warn!("Fail to add_access_right. App '{}' does not exist.", app_name);
        return Err(ApiError::InvalidArgument { 
            argument: "app".to_string(),
            message: "Given app does not exist.".to_string(),
        });
    }

    db_result_handler(
        dao::access_right::add_access_right(db_conn, username, app_name).await, 
        "add_access_right")?;

    Ok(())
}

pub async fn delete_access_right(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<(), ApiError> {
    db_result_handler(
        dao::access_right::delete_access_right(db_conn, username, app_name).await, 
        "delete_access_right")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::db_test_util::*;
    use crate::dao;

    async fn create_access_right_test_db(test_case_name: &str) -> DbConnection{
        // create the connection
        let db_conn = create_test_db(test_case_name).await;
        // insert base data
        dao::user::create_user(&db_conn, "u_root", "root", "password").await.unwrap();
        dao::user::create_user(&db_conn, "u_admin", "admin", "password").await.unwrap();
        dao::user::create_user(&db_conn, "u_app", "app", "password").await.unwrap();
        db_conn
    }

    #[tokio::test]
    async fn test_check_access_right_exists(){
        let db_conn = create_access_right_test_db("check_access_right_exists").await;
        let has_access = check_access_right_exists(&db_conn, "u_admin", "test_app").await.unwrap();
        assert_eq!(has_access, false);
    }
}