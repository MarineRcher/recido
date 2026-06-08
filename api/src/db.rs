use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;


pub async fn create_pool() -> PgPool {
    dotenv().ok();
	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to DB successfully");
            pool
        }
        Err(err) => {
            eprintln!("Failed to connect to DB: {}", err);
            std::process::exit(1);
        }
    }
}
