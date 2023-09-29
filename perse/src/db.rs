
pub type DatabasePool = sqlx::Pool<sqlx::Postgres>;
pub static DATABASE_POOL: once_cell::sync::OnceCell<DatabasePool> = once_cell::sync::OnceCell::new();

pub struct Database {}
impl Database {
    /// Initialise the database connection pool
    #[cfg(feature = "ssr")]
    pub async fn initialise() -> Result<DatabasePool, sqlx::Error> {
        // Create a new PostgreSQL database connection pool
        dotenv::dotenv().ok();
        Self::create_connection_pool(
            std::env::var("PERSE_DATABASE_URL")
                .expect("The `PERSE_DATABASE_URL` environment variable is not available."),
            std::env::var("PERSE_DATABASE_MAX_CONNECTIONS")
                .expect("The `PERSE_DATABASE_MAX_CONNECTIONS` environment variable is not available.")
                .parse::<u32>() 
                .expect("The `PERSE_DATABASE_MAX_CONNECTIONS` environment variable is in an incorrect format."),
        )
        .await
    }

    /// Get the existing database pool
    #[cfg(feature = "ssr")]
    pub fn get_connection_pool() -> Option<&'static DatabasePool> {
        super::db::DATABASE_POOL.get()
    }

    /// Create the initial database connection pool
    #[cfg(feature = "ssr")]
    async fn create_connection_pool(database_url: String, max_connections: u32) -> Result<DatabasePool, sqlx::Error> {
        // Setup a new PostgreSQL database connection pool with provided configuration
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&database_url)
            .await
    }
}
