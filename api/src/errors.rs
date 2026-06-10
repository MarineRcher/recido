use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg)      => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg)      => (StatusCode::CONFLICT, msg),
            AppError::Unauthorized(msg)  => (StatusCode::UNAUTHORIZED, msg),
            AppError::BadRequest(msg)    => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Email or login already taken".to_string())
            }
            _ => AppError::InternalError("Database error".to_string()),
        }
    }
}
