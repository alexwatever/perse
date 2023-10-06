use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// # Perse Controller

/// ## Default Controller
#[component]
pub fn Controller() -> impl IntoView {
    provide_meta_context();

    view! {
        // Set Document Title
        <Title text="Welcome to Perse"/>

        // Setup CSS with hot-reloading (id="leptos")
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // Setup Routes
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// ## Home Page
#[component]
fn HomePage() -> impl IntoView {
    // Setup Button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// ## Not Found (404)
#[component]
fn NotFound() -> impl IntoView {
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
