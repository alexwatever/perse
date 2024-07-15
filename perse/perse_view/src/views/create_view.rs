use leptos::*;
use leptos_router::*;
use perse_data::views::schema::CreateView;

/// # Load a list of views from the database
///
/// ## Fields
/// * `_count` - The number of signal updates
///
/// ## Returns  
/// * `Result<Vec<View>, PerseError>` - A list of views
async fn load_views(_count: i32) -> String {
    String::new()

    // // Get a database connection
    // match perse_data::Database::get_connection_pool() {
    //     // Get the list of views
    //     Ok(conn) => View::get_list(conn)
    //         .await
    //         .map(|value| format!("Server returned {value:?}"))
    //         .map_err(|err| format!("Server returned an error: {err:?}"))
    //         .unwrap()
    //     Err(err) => {
    //         format!("Error getting connection pool: {err:?}")
    //     }
    // }
}

/// ## View for "Create View"
#[component]
pub fn Create() -> impl IntoView {
    // Create the API server function
    let server_function = Action::<CreateViewHandler, _>::server();

    // ### Default handler

    // Signal for the server response
    let api_signal: Signal<Option<Result<String, ServerFnError>>> =
        Signal::derive(move || server_function.value().get());

    // Create an action for the server response signal
    let api_signal_action = move || {
        // Get the response
        let value = {
            move || {
                api_signal
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

    // ### Views List

    // Signal for the views list
    let (views_list_signal, set_views_list_signal) = create_signal(0);

    // Create a resource for tracking the views list signal
    let views_list_signal_resource = create_resource(
        // the first is the "source signal"
        views_list_signal,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_views(value).await },
    );

    // Create an action for the views list signal
    let views_list_signal_action = move || {
        // Get the signals value
        views_list_signal_resource
            .get()
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // ### Signal effects
    Effect::new_isomorphic(move |_| {
        // Log the signal
        logging::log!("Received signal = {:?}", api_signal.get());

        // Update the views list
        set_views_list_signal.update(|n| *n += 1);
    });

    // ### Views

    // Loader View
    let loader = move || {
        view! { <p>"Loading..."</p> }
    };

    // Content
    const APP_NAME: &str = "perse";
    view! {
        <nav id="navbar">
            <a id="brand-link" href="/" aria-label=APP_NAME><strong>{APP_NAME}</strong></a>
        </nav>

        <article class=move || { format!("{name}-block", name = APP_NAME ) }>
            <header><h1>"Create View"</h1></header>

            <main>
                <section>
                    <ActionForm action=server_function
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
                            <Transition fallback=loader>{api_signal_action}</Transition>
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

/// ## Create View API
#[server(name = CreateViewHandler, prefix = "/api/v1", endpoint = "view/create")]
pub async fn create_view(data: CreateView) -> Result<String, ServerFnError> {
    use perse_data::views::schema::View;
    use validator::Validate;

    println!("Request: {:?}", data);

    // Request validation
    data.validate()?;

    // Create and return the new view
    let view: View = View::new(data).await?;
    let view: String = serde_json::to_string(&view)?;

    // TODO: Create and return `Success` schema
    // TODO: Insert/refresh data on the frontend

    println!("Response: {:?}", view);

    Ok(view)
}
