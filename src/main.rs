use axum::Router;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

use my_leptos_app::app::*;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;

    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App);

    let addr = leptos_options.site_addr;
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
