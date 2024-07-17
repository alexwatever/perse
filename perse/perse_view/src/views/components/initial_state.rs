use leptos::{html::Span, view, HtmlElement};

// # Modules
use super::PerseComponent;

// # Initial State Component
pub struct InitialState {}

impl PerseComponent for InitialState {
    type InputList = String;
    type ReturnedComponent = Span;

    /// # Create a standalone InitialState component
    ///
    /// ## Returns
    /// * `HtmlElement` - The InitialState `PerseComponent` as a `HtmlElement`
    fn create() -> HtmlElement<Self::ReturnedComponent> {
        view! {
            <span></span>
        }
    }

    /// # Build a InitialState component using the provided data
    ///
    /// ## Returns
    /// * `HtmlElement` - The InitialState `PerseComponent` as a `HtmlElement`
    fn build_from(input: Self::InputList) -> HtmlElement<Self::ReturnedComponent> {
        view! {
            <span>{input}</span>
        }
    }

    /// # Build a collection of InitialState components using the provided data
    ///
    /// ## Returns
    /// * `Vec<HtmlElement<Self::ReturnedComponent>>` - The InitialState `PerseComponent` as a `Vec<HtmlElement<Self::ReturnedComponent>>`
    fn build_from_iter(input: Vec<Self::InputList>) -> Vec<HtmlElement<Self::ReturnedComponent>> {
        input
            .iter()
            .map(|item| {
                view! {
                    <span>{item}</span>
                }
            })
            .collect()
    }
}
