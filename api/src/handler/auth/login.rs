use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use axum::extract::ConnectInfo;

use std::net::SocketAddr;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use time::Duration as CookieDuration;
use crate::{AppState, errors::AppError, utils::{password::verify_password, session::{session::create_session, token::{REFRESH_TOKEN_TTL_DAYS, create_access_token}}, validators::email::validate_email}};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub role: String,
    pub access_token: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: CookieJar,
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
    .fetch_one(&state.db)
    .await?;


    let is_valid = verify_password(&body.password, &user_auth.password_hash)?;
	if !is_valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }
    struct UserRole {
        role: String,
    }

    let user_role = sqlx::query_as!(
        UserRole,
        r#"
        SELECT r.level AS "role!: String"
        FROM users.user_roles ur
        JOIN users.roles r ON ur.role_id = r.id_role
        WHERE ur.user_id = $1
        "#,
        user_auth.user_id
    )
    .fetch_one(&state.db)
    .await?;

    let refresh_token = create_session(&state.db, user_auth.user_id, Some(addr.ip()), None).await?;
    let access_token = create_access_token(user_auth.user_id, &user_role.role)?;

    let cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/auth")
        .max_age(CookieDuration::days(REFRESH_TOKEN_TTL_DAYS))
        .build();

    let jar = jar.add(cookie);

    let response = LoginResponse {
        user_id: user_auth.user_id,
        role: user_role.role,
        access_token,
    };

    Ok((StatusCode::OK, jar, Json(response)))
}
