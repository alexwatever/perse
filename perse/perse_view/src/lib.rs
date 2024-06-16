use leptos::*;
use leptos_router::*;
use perse_data::views::schema::CreateView;

// # Perse View

/// ## View for "Create View"
#[component]
pub fn Create() -> impl IntoView {
    // Set the API server function
    let create_view = Action::<CreateViewHandler, _>::server();

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
                            <label for="visibility">"Visibility"</label>
                            <select id="visibility" name="data[visibility]">
                                <option value="visibility_public">"Public"</option>
                                <option value="visibility_hidden">"Hidden"</option>
                                <option value="visibility_unlisted">"Unlisted"</option>
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
                            <input id="route" name="data[route]" type="text" placeholder="about-me" />

                            <div>
                                <input id="automatic_route" name="data[automatic_route]" type="checkbox" />
                                <label for="route-checkbox">"Create from the "<strong>"Route"</strong>" automatically"</label>
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

/// ## Create View API
#[server(name = CreateViewHandler, prefix = "/api/v1", endpoint = "view/create")]
pub async fn create_view(data: CreateView) -> Result<String, ServerFnError> {
    use perse_data::{views::schema::View, ApiRequests};
    use validator::Validate;

    println!("Request: {:?}", data);

    // Request validation
    data.validate()?;

    // Custom validation
    data.is_valid()?;

    // Create and return the new view
    let view: View = View::new(data).await?;
    let view: String = serde_json::to_string(&view)?;

    Ok(view)
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
