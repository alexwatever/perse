
/////////////
/// # Perse
/////////////
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos::logging::log;
    log!("Launching Perse...");

    // Get Leptos Configuration
    log!("Getting the Web Server configuration...");
    let conf = get_configuration(None).await.expect("Failed to load the Leptos configuration.");
    let addr = conf.leptos_options.site_addr;

    // Initialising the Database connection
    log!("Initialising the Database connection pool and checking for pending migrations...");
    let database = perse_data::Database::setup().await;

    // Start Web Server
    log!("Starting Perse!");
    use perse::modules::app::*;
    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        App::new()
            // setup the server functions
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other Assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the Favicon from /favicon.ico
            .service(favicon)
            // set the Routes
            .leptos_routes(
                leptos_options.to_owned(),
                generate_route_list(|| view! { <App/> }),
                || view! { <App/> },
            )
            // set the Application State
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(web::Data::new(PerseState {
                database,
                // env: Configuration::init(),
            }))
            //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

/// # Application State
#[cfg(feature = "ssr")]
pub struct PerseState<'a> {
    database: &'a perse_data::DatabasePool,
    // env: Configuration,
}

/// # Favicon Configuration
#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open(format!(
        "{}/favicon.ico", &leptos_options.into_inner().site_root
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use leptos_start::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move || {
        // note: for testing it may be preferrable to replace this with a
        // more specific component, although leptos_router should still work
        view! {<App/> }
    });
}
