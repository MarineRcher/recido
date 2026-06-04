pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL not define"),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("Invalid port"),
        }
    }
}
