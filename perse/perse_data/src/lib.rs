// # Modules
pub mod views;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use once_cell::sync::OnceCell;
        use perse_utils::results::{ErrorTypes, PerseError};
        use std::{env, marker};
        use sqlx::{PgPool, Pool, Postgres, postgres::PgPoolOptions};

        /// # Perse Data

        /// # API path prefix
        pub const PATH_PREFIX: &str = "/api/v1";

        /// # Database Pool
        pub type DatabasePool = Pool<Postgres>;
        pub static DATABASE_POOL: OnceCell<DatabasePool> = OnceCell::new();

        /// # Database
        pub struct Database {}
        impl Database {
            // ## Initialise and return a reference to the database connection pool
            //
            // ### Returns
            // * `&'static DatabasePool` - A reference to the database connection pool
            pub async fn setup() -> &'static DatabasePool {
                dotenv::dotenv().ok();

                // ### Create a new PostgreSQL database connection pool
                let database: DatabasePool = Database::create(
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


            // ## Create the initial database connection pool
            //
            // ### Fields
            // * `database_url` - The URL of the database to connect to
            // * `max_connections` - The maximum number of connections to allow
            //
            // ### Returns
            // * `DatabasePool` - A new database connection pool
            async fn create(database_url: &str, max_connections: u32) -> DatabasePool {
                // Setup a new PostgreSQL database connection pool with provided configuration
                PgPoolOptions::new()
                    .max_connections(max_connections)
                    .connect(database_url)
                    .await
                    .expect("Failed to create a database connection pool.")
            }

            // ## Get the existing database pool
            //
            // ### Returns
            // * `Result<&'static DatabasePool, PerseError>` - A reference to the database connection pool
            pub fn get() -> Result<&'static DatabasePool, PerseError> {
                DATABASE_POOL
                    .get()
                    .ok_or(PerseError::new(ErrorTypes::InternalError, "The database connection pool could not be retrieved."))
            }
        }

        /// # Trait for API requests
        pub trait PerseApiRequests {
            /// # Validate an incoming API request
            ///
            /// ## Fields
            /// * `self` - The API request payload to validate
            ///
            /// ## Returns
            /// * `Result<(), PerseError>` - A `Result` of the validation
            fn is_valid(&self) -> Result<(), PerseError>;
        }

        /// # Trait for Database models
        pub trait PerseDatabaseModels {
            /// The payload schema to create a new database entity
            type CreateRequest;

            /// # Insert an entity into the Database
            ///
            /// ## Fields
            /// * `self` - The database entity to insert
            ///
            /// ## Returns
            /// * `Result<Self, PerseError>` - The `View` created
            fn create(
                conn: &PgPool,
                new_record: &Self::CreateRequest,
            ) -> impl std::future::Future<Output = Result<Self, PerseError>> + Send
            where
                Self: marker::Sized;

            /// # Retrieve a database entity from the Database
            ///
            /// ## Fields
            /// * `self` - The database entity to retrieve
            ///
            /// ## Returns
            /// * `Result<Self, PerseError>` - The `View` requested
            fn get_by_id(
                conn: &PgPool,
                id: &str,
            ) -> impl std::future::Future<Output = Result<Self, PerseError>> + Send
            where
                Self: marker::Sized;

            /// # Retrieve a collection of all entities from the Database
            ///
            /// ## Fields
            /// * `self` - The database entity to retrieve
            ///
            /// ## Returns
            /// * `Result<Vec<View>, PerseError>` - A collection of `View` entities
            fn get_all(conn: &PgPool) -> impl std::future::Future<Output = Result<Vec<Self>, PerseError>> + Send
            where
                Self: marker::Sized;
        }
    }
}
