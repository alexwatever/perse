use leptos::*;

/// ## View for Not Found (404)
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
