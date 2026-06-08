use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Role {
    pub id_role: Uuid,
    pub level: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
pub struct UserRole {
    pub id_user_roles: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub granted_at: Option<DateTime<Utc>>,
    pub granted_by: Option<Uuid>,
}
