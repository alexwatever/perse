
/// # Perse

/// # Backend Entry Point
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos::logging::log;
    use perse_data::Database;
    console_error_panic_hook::set_once();

    // Get Leptos Configuration
    log!("Configuring Perse...");
    let conf = get_configuration(None).await.expect("Failed to load the Leptos configuration.");
    let addr = conf.leptos_options.site_addr;
    
    // Initialising the Database connection
    log!("Initialising the Database connection pool and checking for pending migrations...");
    let database = Database::setup().await;

    // Importing the Routes and Components
    log!("Importing the Routes and Components...");
    use perse_controller::*;
    
    // Start Web Server
    log!("Launching Perse!");
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
            // setup the Routes
            .leptos_routes(
                leptos_options.to_owned(),
                generate_route_list(|| view! { <Controller/> }),
                || view! { <Controller/> },
            )
            // set the Application State
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(web::Data::new(PerseState {
                _database: database,
                // env: Configuration::setup(),
            }))
            //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

/// # Frontend Entry Point
#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use leptos::*;
    use perse_controller::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    // Setup Debugging
    _ = console_log::init_with_level(tracing::log::Level::Debug);
    console_error_panic_hook::set_once();

    // Mount Controller
    leptos::mount_to_body(move || {
        view! {<Controller/> }
    });
}

/// # Standard Entry Point
#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
}

/// # Application State
#[cfg(feature = "ssr")]
pub struct PerseState<'a> {
    _database: &'a perse_data::DatabasePool,
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
