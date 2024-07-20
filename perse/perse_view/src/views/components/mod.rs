use leptos::{view, IntoView};

// # Modules
pub mod initial_state;
pub mod loader;
pub mod navbar;
pub mod view_list;

/// # Component Trait
pub trait PerseComponent {
    /// # Define the input type for a collection of data
    type InputType;

    /// # Build a component using the provided data
    ///
    /// ## Fields
    /// * `value` - The `Option<Self::InputType>` to build the component
    ///
    /// ## Returns
    /// * `Fragment` - The `PerseComponent` as a `Fragment`
    fn build(value: Option<Self::InputType>) -> impl IntoView;

    /// # Build a collection of components using the provided data
    ///
    /// ## Fields
    /// * `values` - The `Vec<Self::InputType>` to build the collection of components
    ///
    /// ## Returns
    /// * `Fragment` - The `PerseComponent` as an iterator of `Fragment`
    fn build_iter(values: Option<Vec<Self::InputType>>) -> impl IntoView;

    /// # Implement the component style
    ///
    /// ## Fields
    /// * `component` - The `PerseComponent` CSS to format
    ///
    /// ## Returns
    /// * `Fragment` - The `PerseComponent` as a `Fragment`
    fn style() -> impl IntoView;
}

pub struct PerseStyle {}
impl PerseStyle {
    /// # Style Component wrapper
    ///
    /// ## Fields
    /// * `css` - The `PerseComponent` CSS to format
    ///
    /// ## Returns
    /// * `Fragment` - The `PerseComponent` as a `Fragment`
    pub fn from_css(css: &'static str) -> impl IntoView {
        view! {
            <style>
            {css}
            </style>
        }
    }
}
