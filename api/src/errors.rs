use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;


/// Application-wide error type.
///
/// Each variant carries a human-readable message and maps to a specific
/// HTTP status code when converted into a response
/// [`AppError::into_response`]).
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    BadRequest(String),
    InternalError(String),
}


impl IntoResponse for AppError {
    /// Converts the error into an HTTP response.
    ///
    /// The resulting body is a JSON object of the form
    /// `{ "error": "<message>" }`, with a status code corresponding to
    /// the matched [`AppError`] variant.
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
    /// Converts a low-level [`sqlx::Error`] into an [`AppError`].
    ///
    /// This allows handlers to use `?` on `sqlx` calls and have the
    /// resulting error automatically turned into an appropriate HTTP
    /// response, without leaking database internals to the client.
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                let msg = db_err.message();
                if msg.contains("users_email_key") {
                    AppError::Conflict("Email already taken".to_string())
                } else if msg.contains("users_login_key") {
                    AppError::Conflict("Login already taken".to_string())
                } else {
                    AppError::Conflict("Unique constraint violation".to_string())
                }
            }
            _ => AppError::InternalError("Database error".to_string()),
        }
    }
}
