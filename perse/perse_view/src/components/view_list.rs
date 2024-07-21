use leptos::{view, CollectView, IntoView};

// # Modules
use super::{PerseComponent, PerseStyle};
use perse_data::views::schema::{View, ViewVisibilityTypes};

// # View List Component
pub struct ViewList {}

// # View List Component
impl ViewList {
    /// # Build a link for a View
    ///
    /// ## Fields
    /// * `view` - The `View` to build the link for
    ///
    /// ## Returns
    /// * `impl IntoView` - The link as a `Fragment`
    fn build_link(view: View) -> impl IntoView {
        view! {
            <a href={format!("/{}", view.route)} title={view.title.clone()} aria-label={view.title.clone()}>
            {
                format!(
                    "/{} ({}), {}{}",
                    view.route,
                    view.title,
                    match view.visibility {
                        ViewVisibilityTypes::VisibilityPublic => "(Public)",
                        ViewVisibilityTypes::VisibilityUnlisted => "(Unlisted)",
                        ViewVisibilityTypes::VisibilityHidden => "(Hidden)",
                    },
                    match view.is_homepage {
                        true => ", (Homepage)",
                        false => "",
                    },
                )
            }
            </a>
        }
    }
}

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
                        {Self::build_link(view)}
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
                            {Self::build_link(view)}
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
            ul.view_list {
                display: flex;
                flex-direction: column;
                margin: 0;
                padding: 0;
                list-style: none;
            }

            ul.view_list li.view_list-item {
                display: flex;
                flex-direction: column;
                margin: 0.5rem 0;
                padding: 0;
                background: white;
                border: 1px dotted black;
                border-radius: 5px;
                font-size: 1.2rem;
                line-height: 1.2;
                color: #222;
            }

            ul.view_list li.view_list-item > a {
                padding: 0.5rem;
                text-decoration: none;
                color: #222;
                cursor: pointer;
            }

            ul.view_list li.view_list-item > a:hover,
            ul.view_list li.view_list-item > a:focus,
            ul.view_list li.view_list-item > a:active {
                text-decoration: underline;
            }
            ";

            PerseStyle::from_css(css)
        }}
    }
}
