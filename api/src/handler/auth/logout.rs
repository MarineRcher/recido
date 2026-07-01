use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use std::sync::Arc;
use time::Duration as CookieDuration;

use crate::{errors::AppError, utils::session::token::hash_refresh_token, AppState};

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    if let Some(cookie) = jar.get("refresh_token") {
        let token_hash = hash_refresh_token(cookie.value());

        sqlx::query!(
            r#"
            UPDATE users.sessions
            SET revoked_at = now()
            WHERE refresh_token_hash = $1 AND revoked_at IS NULL
            "#,
            token_hash
        )
        .execute(&state.db)
        .await?;
    }

    // Cookie de suppression : même nom/path que celui posé au login,
    // avec une durée de vie nulle pour que le navigateur le supprime.
    let expired_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/auth")
        .max_age(CookieDuration::seconds(0))
        .build();

    let jar = jar.add(expired_cookie);

    Ok((StatusCode::OK, jar))
}
