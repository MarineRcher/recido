use axum::Router;
use axum::routing::{get};
use std::sync::Arc;
use crate::AppState;
use crate::handler::user::{me::me};


/// Builds the user-related sub-router.
pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(me))
}
