use leptos::*;
use leptos_router::*;
use perse_data::views::schema::CreateViewRequest;

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
pub async fn create_view(data: CreateViewRequest) -> Result<String, ServerFnError> {
    use perse_data::views::schema::View;
    use validator::Validate;

    // Request validation
    data.validate()?;

    // Create and return the new view
    let view: View = View::new(data.try_into()?).await?;
    let view: String = serde_json::to_string(&view)?;

    Ok(view)
}
