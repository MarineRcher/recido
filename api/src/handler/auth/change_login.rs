use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    AppState,
    errors::AppError,
    models::users::{enums::{LogAction, LogEntity}, log::LogEntry},
    utils::{logs::insert_log, validators::login::validate_login},
    utils::session::auth::AuthUser, // adapte le chemin selon où tu l'as mis
};

#[derive(Deserialize)]
pub struct ChangeLoginRequest {
    pub new_login: String,
}

#[derive(Serialize)]
pub struct ChangeLoginResponse {
    pub login: String,
}

pub async fn change_login(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(body): Json<ChangeLoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    validate_login(&body.new_login)?;

    let mut tx = state.db.begin().await?;


    let updated = sqlx::query_scalar!(
        r#"
        UPDATE users.users
        SET login = $1
        WHERE user_id = $2
        RETURNING login
        "#,
        body.new_login,
        auth.user_id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.is_unique_violation() {
                return AppError::Conflict("This login is already taken".to_string());
            }
        }
        AppError::from(e)
    })?;

    let new_login = updated.ok_or_else(|| {
        AppError::NotFound("User not found".to_string())
    })?;


    tx.commit().await?;

    Ok((StatusCode::OK, Json(ChangeLoginResponse { login: new_login })))
}
