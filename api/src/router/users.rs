use axum::Router;
use axum::routing::post;
use std::sync::Arc;
use crate::AppState;
use crate::handler::users::register::register;


/// Builds the user-related sub-router.
pub fn users_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
}
