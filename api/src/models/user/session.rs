use std::net::IpAddr;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Session {
    pub id_session: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub device_info: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_revoked: bool,
}
