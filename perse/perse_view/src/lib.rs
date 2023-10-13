use leptos::*;

/// # Perse View

/// ## Create View
#[component]
pub fn Create() -> impl IntoView {
    // Setup Button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        // <button on:click=on_click>"Click Me: " {count}</button>

        <nav id="navbar">
            <a id="brand-link" href="/" aria-label="Perse"><strong>"Perse"</strong></a>
        </nav>

        <article id="create-view">
            <header>
                <h1>"Create View"</h1>
            </header>

            <main>
                <form>
                    <div>
                        <label for="title">"Title"</label>
                        <input id="title" type="text" placeholder="About Me" />
                    </div>

                    <div>
                        <label for="content_body">"Body Content"</label>
                        <textarea id="content_body" placeholder="It was a bright cold day in April..."></textarea>
                    </div>

                    <div>
                        <label for="content_head">"Head Content"</label>
                        <textarea id="content_head" placeholder=""></textarea>
                    </div>
                    <div>
                        <label for="description">"Description"</label>
                        <textarea id="description" placeholder=""></textarea>
                    </div>
                    <div>
                        <label for="content">"Keywords"</label>
                        <input id="keywords" type="text" placeholder="" />
                    </div>
                    <div>
                        <label for="route">"Route"</label>
                        <input id="route" type="text" placeholder="about-me" />

                        <div>
                            <input id="route-checkbox" type="checkbox" />
                            <label for="route-checkbox">"Create from the "<strong>"Title"</strong>" automatically"</label>
                        </div>
                    </div>

                    <button type="submit" aria-label="Save View">"Save"</button>
                </form>
            </main>
        </article>
    }
}


/// ## Not Found (404)
#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        // Configure Response
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
