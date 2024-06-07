use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

/// # Perse View

/// ## View for "Create View"
#[component]
pub fn Create() -> impl IntoView {
    // Set the API server function
    let create_view = Action::<CreateView, _>::server();

    // Content
    let app_name: &str = "perse";
    view! {
        <nav id="navbar">
            <a id="brand-link" href="/" aria-label=app_name><strong>{ app_name.to_string() }</strong></a>
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
                            <input id="title" name="data[title]" type="text" placeholder="About Me" />
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
                            <label for="content">"Keywords"</label>
                            <input id="keywords" name="data[keywords]" type="text" placeholder="" />
                        </div>
                        <br /><br />
                    </div>

                    <div>
                        <div>
                            <label for="visibility">"Visibility"</label>
                            <select id="visibility" name="data[visibility]">
                                <option value="visibility_public">"Public"</option>
                                <option value="visibility_hidden">"Hidden"</option>
                                <option value="visibility_unlisted">"Unlisted"</option>
                            </select>
                        </div>
                        <div>
                            <label for="route">"Route"</label>
                            <input id="route" name="data[route]" type="text" placeholder="about-me" />

                            <div>
                                <input id="automatic_title" name="data[automatic_title]" type="checkbox" />
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateViewData {
    title: Option<String>,
    content_body: Option<String>,
    content_head: Option<String>,
    description: Option<String>,
    keywords: Option<String>,
    visibility: Option<String>,
    route: Option<String>,
    automatic_title: Option<String>,
}

#[server(name = CreateView, prefix = "/api/v1", endpoint = "view/create")]
pub async fn create_view(data: CreateViewData) -> Result<String, ServerFnError> {
    // Print the results
    let results: String = format!("{:?}", data);

    // TODO: Validate the request

    // TODO: Determine the URL path

    // TODO: Insert the new view

    // TODO: Return the new view

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
