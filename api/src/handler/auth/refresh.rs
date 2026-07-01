use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::Serialize;
use std::sync::Arc;
use time::Duration as CookieDuration;
use uuid::Uuid;

use crate::{
    errors::AppError,
    utils::session::token::{
        create_access_token, generate_refresh_token, hash_refresh_token, REFRESH_TOKEN_TTL_DAYS,
    },
    AppState,
};

#[derive(Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let raw_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or_else(|| AppError::Unauthorized("Missing refresh token".to_string()))?;

    let token_hash = hash_refresh_token(&raw_token);

    struct SessionRow {
        session_id: Uuid,
        user_id: Uuid,
        role: String,
    }

    let mut tx = state.db.begin().await?;

    let session = sqlx::query_as!(
        SessionRow,
        r#"
        SELECT s.session_id, s.user_id, r.level AS "role!: String"
        FROM users.sessions s
        JOIN users.user_roles ur ON ur.user_id = s.user_id
        JOIN users.roles r ON r.id_role = ur.role_id
        WHERE s.refresh_token_hash = $1
          AND s.revoked_at IS NULL
          AND s.expires_at > now()
        "#,
        token_hash
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

    let new_raw_token = generate_refresh_token();
    let new_hash = hash_refresh_token(&new_raw_token);
    let new_expires_at = chrono::Utc::now() + chrono::Duration::days(REFRESH_TOKEN_TTL_DAYS);

    sqlx::query!(
        r#"
        UPDATE users.sessions
        SET refresh_token_hash = $1, expires_at = $2, last_used_at = now()
        WHERE session_id = $3
        "#,
        new_hash,
        new_expires_at,
        session.session_id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let access_token = create_access_token(session.user_id, &session.role)?;

    let cookie = Cookie::build(("refresh_token", new_raw_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/auth")
        .max_age(CookieDuration::days(REFRESH_TOKEN_TTL_DAYS))
        .build();

    let jar = jar.add(cookie);

    Ok((StatusCode::OK, jar, Json(RefreshResponse { access_token })))
}
