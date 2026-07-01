use chrono::{Duration, Utc};
use sqlx::{Executor, Postgres};
use std::net::IpAddr;
use uuid::Uuid;

use crate::errors::AppError;
use crate::utils::session::token::{
    generate_refresh_token, hash_refresh_token, MAX_SESSIONS_PER_USER, REFRESH_TOKEN_TTL_DAYS,
};

pub async fn create_session<'e, E>(
    executor: E,
    user_id: Uuid,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> Result<String, AppError>
where
    E: Executor<'e, Database = Postgres> + Copy,
{
    let active_count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "count!"
        FROM users.sessions
        WHERE user_id = $1 AND revoked_at IS NULL AND expires_at > now()
        "#,
        user_id
    )
    .fetch_one(executor)
    .await?;

    if active_count >= MAX_SESSIONS_PER_USER {
        sqlx::query!(
            r#"
            UPDATE users.sessions
            SET revoked_at = now()
            WHERE session_id = (
                SELECT session_id FROM users.sessions
                WHERE user_id = $1 AND revoked_at IS NULL AND expires_at > now()
                ORDER BY created_at ASC
                LIMIT 1
            )
            "#,
            user_id
        )
        .execute(executor)
        .await?;
    }

    let raw_token = generate_refresh_token();
    let token_hash = hash_refresh_token(&raw_token);
    let expires_at = Utc::now() + Duration::days(REFRESH_TOKEN_TTL_DAYS);

    sqlx::query!(
        r#"
        INSERT INTO users.sessions (user_id, refresh_token_hash, expires_at, ip_address, user_agent)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        user_id,
        token_hash,
        expires_at,
        ip_address as _,
        user_agent,
    )
    .execute(executor)
    .await?;

    Ok(raw_token)
}
