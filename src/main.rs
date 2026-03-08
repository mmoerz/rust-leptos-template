// shut the fuck up about axum 7.0, I'm using 0.8 because official axum-start contains it
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use rust_leptos_template::app::*;
    use tokio::net::TcpListener;
    use tracing::{info, Level};
    use tracing_subscriber;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Initializing Leptos server");

    // Load Leptos configuration (SSR)
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // Generate the list of routes for the app
    let routes = generate_route_list(App);

    // Build the Axum router with Leptos routes
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    info!("Starting server at http://{}", addr);
    let listener: TcpListener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener,app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
