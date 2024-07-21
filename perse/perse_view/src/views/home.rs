use leptos::*;
use leptos_meta::*;

// # Modules
use super::errors::NotFound;
use perse_data::views::schema::View as PerseView;

// # Components
use crate::components::{initial_state::InitialState, loader::Loader, PerseComponent};

/// # View for the "Home"
#[component]
pub fn Home() -> impl IntoView {
    // ## Server Functions

    // Create a Server API for the Get Homepage request
    create_server_action::<GetAllHandler>();

    // ## Signals

    // Signal for the Get Homepage response
    let (get_home_signal, _set_home_signal) = create_signal(0);

    // ### Get Homepage signal

    // Resource for tracking the Get Homepage signal
    let get_home_signal_resource = create_resource(
        // Signal source
        get_home_signal,
        // Loader
        |_signal_count| async move { get_home().await },
    );

    // Action for the Get Homepage signal
    let get_home_signal_action = move || get_home_signal_resource.get();

    // ## Views

    // Main View
    view! {
        <Transition fallback=|| Loader::build(None).into_view()>
            // Action for the Get Homepage signal
            {move || get_home_signal_action().map(|response| response.map(|view| view! {
                    // Set Metadata
                    <Title text=view.title />
                    <Meta name="description" content=view.description.unwrap_or_default() />

                    {view.content_body}
                }
                .into_view())
                // View for the server error
                .unwrap_or_else(|err| view! { <NotFound err=Some(err) /> }.into_view()
            ))
            // Initial state
            .unwrap_or_else(|| Some(InitialState::build(None)).collect_view())
            }
        </Transition>
    }
}

/// # Retrieve the Homepage `View` from the database
///
/// ## Returns  
/// * `Result<View, ServerFnError>` - A list of views
#[server(name = GetAllHandler, prefix = "/api/v1", endpoint = "views/home")]
async fn get_home() -> Result<PerseView, ServerFnError> {
    use perse_data::Database;

    // Get the HomePage View
    Ok(PerseView::get_homepage(Database::get()?).await?)
}
