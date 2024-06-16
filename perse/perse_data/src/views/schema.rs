#[cfg(feature = "ssr")]
use parse_display::{Display, FromStr};
use perse_utils::errors::ErrorTypes;
#[cfg(feature = "ssr")]
use perse_utils::errors::PerseError;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{FromRow, Type};
use validator::Validate;

/// # "View" model
///
/// ## Fields
///
/// * `id` - ID of the View
/// * `visibility` - Visibility of the View
/// * `title` - Title of the View
/// * `content_body` - Body content
/// * `content_head` - Head Content
/// * `description` - Description of the View
/// * `route` - Route of the View
#[cfg(feature = "ssr")]
#[derive(Deserialize, Serialize, Clone, FromRow, Debug)]
pub struct View {
    pub id: uuid::Uuid,
    pub visibility: ViewVisibilityTypes,
    pub title: String,
    pub content_body: Option<String>,
    pub content_head: Option<String>,
    pub description: Option<String>,
    pub route: String,
}

/// # "ViewVisibilityTypes" model
///
/// The enum name's and serde's `rename_all` are important, and must match with the field's `name` in the View.
#[cfg(feature = "ssr")]
#[derive(Deserialize, Serialize, Display, FromStr, Type, Clone, Debug)]
#[sqlx(type_name = "visibility_types", rename_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
#[display(style = "CamelCase")]
pub enum ViewVisibilityTypes {
    VisibilityPublic,
    VisibilityUnlisted,
    VisibilityHidden,
}

/// # "CreateView" request model
///
/// The order, optional status, and type is important, and must match with the field order in the View.
///
/// ## Fields
///
/// * `visibility` - Visibility of the View, as enum `ViewVisibilityTypes`
/// * `title` - Title of the View
/// * `content_body` - Body content
/// * `content_head` - Head Content
/// * `description` - Description of the View
/// * `route` - Route of the View
#[cfg(feature = "ssr")]
#[derive(Deserialize, Clone, Debug)]
pub struct CreateView {
    pub visibility: ViewVisibilityTypes,
    pub title: String,
    pub content_body: Option<String>,
    pub content_head: Option<String>,
    pub description: Option<String>,
    pub route: String,
}

/// # "CreateViewRequest" request model
///
/// ## Fields
///
/// * `visibility` - Visibility of the View, as a String
/// * `title` - Title of the View
/// * `content_body` - Body content
/// * `content_head` - Head Content
/// * `description` - Description of the View
/// * `route` - Route of the View
/// * `automatic_route` - Whether a route should be created automatically
#[derive(Deserialize, Serialize, Clone, Validate, Debug)]
pub struct CreateViewRequest {
    pub visibility: String,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1, max = 255))]
    pub content_body: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub content_head: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub route: Option<String>,
    pub automatic_route: Option<String>,
}

#[cfg(feature = "ssr")]
impl TryFrom<CreateViewRequest> for CreateView {
    type Error = PerseError;

    /// # Try to cast from `CreateViewRequest` to `CreateView`
    fn try_from(value: CreateViewRequest) -> Result<Self, PerseError> {
        let visibility: ViewVisibilityTypes = value.visibility.parse()?;
        let route: String = value.route.ok_or_else(|| {
            PerseError::new(ErrorTypes::InternalError, "A route must be provided.")
        })?;

        Ok(Self {
            visibility,
            title: value.title,
            content_body: value.content_body,
            content_head: value.content_head,
            description: value.description,
            route,
        })
    }
}
