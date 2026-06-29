//! Audit-log models.
//!
//! [`Log`] represents a row read from `users.logs`, including
//! database-generated fields (`id`, `created_at`). [`LogEntry`] is the
//! input counterpart used to build a new log entry before insertion,
//! deliberately excluding those generated fields. See
//! [`crate::utils::log::insert_log`] for the insertion logic.

use std::net::IpAddr;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::JsonValue;
use uuid::Uuid;
use crate::models::users::enums::{LogAction, LogEntity};

/// A row read from `users.logs`.
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
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// entry to be inserted into `users.logs`.
#[derive(Debug)]
pub struct LogEntry {
    pub user_id: Option<Uuid>,
    pub action_type: LogAction,
    pub entity_type: LogEntity,
    pub entity_id: Option<Uuid>,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub ip_address: Option<IpAddr>,
    pub error_message: Option<String>,
}
