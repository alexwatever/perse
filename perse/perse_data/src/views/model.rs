cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use perse_utils::results::{ErrorTypes, PerseError};
        use sqlx::{query, query_as, types::Uuid, PgPool, Postgres, Transaction};

        // # Modules
        use super::{
            super::{PerseApiRequests, PerseDatabaseModels},
            schema::{CreateView, View, ViewVisibilityTypes},
        };

        impl View {
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
                    .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to update the homepage view: {err}")))?;

                // Set the new homepage view
                query!("UPDATE views SET is_homepage = TRUE WHERE id = $1", view_id)
                    .execute(&mut **transaction)
                    .await
                    .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to set the new homepage view: {err}")))?;

                Ok(())
            }
        }

        impl PerseDatabaseModels for View {
            type CreateRequest = View;

            /// # Create and return a new `View` record
            ///
            /// ## Fields
            /// * `transaction` - The database transaction in use
            /// * `view` - The `CreateView` to insert into the Database
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
                .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to create View #{view:?}: {err}")))?;

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

        impl PerseApiRequests for CreateView {
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
