
cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use once_cell::sync::OnceCell;
        use sqlx::{Pool, Postgres, postgres::PgPoolOptions, Error as SqlxError};
        
        pub type DatabasePool = Pool<Postgres>;
        pub static DATABASE_POOL: OnceCell<DatabasePool> = OnceCell::new();
        
        pub struct Database {}
        impl Database {
            /// Initialise the database connection pool
            pub async fn initialise() -> Result<DatabasePool, SqlxError> {
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
            pub fn get_connection_pool() -> Option<&'static DatabasePool> {
                super::db::DATABASE_POOL.get()
            }
        
            /// Create the initial database connection pool
            async fn create_connection_pool(database_url: String, max_connections: u32) -> Result<DatabasePool, SqlxError> {
                // Setup a new PostgreSQL database connection pool with provided configuration
                PgPoolOptions::new()
                    .max_connections(max_connections)
                    .connect(&database_url)
                    .await
            }
        }

   }
}
