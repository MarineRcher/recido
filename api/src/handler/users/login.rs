use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, errors::AppError, utils::{password::verify_password, validators::email::validate_email}};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub role: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
	validate_email(&body.email)?;

    struct UserAuth {
        user_id: Uuid,
        password_hash: String,
    }

    let user_auth = sqlx::query_as!(
        UserAuth,
        r#"
        SELECT
            u.user_id,
            u.password AS password_hash
        FROM users.users u
        WHERE u.email = $1
        "#,
        body.email
    )
    .fetch_one(&state.db) // Utilisation directe du pool, pas de transaction inutile ici
    .await?;


    let is_valid = verify_password(&body.password, &user_auth.password_hash)?;
	if !is_valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }
    let user_response = sqlx::query_as!(
        LoginResponse,
        r#"
        SELECT
            u.user_id,
            r.level AS "role!: String"
        FROM users.users u
        JOIN users.user_roles ur ON u.user_id = ur.user_id
        JOIN users.roles r ON ur.role_id = r.id_role
        WHERE u.user_id = $1
        "#,
        user_auth.user_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::OK, Json(user_response)))
}
