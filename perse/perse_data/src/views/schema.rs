use serde::{Deserialize, Serialize};
use validator::Validate;

/// # "View" model
///
/// ## Fields
///
/// * `id` - ID of the View
/// * `route` - Route of the View
/// * `title` - Title of the View
/// * `content_body` - Body content
/// * `content_head` - Head Content
/// * `description` - Description of the View
/// * `visibility` - Visibility of the View
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct View {
    pub id: u32,
    pub visibility: bool,
    pub title: String,
    pub content_body: Option<String>,
    pub content_head: Option<String>,
    pub description: Option<String>,
    pub route: String,
}

/// # "CreateView" request model
///
/// The order here is important, and must coincide with the field order in the View.
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
    pub visibility: bool,
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
    pub automatic_route: bool,
}
