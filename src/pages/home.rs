use leptos::*;
use leptos::prelude::{ElementChild, ClassAttribute, Get, Set, signal};

use crate::components::{Navbar, Footer, Button};
use crate::components::form::{FormInput, FormSelect};

use web_sys::console;

#[component]
pub fn Home() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (favorite, set_favorite) = signal(String::new());

    let options = vec![
        ("rust".to_string(), "Rust".to_string()),
        ("js".to_string(), "JavaScript".to_string()),
        ("python".to_string(), "Python".to_string()),
    ];

    view! {
        <>
            <Navbar/>
            <main class="home-main">
                <h1>"Welcome to MyLeptosApp!"</h1>
                <p>"This is your home page powered by Leptos."</p>

                <section class="form-section">
                    <h2>"Tell us about yourself"</h2>
                    <FormInput
                        value=name.get()
                        placeholder="Enter your name"
                    />
                    <FormSelect
                        value=favorite.get()
                        options=options
                        placeholder="Select your favorite language"
                        on_change=move |val| set_favorite.set(val)
                    />
                    <Button on_click=move |_: web_sys::MouseEvent| {
    console::log_1(
        &format!("Name: {}, Favorite: {}", name.get(), favorite.get()).into()
    );

}>
                        "Submit"
                    </Button>
                </section>
            </main>
            <Footer/>
        </>
    }
}