use crate::db::db_connection::DbConnection;
use crate::db::db_executor::db_result_handler;
use crate::dao::access_right::AccessRightTable;
use crate::dao::user::UserTable;
use crate::error::ApiError;

use tracing::*;

pub struct AccessRightRepository {}
impl AccessRightRepository {
    pub async fn check_access_right_exists(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<bool, ApiError>{
        let exists = db_result_handler(
            AccessRightTable::check_access_right_exists(db_conn, username, app_name).await,
            "check_access_right_exists")?;
        Ok(exists)
    }

    pub async fn add_access_right(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<(), ApiError> {
        // check if the app exists
        let app_exists = db_result_handler(
            UserTable::check_user_exists(db_conn, app_name).await,
            "check app exists")?;

        if !app_exists {
            warn!("Fail to add_access_right. App '{}' does not exist.", app_name);
            return Err(ApiError::InvalidArgument { 
                argument: "app".to_string(),
                message: "Given app does not exist.".to_string(),
            });
        }

        db_result_handler(
            AccessRightTable::add_access_right(db_conn, username, app_name).await, 
            "add_access_right")?;

        Ok(())
    }

    pub async fn delete_access_right(db_conn: &DbConnection, username: &str, app_name: &str) -> Result<(), ApiError> {
        db_result_handler(
            AccessRightTable::delete_access_right(db_conn, username, app_name).await, 
            "delete_access_right")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use well_i_known_core::modal::user::UserRole;
    use crate::db::db_test_util::*;
    use crate::dao::user::UserTable;

    async fn create_access_right_test_db(test_case_name: &str) -> DbConnection{
        // create the connection
        let db_conn = create_test_db(test_case_name).await;
        // insert base data
        UserTable::create_user(&db_conn, "u_root", &UserRole::Root, "password").await.unwrap();
        UserTable::create_user(&db_conn, "u_admin", &UserRole::Admin, "password").await.unwrap();
        UserTable::create_user(&db_conn, "u_app", &UserRole::App, "password").await.unwrap();
        db_conn
    }

    #[tokio::test]
    async fn test_check_access_right_exists(){
        let db_conn = create_access_right_test_db("check_access_right_exists").await;
        let has_access = AccessRightRepository::check_access_right_exists(&db_conn, "u_admin", "test_app").await.unwrap();
        assert_eq!(has_access, false);
    }
}