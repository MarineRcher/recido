use axum::Router;
use axum::routing::{post, get};
use std::sync::Arc;
use crate::AppState;
use crate::handler::users::{register::register, login::login};


/// Builds the user-related sub-router.
pub fn users_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", get(login))
}
