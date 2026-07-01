use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    AppState,
    errors::AppError,
    utils::session::auth::AuthUser,
    utils::{
        password::{hash_password, verify_password},
        validators::password::validate_password,
    },
};

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct ChangePasswordResponse {
    pub message: String,
}

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    validate_password(&body.new_password)?;

    let mut tx = state.db.begin().await?;

    let current_hash = sqlx::query_scalar!(
        r#"SELECT password FROM users.users WHERE user_id = $1"#,
        auth.user_id
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let is_valid = verify_password(&body.current_password, &current_hash)?;
    if !is_valid {
        return Err(AppError::Unauthorized("Current password is incorrect".to_string()));
    }

    let new_hash = hash_password(&body.new_password)?;

    sqlx::query!(
        r#"UPDATE users.users SET password = $1 WHERE user_id = $2"#,
        new_hash,
        auth.user_id,
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        UPDATE users.sessions
        SET revoked_at = now()
        WHERE user_id = $1 AND revoked_at IS NULL
        "#,
        auth.user_id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((
        StatusCode::OK,
        Json(ChangePasswordResponse {
            message: "Password updated, please log in again".to_string(),
        }),
    ))
}
