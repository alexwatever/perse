use leptos::{view, CollectView, IntoView};

// # Modules
use super::{PerseComponent, PerseStyle};
use crate::APP_NAME;
use perse_data::views::schema::View;

// # NavBar Component
pub struct NavBar {}

impl PerseComponent for NavBar {
    type InputType = View;

    /// # Build a NavBar component using the provided data
    ///
    /// ## Fields
    /// * `value` - The `Option<Self::InputType>` to build the View component
    ///
    /// ## Returns
    /// * `impl IntoView` - The NavBar `PerseComponent` as a `Fragment`
    fn build(value: Option<Self::InputType>) -> impl IntoView {
        view! {
            {Self::style()}

            <nav id="navbar">
                <a href="/" title={APP_NAME} title={APP_NAME} aria-label={APP_NAME} id="brand">
                    <span>{APP_NAME}</span>
                </a>

                {value.map(|item| view! {
                    <a href={format!("/{}", item.route)} title={item.title.clone()} aria-label={item.title.clone()} class="navbar-item">
                        <span>{item.title}</span>
                    </a>
                })}
            </nav>
        }
    }

    /// # Build a collection of NavBar components using the provided data
    ///
    /// ## Fields
    /// * `values` - The `Option<Vec<Self::InputType>>` to build the collection of View components
    ///
    /// ## Returns
    /// * `impl IntoView` - The NavBar `PerseComponent` as a `Fragment`
    fn build_iter(values: Option<Vec<Self::InputType>>) -> impl IntoView {
        view! {
            {Self::style()}

            <nav id="navbar">
                <a href="/" title={APP_NAME} title={APP_NAME} aria-label={APP_NAME} id="brand">
                    <span>{APP_NAME}</span>
                </a>

                {values.map(|items| items.into_iter().map(|item| view! {
                    <a href={format!("/{}", item.route)} title={item.title.clone()} aria-label={item.title.clone()} class="navbar-item">
                        <span>{item.title}</span>
                    </a>
                }).collect_view())}
            </nav>
        }
    }

    /// # Implement the NavBar component style
    ///
    /// ## Returns
    /// * `Fragment` - The `PerseComponent` as a `Fragment`
    fn style() -> impl IntoView {
        view! {{
            let css: &str = "
            nav#navbar {
                display: flex;
                place-content: center;
                margin-bottom: 1rem;
              
                a#brand {
                  font-size: 2rem;
                  font-weight: 600;
                  color: #EB0C6A;
                  text-decoration: none;
                }
            }
            ";

            PerseStyle::from_css(css)
        }}
    }
}
