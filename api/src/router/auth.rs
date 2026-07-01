use axum::Router;
use axum::routing::{post};
use std::sync::Arc;
use crate::AppState;
use crate::handler::auth::{register::register, login::login, refresh::refresh, logout::logout};


/// Builds the user-related sub-router.
pub fn auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
}
