//! Top-level router configuration.
//!
//! Builds the application's [`Router`], wires up shared state, and
//! mounts the various sub-routers (e.g. user routes) under their
//! respective base paths.

use std::sync::Arc;
use axum::Router;
use sqlx::PgPool;
use crate::{AppState, router::user::user_router};
use super::auth::auth_router;

/// Builds the application's root .
/// # Arguments
/// * `pool` - The PostgreSQL connection pool to share across all
///   handlers via [`AppState`].
pub fn create_router(pool: PgPool) -> Router {
    let state = Arc::new(AppState { db: pool });

    Router::new()
        .nest("/api/auth", auth_router())
        .nest("/api/user", user_router())
        .with_state(state)
}
