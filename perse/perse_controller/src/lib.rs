use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// # Modules
use perse_data::views::schema::View as PerseView;
use perse_view::{
    components::{initial_state::InitialState, loader::Loader, PerseComponent},
    views::{errors::NotFound, home::Home, new::New},
};

/// # Perse Controller

/// ## Default Controller
#[component]
pub fn Controller() -> impl IntoView {
    // Load Metadata
    provide_meta_context();

    // Setup Router
    SetupRouter()
}

/// # Setup the Controller
///
/// ## Returns
/// * `impl IntoView` - The view for the Controller
#[component]
fn SetupRouter() -> impl IntoView {
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

                // Look for other routes in the Database
                <Route
                    path="/:route"
                    view=PathFinder
                    ssr=SsrMode::Async
                />
            </Routes>
        </Router>
    }
}

/// # PathFinder Request Parameters
#[derive(Params, Clone, Debug, PartialEq, Eq)]
struct PathFinderParams {
    route: Option<String>,
}

/// # Path Finder
///
/// ## Returns
/// * `impl IntoView` - The view for the requested route, if one exists
#[component]
fn PathFinder() -> impl IntoView {
    let query = use_params::<PathFinderParams>();
    let requested_route = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.route.clone().unwrap_or_default())
                .unwrap()
        })
    };

    // ## Server Functions

    // Create a Server API for the Get Route request
    create_server_action::<GetRouteHandler>();

    // ## Signals

    // Signal for the Get Route response
    let (get_route_signal, _set_route_signal) = create_signal(0);

    // ### Get Route signal

    // Resource for tracking the Get Route signal
    let get_route_signal_resource = create_resource(
        // Signal source
        get_route_signal,
        // Loader
        move |_signal_count| async move { get_route(requested_route()).await },
    );

    // Action for the Get Route signal
    let get_route_signal_action = move || get_route_signal_resource.get();

    // ## Views

    // Main View
    view! {
        <Transition fallback=|| Loader::build(None).into_view()>
            // Action for the Get Route signal
            {move || get_route_signal_action().map(|response| response.map(|view| view! {
                // Set Metadata
                <Title text=view.title />
                <Meta name="description" content=view.description.unwrap_or_default() />

                {view.content_body}
            }.into_view())
            // View for the server error, or if the requested route does not exist
            .unwrap_or_else(|_err_or_not_found| view! {
                <NotFound err=None />
            }.into_view()))
            // Initial state
            .unwrap_or_else(|| Some(InitialState::build(None)).collect_view())}
        </Transition>
    }
}

/// # Retrieve the requested route from the database
///
/// ## Returns  
/// * `Result<View, ServerFnError>` - The requested route, if one exists
#[server(name = GetRouteHandler, prefix = "/api/v1", endpoint = "views/lookup")]
async fn get_route(route: String) -> Result<PerseView, ServerFnError> {
    use perse_data::Database;

    // Server Validation
    if route.is_empty() {
        Err(ServerFnError::new("Route is empty".to_string()))?;
    }

    // Get the View using the requested route
    Ok(PerseView::get_by_route(Database::get()?, &route).await?)
}
