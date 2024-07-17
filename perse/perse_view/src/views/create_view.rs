use leptos::*;
use leptos_router::*;
use perse_data::views::schema::{CreateView, View as PerseView};

// # Components
use crate::views::components::{loader::Loader, PerseComponent};

/// # View for "Create View"
#[component]
pub fn Create() -> impl IntoView {
    // ## Server Functions

    // Create a Frontend API for the Create View request
    let create_view_api = Action::<CreateViewHandler, _>::server();

    // Create a Server API for the Get Views request
    create_server_action::<GetAllHandler>();

    // ## Signals

    // Signal for the create view response
    let create_view_signal = Signal::derive(move || create_view_api.value().get());

    // Signal for the get all views response
    let (get_all_views_signal, set_all_views_signal) = create_signal(0);

    // ### Create View signal

    // Resource for tracking the Create View signal
    let create_view_signal_resource = create_resource(
        // Signal source
        create_view_signal,
        // Loader
        |create_view_response| async move { create_view_response },
    );

    // Action for the Create View signal
    let create_view_signal_action = move || {
        create_view_signal_resource.get().and_then(|response| {
            // Refresh the Get Views View
            if response.is_some() {
                set_all_views_signal.update(|n| *n += 1);
            }

            response
        })
    };

    // TODO: Validate the Create View client request
    let create_view_validation = move |_event| {
        // let data = CreateView::from_event(&_event).expect("to parse form data");
        // if data.title == "nope!" {
        //     // ev.prevent_default() will prevent form submission
        //     ev.prevent_default();
        // }
    };

    // ### Get Views signal

    // Resource for tracking the Get Views signal
    let get_all_views_signal_resource = create_resource(
        // Signal source
        get_all_views_signal,
        // Loader
        |_signal_count| async move { get_all_views().await },
    );

    // Action for the Get Views signal
    let views_list_signal_action = move || get_all_views_signal_resource.get();

    // ## Views

    // Components
    let loader = Loader::get;

    // Main View
    const APP_NAME: &str = "perse";
    view! {
        <nav id="navbar">
            <a id="brand-link" href="/" aria-label=APP_NAME><strong>{APP_NAME}</strong></a>
        </nav>

        <article class=move || { format!("{APP_NAME}-block") }>
            <header><h1>"Create View"</h1></header>

            <main>
                <section>
                    <ActionForm action=create_view_api on:submit=create_view_validation>
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
                            <div>
                                <label for="route">"Route"</label>
                                <input id="route" name="data[route]" type="text" placeholder="about-me" required />
                            </div>
                            <br />
                            <div>
                                <label for="is_homepage">"Is this the new homepage?"</label>
                                <input id="is_homepage" name="data[is_homepage]" type="checkbox" />
                            </div>
                            <br />
                        </div>

                        <div>
                            <button type="submit" aria-label="Save View">"Save"</button>
                            <br />
                        </div>

                        <div>
                            <Transition fallback=loader>
                                <br />
                                <section>
                                    {move || {
                                        // Action for the Create View signal
                                        create_view_signal_action()
                                            .map(|response| {
                                                // View for the Create View result
                                                response
                                                    .map(|view| {
                                                        Some(view! {
                                                            <header><h2>"Success"</h2></header>
                                                            <main>
                                                                <p>"Your new view has been created!"</p>
                                                                <p>
                                                                    <a href={format!("/{}", view.route)} title={view.title.clone()}>
                                                                        {format!("/{} ({})", view.route, view.title)}
                                                                    </a>
                                                                </p>
                                                            </main>
                                                        })
                                                        .collect_view()
                                                    })
                                                    // View for the Create View server error
                                                    .unwrap_or_else(|err| {
                                                        Some(view! {
                                                            <header><h2>"Something went wrong"</h2></header>
                                                            <p>{err.to_string()}</p>
                                                        })
                                                        .collect_view()
                                                    })
                                            })
                                            // Initial loading state
                                            .unwrap_or_else(|| {
                                                Some(view! {}).collect_view()
                                            })
                                    }}
                                </section>
                            </Transition>
                        </div>
                    </ActionForm>
                </section>

                <section>
                    <br />
                    <header><h2>"Your Views"</h2></header>

                    <main>
                        <Transition fallback=loader>
                            {move || {
                                // Action for the Get Views signal
                                views_list_signal_action()
                                    .map(|response| {
                                        response
                                            // View for the Get Views result
                                            .map(|views| {
                                                views.into_iter()
                                                    // Iterate through Views
                                                    .map(|view| {
                                                        view! {
                                                            <li>
                                                                <a href={format!("/{}", view.route)} title={view.title.clone()}>
                                                                    {format!("/{} ({})", view.route, view.title)}
                                                                </a>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()
                                            })
                                            // View for the server error
                                            .unwrap_or_else(|err| {
                                                Some({
                                                    view! {
                                                        <br />
                                                        <section>
                                                            <header><h2>"Something went wrong"</h2></header>
                                                            <p>{err.to_string()}</p>
                                                        </section>
                                                    }
                                                })
                                                .collect_view()
                                            })
                                        })
                                        // Initial loading state
                                        .unwrap_or_else(|| {
                                            Some({
                                                view! {
                                                    <br />
                                                    <section>
                                                        <main>
                                                            <p>"..."</p>
                                                        </main>
                                                    </section>
                                                }
                                            })
                                            .collect_view()
                                        })
                                }}
                        </Transition>
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
/// * `Result<PerseView, ServerFnError>` - The successful response as a Perse View
#[server(name = CreateViewHandler, prefix = "/api/v1", endpoint = "view/create")]
async fn create_view(data: CreateView) -> Result<PerseView, ServerFnError> {
    use perse_data::{Database, PerseApiRequests, PerseDatabaseModels};

    // Declare mutable, and run Request & Custom validation
    let mut data: CreateView = data;
    data.is_valid()?;

    // Get a database connection and start a transaction
    let mut transaction = Database::get()?.begin().await?;

    // Determine the URL path
    data.route = CreateView::determine_url_path(&mut transaction, &data).await?;

    // Create the new View
    let data: PerseView = PerseView::create(&mut transaction, &data.into()).await?;
    // let data: String = serde_json::to_string(&data)?;

    // Commit the transaction and return the new View
    transaction.commit().await?;

    Ok(data)
}

/// # Retrieve the collection of `View` records from the database
///
/// ## Fields
/// * `_signal_count` - The number of signal updates
///
/// ## Returns  
/// * `Result<Vec<View>, ServerFnError>` - A list of views
#[server(name = GetAllHandler, prefix = "/api/v1", endpoint = "views")]
async fn get_all_views() -> Result<Vec<PerseView>, ServerFnError> {
    use perse_data::{Database, PerseDatabaseModels};

    // Get a database connection
    let conn = Database::get()?;

    // Create and return the collection of Views
    let data = PerseView::get_all(conn).await?;

    Ok(data)
}
