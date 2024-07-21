use leptos::*;
use leptos_meta::*;

/// ## View for Not Found (404)
#[component]
pub fn NotFound(err: Option<ServerFnError>) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use actix_web::http::StatusCode;
        use leptos_actix::ResponseOptions;

        // Configure Response
        let resp: ResponseOptions = expect_context();
        resp.set_status(StatusCode::NOT_FOUND);
    }

    view! {
        // Set Metadata
        <Title text="Not Found | Perse" />
        <Meta name="description" content="The page you are looking for does not exist." />

        <main>
            <article>
                <header>
                    <h1>"Not Found"</h1>
                </header>

                <main>
                    <p>"The page you are looking for does not exist."</p>

                    {err.map(|err| view! { <p><strong>"Error:"</strong> {err.to_string()}</p> })}
                </main>
            </article>
        </main>
    }
}
