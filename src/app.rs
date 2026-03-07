use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::routes::app_routes;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            {app_routes()}
        </Router>
    }
}