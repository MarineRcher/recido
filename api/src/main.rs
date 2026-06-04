mod config;
mod db;

use axum::{routing::get, Router};

// AppState est partagé entre tous les handlers
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let pool = db::create_pool(&config.database_url).await;
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("La DB ne répond pas");
    println!("DB OK");

    let app = Router::new()
        .route("/", get(|| async { "Hello from Recido!" }));

    let addr = format!("0.0.0.0:{}", config.port);
    println!("listen on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
