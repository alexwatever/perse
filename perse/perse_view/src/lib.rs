use leptos::*;
use leptos_router::ActionForm;

/// # Perse View

/// ## View for "Create View"
#[component]
pub fn Create() -> impl IntoView {
    // Set the API server function
    let create_view = create_server_action::<CreateView>();

    // Content
    let app_name: &str = "perse";
    view! {
        <nav id="navbar">
            <a id="brand-link" href="/" aria-label=app_name><strong>{ format!("{name}", name = app_name ) }</strong></a>
        </nav>

        <article class=move || { format!("{name}-block", name = app_name ) }>
            <header>
                <h1>"Create View"</h1>
            </header>

            <main>
                <ActionForm action=create_view>
                    <div>
                        <div>
                            <label for="title">"Title"</label>
                            <input id="title" name="title" type="text" placeholder="About Me" />
                        </div>
                        <div>
                            <label for="content_body">"Body Content"</label>
                            <textarea id="content_body" name="content_body" placeholder="It was a bright cold day in April..."></textarea>
                        </div>
                        <div>
                            <label for="content_head">"Head Content"</label>
                            <textarea id="content_head" name="content_head" placeholder=""></textarea>
                        </div>
                        <div>
                            <label for="description">"Description"</label>
                            <textarea id="description" name="description" placeholder=""></textarea>
                        </div>
                        <div>
                            <label for="content">"Keywords"</label>
                            <input id="keywords" name="keywords" type="text" placeholder="" />
                        </div>
                        <br /><br />
                    </div>

                    <div>
                        <div>
                            <label for="visibility">"Visibility"</label>
                            <select id="visibility" name="visibility">
                                <option value="visibility_public">"Public"</option>
                                <option value="visibility_hidden">"Hidden"</option>
                                <option value="visibility_unlisted">"Unlisted"</option>
                            </select>
                        </div>
                        <div>
                            <label for="route">"Route"</label>
                            <input id="route" name="route" type="text" placeholder="about-me" />

                            <div>
                                <input id="automatic_title" name="automatic_title" type="checkbox" />
                                <label for="route-checkbox">"Create from the "<strong>"Title"</strong>" automatically"</label>
                            </div>
                        </div>
                        <br /><br />
                    </div>

                    <div>
                        <button type="submit" aria-label="Save View">"Save"</button>
                    </div>
                </ActionForm>
            </main>
        </article>
    }
}

/// ### API for "Create View"
#[server(name = CreateView, prefix = "/api/v1", endpoint = "view/create")]
pub async fn create_view(
    title: Option<String>,
    content_body: Option<String>,
    content_head: Option<String>,
    description: Option<String>,
    keywords: Option<String>,
    visibility: Option<String>,
    route: Option<String>,
    automatic_title: Option<String>,
) -> Result<String, ServerFnError> {
    // print results
    let results: String = format!("{title:?}, {content_body:?}, {content_head:?}, {description:?}, {keywords:?}, {visibility:?}, {route:?}, {automatic_title:?}");

    Ok(results)
}

/// ## Not Found (404)
#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use actix_web::http::StatusCode;
        use leptos_actix::ResponseOptions;

        // Configure Response
        let resp: ResponseOptions = expect_context();
        resp.set_status(StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
