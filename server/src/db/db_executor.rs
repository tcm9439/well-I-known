use crate::error::ApiError;
use tracing::*;

/// Handle basic database errors.
/// Convert database errors to ApiError.
pub fn db_result_handler<T>(db_result: anyhow::Result<T>, operation_name: &str) -> Result<T, ApiError> {
    match db_result {
        Ok(result) => Ok(result),
        Err(err) => {
            warn!("Fail to {}. Database error: {}", operation_name, err);
            Err(ApiError::DatabaseError)
        },
    }
}
