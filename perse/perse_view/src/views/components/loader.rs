use leptos::{html::P, view, HtmlElement};

// # Modules
use super::PerseComponent;

// # Loader Component
pub struct Loader {}

impl PerseComponent for Loader {
    type InputList = String;
    type ReturnedComponent = P;

    /// # Create a standalone Loader component
    ///
    /// ## Returns
    /// * `HtmlElement` - The Loader `PerseComponent` as a `HtmlElement`
    fn create() -> HtmlElement<Self::ReturnedComponent> {
        view! {
            <p>"Loading..."</p>
        }
    }

    /// # Build a Loader component using the provided data
    ///
    /// ## Returns
    /// * `HtmlElement` - The Loader `PerseComponent` as a `HtmlElement`
    fn build_from(input: Self::InputList) -> HtmlElement<Self::ReturnedComponent> {
        view! {
            <p>"Loading" <span>{input}</span> "..."</p>
        }
    }

    /// # Build a collection of Loader components using the provided data
    ///
    /// ## Returns
    /// * `Vec<HtmlElement<Self::ReturnedComponent>>` - The Loader `PerseComponent` as a `Vec<HtmlElement<Self::ReturnedComponent>>`
    fn build_from_iter(input: Vec<Self::InputList>) -> Vec<HtmlElement<Self::ReturnedComponent>> {
        input
            .iter()
            .map(|item| {
                view! {
                    <p>"Loading" <span>{item}</span> "..."</p>
                }
            })
            .collect()
    }
}
