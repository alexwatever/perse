use parse_display::FromStr;
use serde::{Deserialize, Serialize};
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
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Deserialize, Serialize, Clone, Debug)]
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
#[derive(Deserialize, Serialize, FromStr, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(
    feature = "ssr",
    sqlx(type_name = "visibility_types", rename_all = "PascalCase")
)]
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
#[derive(Deserialize, Serialize, Clone, Validate, Debug)]
pub struct CreateView {
    pub visibility: ViewVisibilityTypes,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1, max = 255))]
    pub content_body: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub content_head: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub route: String,
}
