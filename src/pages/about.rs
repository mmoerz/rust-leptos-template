use leptos::*;
use leptos::prelude::{ElementChild, ClassAttribute};

use crate::components::{Navbar, Footer};

/// About page component
#[component]
pub fn About() -> impl IntoView {
    view! {
        <>
            <Navbar/>
            <main class="about-main">
                <h1>"About MyLeptosApp"</h1>
                <p>
                    "This application is built using Leptos, a reactive Rust framework for building fast and modern web apps."
                </p>
                <p>
                    "It demonstrates a clean structure with components, pages, forms, modals, and SSR support using Axum."
                </p>
                <p>
                    "You can extend this app with your own pages, forms, and functionality as needed."
                </p>
            </main>
            <Footer/>
        </>
    }
}