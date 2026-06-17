//! Top-level router configuration.
//!
//! Builds the application's [`Router`], wires up shared state, and
//! mounts the various sub-routers (e.g. user routes) under their
//! respective base paths.

use std::sync::Arc;
use axum::Router;
use sqlx::PgPool;
use crate::AppState;
use super::users::users_router;

/// Builds the application's root [`Router`].
///
/// Wraps the given database pool in [`AppState`] (shared via an
/// [`Arc`] so it can be cheaply cloned across requests), then assembles
/// all routes:
/// - `/api/user/*` — user-related routes, nested from
///   [`users_router`].
///
/// # Arguments
/// * `pool` - The PostgreSQL connection pool to share across all
///   handlers via [`AppState`].
pub fn create_router(pool: PgPool) -> Router {
    let state = Arc::new(AppState { db: pool });

    Router::new()
        .nest("/api/user", users_router())
        .with_state(state)
}
