use leptos::{view, CollectView, IntoView};

// # Modules
use super::{PerseComponent, PerseStyle};

// # Loader Component
pub struct Loader {}

impl PerseComponent for Loader {
    type InputType = String;

    /// # Build a Loader component using the provided data
    ///
    /// ## Fields
    /// * `value` - The `Option<Self::InputType>` to build the Loader component
    ///
    /// ## Returns
    /// * `HtmlElement` - The Loader `PerseComponent` as a `HtmlElement`
    fn build(value: Option<Self::InputType>) -> impl IntoView {
        view! {
            {Self::style()}

            <div class="loader">
                {value.map(|view| view! {
                    <span class="loader-item">"Loading" <span>
                        {view}
                    </span> "..."</span>
                })}
            </div>
        }
    }

    /// # Build a collection of Loader components using the provided data
    ///
    /// ## Fields
    /// * `values` - The `Option<Vec<Self::InputType>>` to build the collection of Loader components
    ///
    /// ## Returns
    /// * `impl IntoView` - The Loader `PerseComponent` as a `Fragment`
    fn build_iter(values: Option<Vec<Self::InputType>>) -> impl IntoView {
        view! {
            {Self::style()}

            <div class="loader">
            {
                values.map(|views| views.into_iter().map(|view| view! {
                    <span class="loader-item">"Loading" <span>{view}</span> "..."</span>
                }).collect_view())
            }
            </div>
        }
    }

    /// # Implement the Loader component style
    ///
    /// ## Returns
    /// * `Fragment` - The `PerseComponent` as a `Fragment`
    fn style() -> impl IntoView {
        view! {{
            let css: &str = "
            .loader {
            }
            ";

            PerseStyle::from_css(css)
        }}
    }
}
