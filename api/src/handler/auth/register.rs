use std::sync::Arc;
use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use axum::extract::ConnectInfo;
use std::net::SocketAddr;
use time::Duration as CookieDuration;

use crate::{
    AppState, errors::AppError, models::users::{enums::{LogAction, LogEntity}, log::LogEntry}, utils::{
        logs::insert_log, password::hash_password, session::session::create_session,
        session::token::{create_access_token, REFRESH_TOKEN_TTL_DAYS}, validators::{
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
struct RegisterUserRow {
    user_id: Uuid,
    login: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub login: String,
    /// Absent si la création de session a échoué : l'utilisateur devra
    /// se logger manuellement via /login.
    pub access_token: Option<String>,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: CookieJar,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    validate_email(&body.email)?;
    validate_login(&body.login)?;
    validate_password(&body.password)?;

    let password_hash = hash_password(&body.password)?;

    let mut tx = state.db.begin().await?;

    let user = sqlx::query_as!(
        RegisterUserRow,
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

    insert_log(
        &mut *tx,
        LogEntry::new(LogAction::Create, LogEntity::User)
            .user_id(user.user_id)
            .entity_id(user.user_id)
            .ip_address(addr.ip())
            .new_values(serde_json::json!({
                "email": body.email,
                "login": body.login,
            })),
    )
    .await?;

    tx.commit().await?;


    let session_result = create_session(&state.db, user.user_id, Some(addr.ip()), None).await;

    let (access_token, jar) = match session_result {
        Ok(refresh_token) => {
            match create_access_token(user.user_id, "user") {
                Ok(token) => {
                    let cookie = Cookie::build(("refresh_token", refresh_token))
                        .http_only(true)
                        .secure(true)
                        .same_site(SameSite::Strict)
                        .path("/auth")
                        .max_age(CookieDuration::days(REFRESH_TOKEN_TTL_DAYS))
                        .build();
                    (Some(token), jar.add(cookie))
                }
                Err(err) => {
                    eprintln!(
                        "register: failed to create access token for user {}: {err:?}",
                        user.user_id
                    );
                    (None, jar)
                }
            }
        }
        Err(err) => {
            eprintln!(
                "register: failed to create session for user {}: {err:?}",
                user.user_id
            );
            (None, jar)
        }
    };

    let response = RegisterResponse {
        user_id: user.user_id,
        login: user.login,
        access_token,
    };

    Ok((StatusCode::CREATED, jar, Json(response)))
}
