use perse_utils::errors::PerseError;

// # Modules
use super::{
    super::{ApiRequests, DatabaseModels},
    schema::{CreateView, View},
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
    pub fn new(mut data: CreateView) -> Result<View, PerseError> {
        // Determine the URL path
        data.route = CreateView::determine_url_path(&data)?;

        // Insert the new View into the Database
        View::insert_into_db(&data)?;

        // Retrieve the new View from the Database
        let view = View::retrieve_from_db(1)?;

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

    // Insert a new View into the Database
    fn insert_into_db(_new_record: &CreateView) -> Result<(), PerseError> {
        Ok(())
    }

    // Get the requested View from the Database
    fn retrieve_from_db(_view_id: u32) -> Result<Self, PerseError> {
        let _view = Self {
            id: 1,
            route: String::from("Example route"),
            title: String::from("Example title"),
            content_body: Some(String::from("Example content_body")),
            content_head: Some(String::from("Example content_head")),
            description: Some(String::from("Example description")),
            visibility: true,
        };

        Ok(_view)
    }
}

impl CreateView {
    /// # Get a new View from the Database
    ///
    /// ## Fields
    ///
    /// * `view_id` - The ID of the View to retrieve
    pub fn get_new_view(view_id: u32) -> Result<View, PerseError> {
        View::retrieve_from_db(view_id)
    }

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
        View::generate_new_route(&data.route, data.automatic_route)
    }
}

impl ApiRequests for CreateView {
    /// # Validate the incoming `CreateView` API request
    fn is_valid(&self) -> Result<bool, PerseError> {
        Ok(true)
    }
}
