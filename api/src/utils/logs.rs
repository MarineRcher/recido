//! Logging utilities for the audit trail table `users.logs`.

use serde_json::Value as JsonValue;
use sqlx::{Executor, Postgres};
use std::net::IpAddr;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::users::enums::{LogAction, LogEntity};
use crate::models::users::log::LogEntry;

impl LogEntry {
    /// Creates a new log entry for the given action and entity type,
    /// leaving all optional fields unset.
    pub fn new(action_type: LogAction, entity_type: LogEntity) -> Self {
        Self {
            user_id: None,
            action_type,
            entity_type,
            entity_id: None,
            old_values: None,
            new_values: None,
            ip_address: None,
            error_message: None,
        }
    }

    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn entity_id(mut self, entity_id: Uuid) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    pub fn old_values(mut self, old_values: JsonValue) -> Self {
        self.old_values = Some(old_values);
        self
    }

    pub fn new_values(mut self, new_values: JsonValue) -> Self {
        self.new_values = Some(new_values);
        self
    }

    pub fn ip_address(mut self, ip_address: IpAddr) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
        self.error_message = Some(error_message.into());
        self
    }
}

/// Inserts a log entry into `users.logs`.
pub async fn insert_log<'e, E>(executor: E, entry: LogEntry) -> Result<(), AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query!(
    r#"
    INSERT INTO users.logs (
        user_id, action_type, entity_type, entity_id,
        old_values, new_values, ip_address, error_message
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "#,
    entry.user_id,
    entry.action_type as _,
    entry.entity_type as _,
    entry.entity_id,
    entry.old_values,
    entry.new_values,
    entry.ip_address as _,
    entry.error_message,
    )
    .execute(executor)
    .await?;

    Ok(())
}
