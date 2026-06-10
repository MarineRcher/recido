use std::sync::Arc;
use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::{AppState, errors::AppError, utils::password::hash_password};
use uuid::Uuid;

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
    let password_hash = hash_password(&body.password)?;

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
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}
