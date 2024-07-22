use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use perse_data::views::schema::{NewView, View as PerseView};

// # Components
use crate::components::{
    initial_state::InitialState, loader::Loader, navbar::NavBar, stylesheet::PerseStylesheet,
    view_list::ViewList, PerseComponent,
};
use crate::APP_NAME;

/// # View for "New View"
#[component]
pub fn New() -> impl IntoView {
    // ## Server Functions

    // Create a Frontend API for the New View request
    let new_view_api = Action::<NewViewHandler, _>::server();

    // Create a Server API for the Get Views request
    create_server_action::<GetAllHandler>();

    // ## Signals

    // Signal for the create view response
    let new_view_signal = Signal::derive(move || new_view_api.value().get());

    // Signal for the get all views response
    let (get_all_views_signal, set_all_views_signal) = create_signal(0);

    // ### New View signal

    // Resource for tracking the New View signal
    let new_view_signal_resource = create_resource(
        // Signal source
        new_view_signal,
        // Loader
        |new_view_response| async move { new_view_response },
    );

    // Action for the New View signal
    let new_view_signal_action = move || {
        new_view_signal_resource.get().and_then(|response| {
            // Refresh the Get Views View
            if response.is_some() {
                set_all_views_signal.update(|n| *n += 1);
            }

            response
        })
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
    let loader = move || Loader::build(None).into_view();

    // Main View
    view! {
        // Set Metadata
        <Title text="Create a new View | Perse" />
        <Meta name="description" content="Create a new Perse view" />
        {PerseStylesheet::build()}

        <header>
            {NavBar::build(None)}
        </header>

        <main>
            <article class=move || { format!("{APP_NAME}-block") }>
                <header><h1>"New View"</h1></header>

                <main>
                    <ActionForm action=new_view_api>
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

                        <aside>
                            <Transition fallback=loader>
                                <br />
                                // Action for the New View signal
                                {move || new_view_signal_action().map(|response| {
                                    // View for the New View result
                                    response.map(|view| view! {
                                        <header><h2>"Success"</h2></header>
                                        <main>
                                            <p>"Your new view has been created!"</p>
                                            <main>
                                                {ViewList::build(Some(view))}
                                            </main>
                                        </main>
                                    }.into_view())
                                    // View for the New View server error
                                    .unwrap_or_else(|err| view! {
                                        <header><h4>"Something went wrong"</h4></header>
                                        <main>
                                            <p>{err.to_string()}</p>
                                        </main>
                                    }.into_view())
                                })
                                // Initial state
                                .unwrap_or_else(|| {
                                    Some(InitialState::build(None)).collect_view()
                                })}
                            </Transition>
                        </aside>
                    </ActionForm>

                    <Transition fallback=loader>
                        <section>
                            <header><h2>"Your Views"</h2></header>

                            <main>
                                // Action for the Get Views signal
                                {move || views_list_signal_action().map(|response| {
                                    // View for the Get Views result
                                    response.map(|views| {
                                        ViewList::build_iter(Some(views)).into_view()
                                    })
                                    // View for the server error
                                    .unwrap_or_else(|err| {
                                        Some({ view! {
                                            <br />
                                            <section>
                                                <header><h4>"Something went wrong"</h4></header>
                                                <p>{err.to_string()}</p>
                                            </section>
                                        }}).collect_view()
                                    })
                                })
                                // Initial state
                                .unwrap_or_else(|| {
                                    Some(InitialState::build(None)).collect_view()
                                })}
                            </main>
                        </section>
                    </Transition>
                </main>
            </article>
        </main>
    }
}

/// # Create a new `View` record in the database
///
/// ## Fields
/// * `data` - The data to create a new `View` record with
///
/// ## Returns
/// * `Result<PerseView, ServerFnError>` - The successful response as a Perse View
#[server(name = NewViewHandler, prefix = "/api/v1", endpoint = "new")]
async fn new_view(data: NewView) -> Result<PerseView, ServerFnError> {
    use perse_data::{Database, PerseApiRequests, PerseDatabaseModels};

    // Re-declare the data as mutable, and validate the request
    let mut data: NewView = data;
    data.is_valid()?;

    // Get a database connection and start a transaction
    let mut transaction = Database::get()?.begin().await?;

    // Determine the URL path
    data.route = NewView::determine_url_path(&mut transaction, &data).await?;

    // Create the new View
    let data: PerseView = PerseView::create(&mut transaction, &data.into()).await?;

    // Commit the transaction and return the new View
    transaction.commit().await?;

    Ok(data)
}

/// # Retrieve the collection of `View` records from the database
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
