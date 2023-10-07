use leptos::*;

/// # Perse Controller

/// ## Default Controller
#[component]
pub fn Controller() -> impl IntoView {
    use leptos_meta::*;
    use leptos_router::*;
    use perse_view;

    // Load Metadata
    provide_meta_context();
    
    view! {
        // Set Document Title
        <Title text="Welcome to Perse"/>
        
        // Setup CSS with hot-reloading (id="leptos")
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        
        // Routes
        <Router>
            <main>
                <Routes>
                    // Attach Views
                    <Route path="" view=perse_view::HomePage/>
                    <Route path="/*any" view=perse_view::NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
