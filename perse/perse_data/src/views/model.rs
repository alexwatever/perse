cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use perse_utils::results::{ErrorTypes, PerseError};
        use sqlx::{query_as, types::Uuid, PgPool};

        // # Modules
        use super::{
            super::{ApiRequests, DatabaseModels},
            schema::{CreateView, View, ViewVisibilityTypes},
        };

        impl DatabaseModels for View {
            type CreateRequest = CreateView;

            /// # Create and return a new `View` record
            ///
            /// ## Fields
            /// * `conn` - The database connection to use
            /// * `view` - The `CreateView` to insert into the Database
            ///
            /// ## Returns
            /// * `Result<Self, PerseError>` - The newly created View
            async fn create(conn: &PgPool, view: &CreateView) -> Result<Self, PerseError> {
                // Create and retrieve entity
                let visib = view.visibility.clone();
                query_as!(
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
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to create View #{view:?}: {err}")))
            }

            /// # Retrieve a `View` record from the database by ID
            ///
            /// ## Fields
            /// * `conn` - The database connection to use
            /// * `id` - The ID of the View to retrieve
            ///
            /// ## Returns
            /// * `Result<Self, PerseError>` - The View record
            async fn get_by_id(conn: &PgPool, id: &str) -> Result<Self, PerseError> {
                // Parse the ID into a UUID
                let id: Uuid = Uuid::parse_str(id)
                    .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to parse the ID as a UUID: {err}")))?;

                // Retrieve the View record from the database
                query_as!(
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
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Unable to retrieve View #{id}: {err}")))
            }

            /// # Retrieve a collection of all Views from the Database
            ///
            /// ## Fields
            /// * `conn` - The database connection to use
            ///
            /// ## Returns
            /// * `Result<Vec<View>, PerseError>` - A collection of all Views
            async fn get_all(conn: &PgPool) -> Result<Vec<View>, PerseError> {
                query_as!(
                    View,
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
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to get the Views collection: {err}")))
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
            ///
            /// ## Fields
            /// * `self` - The `CreateView` to validate
            ///
            /// ## Returns
            /// * `Result<bool, PerseError>` - Whether the `CreateView` is valid
            fn is_valid(&self) -> Result<(), PerseError> {
                use validator::Validate;

                // Request validation
                self.validate()?;

                // TODO: Custom Validation

                Ok(())
            }
        }
    }
}
