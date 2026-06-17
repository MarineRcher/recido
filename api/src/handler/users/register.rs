use std::sync::Arc;
use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{
    AppState,
    errors::AppError,
    utils::{
        password::hash_password,
        validators::{
            email::validate_email,
            login::validate_login,
            password::validate_password,
        },
    },
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub login: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub login: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    validate_email(&body.email)?;
    validate_login(&body.login)?;
    validate_password(&body.password)?;

    let password_hash = hash_password(&body.password)?;

    let mut tx = state.db.begin().await?;

    let user = sqlx::query_as!(
        RegisterResponse,
        r#"
        INSERT INTO users.users (email, password, login)
        VALUES ($1, $2, $3)
        RETURNING user_id, login
        "#,
        body.email,
        password_hash,
        body.login,
    )
    .fetch_one(&mut *tx)
    .await?;

    let role = sqlx::query_scalar!(
        r#"
        INSERT INTO users.user_roles (user_id, role_id)
        SELECT $1, id_role FROM users.roles WHERE level = 'user'
        RETURNING role_id
        "#,
        user.user_id,
    )
    .fetch_optional(&mut *tx)
    .await?;

    if role.is_none() {
        return Err(AppError::InternalError("Default role 'user' not found".to_string()));
    }

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(user)))
}
