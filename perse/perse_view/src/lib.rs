use leptos::*;

/// # Perse View

/// ## Create View
#[component]
pub fn Create() -> impl IntoView {
    // Setup Button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Create:"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
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
