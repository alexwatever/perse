use leptos::{html::Li, view, HtmlElement};

// # Modules
use super::PerseComponent;
use perse_data::views::schema::View;

// # View List Component
pub struct ViewList {}

impl PerseComponent for ViewList {
    type InputList = View;
    type ReturnedComponent = Li;

    /// # Create a standalone View List component
    ///
    /// ## Returns
    /// * `HtmlElement` - The ViewList `PerseComponent` as a `HtmlElement`
    fn create() -> HtmlElement<Self::ReturnedComponent> {
        view! {
            <li>
                <a href="#" title="">""</a>
            </li>
        }
    }

    /// # Build a View List component using the provided data
    ///
    /// ## Returns
    /// * `HtmlElement` - The ViewList `PerseComponent` as a `HtmlElement`
    fn build_from(input: Self::InputList) -> HtmlElement<Self::ReturnedComponent> {
        view! {
            <li>
                <a href={format!("/{}", input.route)} title={input.title.clone()}>
                    {format!("/{} ({})", input.route, input.title)}
                </a>
            </li>
        }
    }

    /// # Build a collection of View List components using the provided data
    ///
    /// ## Returns
    /// * `Vec<HtmlElement<Self::ReturnedComponent>>` - The ViewList `PerseComponent` as a `Vec<HtmlElement<Self::ReturnedComponent>>`
    fn build_from_iter(input: Vec<Self::InputList>) -> Vec<HtmlElement<Self::ReturnedComponent>> {
        input
            .into_iter()
            // Iterate through Views
            .map(ViewList::build_from)
            .collect()
    }
}
