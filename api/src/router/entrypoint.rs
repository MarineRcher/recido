use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use crate::AppState;
use crate::handler::hello_world::hello_world;
use super::users::users_router;


/// Builds the application's root [`Router`].
///
/// Wraps the given database pool in [`AppState`] (shared via an
/// [`Arc`] so it can be cheaply cloned across requests), then assembles
/// all routes
/// # Arguments
/// * `pool` - The PostgreSQL connection pool to share across all
///   handlers via [`AppState`].
///
pub fn create_router(pool: PgPool) -> Router {
    let state = Arc::new(AppState { db: pool });

    Router::new()
        .route("/api", get(hello_world))
        .nest("/api/user", users_router())
        .with_state(state)
}
