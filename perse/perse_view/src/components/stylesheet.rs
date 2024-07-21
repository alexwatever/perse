use leptos::{view, IntoView};
use leptos_meta::Stylesheet;

// # Stylesheet Component
pub struct PerseStylesheet {}

impl PerseStylesheet {
    /// # Build a Stylesheet component
    ///
    /// ## Returns
    /// * `HtmlElement` - The Loader `PerseComponent` as a `HtmlElement`
    pub fn build() -> impl IntoView {
        const GLOBAL_CSS: &str = "/pkg/perse.css";

        view! {
            <Stylesheet id="leptos" href={GLOBAL_CSS} />
        }
    }
}
