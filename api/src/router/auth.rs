use axum::Router;
use axum::routing::{post, patch, delete};
use std::sync::Arc;
use crate::AppState;
use crate::handler::auth::change_login::change_login;
use crate::handler::auth::delete_user::delete_user;
use crate::handler::auth::password::change_password;
use crate::handler::auth::{register::register, login::login, refresh::refresh, logout::logout};


/// Builds the user-related sub-router.
pub fn auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/change-login", patch(change_login))
        .route("/change-password", patch(change_password))
        .route("/delete-user", delete(delete_user))
}
