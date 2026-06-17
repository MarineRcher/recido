use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;


/// Creates and returns a PostgreSQL connection pool.
///
/// Loads environment variables
/// # Panics
/// Panics if `DATABASE_URL` is not set in the environment.
///
/// # Process exit
/// If the connection attempt fails (invalid credentials, unreachable
/// host, etc.), this function logs the error to stderr and terminates
/// the process with exit code `1` rather than returning an error.
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
