use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::Serialize;

use crate::errors::AppError;
use crate::utils::session::auth::AuthUser;

#[derive(Serialize)]
pub struct MeResponse {
    pub user_id: uuid::Uuid,
    pub role: String,
}

pub async fn me(auth: AuthUser) -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::OK, Json(MeResponse {
        user_id: auth.user_id,
        role: auth.role,
    })))
}
