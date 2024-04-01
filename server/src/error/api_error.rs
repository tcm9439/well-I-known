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
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            ApiError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            ApiError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            ApiError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid JWT token"),
            ApiError::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}