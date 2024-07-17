use leptos::{html::ElementDescriptor, HtmlElement};

// # Modules
pub mod initial_state;
pub mod loader;
pub mod view_list;

/// # Component Trait
pub trait PerseComponent {
    /// # Define the input type for a collection of data
    type InputList;

    /// # Define the child element type
    type ReturnedComponent;

    /// # Create a standalone component
    ///
    /// ## Returns
    /// * `HtmlElement` - The `PerseComponent` as a `HtmlElement`
    fn create() -> HtmlElement<Self::ReturnedComponent>
    where
        <Self as PerseComponent>::ReturnedComponent: ElementDescriptor;

    /// # Build a component using the provided data
    ///
    /// ## Returns
    /// * `HtmlElement` - The `PerseComponent` as a `HtmlElement`
    fn build_from(input: Self::InputList) -> HtmlElement<Self::ReturnedComponent>
    where
        <Self as PerseComponent>::ReturnedComponent: ElementDescriptor;

    /// # Build a collection of components using the provided data
    ///
    /// ## Returns
    /// * `Vec<HtmlElement>` - The `PerseComponent` as an iterator of `HtmlElement`
    fn build_from_iter(input: Vec<Self::InputList>) -> Vec<HtmlElement<Self::ReturnedComponent>>
    where
        <Self as PerseComponent>::ReturnedComponent: ElementDescriptor;
}
