use leptos::{html::P, view, HtmlElement};

// # Modules
use super::PerseComponent;

// # Loader
pub struct Loader {}

impl PerseComponent for Loader {
    type Element = P;

    /// # Get the Loader Component
    ///
    /// ## Returns
    /// * `HtmlElement` - The Loader `PerseComponent` as a `HtmlElement`
    fn get() -> HtmlElement<Self::Element> {
        view! {
            <p>"Loading..."</p>
        }
    }
}
