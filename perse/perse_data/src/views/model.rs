cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use perse_utils::errors::{PerseError, ErrorTypes};
        use sqlx::{query_as, types::Uuid, PgPool};

        // # Modules
        use super::{
            super::{ApiRequests, Database, DatabaseModels},
            // super::{ApiRequests, Database, DatabaseModels},
            schema::{CreateView, View, ViewVisibilityTypes},
        };

        impl View {
            /// # Insert a new View into the Database
            ///
            /// ## Fields (mut)
            ///
            /// * `data` - The `View` to insert into the Database
            ///
            /// ## Returns
            ///
            /// * `Result<View, PerseError>` - The newly created View
            pub async fn new(mut data: CreateView) -> Result<View, PerseError> {
                // Get a database connection
                let conn = Database::get_connection_pool()?;

                // Determine the URL path
                data.route = Some(CreateView::determine_url_path(&data)?);

                // Insert and retrieve the new View
                let view = View::create(conn, &data).await?;

                Ok(view)
            }

            /// # Logic to generate a new URL route
            ///
            /// ## Fields
            ///
            /// * `suggested_path` - The suggested URL route
            /// * `automatic_route` - Whether a route should be created automatically, ignoring the `suggested_path`
            pub fn generate_new_route(
                suggested_path: &str,
                _automatic_route: bool,
            ) -> Result<String, PerseError> {
                Ok(suggested_path.to_string())
            }
        }

        impl DatabaseModels for View {
            type CreateRequest = CreateView;

            /// Create and return a new `View` record
            async fn create(conn: &PgPool, view: &CreateView) -> Result<Self, PerseError> {
                // Create and retrieve
                // let query: View = query_as!(
                //     View,
                //     r#"
                //     INSERT INTO views (visibility, title, content_body, content_head, description, route)
                //     VALUES ($1, $2, $3, $4, $5, $6)
                //     RETURNING id, visibility as "visibility: _", title, content_body, content_head, description, route
                //     "#,
                //     view.visibility.clone() as ViewVisibilityTypes,
                //     view.title,
                //     view.content_body,
                //     view.content_head,
                //     view.description,
                //     view.route,
                // )
                // .fetch_one(conn)
                // .await?;
                let query = View {
                    id: 1_u32,
                    visibility: ViewVisibilityTypes::VisibilityPublic,
                    title: "".to_string(),
                    content_body: Some("".to_string()),
                    content_head: Some("".to_string()),
                    description: Some("".to_string()),
                    route: "".to_string(),
                };

                Ok(query)
            }

            /// Retrieve a `View` record from the database by ID\
            async fn get_by_id(conn: &PgPool, id: &str) -> Result<Option<Self>, PerseError> {
                // Parse the ID into a UUID
                let id: Uuid = Uuid::parse_str(id)
                    .map_err(|err| PerseError::new(ErrorTypes::InternalError, format!("Failed to parse the ID as a UUID: {err}")))?;

                // Retrieve the View record from the database
                // let query = query_as!(
                //     View,
                //     r#"
                //     SELECT id, visibility as "visibility: _", title, content_body, content_head, description, route
                //     FROM views
                //     WHERE id = $1
                //     "#,
                //     id
                // )
                // .fetch_optional(conn)
                // .await?;
                let query = Some(View {
                    id: 1_u32,
                    visibility: ViewVisibilityTypes::VisibilityPublic,
                    title: "".to_string(),
                    content_body: Some("".to_string()),
                    content_head: Some("".to_string()),
                    description: Some("".to_string()),
                    route: "".to_string(),
                });

                Ok(query)
            }
        }

        impl CreateView {
            /// # Determine the URL path for a new View
            ///
            /// ## Fields
            ///
            /// * `data` - The `CreateView` data to generate the URL path from
            ///
            /// ## Returns
            ///
            /// * `Result<String, PerseError>` - The URL path for the new View
            pub fn determine_url_path(data: &Self) -> Result<String, PerseError> {
                // TODO:
                View::generate_new_route(&data.route.to_owned().unwrap_or_default(), true)
            }
        }

        impl ApiRequests for CreateView {
            /// # Validate the incoming `CreateView` API request
            fn is_valid(&self) -> Result<bool, PerseError> {
                // TODO:
                Ok(true)
            }
        }
    }
}
