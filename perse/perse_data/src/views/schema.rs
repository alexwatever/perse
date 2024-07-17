use parse_display::FromStr;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// # "View" model
///
/// ## Fields
///
/// * `id` - ID of the View. This value cannot actually be NULL on retrieval, but this allows us to enforce types for sqlx inserts.
/// * `created_at` - Creation date of the View
/// * `updated_at` - Last updated date of the View
/// * `visibility` - Visibility of the View
/// * `title` - Title of the View
/// * `content_body` - Body content
/// * `content_head` - Head Content
/// * `description` - Description of the View
/// * `route` - Route of the View
/// * `is_homepage` - Whether the View is the homepage
// #[cfg(feature = "ssr")]
#[derive(Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct View {
    pub id: Option<uuid::Uuid>,
    #[cfg(feature = "ssr")]
    pub created_at: Option<sqlx::types::chrono::NaiveDateTime>,
    #[cfg(not(feature = "ssr"))]
    pub created_at: Option<String>,
    #[cfg(feature = "ssr")]
    pub updated_at: Option<sqlx::types::chrono::NaiveDateTime>,
    #[cfg(not(feature = "ssr"))]
    pub updated_at: Option<String>,
    pub visibility: ViewVisibilityTypes,
    pub title: String,
    pub content_body: Option<String>,
    pub content_head: Option<String>,
    pub description: Option<String>,
    pub route: String,
    pub is_homepage: bool,
}

impl Default for View {
    /// # Default for View
    ///
    /// ## Returns
    /// * `View` - A new View with the visibility set to `VisibilityHidden`
    fn default() -> Self {
        View {
            id: None,
            created_at: None,
            updated_at: None,
            visibility: ViewVisibilityTypes::VisibilityHidden,
            title: String::new(),
            content_body: None,
            content_head: None,
            description: None,
            route: String::new(),
            is_homepage: false,
        }
    }
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
/// * `is_homepage` - Whether the View is the homepage
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
    pub is_homepage: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<CreateView> for View {
    fn from(view: CreateView) -> Self {
        View {
            id: None,
            created_at: None,
            updated_at: None,
            visibility: view.visibility,
            title: view.title,
            content_body: view.content_body,
            content_head: view.content_head,
            description: view.description,
            route: view.route,
            is_homepage: view.is_homepage.is_some(),
        }
    }
}
