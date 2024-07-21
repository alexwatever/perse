use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// # Views
// use perse_data::views::schema::View as PerseView;
use perse_view::views::{errors::NotFound, home::Home, new::New};

/// # Perse Controller

/// ## Default Controller
#[component]
pub fn Controller() -> impl IntoView {
    // Load Metadata
    provide_meta_context();

    // Setup Controller
    init_controller()
}

/// ## Initialise the default Controller
fn init_controller() -> impl IntoView {
    view! {
        // Routes
        <Router>
            <Routes>
                // Load the Home Page first
                <Route
                    path=""
                    view=Home
                />

                // Setup the System routes
                <Route
                    path="/p/new"
                    view=New
                    ssr=SsrMode::Async
                />

                // Look for any other routes
                <Route
                    path="/*any"
                    view=|| view! { <NotFound err=None /> }
                    ssr=SsrMode::Async
                />

                // Setup the Database routes
                // {routes.iter().map(|route| view! {
                //     <Route path={format!("/{route}")} view=New />
                // }).collect_view()}
            </Routes>
        </Router>
    }
}
