// # Modules
pub mod views;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use perse_utils::results::PerseError;
        use sqlx::PgPool;
        use std::marker;
        use once_cell::sync::OnceCell;
        use perse_utils::results::ErrorTypes;
        use std::env;
        // use views::schema::Database;

        /// # Perse Data

        /// # Database Pool
        pub type DatabasePool = sqlx::Pool<sqlx::Postgres>;
        pub static DATABASE_POOL: OnceCell<DatabasePool> = OnceCell::new();

        /// # Database
        pub struct Database {}
        impl Database {
            // ## Initialise and return a reference to the database connection pool
            pub async fn setup() -> &'static DatabasePool {
                dotenv::dotenv().ok();

                // Create a new PostgreSQL database connection pool
                let database: DatabasePool = Database::create_connection_pool(
                        &env::var("PERSE_DATABASE_URL")
                            .expect("The `PERSE_DATABASE_URL` environment variable is not available."),
                        env::var("PERSE_DATABASE_MAX_CONNECTIONS")
                            .expect("The `PERSE_DATABASE_MAX_CONNECTIONS` environment variable is not available.")
                            .parse::<u32>()
                            .expect("The `PERSE_DATABASE_MAX_CONNECTIONS` environment variable is in an incorrect format."),
                    )
                    .await;

                // Check and run Database Migrations
                // TODO: This fails with a `VersionMissing` error, they won't be run automatically for the time being
                // sqlx::migrate!()
                //     .run(&database)
                //     .await
                //     .expect("Unable to run the database migrations.");

                // Allocate the Database connection pool reference
                DATABASE_POOL
                    .set(database)
                    .expect("The database connection pool could not be created.");

                // Retrieve and return the Database connection pool
                DATABASE_POOL
                    .get()
                    .expect("The database connection pool could not be retrieved.")
            }

            // Get the existing database pool
            pub fn get_connection_pool() -> Result<&'static DatabasePool, PerseError> {
                DATABASE_POOL
                    .get()
                    .ok_or(PerseError::new(ErrorTypes::InternalError, "The database connection pool could not be retrieved."))
            }

            // Create the initial database connection pool
            async fn create_connection_pool(database_url: &str, max_connections: u32) -> DatabasePool {
                // Setup a new PostgreSQL database connection pool with provided configuration
                sqlx::postgres::PgPoolOptions::new()
                    .max_connections(max_connections)
                    .connect(database_url)
                    .await
                    .expect("Failed to create a database connection pool.")
            }
        }

        /// # Trait for API requests
        pub trait ApiRequests {
            /// # Validate an incoming API request
            ///
            /// ## Fields
            ///
            /// * `self` - The API request payload to validate
            fn is_valid(&self) -> Result<bool, PerseError>;
        }

        /// # Trait for Database models
        pub trait DatabaseModels {
            /// The payload schema to create a new database entity
            type CreateRequest;

            /// # Insert database model into the Database
            ///
            /// ## Fields
            ///
            /// * `self` - The database model to insert
            fn create(
                conn: &PgPool,
                new_record: &Self::CreateRequest,
            ) -> impl std::future::Future<Output = Result<Self, PerseError>> + Send
            where
                Self: marker::Sized;

            /// # Retrieve a database model from the Database
            ///
            /// ## Fields
            ///
            /// * `self` - The database model to retrieve
            fn get_by_id(
                conn: &PgPool,
                id: &str,
            ) -> impl std::future::Future<Output = Result<Self, PerseError>> + Send
            where
                Self: marker::Sized;
        }
    }
}
