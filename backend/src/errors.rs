use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Custom error type for the application
#[derive(Debug)]
pub enum AppError {
    SqlxError(sqlx::Error),
    NotFound,
    ValidationError(String),
    NoFieldsToUpdate,
    Unauthorized(String),
    InternalServerError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::SqlxError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::SqlxError(error) => {
                tracing::error!("Database error: {:?}", error);
                match error {
                    sqlx::Error::RowNotFound => {
                        (StatusCode::NOT_FOUND, "Resource not found".to_string())
                    }
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    ),
                }
            }
            AppError::NotFound => {
                tracing::warn!("Resource not found");
                (StatusCode::NOT_FOUND, "Resource not found".to_string())
            }
            AppError::ValidationError(msg) => {
                tracing::warn!("Validation error: {}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::NoFieldsToUpdate => {
                tracing::warn!("No fields to update provided in request");
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "No fields to update".to_string(),
                )
            }
            AppError::Unauthorized(msg) => {
                tracing::warn!("Unauthorized: {}", msg);
                (StatusCode::UNAUTHORIZED, msg)
            }
            AppError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
