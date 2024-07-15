cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use perse_utils::results::{ErrorTypes, PerseError};
        use sqlx::{query_as, types::Uuid, PgPool};

        // # Modules
        use super::{
            super::{ApiRequests, Database, DatabaseModels},
            schema::{CreateView, View, ViewVisibilityTypes},
        };

        impl View {
            /// # Insert a new View into the Database
            ///
            /// ## Fields
            /// * `data` (mut) - The `CreateView` to insert into the Database
            ///
            /// ## Returns
            /// * `Result<View, PerseError>` - The newly created View
            pub async fn new(mut data: CreateView) -> Result<View, PerseError> {
                // Custom validation
                data.is_valid()?;

                // Get a database connection
                let conn = Database::get_connection_pool()?;

                // Determine the URL path
                data.route = CreateView::determine_url_path(&data)?;

                // Insert and retrieve the new View
                let view = View::create(conn, &data).await?;

                Ok(view)
            }
        }

        impl DatabaseModels for View {
            type CreateRequest = CreateView;

            /// Create and return a new `View` record
            async fn create(conn: &PgPool, view: &CreateView) -> Result<Self, PerseError> {
                // Create and retrieve
                let visib = view.visibility.clone();
                let query: View = query_as!(
                    View,
                    "
                    INSERT INTO views (visibility, title, content_body, content_head, description, route)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING id, visibility AS \"visibility: ViewVisibilityTypes\", title, content_body, content_head, description, route
                    ",
                    visib as ViewVisibilityTypes,
                    view.title,
                    view.content_body,
                    view.content_head,
                    view.description,
                    view.route,
                )
                .fetch_one(conn)
                .await
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to create View #{view:?}: {err}")))?;

                Ok(query)
            }

            /// Retrieve a `View` record from the database by ID
            async fn get_by_id(conn: &PgPool, id: &str) -> Result<Self, PerseError> {
                // Parse the ID into a UUID
                let id: Uuid = Uuid::parse_str(id)
                    .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to parse the ID as a UUID: {err}")))?;

                // Retrieve the View record from the database
                let query: Self = query_as!(
                    Self,
                    "
                    SELECT 
                    id,
                    visibility AS \"visibility: ViewVisibilityTypes\",
                    title,
                    content_body,
                    content_head,
                    description,
                    route
                    FROM views
                    WHERE id = $1
                    ",
                    id,
                )
                .fetch_one(conn)
                .await
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Unable to retrieve View #{id}: {err}")))?;

                Ok(query)
            }
        }

        impl CreateView {
            /// # Determine the URL path for a new View
            ///
            /// ## Fields
            /// * `data` - The `CreateView` data to generate the URL path from
            ///
            /// ## Returns
            /// * `Result<String, PerseError>` - The URL path for the new View
            pub fn determine_url_path(data: &Self) -> Result<String, PerseError> {
                // TODO: Check if the Route already exists

                // TODO: Generate an eligible URL
                let final_url = data.route.to_string();

                Ok(final_url)
            }
        }

        impl ApiRequests for CreateView {
            /// # Validate the incoming `CreateView` API request
            fn is_valid(&self) -> Result<bool, PerseError> {
                // TODO: Server Validation
                Ok(true)
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "csr")] {
        impl View {
            /// Retrieve the collection of `View` records from the database
            pub async fn get_list(conn: &sqlx::PgPool) -> Result<Vec<Self>, PerseError> {
                // Retrieve the View records from the database
                let query: Vec<Self> = query_as!(
                    Self,
                    "
                    SELECT
                    id,
                    visibility AS \"visibility: ViewVisibilityTypes\",
                    title,
                    content_body,
                    content_head,
                    description,
                    route
                    FROM views
                    ",
                )
                .fetch_all(conn)
                .await
                .map_err(|err| {
                    PerseError::new(
                        ErrorTypes::InternalError,
                        format!("Unable to retrieve Views: {err}"),
                    )
                })?;

                Ok(query)
            }
        }
    }
}
