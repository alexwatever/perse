use leptos::{html::ElementDescriptor, HtmlElement};

// # Modules
pub mod loader;

/// # Trait for Components
pub trait PerseComponent {
    /// # Define the child element type
    type Element;

    /// # Get the Component
    ///
    /// ## Returns
    /// * `HtmlElement` - The `PerseComponent` as a `HtmlElement`
    fn get() -> HtmlElement<Self::Element>
    where
        <Self as PerseComponent>::Element: ElementDescriptor;
}
