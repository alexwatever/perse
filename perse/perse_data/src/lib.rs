
cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use once_cell::sync::OnceCell;
        use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
        
        pub type DatabasePool = Pool<Postgres>;
        pub static DATABASE_POOL: OnceCell<DatabasePool> = OnceCell::new();
        
        pub struct Database {}
        impl Database {
            /// Initialise and return a reference to the database connection pool
            pub async fn setup() -> &'static DatabasePool {
                dotenv::dotenv().ok();
                
                // Create a new PostgreSQL database connection pool
                let database: DatabasePool = Self::create_connection_pool(
                        std::env::var("PERSE_DATABASE_URL")
                            .expect("The `PERSE_DATABASE_URL` environment variable is not available."),
                        std::env::var("PERSE_DATABASE_MAX_CONNECTIONS")
                            .expect("The `PERSE_DATABASE_MAX_CONNECTIONS` environment variable is not available.")
                            .parse::<u32>() 
                            .expect("The `PERSE_DATABASE_MAX_CONNECTIONS` environment variable is in an incorrect format."),
                    )
                    .await;

                // Check and run Database Migrations
                sqlx::migrate!()
                    .run(&database)
                    .await
                    .expect("Unable to run the database migrations.");

                // Allocate the Database connection pool reference
                DATABASE_POOL
                    .set(database)
                    .expect("The database connection pool could not be created.");

                // Retrieve the Database connection pool
                let database = Self::get_connection_pool()
                    .expect("The database connection pool could not be retrieved.");

                // Return the connection pool
                database
            }
        
            /// Get the existing database pool
            pub fn get_connection_pool() -> Option<&'static DatabasePool> {
                DATABASE_POOL.get()
            }
        
            /// Create the initial database connection pool
            async fn create_connection_pool(database_url: String, max_connections: u32) -> DatabasePool {
                // Setup a new PostgreSQL database connection pool with provided configuration
                PgPoolOptions::new()
                    .max_connections(max_connections)
                    .connect(&database_url)
                    .await
                    .expect("Failed to create a database connection pool.")
            }
        }

   }
}
