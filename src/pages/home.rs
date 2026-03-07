use leptos::*;
use leptos_router::*;

use crate::pages::*;

pub fn app_routes() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view=Home/>
            <Route path="/about" view=About/>
        </Routes>
    }
}