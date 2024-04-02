use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    ServerError,
    Unauthorized { message: String },
    DatabaseError { message: String },
    RecordNotFound,     // Try to update / delete a record that does not exist
    DuplicateRecord,    // Try to create a record with a duplicate primary key
    InvalidArgument { argument: String, message: String },    // Invalid argument provided
}

/// Convert the ApiError into a HTTP response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string()),
            ApiError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials".to_string()),
            ApiError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error".to_string()),
            ApiError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid JWT token".to_string()),
            ApiError::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Server error".to_string()),
            ApiError::Unauthorized{ message } => {
                let error_message = format!("Unauthorized: {}", message);
                (StatusCode::UNAUTHORIZED, error_message)
            },
            ApiError::DatabaseError{ message } => {
                let error_message = format!("Database error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, error_message)
            },
            ApiError::RecordNotFound => (StatusCode::NOT_FOUND, "Record not found".to_string()),
            ApiError::DuplicateRecord => (StatusCode::BAD_REQUEST, "Duplicate primary key".to_string()),
            ApiError::InvalidArgument{ argument, message } => {
                let error_message = format!("Invalid argument: '{}'. {}", argument, message);
                (StatusCode::BAD_REQUEST, error_message)
            },
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
