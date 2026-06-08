use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use crate::AppState;
use crate::handler::hello_world::hello_world;

pub fn create_router(pool: PgPool) -> Router {
    let state = Arc::new(AppState { db: pool });

    Router::new()
        .route("/api", get(hello_world))
        .with_state(state)
}
