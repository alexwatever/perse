use leptos::{view, CollectView, IntoView};

// # Modules
use super::{PerseComponent, PerseStyle};

// # Initial State Component
pub struct InitialState {}

impl PerseComponent for InitialState {
    type InputType = String;

    /// # Build a InitialState component using the provided data
    ///
    /// ## Fields
    /// * `value` - The `Option<Self::InputType>` to build the View component
    ///
    /// ## Returns
    /// * `impl IntoView` - The InitialState `PerseComponent` as a `Fragment`
    fn build(value: Option<Self::InputType>) -> impl IntoView {
        view! {
            {Self::style()}

            <div class="initial_state">
                {value.map(|view| view! {
                    <span class="initial_state-item">
                        {view}
                    </span>
                })}
            </div>
        }
    }

    /// # Build a collection of InitialState components using the provided data
    ///
    /// ## Fields
    /// * `values` - The `Option<Vec<Self::InputType>>` to build the collection of View components
    ///
    /// ## Returns
    /// * `impl IntoView` - The Initial State `PerseComponent` as a `Fragment`
    fn build_iter(values: Option<Vec<Self::InputType>>) -> impl IntoView {
        view! {
            {Self::style()}

            <div class="initial_state">
            {
                values.map(|views| views.into_iter().map(|view| view! {
                    <span class="initial_state-item">{view}</span>
                }).collect_view())
            }
            </div>
        }
    }

    /// # Implement the Initial State component style
    ///
    /// ## Returns
    /// * `Fragment` - The `PerseComponent` as a `Fragment`
    fn style() -> impl IntoView {
        view! {{
            let css: &str = "
            .initial_state {
            }
            ";

            PerseStyle::from_css(css)
        }}
    }
}
