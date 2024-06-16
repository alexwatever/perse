use serde::{Deserialize, Serialize};
// #[cfg(feature = "ssr")]
// use sqlx::{prelude::Type, FromRow};
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
#[derive(Deserialize, Serialize, Clone, Debug)]
// #[derive(Deserialize, Serialize, FromRow, Clone, Debug)]
pub struct View {
    pub id: u32,
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
#[derive(Deserialize, Serialize, Clone, Debug)]
// #[derive(Deserialize, Serialize, Type, Clone, Debug)]
#[serde(rename_all = "snake_case")]
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
/// * `visibility` - Visibility of the View
/// * `title` - Title of the View
/// * `content_body` - Body content
/// * `content_head` - Head Content
/// * `description` - Description of the View
/// * `route` - Route of the View
/// * `automatic_route` - Whether a route should be created automatically
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
    pub route: Option<String>,
    pub automatic_route: Option<String>,
}
