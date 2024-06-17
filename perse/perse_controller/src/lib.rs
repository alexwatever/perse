use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// # Views
use perse_view::{create_view::Create, errors::NotFound};

/// # Perse Controller

/// ## Default Controller
#[component]
pub fn Controller() -> impl IntoView {
    // Load Metadata
    provide_meta_context();
    let global_css: View = view! { <Stylesheet id="leptos" href="/pkg/perse.css"/> };

    // Setup Controller
    if let Some(_routes) = get_user_routes() {
        // With User Routes
        init_controller(global_css)
    } else {
        // Fallback
        init_fallback_controller(global_css)
    }
}

/// ## Get User Routes
fn get_user_routes() -> Option<Vec<&'static str>> {
    let routes: Option<Vec<&'static str>> = None;
    routes
}

/// ## Initialise the default Controller
fn init_controller(global_css: View) -> Fragment {
    view! {
        // Metadata
        {global_css}
        <Title text="Welcome to Perse"/>

        // Routes
        <Router>
            <main>
                <Routes>
                    // Attach System Views
                    <Route path="/p/create/view" view=Create/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// ## Initialise the fallback Controller
fn init_fallback_controller(global_css: View) -> Fragment {
    view! {
        // Metadata
        {global_css}
        <Title text="Welcome to Perse"/>

        // Routes
        <Router>
            <main>
                <Routes>
                    // Attach System Views
                    <Route path="/*any" view=Create/>
                </Routes>
            </main>
        </Router>
    }
}
