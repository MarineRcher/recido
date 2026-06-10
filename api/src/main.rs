mod db;
mod router;
mod handler;
mod models;
mod utils;
mod errors;

pub struct AppState {
    pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::create_pool().await;
    let app = router::entrypoint::create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
