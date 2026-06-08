
mod db;
mod router;
mod handler;
mod models;

pub struct AppState {
    pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;
    let app = router::entrypoint::create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 127.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
