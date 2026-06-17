mod db;
mod router;
mod handler;
mod models;
mod utils;
mod errors;

/// Shared application state, injected into Axum handlers via the
/// `State` extractor.
///
/// Holds the PostgreSQL connection pool, which is reused across all
/// requests to avoid opening a new connection on every call.
pub struct AppState {
    pub db: sqlx::PgPool,
}

/// Starts the HTTP server.
///
/// # Steps
/// 1. Creates the database connection pool.
/// 2. Builds the application router with its routes and middlewares.
/// 3. Starts listening on `0.0.0.0:3000`.
///
/// # Panics
/// Panics if the connection pool cannot be created, or if port `3000`
/// is already in use / cannot be bound.
#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;
    let app = router::entrypoint::create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
