use leptos::{view, CollectView, IntoView};

// # Modules
use super::{PerseComponent, PerseStyle};
use perse_data::views::schema::View;

// # View List Component
pub struct ViewList {}

impl PerseComponent for ViewList {
    type InputType = View;

    /// # Build a View List component using the provided data
    ///
    /// ## Fields
    /// * `value` - The `Option<Self::InputType>` to build the View List component
    ///
    /// ## Returns
    /// * `impl IntoView` - The ViewList `PerseComponent` as a `Fragment`
    fn build(value: Option<Self::InputType>) -> impl IntoView {
        view! {
            {Self::style()}

            <ul class="view_list">
                {value.map(|view| view! {
                    <li class="view_list-item">
                        <a href={format!("/{}", view.route)} title={view.title.clone()}>
                            {format!("/{} ({})", view.route, view.title)}
                        </a>
                    </li>
                })}
            </ul>
        }
    }

    /// # Build a collection of View List components using the provided data
    ///
    /// ## Fields
    /// * `values` - The `Option<Vec<Self::InputType>>` to build the collection of View List components
    ///
    /// ## Returns
    /// * `impl IntoView` - The View List `PerseComponent` as a `Fragment`
    fn build_iter(values: Option<Vec<Self::InputType>>) -> impl IntoView {
        view! {
            {Self::style()}

            <ul class="view_list">
            {
                values
                    .map(|views| views.into_iter().map(|view| view! {
                        <li class="view_list-item">
                            <a href={format!("/{}", view.route)} title={view.title.clone()}>
                                {format!("/{} ({})", view.route, view.title)}
                            </a>
                        </li>
                    }).collect_view())
            }
            </ul>
        }
    }

    /// # Implement the View List component CSS style
    ///
    /// ## Returnss
    /// * `Fragment` - The `PerseComponent` as a `Fragment`
    fn style() -> impl IntoView {
        view! {{
            let css: &str = "
            .view_list {
            }
            ";

            PerseStyle::from_css(css)
        }}
    }
}
