cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use perse_utils::results::{ErrorTypes, PerseError};
        use sqlx::{query, query_as, types::Uuid, PgPool, Postgres, Transaction};

        // # Modules
        use super::{
            super::{PerseApiRequests, PerseDatabaseModels},
            schema::{NewView, View, ViewVisibilityTypes},
        };

        impl View {
            /// # Retrieve a the Homepage View from the Database
            ///
            /// ## Fields
            /// * `conn` - The database connection to use
            ///
            /// ## Returns
            /// * `Result<Self, PerseError>` - The Homepage View
            pub async fn get_homepage(conn: &PgPool) -> Result<Self, PerseError> {
                query_as!(
                    Self,
                    "
                    SELECT 
                    id,
                    created_at,
                    updated_at,
                    visibility AS \"visibility: ViewVisibilityTypes\",
                    title,
                    content_body,
                    content_head,
                    description,
                    route,
                    is_homepage
                    FROM views
                    WHERE visibility = $1 AND is_homepage = TRUE
                    ",
                    // Only retrieve a route that is visible to the public
                    ViewVisibilityTypes::VisibilityPublic as ViewVisibilityTypes,
                )
                .fetch_one(conn)
                .await
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to retrieve the Homepage View: {err}")))
            }

            /// # Retrieve a View from the Database by Route, if one exists
            ///
            /// ## Fields
            /// * `conn` - The database connection to use
            ///
            /// ## Returns
            /// * `Result<Self, PerseError>` - The View
            pub async fn get_by_route(conn: &PgPool, route: &str) -> Result<Self, PerseError> {
                query_as!(
                    Self,
                    "
                    SELECT 
                    id,
                    created_at,
                    updated_at,
                    visibility AS \"visibility: ViewVisibilityTypes\",
                    title,
                    content_body,
                    content_head,
                    description,
                    route,
                    is_homepage
                    FROM views
                    WHERE visibility = $1 AND route = $2
                    ",
                    // Only retrieve a route that is visible to the public
                    ViewVisibilityTypes::VisibilityPublic as ViewVisibilityTypes,
                    route,
                )
                .fetch_one(conn)
                .await
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to retrieve View by Route: {err}")))
            }

            // /// # Retrieve a collection of all active routes from the Database
            // ///
            // /// ## Returns
            // /// * `Result<Vec<String>, PerseError>` - A collection of all active routes
            // pub async fn _get_active_routes() -> Result<Vec<String>, PerseError> {
            //     Ok(
            //         query_scalar::<_, String>(
            //             r#"
            //             SELECT
            //             route
            //             FROM views
            //             WHERE visibility = $1
            //             -- Order by is_homepage DESC NULLS LAST to ensure that the homepage is always first
            //             ORDER BY is_homepage DESC NULLS LAST
            //             "#
            //         )
            //         // Only retrieve routes that are visible to the public
            //         .bind(ViewVisibilityTypes::VisibilityPublic as ViewVisibilityTypes)
            //         // Get a database connection
            //         .fetch_all(Database::get()?)
            //         .await?
            //     )
            // }

            /// # Update the homepage view
            ///
            /// ## Fields
            /// * `transaction` - The database transaction in use
            /// * `view_id` - The ID of the View to update
            ///
            /// ## Returns
            /// * `Result<(), PerseError>` - Returns void
            pub async fn update_homepage(
                transaction: &mut Transaction<'_, Postgres>,
                view_id: &uuid::Uuid
            ) -> Result<(), PerseError> {
                // Update the homepage view
                query!("UPDATE views SET is_homepage = FALSE WHERE is_homepage = TRUE")
                    .execute(&mut **transaction)
                    .await
                    .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to remove the old homepage view: {err}")))?;

                // Set the new homepage view
                query!("UPDATE views SET is_homepage = TRUE WHERE id = $1", view_id)
                    .execute(&mut **transaction)
                    .await
                    .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to update the new homepage view: {err}")))?;

                Ok(())
            }
        }

        impl PerseDatabaseModels for View {
            type CreateRequest = View;

            /// # Create and return a new `View` record
            ///
            /// ## Fields
            /// * `transaction` - The database transaction in use
            /// * `view` - The `NewView` to insert into the Database
            ///
            /// ## Returns
            /// * `Result<Self, PerseError>` - The newly created View
            async fn create(transaction: &mut Transaction<'_, Postgres>, view: &Self::CreateRequest) -> Result<Self, PerseError> {
                // Create and retrieve entity
                let view = query_as!(
                    Self,
                    "
                    INSERT INTO views (visibility, title, content_body, content_head, description, route, is_homepage)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    RETURNING id, created_at, updated_at, visibility AS \"visibility: ViewVisibilityTypes\", title, content_body, content_head, description, route, is_homepage
                    ",
                    view.visibility.clone() as ViewVisibilityTypes,
                    view.title,
                    view.content_body,
                    view.content_head,
                    view.description,
                    view.route,
                    view.is_homepage
                )
                .fetch_one(&mut **transaction)
                .await
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to create View: {err}")))?;

                // Update this View if it's been declared as the new home page
                if view.is_homepage {
                    if let Some(view_id) = view.id {
                        View::update_homepage(transaction, &view_id).await?;
                    }
                }

                Ok(view)
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
                let query = query_as!(
                    Self,
                    "
                    SELECT 
                    id,
                    created_at,
                    updated_at,
                    visibility AS \"visibility: ViewVisibilityTypes\",
                    title,
                    content_body,
                    content_head,
                    description,
                    route,
                    is_homepage
                    FROM views
                    WHERE id = $1
                    ",
                    id,
                )
                .fetch_one(conn)
                .await
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to retrieve View by ID: {err}")))?;

                Ok(query)
            }

            /// # Retrieve a collection of all Views from the Database
            ///
            /// ## Fields
            /// * `conn` - The database connection to use
            ///
            /// ## Returns
            /// * `Result<Vec<View>, PerseError>` - A collection of all Views
            async fn get_all(conn: &PgPool) -> Result<Vec<View>, PerseError> {
                let query =query_as!(
                    Self,
                    "
                    SELECT
                    id,
                    created_at,
                    updated_at,
                    visibility AS \"visibility: ViewVisibilityTypes\",
                    title,
                    content_body,
                    content_head,
                    description,
                    route,
                    is_homepage
                    FROM views
                    ORDER BY is_homepage DESC NULLS LAST
                    ",
                )
                .fetch_all(conn)
                .await
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to retrieve all Views: {err}")))?;

                Ok(query)
            }
        }

        impl NewView {
            /// # Determine the URL path for a new View
            ///
            /// ## Fields
            /// * `data` - The `NewView` data to generate the URL path from
            ///
            /// ## Returns
            /// * `Result<String, PerseError>` - The URL path for the new View
            pub async fn determine_url_path(
                transaction: &mut Transaction<'_, Postgres>,
                data: &Self
            ) -> Result<String, PerseError> {
                let mut route = data.route.to_string();

                // Check if the Route already exists using sqlx. And if it does, append a number to the end of the route until it doesn't exist
                let mut i = 0;
                loop {
                    // Check if the Route already exists in the database, using sqlx
                    let count: i64 = query!("SELECT COUNT(route) FROM views WHERE route = $1", route)
                        .map(|row| row.count)
                        .fetch_optional(&mut **transaction)
                        .await
                        .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to determine if the route already exists: {err}")))?
                        .unwrap_or(Some(0))
                        .unwrap_or(0);

                    // If the route already exists, increment the counter and route
                    if count != 0 {
                        // If we've attempted to generate a unique URL path 10 times, stop
                        i += 1;
                        if i >= 10 {
                            Err(PerseError::new(ErrorTypes::InternalError, "Failed to determine a unique route for the requested URL".to_string()))?;
                        }

                        // Append a number to the end of the route
                        route = format!("{route}-");
                        continue;
                    }

                    break;
                }

                Ok(route)
            }
        }

        impl PerseApiRequests for NewView {
            /// # Validate the incoming `NewView` API request
            ///
            /// ## Fields
            /// * `self` - The `NewView` to validate
            ///
            /// ## Returns
            /// * `Result<bool, PerseError>` - Whether the `NewView` is valid
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
