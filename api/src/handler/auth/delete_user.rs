use axum::{extract::State, response::IntoResponse, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::Deserialize;
use std::sync::Arc;
use time::Duration as CookieDuration;

use crate::{
    AppState,
    errors::AppError,
    utils::session::auth::AuthUser,
    models::users::{enums::{LogAction, LogEntity}, log::LogEntry},
    utils::{logs::insert_log, password::verify_password},
};

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub current_password: String,
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    jar: CookieJar,
    axum::Json(body): axum::Json<DeleteUserRequest>,
) -> Result<impl IntoResponse, AppError> {
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

    insert_log(
        &mut *tx,
        LogEntry::new(LogAction::Delete, LogEntity::User)
            .user_id(auth.user_id)
            .entity_id(auth.user_id),
    )
    .await?;

   let deleted = sqlx::query!(
        r#"DELETE FROM users.users WHERE user_id = $1"#,
        auth.user_id,
    )
    .execute(&mut *tx)
    .await?;

    if deleted.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    tx.commit().await?;

    // Nettoyage du cookie côté client.
    let expired_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/auth")
        .max_age(CookieDuration::seconds(0))
        .build();

    let jar = jar.add(expired_cookie);

    Ok((StatusCode::NO_CONTENT, jar))
}
