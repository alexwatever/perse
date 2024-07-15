use leptos::*;
use leptos_router::*;
use perse_data::views::schema::{CreateView, View as PerseView};
use tracing::debug;

/// # View for "Create View"
#[component]
pub fn Create() -> impl IntoView {
    // Frontend APIs
    let create_view_api = Action::<CreateViewHandler, _>::server();

    // Helper APIs
    create_server_action::<GetAllHandler>();

    // ## Create View signal

    // Signal for the server response
    let create_view_signal: Signal<Option<Result<String, ServerFnError>>> =
        Signal::derive(move || create_view_api.value().get());

    // Create an action for the server response signal
    let create_view_action = move || {
        // Get the response
        let value = {
            move || {
                create_view_signal
                    .get()
                    // Check the response
                    .map(move |response| {
                        response.unwrap_or_else(|err| {
                            // TODO: Handle errors

                            // Return a result
                            format!("There was an error: {err}")
                        })
                    })
                    // Loading state for before the first load
                    .unwrap_or_else(|| "Pending...".into())
            }
        };

        // TODO: Deserialize the response

        // Return a view
        view! {
            <p><strong>Result:</strong> {value}</p>
        }
    };

    // ## Views List signal

    // Signal for the views list
    let (views_list_signal, set_views_list_signal) = create_signal(0);

    // Create a resource for tracking the views list signal
    let views_list_signal_resource = create_resource(
        // source signal
        views_list_signal,
        // loader
        |signal_count| async move {
            get_all(signal_count)
                .await
                .map_err(|err| format!("Server returned an error: {err:?}"))
                .unwrap()
        },
    );

    // Create an action for the views list signal
    let views_list_signal_action = move || {
        // Get the signals value
        views_list_signal_resource
            .get()
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // ## Signal Effects
    Effect::new_isomorphic(move |_| {
        // Log the signals
        debug!("`create_view_signal`: {:?}", create_view_signal.get());

        // Update the All Views list
        set_views_list_signal.update(|n| *n += 1);
    });

    // ## Views

    // Loader View
    let loader = move || {
        view! { <p>"Loading..."</p> }
    };

    // Main View
    const APP_NAME: &str = "perse";
    view! {
        <nav id="navbar">
            <a id="brand-link" href="/" aria-label=APP_NAME><strong>{APP_NAME}</strong></a>
        </nav>

        <article class=move || { format!("{name}-block", name = APP_NAME ) }>
            <header><h1>"Create View"</h1></header>

            <main>
                <section>
                    <ActionForm action=create_view_api
                        on:submit=move |_event| {
                            // # TODO: Client Validation

                            // let data = CreateView::from_event(&_event).expect("to parse form data");
                            // // silly example of validation: if the todo is "nope!", nope it
                            // if data.title == "nope!" {
                            //     // ev.prevent_default() will prevent form submission
                            //     ev.prevent_default();
                            // }
                        }
                    >
                        <div>
                            <div>
                                <label for="visibility">"Visibility"</label>
                                <select id="visibility" name="data[visibility]">
                                    <option value="VisibilityPublic">"Public"</option>
                                    <option value="VisibilityHidden">"Hidden"</option>
                                    <option value="VisibilityUnlisted">"Unlisted"</option>
                                </select>
                            </div>
                            <div>
                                <label for="title">"Title"<sup> "*"</sup></label>
                                <input id="title" name="data[title]" type="text" placeholder="About Me" required />
                            </div>
                            <div>
                                <label for="content_body">"Body Content"</label>
                                <textarea id="content_body" name="data[content_body]" placeholder="It was a bright cold day in April..."></textarea>
                            </div>
                            <div>
                                <label for="content_head">"Head Content"</label>
                                <textarea id="content_head" name="data[content_head]" placeholder=""></textarea>
                            </div>
                            <div>
                                <label for="description">"Description"</label>
                                <textarea id="description" name="data[description]" placeholder=""></textarea>
                            </div>
                            <br /><br />
                        </div>

                        <div>
                            <div>
                                <label for="route">"Route"</label>
                                <input id="route" name="data[route]" type="text" placeholder="about-me" required />
                            </div>
                            <br /><br />
                        </div>

                        <div>
                            <button type="submit" aria-label="Save View">"Save"</button>
                        </div>

                        <div>
                            <Transition fallback=loader>{create_view_action}</Transition>
                        </div>
                    </ActionForm>
                </section>

                <section>
                    <header><h2>"Your Views"</h2></header>

                    <main>
                        <Transition fallback=loader>{views_list_signal_action}</Transition>
                    </main>
                </section>
            </main>
        </article>
    }
}

/// # Create a new `View` record in the database
///
/// ## Fields
/// * `data` - The data to create a new `View` record with
///
/// ## Returns
/// * `Result<String, ServerFnError>` - The successful response as a String
#[server(name = CreateViewHandler, prefix = "/api/v1", endpoint = "view/create")]
async fn create_new(data: CreateView) -> Result<String, ServerFnError> {
    use perse_data::{ApiRequests, Database, DatabaseModels};

    // Declare mutable, and run Request & Custom validation
    let mut data: CreateView = data;
    data.is_valid()?;

    // Get a database connection
    let conn = Database::get()?;

    // Determine the URL path
    data.route = CreateView::determine_url_path(&data)?;

    // TODO: Has this been declared the new home page
    // TODO: Update any current home page

    // Create and return the new View
    let value: PerseView = PerseView::create(conn, &data).await?;
    let value: String = serde_json::to_string(&value)?;

    // TODO: Create and return a `Success` schema

    Ok(value)
}

/// # Retrieve the collection of `View` records from the database
///
/// ## Fields
/// * `_signal_count` - The number of signal updates
///
/// ## Returns  
/// * `Result<Vec<View>, PerseError>` - A list of views
#[server(name = GetAllHandler, prefix = "/api/v1", endpoint = "views")]
async fn get_all(_signal_count: i32) -> Result<String, ServerFnError> {
    use perse_data::{Database, DatabaseModels};

    // Get a database connection
    let conn = Database::get()?;

    // Create and return the collection of Views
    let value: Vec<PerseView> = PerseView::get_all(conn).await?;
    let value: String = serde_json::to_string(&value)?;

    // TODO: Create and return a `Success` schema

    Ok(value)
}
