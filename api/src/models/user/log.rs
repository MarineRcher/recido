use std::net::IpAddr;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::JsonValue;
use uuid::Uuid;
use crate::models::user::enums::{LogAction, LogEntity};

#[derive(Debug, FromRow)]
pub struct Log {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action_type: LogAction,
    pub entity_type: LogEntity,
    pub entity_id: Option<Uuid>,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}
